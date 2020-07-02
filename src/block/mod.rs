use crate::{js_object::JsObject, random_id, Promise};
use js_sys::Date;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
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
#[derive(Debug)]
pub struct BlockId {
    count: Rc<Cell<usize>>,
    garbage: Rc<RefCell<HashSet<u128>>>,
    internal_id: u128,
}

#[allow(private_in_public)]
struct FieldBlock {
    count: Rc<Cell<usize>>,
    timestamp: Timestamp,
    payload: Option<Box<dyn Any>>,
}

#[allow(private_in_public)]
pub struct Field {
    table: BlockTable,
    garbage: Rc<RefCell<HashSet<u128>>>,
}

impl BlockId {
    fn new(count: Rc<Cell<usize>>, garbage: Rc<RefCell<HashSet<u128>>>, internal_id: u128) -> Self {
        count.set(count.get() + 1);
        Self {
            count,
            garbage,
            internal_id,
        }
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
        let count = self.count.get() + 1;
        self.count.set(count);

        let count = Rc::clone(&self.count);
        let garbage = Rc::clone(&self.garbage);
        let internal_id = self.internal_id;

        Self {
            count,
            internal_id,
            garbage,
        }
    }
}

impl Drop for BlockId {
    fn drop(&mut self) {
        let count = self.count.get() - 1;

        if count > 1 {
            self.count.set(count);
        } else {
            self.garbage.borrow_mut().insert(self.to_u128());
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
    fn new<T: Block + 'static>(timestamp: f64, block: T) -> Self {
        Self {
            count: Rc::new(Cell::new(0)),
            timestamp: timestamp,
            payload: Some(Box::new(block)),
        }
    }

    fn none(timestamp: f64) -> Self {
        Self {
            count: Rc::new(Cell::new(0)),
            timestamp: timestamp,
            payload: None,
        }
    }

    pub fn pack(&self) -> Promise<JsValue> {
        let payload = self.payload.as_ref();
        if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Chat>()) {
            payload.pack()
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<chat::Item>()) {
            payload.pack()
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<chat::Tab>()) {
            payload.pack()
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Table>()) {
            payload.pack()
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<table::Texture>()) {
            payload.pack()
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Character>()) {
            payload.pack()
        } else if let Some(payload) = payload.and_then(|p| p.downcast_ref::<Property>()) {
            payload.pack()
        } else {
            Promise::new(|resolve| resolve(None))
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
    pub fn block_id(&mut self, internal_id: u128) -> BlockId {
        if let Some(field_block) = self.table.get(&internal_id) {
            let count = Rc::clone(&field_block.count);
            let garbage = Rc::clone(&self.garbage);
            BlockId::new(count, garbage, internal_id)
        } else {
            let dummy = FieldBlock::none(0.0);
            let count = Rc::clone(&dummy.count);
            let garbage = Rc::clone(&self.garbage);
            self.table.insert(internal_id, dummy);
            BlockId::new(count, garbage, internal_id)
        }
    }

    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            garbage: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    #[allow(private_in_public)]
    pub fn add<T: Block + 'static>(&mut self, block: T) -> BlockId {
        let block_id = self.block_id(random_id::u128val());
        self.assign(block_id.clone(), Date::now(), block);
        block_id
    }

    #[allow(private_in_public)]
    pub fn assign<T: Block + 'static>(
        &mut self,
        block_id: BlockId,
        timestamp: Timestamp,
        block: T,
    ) {
        if let Some(field_block) = self.table.get_mut(&block_id.to_u128()) {
            if field_block.payload.is_none() || field_block.timestamp < timestamp {
                field_block.timestamp = timestamp;
                field_block.payload = Some(Box::new(block));
            }
        } else {
            let block = FieldBlock::new(timestamp, block);
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
                    let block_id =
                        BlockId::new(Rc::clone(&fb.count), Rc::clone(&self.garbage), *id);
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
}
