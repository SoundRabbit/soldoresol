use super::BlockId;
use wasm_bindgen::{prelude::*, JsCast};

pub struct World {
    selecting_table: BlockId,
    tables: Vec<BlockId>,
    characters: Vec<BlockId>,
    memos: Vec<BlockId>,
    tags: Vec<BlockId>,
}

impl World {
    pub fn new(selecting_table: BlockId) -> Self {
        let tables = vec![BlockId::clone(&selecting_table)];
        Self {
            selecting_table,
            tables,
            characters: vec![],
            memos: vec![],
            tags: vec![],
        }
    }

    pub fn clone(this: &Self) -> Self {
        let characters = this
            .characters
            .iter()
            .map(|b_id| BlockId::clone(b_id))
            .collect::<Vec<_>>();
        let tables = this
            .tables
            .iter()
            .map(|b_id| BlockId::clone(b_id))
            .collect::<Vec<_>>();
        let memos = this
            .memos
            .iter()
            .map(|b_id| BlockId::clone(b_id))
            .collect::<Vec<_>>();
        let tags = this
            .tags
            .iter()
            .map(|b_id| BlockId::clone(b_id))
            .collect::<Vec<_>>();
        Self {
            selecting_table: BlockId::clone(&this.selecting_table),
            characters,
            tables,
            memos,
            tags,
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

    pub fn remove_table(&mut self, table_id: &BlockId) {
        if let Some(pos) = self.tables.iter().position(|x| *x == *table_id) {
            self.tables.remove(pos);
        }
    }

    pub fn replace_table(&mut self, table_id_a: &BlockId, table_id_b: BlockId) {
        if let Some(pos) = self.tables.iter().position(|x| *x == *table_id_a) {
            self.tables[pos] = table_id_b;
        }
    }

    pub fn characters(&self) -> impl Iterator<Item = &BlockId> {
        self.characters.iter()
    }

    pub fn add_character(&mut self, character_id: BlockId) {
        self.characters.push(character_id)
    }

    pub fn remove_character(&mut self, character_id: &BlockId) {
        if let Some(pos) = self.characters.iter().position(|x| *x == *character_id) {
            self.characters.remove(pos);
        }
    }

    pub fn memos(&self) -> impl Iterator<Item = &BlockId> {
        self.memos.iter()
    }

    pub fn add_memo(&mut self, memo_id: BlockId) {
        self.memos.push(memo_id);
    }

    pub fn remove_memo(&mut self, memo_id: &BlockId) {
        if let Some(pos) = self.characters.iter().position(|x| *x == *memo_id) {
            self.memos.remove(pos);
        }
    }

    pub fn tags(&self) -> impl Iterator<Item = &BlockId> {
        self.tags.iter()
    }

    pub fn add_tag(&mut self, tag_id: BlockId) {
        self.tags.push(tag_id);
    }

    pub fn remove_tag(&mut self, tag_id: &BlockId) {
        if let Some(pos) = self.characters.iter().position(|x| *x == *tag_id) {
            self.tags.remove(pos);
        }
    }

    pub async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    pub async fn unpack(_val: JsValue) -> Option<Self> {
        unimplemented!();
    }
}
