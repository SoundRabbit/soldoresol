use super::{Block, BlockId, Field};
use crate::Promise;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum Value {
    None,
    Num(f64),
    Str(String),
    Children(Vec<BlockId>),
}

#[derive(Clone)]
pub struct Property {
    name: String,
    is_selected: bool,
    value: Value,
}

impl Value {
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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_selected: false,
            value: Value::None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value
    }
}

impl Block for Property {
    fn pack(&self) -> Promise<JsValue> {
        let value = match &self.value {
            Value::Str(a) => object! {
                type: "string",
                payload: a
            },
            Value::Num(a) => object! {
                type: "number",
                payload: *a
            },
            Value::Children(list) => {
                let children = js_sys::Array::new();
                for child in list {
                    children.push(&JsValue::from(child.to_string()));
                }
                object! {
                    type: "children",
                    payload: children
                }
            }
            Value::None => object! {
                type: "none",
                payload: JsValue::undefined()
            },
        };
        let value: js_sys::Object = value.into();
        let value: JsValue = value.into();

        let data = object! {
            name: self.name(),
            is_selected: self.is_selected,
            value: value
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();

        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}

impl Field {
    pub fn sainome_ref_of(&self, block_id: &BlockId) -> Option<sainome::Ref> {
        if let Some(prop) = self.get::<Property>(block_id) {
            let mut r = sainome::Ref::new(prop.value.as_option_string());
            if let Value::Children(children) = &prop.value {
                let children = self.listed::<Property>(children.iter().collect());
                for (child_id, child) in children {
                    let name = child.name.to_string();
                    r.insert(name, self.sainome_ref_of(&child_id).unwrap());
                }
            }
            Some(r)
        } else {
            None
        }
    }
}
