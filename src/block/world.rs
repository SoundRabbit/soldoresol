use super::{Block, BlockId, Field};
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
}

impl Block for World {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {}
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {}
}
