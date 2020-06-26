use super::{Block, BlockId, Field};
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
    is_selected_to_show: bool,
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
            is_selected_to_show: false,
            value: Value::None,
        }
    }
}

impl Block for Property {
    fn pack(&self, resolve: impl FnOnce(JsValue)) {}
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>)) {}
}

impl Field {
    pub fn sainome_ref_of(&self, block_id: &BlockId) -> Option<sainome::Ref> {
        if let Some(prop) = self.get::<Property>(block_id) {
            let mut r = sainome::Ref::new(prop.value.as_option_string());
            if let Value::Children(children) = &prop.value {
                let children = self.listed::<Property>(children);
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
