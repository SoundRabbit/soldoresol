use super::{Block, BlockId, Field};
use crate::Promise;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Tags {
    tags: Vec<String>,
}

impl Tags {
    pub fn new() -> Self {
        Self { tags: vec![] }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.iter().any(|t| tag == *t) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: String) {
        if let Some(pos) = self.tags.iter().position(|t| tag == *t) {
            self.tags.remove(pos);
        }
    }
}

impl Block for Tags {
    fn pack(&self) -> Promise<JsValue> {
        let data = object! {};
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
