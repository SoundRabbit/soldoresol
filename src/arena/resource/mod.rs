use crate::libs::random_id::U128Id;
use futures::future::join_all;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod data;

pub use data::Data;

#[derive(Hash, PartialEq, Eq)]
pub struct ResourceId {
    id: U128Id,
}

impl ResourceId {
    fn new() -> Self {
        Self { id: U128Id::new() }
    }

    fn clone(this: &Self) -> Self {
        Self {
            id: U128Id::clone(&this.id),
        }
    }
}

pub struct Arena {
    table: Rc<RefCell<HashMap<ResourceId, Rc<Data>>>>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            table: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            table: Rc::clone(&this.table),
        }
    }

    pub fn add(&mut self, data: Data) -> ResourceId {
        let resource_id = ResourceId::new();
        self.assign(ResourceId::clone(&resource_id), data);
        resource_id
    }

    pub fn assign(&mut self, resource_id: ResourceId, data: Data) {
        self.table.borrow_mut().insert(resource_id, Rc::new(data));
    }

    pub async fn pack_all(&self) -> HashMap<ResourceId, JsValue> {
        let mut futures = vec![];
        for key in self.table.borrow().keys() {
            let key = ResourceId::clone(&key);
            let data = Rc::clone(self.table.borrow().get(&key).unwrap());
            futures.push(async move {
                let val = data.pack().await;
                (key, val)
            });
        }
        join_all(futures).await.into_iter().collect()
    }

    pub async fn pack_listed(&self, ids: HashSet<ResourceId>) -> HashMap<ResourceId, JsValue> {
        let mut futures = vec![];
        for key in ids {
            if self.table.borrow().contains_key(&key) {
                let data = Rc::clone(self.table.borrow().get(&key).unwrap());
                futures.push(async move {
                    let val = data.pack().await;
                    (key, val)
                });
            }
        }
        join_all(futures).await.into_iter().collect()
    }
}
