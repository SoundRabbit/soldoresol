#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
use super::BlockMut;
use super::Table;

block! {
    [pub Scene(constructor, pack)]
    (master_table): BlockMut<Table>;
    selecting_table: BlockMut<Table> = BlockMut::clone(&master_table);
    tables: Vec<BlockMut<Table>> = vec![];
    name: String = String::from("新規シーン");
}

impl Scene {
    pub fn selecting_table(&self) -> &BlockMut<Table> {
        &self.selecting_table
    }
    pub fn master_table(&self) -> &BlockMut<Table> {
        &self.master_table
    }
    pub fn tables(&self) -> &Vec<BlockMut<Table>> {
        &self.tables
    }
    pub fn set_selecting_table(&mut self, block_id: &U128Id) {
        if self.master_table.id() == *block_id {
            self.selecting_table = BlockMut::clone(&self.master_table);
        } else {
            if let Some(table) = self.tables.iter().find(|table| table.id() == *block_id) {
                self.selecting_table = BlockMut::clone(table);
            }
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn tables_push(&mut self, table: BlockMut<Table>) {
        self.tables.push(table);
    }
}
