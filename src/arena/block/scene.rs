#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Pack;
use super::BlockMut;
use super::Table;
use super::Textboard;

block! {
    [pub Scene(constructor, pack)]
    selecting_table: BlockMut<Table> = BlockMut::<Table>::none();
    tables: Vec<BlockMut<Table>> = vec![];
    textboards: Vec<BlockMut<Textboard>> = vec![];
    name: String = String::from("新規シーン");
}

impl Scene {
    pub fn selecting_table(&self) -> &BlockMut<Table> {
        &self.selecting_table
    }
    pub fn tables(&self) -> &Vec<BlockMut<Table>> {
        &self.tables
    }
    pub fn textboards(&self) -> &Vec<BlockMut<Textboard>> {
        &self.textboards
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn tables_push(&mut self, table: BlockMut<Table>) {
        if self.tables.len() == 0 {
            self.selecting_table = BlockMut::clone(&table);
        }

        self.tables.push(table);
    }

    pub fn textboards_push(&mut self, textboard: BlockMut<Textboard>) {
        self.textboards.push(textboard);
    }

    pub fn textboards_remove(&mut self, block_id: &U128Id) {
        if let Some(textboard_idx) = self
            .textboards
            .iter()
            .position(|textboard| textboard.id() == *block_id)
        {
            self.textboards.remove(textboard_idx);
        }
    }
}
