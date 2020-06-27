use super::{Block, BlockId, Field};
use crate::Promise;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

pub struct World {
    selecting_table: BlockId,
    tables: Vec<BlockId>,
    characters: HashSet<BlockId>,
}

impl World {
    pub fn new(field: &mut Field) -> Self {
        let selecting_table = field.add(super::Table::new(field));
        let tables = vec![selecting_table.clone()];
        Self {
            selecting_table,
            tables,
            characters: HashSet::new(),
        }
    }

    pub fn selecting_table(&self) -> &BlockId {
        &self.selecting_table
    }

    pub fn add_character(&mut self, character: BlockId) {
        self.characters.insert(character);
    }

    pub fn remove_character(&mut self, character: &BlockId) {
        self.characters.remove(character);
    }
}

impl Block for World {
    fn pack(&self) -> Promise<JsValue, ()> {
        unimplemented!();
    }
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()> {
        unimplemented!();
    }
}
