use crate::{JSZip, Promise};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use xmltree::{Element, XMLNode};

pub use data::Data;

pub struct Character {
    data: Data,
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

    pub fn data(&self) -> &Data {
        &self.data
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
        name: String,
        type_: String,
        value: Value,
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
