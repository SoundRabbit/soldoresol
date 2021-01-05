use crate::{js_object::JsObject, random_id::U128Id, resource::ResourceId};
use async_trait::async_trait;
use downcast_rs::{impl_downcast, Downcast};
use futures::future::join_all;
use js_sys::Date;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::Iterator,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

pub mod character;
pub mod chat;
pub mod memo;
pub mod property;
pub mod table;
pub mod table_object;
pub mod tag;
pub mod world;

pub use character::Character;
pub use chat::Chat;
pub use memo::Memo;
pub use property::Property;
pub use table::Table;
pub use tag::Tag;
pub use world::World;

#[async_trait]
trait Pack: Downcast {
    async fn pack(&self) -> JsValue;
    fn name(&self) -> &'static str;
    fn dependents(&self, field: &Field) -> HashSet<BlockId>;
    fn resources(&self, field: &Field) -> HashSet<ResourceId>;
}
impl_downcast!(Pack);

#[async_trait]
trait Unpack {
    async fn unpack(field: &mut Field, val: JsValue) -> Option<Box<Self>>;
}

trait Block: Pack + Unpack {}

type Timestamp = f64;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BlockId(U128Id);

struct FieldBlock {
    timestamp: Timestamp,
    payload: Option<Box<dyn Pack>>,
}

struct BlockTable(Rc<RefCell<HashMap<U128Id, FieldBlock>>>);

#[derive(Clone)]
pub struct Field {
    table: BlockTable,
}

impl BlockTable {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(HashMap::new())))
    }
}

impl Clone for BlockTable {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Deref for BlockTable {
    type Target = HashMap<U128Id, FieldBlock>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.as_ptr() }
    }
}

impl DerefMut for BlockTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.as_ptr() }
    }
}

impl BlockId {
    fn new(id: U128Id) -> Self {
        Self(id)
    }

    pub fn to_jsvalue(&self) -> JsValue {
        self.0.to_jsvalue()
    }

    pub fn to_id(&self) -> U128Id {
        self.0.clone()
    }
}

impl FieldBlock {
    fn new<T: Block + 'static>(timestamp: f64, block: T) -> Self {
        Self {
            timestamp: timestamp,
            payload: Some(Box::new(block)),
        }
    }

    async fn pack(&self) -> JsValue {
        let (payload, type_name) = if let Some(payload) = self.payload.as_ref() {
            (payload.pack().await, payload.name())
        } else {
            (js_sys::Object::new().into(), "_")
        };
        let timestamp = self.timestamp;

        (object! {
            type_name: type_name,
            timestamp: timestamp,
            payload: payload
        })
        .into()
    }

    async fn unpack(field: &mut Field, val: JsValue) -> Option<Self> {
        if let Ok(val) = val.dyn_into::<JsObject>() {
            let type_name = val.get("type_name").and_then(|x| x.as_string());
            let timestamp = val.get("timestamp").and_then(|x| x.as_f64());
            let payload = val.get("payload").map(|x| {
                let x: js_sys::Object = x.into();
                let x: JsValue = x.into();
                x
            });
            if let Some((type_name, timestamp, payload)) = join_some!(type_name, timestamp, payload)
            {
            }
        }

        None
    }
}

impl Field {
    pub fn new() -> Self {
        Self {
            table: BlockTable::new(),
        }
    }

    pub fn block_id(&mut self, id: U128Id) -> BlockId {
        BlockId::new(id)
    }

    pub fn add<T: Block + 'static>(&mut self, block: T) -> BlockId {
        let block_id = self.block_id(U128Id::new());
        self.assign(block_id.clone(), Date::now(), block);
        block_id
    }

    pub fn assign<T: Block + 'static>(
        &mut self,
        block_id: BlockId,
        timestamp: Timestamp,
        block: T,
    ) {
        let block = FieldBlock::new(timestamp, block);
        self.assign_fb(block_id, block);
    }

    pub fn assign_fb(&mut self, block_id: BlockId, block: FieldBlock) {
        if let Some(field_block) = self.table.get_mut(&block_id.to_id()) {
            let timestamp = block.timestamp;
            let payload = block.payload;
            if field_block.timestamp < timestamp {
                field_block.timestamp = timestamp;
                field_block.payload = payload;
            }
        } else {
            self.table.insert(block_id.to_id(), block);
        }
    }

    pub fn get<T: Block + 'static>(&self, block_id: &BlockId) -> Option<&T> {
        self.table
            .get(&block_id.to_id())
            .and_then(|fb| fb.payload.as_ref())
            .and_then(|p| p.downcast_ref::<T>())
    }

    pub fn dependents_of<T: Block + 'static>(&self, block_id: &BlockId) -> HashSet<BlockId> {
        self.table
            .get(&block_id.to_id())
            .and_then(|fb| fb.payload.as_ref())
            .and_then(|p| p.downcast_ref::<T>())
            .map(|p| p.dependents(self))
            .unwrap_or(set! {})
    }

    pub fn resources_of<T: Block + 'static>(&self, block_id: &BlockId) -> HashSet<ResourceId> {
        self.table
            .get(&block_id.to_id())
            .and_then(|fb| fb.payload.as_ref())
            .and_then(|p| p.downcast_ref::<T>())
            .map(|p| p.resources(self))
            .unwrap_or(set! {})
    }

    pub fn remove(&mut self, block_id: &BlockId) {
        self.table.get_mut(&block_id.to_id()).map(|fb| {
            fb.payload = None;
        });
    }

    pub fn all<T: Block + 'static>(&self) -> Vec<(BlockId, &T)> {
        self.table
            .iter()
            .filter_map(|(id, fb)| {
                if let Some(b) = fb.payload.as_ref().and_then(|p| p.downcast_ref::<T>()) {
                    let block_id = BlockId::new(id.clone());
                    Some((block_id, b))
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
            .get_mut(&block_id.to_id())
            .and_then(|fb| {
                if let Some(timestamp) = timestamp {
                    if fb.timestamp < timestamp {
                        fb.timestamp = timestamp;
                        fb.payload.as_mut()
                    } else {
                        None
                    }
                } else {
                    fb.payload.as_mut()
                }
            })
            .and_then(|p| p.downcast_mut::<T>())
            .map(move |b| {
                f(b);
                None
            })
            .unwrap_or(Some(self))
    }

    pub fn timestamp(&self, block_id: &BlockId) -> Option<&Timestamp> {
        self.table.get(&block_id.to_id()).map(|b| &b.timestamp)
    }

    pub async fn pack_all(&mut self) -> HashMap<BlockId, JsValue> {
        let mut keys = set! {};
        for key in self.table.keys().map(|x| x.clone()).collect::<Vec<_>>() {
            let key = self.block_id(key);
            keys.insert(key);
        }
        self.pack_listed(keys).await
    }

    pub async fn pack_listed(&self, block_ids: HashSet<BlockId>) -> HashMap<BlockId, JsValue> {
        let mut futures = vec![];
        for block_id in block_ids {
            if let Some(block) = self.table.get(&block_id.to_id()) {
                futures.push(async {
                    let val = block.pack().await;
                    (block_id, val)
                });
            }
        }
        join_all(futures).await.into_iter().collect()
    }

    pub async fn unpack_listed(
        &mut self,
        blocks: impl Iterator<Item = (U128Id, JsValue)>,
    ) -> HashMap<BlockId, FieldBlock> {
        let mut res = map! {};
        for (block_id, block) in blocks {
            if let Some(block) = FieldBlock::unpack(self, block).await {
                let block_id = self.block_id(block_id);
                res.insert(block_id, block);
            }
        }
        res
    }
}
