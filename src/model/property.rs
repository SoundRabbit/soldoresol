use crate::{random_id, JsObject};
use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast};

pub enum PropertyValue {
    None,
    Num(f64),
    Str(String),
    Children(Vec<Property>),
}

pub struct Property {
    id: u128,
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
            id: random_id::u128val(),
            name: "".into(),
            value: PropertyValue::None,
        }
    }

    pub fn new_as_num() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            value: PropertyValue::Num(0.0),
        }
    }

    pub fn new_as_str() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            value: PropertyValue::Str("".into()),
        }
    }

    pub fn new_as_parent() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            value: PropertyValue::Children(vec![]),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn push(&mut self, prop: Property) {
        match &mut self.value {
            PropertyValue::Children(children) => {
                children.push(prop);
            }
            PropertyValue::None => {
                self.value = PropertyValue::Children(vec![prop]);
            }
            _ => {}
        }
    }

    pub fn get(&self, id: &u128) -> Option<&Self> {
        if self.id == *id {
            Some(self)
        } else if let PropertyValue::Children(children) = &self.value {
            children.iter().find_map(|x| x.get(id))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: &u128) -> Option<&mut Self> {
        if self.id == *id {
            Some(self)
        } else if let PropertyValue::Children(children) = &mut self.value {
            children.iter_mut().find_map(|x| x.get_mut(id))
        } else {
            None
        }
    }

    pub fn id(&self) -> &u128 {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_value(&mut self, value: PropertyValue) {
        self.value = value;
    }

    pub fn value(&self) -> &PropertyValue {
        &self.value
    }

    pub fn as_object(&self) -> JsObject {
        object! {
            id: self.id.to_string(),
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
        let id = object
            .get("id")
            .and_then(|x| x.as_string())
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        let name = object
            .get("name")
            .and_then(|x| x.as_string())
            .unwrap_or("".into());
        let value = object
            .get("value")
            .map(|x| PropertyValue::from(x))
            .unwrap_or(PropertyValue::None);
        Self { id, name, value }
    }
}
