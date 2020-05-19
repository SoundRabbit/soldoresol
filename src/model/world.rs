use super::table::Table;
use super::Character;
use super::Tablemask;
use crate::random_id;
use std::collections::hash_map;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub struct World {
    table_id: u128,
    table: Table,
    characters: HashMap<u128, Character>,
    tablemasks: HashMap<u128, Tablemask>,
}

impl World {
    pub fn new(table_size: [f64; 2]) -> Self {
        Self {
            table_id: random_id::u128val(),
            table: Table::new(table_size, 64.0),
            characters: HashMap::new(),
            tablemasks: HashMap::new(),
        }
    }

    pub fn table_id(&self) -> u128 {
        self.table_id
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    pub fn characters(&self) -> hash_map::Iter<u128, Character> {
        self.characters.iter()
    }

    pub fn characters_mut(&mut self) -> std::collections::hash_map::IterMut<u128, Character> {
        self.characters.iter_mut()
    }

    pub fn character(&self, character_id: &u128) -> Option<&Character> {
        self.characters.get(character_id)
    }

    pub fn character_mut(&mut self, character_id: &u128) -> Option<&mut Character> {
        self.characters.get_mut(character_id)
    }

    pub fn add_character(&mut self, character: Character) -> u128 {
        let character_id = random_id::u128val();
        self.characters.insert(character_id, character);
        return character_id;
    }

    pub fn add_character_with_id(&mut self, character_id: u128, character: Character) {
        self.characters.insert(character_id, character);
    }

    pub fn tablemasks(&self) -> hash_map::Iter<u128, Tablemask> {
        self.tablemasks.iter()
    }

    pub fn tablemask(&self, tablemask_id: &u128) -> Option<&Tablemask> {
        self.tablemasks.get(tablemask_id)
    }

    pub fn tablemask_mut(&mut self, tablemask_id: &u128) -> Option<&mut Tablemask> {
        self.tablemasks.get_mut(tablemask_id)
    }

    pub fn add_tablemask(&mut self, tablemask: Tablemask) -> u128 {
        let tablemask_id = random_id::u128val();
        self.tablemasks.insert(tablemask_id, tablemask);
        return tablemask_id;
    }

    pub fn data() {}
}

pub struct Data {
    pub table_id: u128,
}
