use super::{Block, BlockId, Field};
use crate::Promise;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

pub mod item;
pub mod tab;

pub use item::Item;
pub use tab::Tab;

#[derive(Clone)]
pub struct Chat {
    tabs: Vec<BlockId>,
}

impl Chat {
    pub fn new(tabs: Vec<BlockId>) -> Self {
        Self { tabs: tabs }
    }
}

impl Block for Chat {
    fn pack(&self) -> Promise<JsValue> {
        let val = js_sys::Array::new();
        for tab_id in &self.tabs {
            val.push(&JsValue::from(tab_id.to_string()));
        }
        Promise::new(|resolve| resolve(Some(val.into())))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let val = js_sys::Array::from(&val).to_vec();
        let mut tabs = vec![];
        for tab in val {
            if let Some(tab_id) = tab.as_string().and_then(|x| x.parse().ok()) {
                tabs.push(field.block_id(tab_id));
            }
        }
        let chat = Self { tabs };
        Promise::new(|resolve| resolve(Some(Box::new(chat))))
    }
}

impl Deref for Chat {
    type Target = Vec<BlockId>;
    fn deref(&self) -> &Self::Target {
        &self.tabs
    }
}

impl DerefMut for Chat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tabs
    }
}