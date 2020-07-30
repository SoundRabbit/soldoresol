use crate::{random_id::U128Id, Promise};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod data;

pub use data::Data;

pub type ResourceId = U128Id;

pub struct Resource {
    data: Rc<RefCell<HashMap<ResourceId, Data>>>,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add(&mut self, data: Data) -> ResourceId {
        let resource_id = U128Id::new();
        self.assign(resource_id.clone(), data);
        resource_id
    }

    pub fn assign(&mut self, resource_id: ResourceId, data: Data) {
        self.data.borrow_mut().insert(resource_id, data);
    }

    pub fn all(&self) -> impl Iterator<Item = (&ResourceId, &Data)> {
        self.iter()
    }

    pub fn pack_all(&self) -> Promise<Vec<(ResourceId, JsValue)>> {
        let mut promises = vec![];
        for (key, data) in self.data.borrow().iter() {
            let key = key.clone();
            promises.push(data.pack().map(move |data| data.map(|data| (key, data))))
        }
        Promise::all(promises).map(|vals| vals.map(|vals| vals.into_iter().collect()))
    }

    pub fn pack_listed(&self, ids: Vec<ResourceId>) -> Promise<Vec<(ResourceId, JsValue)>> {
        let mut promises = vec![];
        for key in ids {
            if let Some(data) = self.data.borrow().get(&key) {
                promises.push(data.pack().map(move |data| data.map(|data| (key, data))));
            }
        }
        Promise::all(promises).map(|vals| vals.map(|vals| vals.into_iter().collect()))
    }
}

impl Deref for Resource {
    type Target = HashMap<ResourceId, Data>;
    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ptr().as_ref().unwrap() }
    }
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        let data = Rc::clone(&self.data);
        Self { data: data }
    }
}
