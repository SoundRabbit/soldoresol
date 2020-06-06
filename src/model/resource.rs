use crate::JsObject;
use js_sys::ArrayBuffer;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;

pub enum Data {
    Image(Rc<web_sys::HtmlImageElement>, Rc<web_sys::Blob>),
}

pub struct Resource {
    data: HashMap<u128, Data>,
}

pub struct ResourceData {
    payload: HashMap<u128, Rc<web_sys::Blob>>,
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
            Data::Image(image, ..) => Some(Rc::clone(image)),
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
                        Data::Image(.., blob) => {
                            resource_data.insert(*key, Rc::clone(blob));
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

impl ResourceData {
    pub fn as_object(&self) -> JsObject {
        let obj = object! {};

        for (id, data) in &self.payload {
            obj.set(
                &id.to_string(),
                object! {
                    type: data.type_().as_str(),
                    payload: data.as_ref()
                }
                .as_ref(),
            );
        }

        obj
    }
}

impl Into<HashMap<u128, Rc<web_sys::Blob>>> for ResourceData {
    fn into(self) -> HashMap<u128, Rc<web_sys::Blob>> {
        self.payload
    }
}

impl From<JsObject> for ResourceData {
    fn from(obj: JsObject) -> Self {
        use js_sys::Object;

        let mut payload = HashMap::new();

        for key in Object::keys(&obj).values() {
            if let Some(key) = key
                .ok()
                .and_then(|key| key.as_string())
                .and_then(|key| key.parse::<u128>().ok())
            {
                if let Some(data) = obj.get(&key.to_string()) {
                    if let (Some(data_type), Some(data)) = (
                        data.get("type").and_then(|t| t.as_string()),
                        data.get("payload")
                            .and_then(|p| p.dyn_into::<ArrayBuffer>().ok()),
                    ) {
                        if let Ok(data) = web_sys::Blob::new_with_buffer_source_sequence_and_options(
                            array![&data].as_ref(),
                            web_sys::BlobPropertyBag::new().type_(data_type.as_str()),
                        ) {
                            use wasm_bindgen::prelude::*;
                            web_sys::console::log_1(&JsValue::from(key.to_string().as_str()));
                            payload.insert(key, Rc::new(data));
                        }
                    }
                }
            }
        }

        Self { payload }
    }
}

impl From<HashMap<u128, Rc<web_sys::Blob>>> for ResourceData {
    fn from(payload: HashMap<u128, Rc<web_sys::Blob>>) -> Self {
        Self { payload }
    }
}

impl From<(u128, Rc<web_sys::Blob>)> for ResourceData {
    fn from(id_data_pair: (u128, Rc<web_sys::Blob>)) -> Self {
        let mut payload = HashMap::new();
        payload.insert(id_data_pair.0, id_data_pair.1);
        Self { payload }
    }
}
