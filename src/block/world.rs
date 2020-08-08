use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, JsObject, Promise};
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Debug)]
pub struct World {
    selecting_table: BlockId,
    tables: Vec<BlockId>,
    characters: Vec<BlockId>,
    memos: Vec<BlockId>,
    tags: Vec<BlockId>,
}

impl World {
    pub fn new(selecting_table: BlockId) -> Self {
        let tables = vec![selecting_table.clone()];
        Self {
            selecting_table,
            tables,
            characters: vec![],
            memos: vec![],
            tags: vec![],
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
}

impl Block for World {
    fn pack(&self) -> Promise<JsValue> {
        let tables = js_sys::Array::new();
        for table in &self.tables {
            tables.push(&table.to_jsvalue());
        }

        let characters = js_sys::Array::new();
        for character in &self.characters {
            characters.push(&character.to_jsvalue());
        }

        let memos = js_sys::Array::new();
        for memo in &self.memos {
            memos.push(&memo.to_jsvalue());
        }

        let tags = js_sys::Array::new();
        for tag in &self.tags {
            tags.push(&tag.to_jsvalue());
        }

        let data = object! {
            selecting_table: self.selecting_table.to_jsvalue(),
            tables: tables,
            characters: characters,
            memos: memos,
            tags: tags
        };

        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();

        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let selecting_table = val
                .get("selecting_table")
                .and_then(|x| U128Id::from_jsvalue(&x))
                .map(|x| field.block_id(x));
            let tables = val.get("tables").map(|x| js_sys::Array::from(&x));
            let characters = val.get("characters").map(|x| js_sys::Array::from(&x));
            let memos = val.get("memos").map(|x| js_sys::Array::from(&x));
            let tags = val.get("tags").map(|x| js_sys::Array::from(&x));
            if let (
                Some(selecting_table),
                Some(raw_tables),
                Some(raw_characters),
                Some(raw_memos),
                Some(raw_tags),
            ) = (selecting_table, tables, characters, memos, tags)
            {
                let mut tables = vec![];
                for id in raw_tables.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        tables.push(id);
                    }
                }

                let mut characters = vec![];
                for id in raw_characters.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        characters.push(id);
                    }
                }

                let mut memos = vec![];
                for id in raw_memos.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        memos.push(id);
                    }
                }

                let mut tags = vec![];
                for id in raw_tags.to_vec() {
                    if let Some(id) = U128Id::from_jsvalue(&id).map(|id| field.block_id(id)) {
                        tags.push(id);
                    }
                }

                Some(Box::new(Self {
                    selecting_table,
                    tables,
                    characters,
                    memos,
                    tags,
                }))
            } else {
                None
            }
        } else {
            None
        };
        Promise::new(move |resolve| resolve(self_))
    }
}
