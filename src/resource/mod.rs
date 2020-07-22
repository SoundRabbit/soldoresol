use crate::{random_id::U128Id, Promise};
use std::collections::HashMap;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

mod data;

pub use data::Data;

pub type ResourceId = U128Id;

pub struct Resource {
    data: HashMap<ResourceId, Data>,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, data: Data) -> ResourceId {
        let resource_id = U128Id::new();
        self.assign(resource_id.clone(), data);
        resource_id
    }

    pub fn assign(&mut self, resource_id: ResourceId, data: Data) {
        self.data.insert(resource_id, data);
    }

    pub fn all(&self) -> impl Iterator<Item = (&ResourceId, &Data)> {
        self.data.iter()
    }

    pub fn pack_all(&self) -> Promise<Vec<(ResourceId, JsValue)>> {
        let mut promises = vec![];
        for (key, data) in &self.data {
            let key = key.clone();
            promises.push(data.pack().map(move |data| data.map(|data| (key, data))))
        }
        Promise::some(promises)
            .map(|vals| vals.map(|vals| vals.into_iter().filter_map(|x| x).collect()))
    }

    pub fn pack_listed(&self, ids: Vec<ResourceId>) -> Promise<Vec<(ResourceId, JsValue)>> {
        let mut promises = vec![];
        for key in ids {
            if let Some(data) = self.data.get(&key) {
                promises.push(data.pack().map(move |data| data.map(|data| (key, data))));
            }
        }
        Promise::some(promises)
            .map(|vals| vals.map(|vals| vals.into_iter().filter_map(|x| x).collect()))
    }
}

impl Deref for Resource {
    type Target = HashMap<ResourceId, Data>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
