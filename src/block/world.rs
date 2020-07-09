use super::{Block, BlockId, Field};
use crate::Promise;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct World {
    selecting_table: BlockId,
    tables: Vec<BlockId>,
    characters: HashSet<BlockId>,
}

impl World {
    pub fn new(selecting_table: BlockId) -> Self {
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

    pub fn set_selecting_table(&mut self, table_id: BlockId) {
        self.selecting_table = table_id;
    }

    pub fn tables(&self) -> impl Iterator<Item = &BlockId> {
        self.tables.iter()
    }

    pub fn add_table(&mut self, table: BlockId) {
        self.tables.push(table);
    }

    pub fn characters(&self) -> impl Iterator<Item = &BlockId> {
        self.characters.iter()
    }

    pub fn add_character(&mut self, character: BlockId) {
        self.characters.insert(character);
    }

    pub fn remove_character(&mut self, character: &BlockId) {
        self.characters.remove(character);
    }
}

impl Block for World {
    fn pack(&self) -> Promise<JsValue> {
        let tables = js_sys::Array::new();
        for table in &self.tables {
            tables.push(&JsValue::from(table.to_string()));
        }
        let characters = js_sys::Array::new();
        for character in &self.characters {
            characters.push(&JsValue::from(character.to_string()));
        }

        let data = object! {
            selecting_table: self.selecting_table.to_string(),
            tables: tables,
            characters: characters
        };

        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();

        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
