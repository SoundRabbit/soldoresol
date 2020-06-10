use super::{
    character::{Character, CharacterData},
    table::{Table, TableData},
    tablemask::{Tablemask, TablemaskData},
};
use crate::{random_id, JsObject};
use std::{
    collections::{hash_map, HashMap},
    rc::Rc,
};

pub struct World {
    table_id: u128,
    table: Rc<Table>,
    characters: HashMap<u128, Character>,
    tablemasks: HashMap<u128, Tablemask>,
}

pub struct WorldData {
    pub table_id: u128,
    pub table_data: TableData,
    pub character_data: HashMap<u128, CharacterData>,
    pub tablemask_data: HashMap<u128, TablemaskData>,
}

impl World {
    pub fn new(table_size: [f64; 2]) -> Self {
        let mut table = Table::new();
        table.set_size(table_size);
        Self {
            table_id: random_id::u128val(),
            table: Rc::new(table),
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

    pub fn table_mut(&mut self) -> Option<&mut Table> {
        Rc::get_mut(&mut self.table)
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

    pub fn add_tablemask_with_id(&mut self, tablemask_id: u128, tablemask: Tablemask) {
        self.tablemasks.insert(tablemask_id, tablemask);
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

    pub fn remove_object(&mut self, object_id: &u128) {
        self.characters.remove(object_id);
        self.tablemasks.remove(object_id);
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

impl WorldData {
    pub fn as_object(&self) -> JsObject {
        let obj = object! {
            table_id: self.table_id.to_string(),
            table_data: self.table_data.as_object()
        };

        let character_data = object! {};
        for (id, data) in &self.character_data {
            character_data.set(&id.to_string(), &data.as_object());
        }
        obj.set("character_data", &character_data);

        let tablemask_data = object! {};
        for (id, data) in &self.tablemask_data {
            tablemask_data.set(&id.to_string(), &data.as_object());
        }
        obj.set("tablemask_data", &tablemask_data);

        obj
    }
}

impl From<JsObject> for WorldData {
    fn from(obj: JsObject) -> Self {
        use js_sys::{Array, Object};
        use wasm_bindgen::JsCast;

        let table_id = obj
            .get("table_id")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .unwrap();
        let table_data = TableData::from(obj.get("table_data").unwrap());

        let mut character_data = HashMap::new();
        for c in Object::entries(&obj.get("character_data").unwrap()).to_vec() {
            let c = c.dyn_into::<Array>().unwrap();
            let id = c.get(0).as_string().unwrap().parse().unwrap();
            let data = CharacterData::from(c.get(1).dyn_into::<JsObject>().unwrap());
            character_data.insert(id, data);
        }

        let mut tablemask_data = HashMap::new();
        for t in Object::entries(&obj.get("tablemask_data").unwrap()).to_vec() {
            let t = t.dyn_into::<Array>().unwrap();
            let id = t.get(0).as_string().unwrap().parse().unwrap();
            let data = TablemaskData::from(t.get(1).dyn_into::<JsObject>().unwrap());
            tablemask_data.insert(id, data);
        }

        Self {
            table_id,
            table_data,
            character_data,
            tablemask_data,
        }
    }
}
