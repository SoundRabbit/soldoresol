use crate::random_id::U128Id;
use futures::future::join_all;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod data;

pub use data::Data;

pub type ResourceId = U128Id;

pub struct Resource {
    table: Rc<RefCell<HashMap<ResourceId, Data>>>,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            table: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add(&mut self, data: Data) -> ResourceId {
        let resource_id = U128Id::new();
        self.assign(U128Id::clone(&resource_id), data);
        resource_id
    }

    pub fn assign(&mut self, resource_id: ResourceId, data: Data) {
        self.table.borrow_mut().insert(resource_id, data);
    }

    pub fn all(&self) -> impl Iterator<Item = (&ResourceId, &Data)> {
        self.iter()
    }

    pub async fn pack_all(&self) -> HashMap<ResourceId, JsValue> {
        let mut futures = vec![];
        for key in self.table.borrow().keys() {
            let key = U128Id::clone(&key);
            let table = Rc::clone(&self.table);
            futures.push(async move {
                let val = table.borrow().get(&key).unwrap().pack().await;
                (key, val)
            });
        }
        join_all(futures).await.into_iter().collect()
    }

    pub async fn pack_listed(&self, ids: HashSet<ResourceId>) -> HashMap<ResourceId, JsValue> {
        let mut futures = vec![];
        for key in ids {
            if self.table.borrow().contains_key(&key) {
                let table = Rc::clone(&self.table);
                futures.push(async move {
                    let val = table.borrow().get(&key).unwrap().pack().await;
                    (key, val)
                });
            }
        }
        join_all(futures).await.into_iter().collect()
    }
}

impl Deref for Resource {
    type Target = HashMap<ResourceId, Data>;
    fn deref(&self) -> &Self::Target {
        unsafe { self.table.as_ptr().as_ref().unwrap() }
    }
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        let table = Rc::clone(&self.table);
        Self { table: table }
    }
}
