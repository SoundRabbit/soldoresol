use crate::{resource, JSZip};
use js_sys::Promise;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use xmltree::{Element, XMLNode};

pub use data::Data;

pub struct Character {
    pub data: Data,
    zip: JSZip,
}

impl Character {
    fn from_str(text: &str, zip: JSZip) -> Option<Self> {
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

    pub async fn from_blob(blob: &web_sys::Blob) -> Option<Self> {
        let zip = JSZip::new();
        let zip = if let Some(zip) = JsFuture::from(zip.load_async(blob))
            .await
            .ok()
            .and_then(|zip| zip.dyn_into::<JSZip>().ok())
        {
            zip
        } else {
            return None;
        };
        let file = if let Some(file) = zip.file("data.xml").or_else(|| zip.file("data0.xml")) {
            file
        } else {
            return None;
        };
        let xml_text = if let Some(xml_text) = JsFuture::from(file.load_async("text"))
            .await
            .ok()
            .and_then(|x| x.as_string())
        {
            xml_text
        } else {
            return None;
        };
        Self::from_str(&xml_text, zip)
    }

    pub async fn texture(&self) -> Option<resource::Data> {
        let texture_id = if let Some(data::Value::Text(x)) = self.data.find("imageIdentifier") {
            x
        } else {
            return None;
        };
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
                    type_ = format!("image/{}", suffix);
                    break;
                }
            }
        }

        let file = if let Some(file) = file {
            file
        } else {
            return None;
        };

        let buf = if let Some(buf) = JsFuture::from(file.load_async("arraybuffer"))
            .await
            .ok()
            .and_then(|x| x.dyn_into::<js_sys::ArrayBuffer>().ok())
        {
            buf
        } else {
            return None;
        };

        let data: JsValue = object! {
            type: type_,
            payload: buf
        }
        .into();

        resource::Data::unpack(data).await
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
