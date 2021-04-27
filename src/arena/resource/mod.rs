use crate::libs::random_id::U128Id;
use crate::libs::try_ref::TryRef;
use futures::future::join_all;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod data;

pub use data::Data;
pub use data::ImageData;

#[derive(Hash, PartialEq, Eq)]
pub struct ResourceId {
    id: U128Id,
}

impl ResourceId {
    fn new() -> Self {
        Self { id: U128Id::new() }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            id: U128Id::clone(&this.id),
        }
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id.to_string())
    }
}

pub struct ArenaRef {
    arena: Arena,
}

impl ArenaRef {
    pub fn clone(this: &Self) -> Self {
        Self {
            arena: Arena::clone(&this.arena),
        }
    }
}

impl std::ops::Deref for ArenaRef {
    type Target = Arena;
    fn deref(&self) -> &Self::Target {
        &self.arena
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

    fn clone(this: &Self) -> Self {
        Self {
            table: Rc::clone(&this.table),
        }
    }

    pub fn as_ref(&self) -> ArenaRef {
        ArenaRef {
            arena: Self::clone(&self),
        }
    }

    pub fn get(&self, resource_id: &ResourceId) -> Option<Rc<Data>> {
        if let Some(data) = self.table.borrow().get(resource_id) {
            Some(Rc::clone(data))
        } else {
            None
        }
    }

    pub fn get_as<T>(&self, resource_id: &ResourceId) -> Option<Rc<T>>
    where
        Data: TryRef<Rc<T>>,
    {
        if let Some(data) = self.table.borrow().get(resource_id) {
            if let Some(data) = data.try_ref() {
                return Some(Rc::clone(data));
            }
        }
        None
    }

    pub fn all_of<T>(&self) -> impl Iterator<Item = (ResourceId, Rc<T>)>
    where
        Data: TryRef<Rc<T>>,
    {
        self.table
            .borrow()
            .iter()
            .filter_map(|(r_id, data)| {
                data.try_ref()
                    .map(|data| (ResourceId::clone(r_id), Rc::clone(data)))
            })
            .collect::<Vec<_>>()
            .into_iter()
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
