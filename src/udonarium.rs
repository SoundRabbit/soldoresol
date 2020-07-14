use crate::{resource, JSZip, Promise};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use xmltree::{Element, XMLNode};

pub use data::Data;

pub struct Character {
    pub data: Data,
    zip: Rc<JSZip>,
}

impl Character {
    fn from_str(text: &str, zip: Rc<JSZip>) -> Option<Self> {
        Element::parse(text.as_bytes()).ok().and_then(|element| {
            if element.name == "character" {
                let mut data = None;
                for node in element.children {
                    match node {
                        XMLNode::Element(element) => {
                            if element.name == "data" {
                                data = Data::from_element(element);
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                data.map(|data| Self { data, zip })
            } else {
                None
            }
        })
    }

    pub fn from_blob(blob: &web_sys::Blob) -> Promise<Self> {
        let zip = JSZip::new();
        let promise = zip.load_async(blob);
        Promise::new(move |resolve| {
            let resolve = Rc::new(RefCell::new(Some(resolve)));

            let on_load = Closure::wrap(Box::new({
                let resolve = Rc::clone(&resolve);
                move |zip: JsValue| {
                    if let Some((zip, file)) = zip
                        .dyn_into::<JSZip>()
                        .ok()
                        .and_then(|zip| zip.file("data.xml").map(|file| (zip, file)))
                    {
                        let on_load = Closure::wrap(Box::new({
                            let resolve = Rc::clone(&resolve);
                            let zip = Rc::new(zip);
                            move |xml: JsValue| {
                                if let Some(resolve) = resolve.borrow_mut().take() {
                                    if let Some(xml_text) = xml.as_string() {
                                        resolve(Self::from_str(&xml_text, Rc::clone(&zip)));
                                    } else {
                                        resolve(None);
                                    }
                                }
                            }
                        })
                            as Box<dyn FnMut(_)>);
                        let _ = file.load_async("text").then(&on_load);
                        on_load.forget();
                    } else if let Some(resolve) = resolve.borrow_mut().take() {
                        resolve(None);
                    }
                }
            }) as Box<dyn FnMut(_)>);

            let _ = promise.then(&on_load);

            on_load.forget();
        })
    }

    pub fn texture(&self) -> Promise<resource::Data> {
        let texture_id = match self.data.find("imageIdentifier") {
            Some(data::Value::Text(x)) => x.clone(),
            _ => {
                return Promise::new(|r| r(None));
            }
        };

        crate::debug::log_1(texture_id.as_str());

        let files = js_sys::Object::keys(&self.zip.files());
        let mut file = None;
        let mut type_ = String::new();

        for i in 0..files.length() {
            if let Some(file_name) = files.get(i).as_string() {
                let fname: Vec<&str> = file_name.split('.').collect();
                let prefix = fname.first().map(|x| x as &str).unwrap_or("");
                let suffix = fname.last().map(|x| x as &str).unwrap_or("");
                if texture_id == prefix {
                    file = self.zip.file(&file_name);
                    type_ = type_ + "image/" + suffix;
                    break;
                }
            }
        }

        if file.is_none() {
            return Promise::new(|r| r(None));
        }

        let file = file.unwrap();

        Promise::new(move |resolve| {
            let resolve = Rc::new(RefCell::new(Some(resolve)));

            let on_load = Closure::wrap(Box::new(move |buf: JsValue| {
                if let Ok(buf) = buf.dyn_into::<js_sys::ArrayBuffer>() {
                    let data = object! {
                        type: type_.clone(),
                        payload: buf
                    };
                    let data: js_sys::Object = data.into();
                    let data: JsValue = data.into();
                    resource::Data::unpack(data).then({
                        let resolve = Rc::clone(&resolve);
                        move |texture| {
                            if let Some(resolve) = resolve.borrow_mut().take() {
                                resolve(texture);
                            }
                        }
                    });
                } else if let Some(resolve) = resolve.borrow_mut().take() {
                    resolve(None);
                }
            }) as Box<dyn FnMut(_)>);

            file.load_async("arraybuffer").then(&on_load);
            on_load.forget();
        })
    }
}

pub mod data {
    use xmltree::{Element, XMLNode};

    #[derive(Clone, Debug)]
    pub enum Value {
        Children(Vec<Data>),
        Text(String),
        None,
    }

    #[derive(Clone, Debug)]
    pub struct Data {
        pub name: String,
        pub type_: String,
        pub value: Value,
    }

    impl Value {
        pub fn from_node(nodes: Vec<XMLNode>) -> Option<Self> {
            let mut text = None;
            let mut children = vec![];
            for node in nodes {
                match node {
                    XMLNode::Element(element) => {
                        if let Some(data) = Data::from_element(element) {
                            children.push(data);
                        }
                    }
                    XMLNode::Text(suf) => {
                        text = Some(text.map(|pre| pre + suf.as_str()).unwrap_or(suf));
                    }
                    _ => (),
                }
            }
            if text.is_some() && !children.is_empty() {
                None
            } else if let Some(text) = text {
                Some(Value::Text(text))
            } else if !children.is_empty() {
                Some(Value::Children(children))
            } else {
                Some(Value::None)
            }
        }
    }

    impl Data {
        pub fn from_str(text: &str) -> Option<Self> {
            Element::parse(text.as_bytes())
                .ok()
                .and_then(Self::from_element)
        }

        pub fn from_element(mut element: Element) -> Option<Self> {
            if element.name == "data" {
                let name = element.attributes.remove("name").unwrap_or(String::new());
                let type_ = element.attributes.remove("type").unwrap_or(String::new());
                Value::from_node(element.children).map(|value| Self { name, type_, value })
            } else {
                None
            }
        }

        pub fn find(&self, name: &str) -> Option<&Value> {
            if self.name == name {
                Some(&self.value)
            } else if let Value::Children(children) = &self.value {
                children.iter().find_map(|child| child.find(name))
            } else {
                None
            }
        }
    }
}
