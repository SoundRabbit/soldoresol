use crate::JsObject;
use js_sys::JsString;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;

pub enum Data {
    Image(Rc<web_sys::HtmlImageElement>),
}

pub struct Resource {
    data: HashMap<u128, Data>,
}

pub enum DataString {
    Image(JsString),
}

pub struct ResourceData {
    payload: HashMap<u128, DataString>,
}

#[allow(dead_code)]
impl Resource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, data_id: u128, data: Data) {
        self.data.insert(data_id, data);
    }

    pub fn get(&self, data_id: u128) -> Option<&Data> {
        self.data.get(&data_id)
    }

    pub fn get_as_image(&self, data_id: &u128) -> Option<Rc<web_sys::HtmlImageElement>> {
        self.data.get(data_id).and_then(|data| match data {
            Data::Image(image) => Some(Rc::clone(image)),
        })
    }

    pub fn get_images(&self) -> Vec<(u128, Rc<web_sys::HtmlImageElement>)> {
        self.data
            .iter()
            .filter_map(|(data_id, _)| self.get_as_image(data_id).map(|image| (*data_id, image)))
            .collect()
    }

    pub fn to_data(&self) -> ResourceData {
        self.to_data_with_n_and_stride(0, 1)
    }

    pub fn to_data_with_n_and_stride(&self, n: usize, stride: usize) -> ResourceData {
        let mut keys = vec![];

        for key in self.data.keys() {
            keys.push(key);
        }

        keys.sort();

        let mut resource_data = HashMap::new();
        let mut i = 0;

        for key in keys {
            if i % stride == n {
                if let Some(data) = self.data.get(key) {
                    match data {
                        Data::Image(image) => {
                            resource_data.insert(
                                *key,
                                DataString::Image(
                                    image
                                        .dyn_ref::<JsObject>()
                                        .unwrap()
                                        .get("src")
                                        .unwrap()
                                        .dyn_into::<JsString>()
                                        .ok()
                                        .unwrap(),
                                ),
                            );
                        }
                    }
                }
            }
            i += 1;
        }

        ResourceData {
            payload: resource_data,
        }
    }
}

impl DataString {
    pub fn as_object(&self) -> JsObject {
        match self {
            Self::Image(data) => object! {
                type: "Image",
                payload: data
            },
        }
    }
}

impl ResourceData {
    pub fn as_object(&self) -> JsObject {
        let obj = object! {};

        for (id, data) in &self.payload {
            obj.set(&id.to_string(), &data.as_object());
        }

        obj
    }
}

impl Into<HashMap<u128, DataString>> for ResourceData {
    fn into(self) -> HashMap<u128, DataString> {
        self.payload
    }
}
