use crate::{js_object::JsObject, random_id, Promise};
use js_sys::Date;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::Iterator,
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

pub mod character;
pub mod chat;
pub mod property;
pub mod table;
pub mod table_object;
pub mod world;

pub use character::Character;
pub use chat::Chat;
pub use property::Property;
pub use table::Table;
pub use world::World;

trait Block {
    fn pack(&self) -> Promise<JsValue, ()>;
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()>;
}

type Timestamp = u32;

type BlockTable = Rc<RefCell<HashMap<BlockId, FieldBlock>>>;

pub struct BlockId {
    table: BlockTable,
    internal_id: u128,
}

struct FieldBlock {
    count: usize,
    timestamp: Timestamp,
    payload: Box<dyn Any>,
}

pub struct Field {
    table: BlockTable,
}

impl BlockId {
    fn new(table: BlockTable, internal_id: u128) -> Self {
        let me = Self { table, internal_id };
        if let Some(block) = me.table.borrow_mut().get_mut(&me) {
            block.count += 1;
        }
        me
    }

    pub fn to_string(&self) -> String {
        self.internal_id.to_string()
    }

    pub fn to_u128(&self) -> u128 {
        self.internal_id
    }
}

impl Clone for BlockId {
    fn clone(&self) -> Self {
        if let Some(block) = self.table.borrow_mut().get_mut(&self) {
            block.count += 1;
        }
        let table = Rc::clone(&self.table);
        let internal_id = self.internal_id;
        Self { table, internal_id }
    }
}

impl Drop for BlockId {
    fn drop(&mut self) {
        if let Some(block) = self.table.borrow_mut().get_mut(&self) {
            if block.count > 1 {
                block.count -= 1;
            } else {
                self.table.borrow_mut().remove(&self);
            }
        }
    }
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.internal_id == other.internal_id
    }
}

impl Eq for BlockId {}

impl Hash for BlockId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal_id.hash(state);
    }
}

impl FieldBlock {
    fn new<T: Block + 'static>(timestamp: u32, block: T) -> Self {
        Self {
            count: 0,
            timestamp: timestamp,
            payload: Box::new(block),
        }
    }

    pub fn pack(&self) -> Promise<JsValue, ()> {
        if let Some(payload) = self.payload.downcast_ref::<Chat>() {
            payload.pack()
        } else if let Some(payload) = self.payload.downcast_ref::<chat::Item>() {
            payload.pack()
        } else if let Some(payload) = self.payload.downcast_ref::<chat::Tab>() {
            payload.pack()
        } else if let Some(payload) = self.payload.downcast_ref::<Table>() {
            payload.pack()
        } else if let Some(payload) = self.payload.downcast_ref::<table::Texture>() {
            payload.pack()
        } else if let Some(payload) = self.payload.downcast_ref::<Character>() {
            payload.pack()
        } else if let Some(payload) = self.payload.downcast_ref::<Property>() {
            payload.pack()
        } else {
            Promise::new(|resolve| resolve(Err(())))
        }
    }

    pub fn unpack(val: JsValue, resolve: impl FnOnce(Option<Self>) + 'static) {
        if let Ok(val) = val.dyn_into::<JsObject>() {
            let type_name = val.get("type_name").and_then(|x| x.as_string());
            let timestamp = val
                .get("timestamp")
                .and_then(|x| x.as_f64().map(|x| x as u32));
            let payload = val.get("payload").map(|x| {
                let x: js_sys::Object = x.into();
                let x: JsValue = x.into();
                x
            });
            if let (Some(type_name), Some(timestamp), Some(payload)) =
                (type_name, timestamp, payload)
            {
            } else {
                resolve(None)
            }
        } else {
            resolve(None)
        }
    }
}

impl Field {
    pub fn block_id(&self, internal_id: u128) -> BlockId {
        BlockId::new(Rc::clone(&self.table), internal_id)
    }

    pub fn new() -> Self {
        Self {
            table: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add<T: Block + 'static>(&mut self, block: T) -> BlockId {
        let block_id = self.block_id(random_id::u128val());
        if !self.table.borrow().contains_key(&block_id) {
            self.assign(block_id.clone(), Date::now() as u32, block);
        }
        block_id
    }

    pub fn assign<T: Block + 'static>(
        &mut self,
        block_id: BlockId,
        timestamp: Timestamp,
        block: T,
    ) {
        if self
            .table
            .borrow()
            .get(&block_id)
            .map(|b| b.timestamp < timestamp)
            .unwrap_or(true)
        {
            let block = FieldBlock::new(timestamp, block);
            self.table.borrow_mut().insert(block_id, block);
        }
    }

    pub fn get<T: Block + 'static>(&self, block_id: &BlockId) -> Option<&T> {
        self.table
            .borrow()
            .get(&block_id)
            .and_then(|fb| fb.payload.downcast_ref::<T>())
    }

    pub fn all<T: Block + 'static>(&self) -> Vec<(&BlockId, &T)> {
        self.table
            .borrow()
            .iter()
            .filter_map(|(id, fb)| {
                if let Some(b) = fb.payload.downcast_ref::<T>() {
                    Some((id, b))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn listed<T: Block + 'static>(
        &self,
        block_ids: Vec<&BlockId>,
    ) -> impl Iterator<Item = (BlockId, &T)> {
        let mut blocks = vec![];
        for block_id in block_ids {
            if let Some(block) = self.get(block_id) {
                blocks.push((block_id.clone(), block))
            }
        }
        blocks.into_iter()
    }

    pub fn update<T: Block + 'static>(
        &mut self,
        block_id: &BlockId,
        timestamp: Option<Timestamp>,
        f: impl FnOnce(&mut T),
    ) -> Option<&mut Self> {
        self.table
            .borrow_mut()
            .get_mut(block_id)
            .and_then(|fb| {
                if let Some(timestamp) = timestamp {
                    if fb.timestamp < timestamp {
                        fb.timestamp = timestamp;
                        fb.payload.downcast_mut::<T>()
                    } else {
                        None
                    }
                } else {
                    fb.payload.downcast_mut::<T>()
                }
            })
            .map(move |b| {
                f(b);
                None
            })
            .unwrap_or(Some(self))
    }

    pub fn timestamp(&self, block_id: &BlockId) -> Option<&Timestamp> {
        self.table.borrow().get(block_id).map(|b| &b.timestamp)
    }

    pub fn pack_listed(&self, block_ids: Vec<&BlockId>) -> Promise<Vec<(BlockId, JsValue)>, ()> {
        let mut promises = vec![];
        for block_id in block_ids {
            if let Some(block) = self.table.borrow().get(block_id) {
                let block_id = block_id.clone();
                promises.push(block.pack().map(move |res| res.map(|val| (block_id, val))));
            }
        }
        Promise::some(promises)
            .map(|vals| vals.map(|vals| vals.into_iter().filter_map(|x| x).collect()))
    }
}
