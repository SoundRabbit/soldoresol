use crate::{js_object::JsObject, random_id, Promise};
use js_sys::Date;
use std::{any::Any, collections::HashMap, hash::Hash, iter::Iterator};
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

#[allow(private_in_public)]
trait Block {
    fn pack(&self) -> Promise<JsValue>;
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>>;
}

#[allow(private_in_public)]
type Timestamp = f64;

#[allow(private_in_public)]
type BlockTable = HashMap<u128, FieldBlock>;

#[allow(private_in_public)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BlockId(u128);

#[allow(private_in_public)]
pub struct FieldBlock {
    timestamp: Timestamp,
    payload: Option<Box<dyn Any>>,
}

#[allow(private_in_public)]
pub struct Field {
    table: BlockTable,
}

impl BlockId {
    fn new(id: u128) -> Self {
        Self(id)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_u128(&self) -> u128 {
        self.0
    }
}

impl FieldBlock {
    fn new<T: Block + 'static>(timestamp: f64, block: T) -> Self {
        Self {
            timestamp: timestamp,
            payload: Some(Box::new(block)),
        }
    }

    fn pack(&self) -> Promise<JsValue> {
        let payload = self.payload.as_ref();
        let (promise, type_name) = if let Some(payload) =
            payload.and_then(|p| p.downcast_ref::<Chat>())
        {
            (payload.pack(), "Chat")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<chat::Item>()) {
            (payload.pack(), "chat::Item")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<chat::Tab>()) {
            (payload.pack(), "chat::Tab")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Table>()) {
            (payload.pack(), "Table")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<table::Texture>()) {
            (payload.pack(), "table::Texture")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<table_object::Area>()) {
            (payload.pack(), "table_object::Area")
        } else if let Some(payload) =
            payload.and_then(|p| p.downcast_ref::<table_object::Boxblock>())
        {
            (payload.pack(), "table_object::Boxblock")
        } else if let Some(payload) =
            payload.and_then(|p| p.downcast_ref::<table_object::Tablemask>())
        {
            (payload.pack(), "table_object::Tablemask")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Character>()) {
            (payload.pack(), "Character")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Property>()) {
            (payload.pack(), "Property")
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<World>()) {
            (payload.pack(), "World")
        } else {
            (
                Promise::new(|resolve| resolve(Some(js_sys::Object::new().into()))),
                "_",
            )
        };
        let timestamp = self.timestamp;
        let promise = promise.map(move |x| {
            x.map(|payload| {
                object! {
                    type_name: type_name,
                    timestamp: timestamp,
                    payload: payload
                }
            })
        });
        promise.map(|x| {
            x.map(|x| {
                let x: js_sys::Object = x.into();
                x.into()
            })
        })
    }

    fn unpack(field: &mut Field, val: JsValue) -> Promise<Self> {
        if let Ok(val) = val.dyn_into::<JsObject>() {
            let type_name = val.get("type_name").and_then(|x| x.as_string());
            let timestamp = val.get("timestamp").and_then(|x| x.as_f64());
            let payload = val.get("payload").map(|x| {
                let x: js_sys::Object = x.into();
                let x: JsValue = x.into();
                x
            });
            if let (Some(type_name), Some(timestamp), Some(payload)) =
                (type_name, timestamp, payload)
            {
                let promise = match type_name.as_str() {
                    "Chat" => Chat::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>)),
                    "chat::Item" => {
                        chat::Item::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>))
                    }
                    "chat::Tab" => {
                        chat::Tab::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>))
                    }
                    "Table" => Table::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>)),
                    "table::Texture" => {
                        table::Texture::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>))
                    }
                    "table_object::Area" => table_object::Area::unpack(field, payload)
                        .map(|x| x.map(|x| x as Box<dyn Any>)),
                    "table_object::Boxblock" => table_object::Boxblock::unpack(field, payload)
                        .map(|x| x.map(|x| x as Box<dyn Any>)),
                    "table_object::Tablemask" => table_object::Tablemask::unpack(field, payload)
                        .map(|x| x.map(|x| x as Box<dyn Any>)),
                    "Character" => {
                        Character::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>))
                    }
                    "Property" => {
                        Property::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>))
                    }
                    "World" => World::unpack(field, payload).map(|x| x.map(|x| x as Box<dyn Any>)),
                    _ => Promise::new(|resolve| resolve(None)),
                };
                promise.map(move |x| {
                    Some(
                        x.map(|payload| Self {
                            timestamp,
                            payload: Some(payload),
                        })
                        .unwrap_or(Self {
                            timestamp,
                            payload: None,
                        }),
                    )
                })
            } else {
                Promise::new(|resolve| resolve(None))
            }
        } else {
            Promise::new(|resolve| resolve(None))
        }
    }
}

impl Field {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn block_id(&mut self, id: u128) -> BlockId {
        BlockId::new(id)
    }

    #[allow(private_in_public)]
    pub fn add<T: Block + 'static>(&mut self, block: T) -> BlockId {
        let block_id = self.block_id(random_id::u128val());
        self.assign(block_id, Date::now(), block);
        block_id
    }

    #[allow(private_in_public)]
    pub fn assign<T: Block + 'static>(
        &mut self,
        block_id: BlockId,
        timestamp: Timestamp,
        block: T,
    ) {
        let block = FieldBlock::new(timestamp, block);
        self.assign_fb(block_id, block);
    }

    #[allow(private_in_public)]
    pub fn assign_fb(&mut self, block_id: BlockId, block: FieldBlock) {
        if let Some(field_block) = self.table.get_mut(&block_id.to_u128()) {
            let timestamp = block.timestamp;
            let payload = block.payload;

            crate::debug::log_1(format!("{}, {}", timestamp, field_block.timestamp));

            if field_block.timestamp < timestamp {
                field_block.timestamp = timestamp;
                field_block.payload = payload;
            }
        } else {
            self.table.insert(block_id.to_u128(), block);
        }
    }

    #[allow(private_in_public)]
    pub fn get<T: Block + 'static>(&self, block_id: &BlockId) -> Option<&T> {
        self.table
            .get(&block_id.to_u128())
            .and_then(|fb| fb.payload.as_ref())
            .and_then(|p| p.downcast_ref::<T>())
    }

    pub fn remove(&mut self, block_id: &BlockId) {
        self.table.get_mut(&block_id.to_u128()).map(|fb| {
            fb.payload = None;
        });
    }

    #[allow(private_in_public)]
    pub fn all<T: Block + 'static>(&self) -> Vec<(BlockId, &T)> {
        self.table
            .iter()
            .filter_map(|(id, fb)| {
                if let Some(b) = fb.payload.as_ref().and_then(|p| p.downcast_ref::<T>()) {
                    let block_id = BlockId::new(*id);
                    Some((block_id, b))
                } else {
                    None
                }
            })
            .collect()
    }

    #[allow(private_in_public)]
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

    #[allow(private_in_public)]
    pub fn update<T: Block + 'static>(
        &mut self,
        block_id: &BlockId,
        timestamp: Option<Timestamp>,
        f: impl FnOnce(&mut T),
    ) -> Option<&mut Self> {
        self.table
            .get_mut(&block_id.to_u128())
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
        self.table.get(&block_id.to_u128()).map(|b| &b.timestamp)
    }

    pub fn pack_all(&mut self) -> Promise<Vec<(BlockId, JsValue)>> {
        let mut keys = vec![];
        for key in self.table.keys().map(|x| *x).collect::<Vec<_>>() {
            let key = self.block_id(key);
            keys.push(key);
        }
        self.pack_listed(keys.iter().collect())
    }

    pub fn pack_listed(&self, block_ids: Vec<&BlockId>) -> Promise<Vec<(BlockId, JsValue)>> {
        let mut promises = vec![];
        for block_id in block_ids {
            if let Some(block) = self.table.get(&block_id.to_u128()) {
                let block_id = block_id.clone();
                promises.push(block.pack().map(move |res| res.map(|val| (block_id, val))));
            }
        }
        Promise::some(promises)
            .map(|vals| vals.map(|vals| vals.into_iter().filter_map(|x| x).collect()))
    }

    pub fn unpack_listed(
        &mut self,
        blocks: impl Iterator<Item = (u128, JsValue)>,
    ) -> Promise<HashMap<BlockId, FieldBlock>> {
        let mut promises = vec![];
        for (block_id, val) in blocks {
            let block_id = self.block_id(block_id);
            promises
                .push(FieldBlock::unpack(self, val).map(move |res| res.map(|val| (block_id, val))));
        }
        Promise::some(promises)
            .map(|vals| vals.map(|vals| vals.into_iter().filter_map(|x| x).collect()))
    }
}
