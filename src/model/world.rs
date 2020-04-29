use super::table::Table;
use super::Character;
use crate::random_id;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub struct World {
    table_id: u32,
    table: Table,
    characters: HashMap<u32, Character>,
}

impl World {
    pub fn new(table_size: [f64; 2]) -> Self {
        Self {
            table_id: 0,
            table: Table::new(table_size, 64.0),
            characters: HashMap::new(),
        }
    }

    pub fn table_id(&self) -> u32 {
        self.table_id
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    pub fn characters(&self) -> std::collections::hash_map::Iter<u32, Character> {
        self.characters.iter()
    }

    pub fn characters_mut(&mut self) -> std::collections::hash_map::IterMut<u32, Character> {
        self.characters.iter_mut()
    }

    pub fn character(&self, character_id: u32) -> Option<&Character> {
        self.characters.get(&character_id)
    }

    pub fn character_mut(&mut self, character_id: u32) -> Option<&mut Character> {
        self.characters.get_mut(&character_id)
    }

    pub fn add_character(&mut self, character: Character) -> u32 {
        loop {
            let character_id = random_id::u32val();
            match self.characters.get(&character_id) {
                Some(_) => continue,
                None => {
                    self.characters.insert(character_id, character);
                    web_sys::console::log_1(&JsValue::from(character_id));
                    return character_id;
                }
            }
        }
    }
}
