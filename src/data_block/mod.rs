use crate::{js_object::JsObject, random_id};
use js_sys::Date;
use std::{any::Any, collections::HashMap, iter::Iterator};
use wasm_bindgen::{prelude::*, JsCast};

pub mod chat;
pub mod property;
pub mod resource;
pub mod table;
pub mod table_object;

pub use chat::Chat;
pub use property::Property;
pub use resource::Resource;
pub use table::Table;

trait Block {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static);
    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static);
}

type Timestamp = u32;

struct FieldBlock {
    timestamp: Timestamp,
    payload: Box<dyn Any>,
}

type BlockId = u128;

pub struct Field {
    blocks: HashMap<BlockId, FieldBlock>,
}

impl FieldBlock {
    pub fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {
        if let Some(payload) = self.payload.downcast_ref::<Chat>() {
        } else if let Some(payload) = self.payload.downcast_ref::<chat::Item>() {
        } else if let Some(payload) = self.payload.downcast_ref::<chat::Tab>() {
        } else if let Some(payload) = self.payload.downcast_ref::<Table>() {
        } else if let Some(payload) = self.payload.downcast_ref::<table::Texture>() {
        } else if let Some(payload) = self.payload.downcast_ref::<table_object::Character>() {
        } else if let Some(payload) = self.payload.downcast_ref::<Property>() {
        } else if let Some(payload) = self.payload.downcast_ref::<Resource>() {
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
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn add<T: Block + 'static>(&mut self, block: T) -> BlockId {
        let block_id = random_id::u128val();
        if !self.blocks.contains_key(&block_id) {
            self.assign(block_id, Date::now() as u32, block);
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
            .blocks
            .get(&block_id)
            .map(|b| b.timestamp < timestamp)
            .unwrap_or(true)
        {
            let block = FieldBlock {
                timestamp: timestamp,
                payload: Box::new(block),
            };
            self.blocks.insert(block_id, block);
        }
    }

    pub fn get<T: Block + 'static>(&self, block_id: &BlockId) -> Option<&T> {
        self.blocks
            .get(block_id)
            .and_then(|fb| fb.payload.downcast_ref::<T>())
    }

    pub fn all<T: Block + 'static>(&self) -> impl Iterator<Item = (&BlockId, &T)> {
        self.blocks.iter().filter_map(|(id, fb)| {
            if let Some(b) = fb.payload.downcast_ref::<T>() {
                Some((id, b))
            } else {
                None
            }
        })
    }

    pub fn listed<T: Block + 'static>(
        &self,
        block_ids: &Vec<BlockId>,
    ) -> impl Iterator<Item = (BlockId, &T)> {
        let mut blocks = vec![];
        for block_id in block_ids {
            if let Some(block) = self.get(block_id) {
                blocks.push((*block_id, block))
            }
        }
        blocks.into_iter()
    }

    pub fn update<T: Block + 'static>(
        &mut self,
        block_id: &BlockId,
        timestamp: Timestamp,
        f: impl FnOnce(&mut T),
    ) -> bool {
        self.blocks
            .get_mut(block_id)
            .and_then(|fb| {
                if fb.timestamp < timestamp {
                    fb.payload.downcast_mut::<T>()
                } else {
                    None
                }
            })
            .map(move |b| {
                f(b);
                true
            })
            .unwrap_or(false)
    }

    pub fn timestamp(&self, block_id: &BlockId) -> Option<&Timestamp> {
        self.blocks.get(block_id).map(|b| &b.timestamp)
    }
}
