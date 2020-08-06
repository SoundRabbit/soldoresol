mod item;

use super::{Block, BlockId, Field};
use crate::Promise;
use wasm_bindgen::prelude::*;

pub use item::Item;

#[derive(Clone)]
pub struct Memo {
    items: Vec<BlockId>,
}

impl Memo {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn items(&self) -> impl Iterator<Item = &BlockId> {
        self.items.iter()
    }

    pub fn add_item(&mut self, item: BlockId) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_idx: usize) {
        self.items.remove(item_idx);
    }
}

impl Block for Memo {
    fn pack(&self) -> Promise<JsValue> {
        unimplemented!();
    }

    fn unpack(_: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
