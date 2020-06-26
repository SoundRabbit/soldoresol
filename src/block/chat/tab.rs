use super::{Block, BlockId, Field};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
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
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {}
}
