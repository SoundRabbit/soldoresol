use crate::{random_id, JsObject};
use js_sys::Array;
use sainome::Ref;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub enum PropertyValue {
    None,
    Num(f64),
    Str(String),
    Children(Vec<Property>),
}

#[derive(Clone)]
pub struct Property {
    id: u128,
    name: String,
    is_selected_to_show: bool,
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

    pub fn as_option_string(&self) -> Option<String> {
        match &self {
            Self::None => None,
            Self::Children(..) => None,
            Self::Num(x) => Some(x.to_string()),
            Self::Str(x) => Some(x.to_string()),
        }
    }
}

impl Property {
    pub fn new_as_none() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            is_selected_to_show: false,
            value: PropertyValue::None,
        }
    }

    pub fn new_as_num() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            is_selected_to_show: false,
            value: PropertyValue::Num(0.0),
        }
    }

    pub fn new_as_str() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            is_selected_to_show: false,
            value: PropertyValue::Str("".into()),
        }
    }

    pub fn new_as_parent() -> Self {
        Self {
            id: random_id::u128val(),
            name: "".into(),
            is_selected_to_show: false,
            value: PropertyValue::Children(vec![]),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_selected_to_show(mut self) -> Self {
        self.is_selected_to_show = true;
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

    pub fn selecteds(&self) -> Vec<&Self> {
        if self.is_selected_to_show {
            vec![self]
        } else if let PropertyValue::Children(children) = &self.value {
            let mut selecteds = vec![];
            for child in children {
                selecteds.append(&mut child.selecteds());
            }
            selecteds
        } else {
            vec![]
        }
    }

    pub fn remove(&mut self, id: u128) {
        if let PropertyValue::Children(children) = &mut self.value {
            let remove_position = children.iter().position(|x| {
                web_sys::console::log_2(
                    &JsValue::from(x.id.to_string()),
                    &JsValue::from(id.to_string()),
                );
                *x.id() == id
            });
            if let Some(remove_position) = remove_position {
                children.remove(remove_position);
            } else {
                for i in 0..children.len() {
                    children[i].remove(id);
                }
            }
        }
    }

    pub fn id(&self) -> &u128 {
        &self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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

    pub fn is_selected_to_show(&self) -> bool {
        self.is_selected_to_show
    }

    pub fn set_is_selected_to_show(&mut self, is_selected_to_show: bool) {
        self.is_selected_to_show = is_selected_to_show;
    }

    pub fn as_object(&self) -> JsObject {
        object! {
            id: self.id.to_string(),
            name: &self.name,
            is_selected_to_show: self.is_selected_to_show,
            value: self.value.as_object()
        }
    }

    pub fn as_sainome_ref(&self) -> Ref {
        let mut r = Ref::new(self.value.as_option_string());
        if let PropertyValue::Children(children) = &self.value {
            for child in children {
                let name = child.name.to_string();
                r.insert(name, child.as_sainome_ref());
            }
        }
        r
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
        let is_selected_to_show = object
            .get("is_selected_to_show")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        Self {
            id,
            name,
            is_selected_to_show,
            value,
        }
    }
}
