use super::{
    character::{Character, CharacterData},
    table::{Table, TableData},
    tablemask::{Tablemask, TablemaskData},
};
use crate::{random_id, JsObject};
use std::{
    collections::{hash_map, HashMap},
    ops::Deref,
    rc::Rc,
};

pub struct World {
    table_id: u128,
    table: Rc<Table>,
    characters: HashMap<u128, Character>,
    tablemasks: HashMap<u128, Tablemask>,
}

pub struct WorldData(JsObject);

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

    pub fn as_data(&self) -> WorldData {
        let mut character_data = JsObject::new();
        for (id, character) in self.characters() {
            character_data.set(&id.to_string(), &character.as_data());
        }

        let mut tablemask_data = JsObject::new();
        for (id, tablemask) in self.tablemasks() {
            tablemask_data.set(&id.to_string(), &tablemask.as_data());
        }

        let table_data: JsObject = self.table().as_data().into();

        WorldData(object! {
            table_id: self.table_id().to_string(),
            table: table_data,
            character: character_data,
            tablemask: tablemask_data
        })
    }

    pub fn remove_object(&mut self, object_id: &u128) {
        self.characters.remove(object_id);
        self.tablemasks.remove(object_id);
    }
}

impl Into<World> for WorldData {
    fn into(self) -> World {
        use js_sys::{Array, Object};
        use wasm_bindgen::JsCast;

        let obj = self.0;

        let table_id = obj
            .get("table_id")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .unwrap();
        let table: Rc<Table> = TableData::from(obj.get("table").unwrap()).into();

        let mut characters = HashMap::new();
        for c in Object::entries(&obj.get("character").unwrap()).to_vec() {
            let c = c.dyn_into::<Array>().unwrap();
            let id = c.get(0).as_string().unwrap().parse().unwrap();
            let data: Character =
                CharacterData::from(c.get(1).dyn_into::<JsObject>().unwrap()).into();
            characters.insert(id, data);
        }

        let mut tablemasks = HashMap::new();
        for t in Object::entries(&obj.get("tablemask_data").unwrap()).to_vec() {
            let t = t.dyn_into::<Array>().unwrap();
            let id = t.get(0).as_string().unwrap().parse().unwrap();
            let data: Tablemask =
                TablemaskData::from(t.get(1).dyn_into::<JsObject>().unwrap()).into();
            tablemasks.insert(id, data);
        }

        World {
            table_id,
            table,
            characters,
            tablemasks,
        }
    }
}

impl Into<JsObject> for WorldData {
    fn into(self) -> JsObject {
        self.0
    }
}

impl From<JsObject> for WorldData {
    fn from(obj: JsObject) -> Self {
        Self(obj)
    }
}

impl Deref for WorldData {
    type Target = JsObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
