use super::Block;
use super::BlockId;
use wasm_bindgen::prelude::*;

pub struct Tab {
    name: String,
    items: Vec<BlockId>,
}

impl Tab {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: vec![],
        }
    }
}

impl Block for Tab {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {}
    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {}
}
