use super::{Block, BlockId, Field};
use crate::Promise;
use std::ops::{Deref, DerefMut};
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

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Block for Tab {
    fn pack(&self) -> Promise<JsValue> {
        unimplemented!();
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}

impl Deref for Tab {
    type Target = Vec<BlockId>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Tab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
