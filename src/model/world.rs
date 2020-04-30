use super::table::Table;
use super::Character;
use super::Tablemask;
use std::collections::hash_map;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub struct World {
    id_counter: u32,
    table_id: u32,
    table: Table,
    characters: HashMap<u32, Character>,
    tablemasks: HashMap<u32, Tablemask>,
}

impl World {
    pub fn new(table_size: [f64; 2]) -> Self {
        Self {
            id_counter: 1,
            table_id: 0,
            table: Table::new(table_size, 64.0),
            characters: HashMap::new(),
            tablemasks: HashMap::new(),
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

    pub fn characters(&self) -> hash_map::Iter<u32, Character> {
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
        let character_id = self.id_counter;
        self.id_counter += 1;
        self.characters.insert(character_id, character);
        return character_id;
    }

    pub fn tablemasks(&self) -> hash_map::Iter<u32, Tablemask> {
        self.tablemasks.iter()
    }

    pub fn tablemask(&self, tablemask_id: &u32) -> Option<&Tablemask> {
        self.tablemasks.get(tablemask_id)
    }

    pub fn tablemask_mut(&mut self, tablemask_id: &u32) -> Option<&mut Tablemask> {
        self.tablemasks.get_mut(tablemask_id)
    }

    pub fn add_tablemask(&mut self, tablemask: Tablemask) -> u32 {
        let tablemask_id = self.id_counter;
        self.id_counter += 1;
        self.tablemasks.insert(tablemask_id, tablemask);
        return tablemask_id;
    }
}
