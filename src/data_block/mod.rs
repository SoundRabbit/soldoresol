use crate::random_id;
use js_sys::Date;
use std::{any::Any, collections::HashMap, iter::Iterator};
use wasm_bindgen::prelude::*;

trait Block: From<JsValue> {
    fn pack(&self) -> JsValue;
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

impl Field {
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
}
