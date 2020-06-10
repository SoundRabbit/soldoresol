use crate::JsObject;
use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};

pub enum PropertyValue {
    None,
    Num(f64),
    Str(String),
    Children(Vec<Property>),
}

pub struct Property {
    name: String,
    value: PropertyValue,
}

impl PropertyValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Num(..) => "Num",
            Self::Str(..) => "Str",
            Self::Children(..) => "Children",
        }
    }

    pub fn as_object(&self) -> JsObject {
        let payload: JsValue = match self {
            Self::None => JsValue::undefined(),
            Self::Num(x) => JsValue::from(*x),
            Self::Str(x) => JsValue::from(x),
            Self::Children(children) => {
                let payload = Array::new();
                for child in children {
                    payload.push(child.as_object().as_ref());
                }
                payload.into()
            }
        };
        object! {
            type: self.type_name(),
            payload: payload
        }
    }
}

impl Property {
    pub fn new_as_none() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::None,
        }
    }

    pub fn new_as_num() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::Num(0.0),
        }
    }

    pub fn new_as_str() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::Str("".into()),
        }
    }

    pub fn new_as_parent() -> Self {
        Self {
            name: "".into(),
            value: PropertyValue::Children(vec![]),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&Self> {
        if let PropertyValue::Children(children) = &self.value {
            children.get(idx)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Self> {
        if let PropertyValue::Children(children) = &mut self.value {
            children.get_mut(idx)
        } else {
            None
        }
    }

    pub fn get_with_address(&self, address: &Vec<usize>) -> Option<&Self> {
        self.impl_get_with_address(address, 0)
    }

    fn impl_get_with_address(&self, address: &Vec<usize>, idx: usize) -> Option<&Self> {
        let child_pos = address[idx];
        if idx < address.len() - 1 {
            if let Some(child) = self.get(child_pos) {
                child.impl_get_with_address(address, idx + 1)
            } else {
                None
            }
        } else {
            self.get(child_pos)
        }
    }

    pub fn get_mut_with_address(&mut self, address: &Vec<usize>) -> Option<&mut Self> {
        self.impl_get_mut_with_address(address, 0)
    }

    fn impl_get_mut_with_address(&mut self, address: &Vec<usize>, idx: usize) -> Option<&mut Self> {
        let child_pos = address[idx];
        if idx < address.len() - 1 {
            if let Some(child) = self.get_mut(child_pos) {
                child.impl_get_mut_with_address(address, idx + 1)
            } else {
                None
            }
        } else {
            self.get_mut(child_pos)
        }
    }

    pub fn as_object(&self) -> JsObject {
        object! {
            name: &self.name,
            value: self.value.as_object()
        }
    }
}

impl From<JsObject> for PropertyValue {
    fn from(object: JsObject) -> Self {
        if let Some(type_name) = object.get("type").and_then(|x| x.as_string()) {
            match type_name.as_str() {
                "Num" => object
                    .get("payload")
                    .and_then(|x| x.as_f64())
                    .map(|x| Self::Num(x))
                    .unwrap_or(Self::None),
                "Str" => object
                    .get("payload")
                    .and_then(|x| x.as_string())
                    .map(|x| Self::Str(x))
                    .unwrap_or(Self::None),
                "Children" => object
                    .get("payload")
                    .map(|x| Array::from(&x))
                    .map(|x| {
                        Self::Children(
                            x.to_vec()
                                .into_iter()
                                .filter_map(|child| {
                                    child
                                        .dyn_into::<JsObject>()
                                        .ok()
                                        .map(|object| Property::from(object))
                                })
                                .collect(),
                        )
                    })
                    .unwrap_or(Self::None),
                _ => Self::None,
            }
        } else {
            Self::None
        }
    }
}

impl From<JsObject> for Property {
    fn from(object: JsObject) -> Self {
        let name = object
            .get("name")
            .and_then(|x| x.as_string())
            .unwrap_or("".into());
        let value = object
            .get("value")
            .map(|x| PropertyValue::from(x))
            .unwrap_or(PropertyValue::None);
        Self { name, value }
    }
}
