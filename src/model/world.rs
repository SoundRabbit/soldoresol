use super::character::{Character, CharacterData};
use super::table::{Table, TableData};
use super::tablemask::{Tablemask, TablemaskData};
use crate::random_id;
use serde::{Deserialize, Serialize};
use std::collections::hash_map;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct World {
    table_id: u128,
    table: Rc<Table>,
    characters: HashMap<u128, Character>,
    tablemasks: HashMap<u128, Tablemask>,
}

#[derive(Deserialize, Serialize)]
pub struct WorldData {
    pub table_id: u128,
    pub table_data: TableData,
    pub character_data: HashMap<u128, CharacterData>,
    pub tablemask_data: HashMap<u128, TablemaskData>,
}

impl World {
    pub fn new(table_size: [f64; 2]) -> Self {
        Self {
            table_id: random_id::u128val(),
            table: Rc::new(Table::new(table_size, 64.0)),
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
        Rc::get_mut(&mut self.table).unwrap()
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

    pub fn to_data(&self) -> WorldData {
        let mut character_data = HashMap::new();
        for (id, character) in self.characters() {
            character_data.insert(*id, character.to_data());
        }

        let mut tablemask_data = HashMap::new();
        for (id, tablemask) in self.tablemasks() {
            tablemask_data.insert(*id, tablemask.to_data());
        }

        WorldData {
            table_id: self.table_id(),
            table_data: self.table().to_data(),
            character_data,
            tablemask_data,
        }
    }
}

impl From<WorldData> for World {
    fn from(world_data: WorldData) -> Self {
        let mut characters = HashMap::new();
        for (id, character_data) in world_data.character_data {
            characters.insert(id, Character::from(character_data));
        }

        let mut tablemasks = HashMap::new();
        for (id, tablemask_data) in world_data.tablemask_data {
            tablemasks.insert(id, Tablemask::from(tablemask_data));
        }

        Self {
            table_id: world_data.table_id,
            table: Rc::from(world_data.table_data),
            characters,
            tablemasks,
        }
    }
}
