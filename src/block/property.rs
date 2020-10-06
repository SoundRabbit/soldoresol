use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, JsObject, Promise};
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

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
    is_collapsed: bool,
}

impl Value {
    pub fn to_jsobject(&self) -> JsObject {
        match self {
            Self::None => object! {
                type: "None"
            },
            Self::Num(x) => object! {
                type: "Num",
                payload: *x
            },
            Self::Str(x) => object! {
                type: "Str",
                payload: x
            },
            Self::Children(children) => {
                let children_object = array![];
                for child in children {
                    children_object.push(&child.to_jsvalue());
                }
                object! {
                    type: "Children",
                    payload: children_object
                }
            }
        }
    }

    pub fn from_jsobject(field: &mut Field, val: JsObject) -> Option<Self> {
        val.get("type")
            .and_then(|t| t.as_string())
            .and_then(|t| match t.as_str() {
                "None" => Some(Self::None),
                "Num" => val
                    .get("payload")
                    .and_then(|x| x.as_f64())
                    .map(|x| Self::Num(x)),
                "Str" => val
                    .get("payload")
                    .and_then(|x| x.as_string())
                    .map(|x| Self::Str(x)),
                "Children" => val.get("payload").map(|x| {
                    let mut children = vec![];
                    let raw_children = js_sys::Array::from(&x);
                    for child in raw_children.to_vec() {
                        if let Some(id) = U128Id::from_jsvalue(&child).map(|x| field.block_id(x)) {
                            children.push(id);
                        }
                    }
                    Self::Children(children)
                }),
                _ => None,
            })
    }
}

impl Property {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_selected: false,
            value: Value::None,
            is_collapsed: true,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn set_is_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value
    }

    pub fn is_collapsed(&self) -> bool {
        self.is_collapsed
    }

    pub fn set_is_collapsed(&mut self, is_collapsed: bool) {
        self.is_collapsed = is_collapsed;
    }

    pub fn add_child(&mut self, child_id: BlockId) {
        match &mut self.value {
            Value::Children(children) => {
                children.push(child_id);
            }
            _ => (),
        }
    }

    pub fn remove_child(&mut self, child_id: &BlockId) {
        if let Value::Children(children) = &mut self.value {
            if let Some(pos) = children.iter().position(|prop_id| *prop_id == *child_id) {
                children.remove(pos);
            }
        }
    }
}

impl Block for Property {
    fn pack(&self) -> Promise<JsValue> {
        let value = self.value.to_jsobject();
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
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let name = val.get("name").and_then(|x| x.as_string());
            let is_selected = val.get("is_selected").and_then(|x| x.as_bool());
            let value = val
                .get("value")
                .and_then(|x| Value::from_jsobject(field, x));
            if let (Some(name), Some(is_selected), Some(value)) = (name, is_selected, value) {
                Some(Box::new(Self {
                    name,
                    is_selected,
                    value,
                    is_collapsed: true,
                }))
            } else {
                None
            }
        } else {
            None
        };
        Promise::new(move |resolve| resolve(self_))
    }
    fn dependents(&self, field: &Field) -> HashSet<BlockId> {
        if let Value::Children(child_ids) = &self.value {
            let mut deps = HashSet::new();

            for child_id in child_ids {
                if let Some(child) = field.get::<Self>(child_id) {
                    let child_deps = child.dependents(field);
                    for child_dep in child_deps {
                        deps.insert(child_dep);
                    }
                    deps.insert(child_id.clone());
                }
            }

            deps
        } else {
            set! {}
        }
    }
}
