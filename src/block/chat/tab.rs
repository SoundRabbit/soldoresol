use super::{Block, BlockId, Field};
use crate::Promise;
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
    fn pack(&self) -> Promise<JsValue, ()> {
        unimplemented!();
    }
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()> {
        unimplemented!();
    }
}
