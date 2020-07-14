use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, Promise};
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

    pub fn tabs(&self) -> &Vec<BlockId> {
        &self.tabs
    }
}

impl Block for Chat {
    fn pack(&self) -> Promise<JsValue> {
        let val = js_sys::Array::new();
        for tab_id in &self.tabs {
            val.push(&tab_id.to_jsvalue());
        }
        Promise::new(|resolve| resolve(Some(val.into())))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let val = js_sys::Array::from(&val).to_vec();
        let mut tabs = vec![];
        for tab in val {
            if let Some(tab_id) = U128Id::from_jsvalue(&tab) {
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
