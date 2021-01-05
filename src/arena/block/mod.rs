use crate::random_id::U128Id;
use crate::JsObject;
use async_std::sync::Mutex;
use std::collections::HashMap;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod table;
pub mod world;

trait TryRef<T> {
    fn try_ref(&self) -> Option<&T>;
}

trait TryMut<T> {
    fn try_mut(&mut self) -> Option<&mut T>;
}

enum ImplBlock {
    World(world::World),
    Table(table::Table),
    TableTexture(table::texture::Texture),
    None,
}

pub struct Block {
    payload: Rc<ImplBlock>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    id: U128Id,
}

type Timestamp = f64;

struct ArenaBlock {
    timestamp: Timestamp,
    payload: Block,
}

pub struct Arena {
    table: Arc<Mutex<HashMap<BlockId, Arc<Mutex<ArenaBlock>>>>>,
}

impl ImplBlock {
    fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    async fn unpack() -> Option<Self> {
        unimplemented!();
    }
}

impl TryRef<world::World> for ImplBlock {
    fn try_ref(&self) -> Option<&world::World> {
        match self {
            Self::World(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<table::Table> for ImplBlock {
    fn try_ref(&self) -> Option<&table::Table> {
        match self {
            Self::Table(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<table::texture::Texture> for ImplBlock {
    fn try_ref(&self) -> Option<&table::texture::Texture> {
        match self {
            Self::TableTexture(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<world::World> for ImplBlock {
    fn try_mut(&mut self) -> Option<&mut world::World> {
        match self {
            Self::World(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<table::Table> for ImplBlock {
    fn try_mut(&mut self) -> Option<&mut table::Table> {
        match self {
            Self::Table(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<table::texture::Texture> for ImplBlock {
    fn try_mut(&mut self) -> Option<&mut table::texture::Texture> {
        match self {
            Self::TableTexture(x) => Some(x),
            _ => None,
        }
    }
}

impl Block {
    fn new(payload: ImplBlock) -> Self {
        Self {
            payload: Rc::new(payload),
        }
    }

    pub fn is_none(&self) -> bool {
        self.payload.is_none()
    }

    fn clone(this: &Self) -> Self {
        let payload = Rc::clone(&this.payload);
        Self { payload }
    }

    fn as_ref(&self) -> &ImplBlock {
        &self.payload.as_ref()
    }
}

impl<T> TryRef<T> for Block
where
    ImplBlock: TryRef<T>,
{
    fn try_ref(&self) -> Option<&T> {
        self.payload.try_ref()
    }
}

impl<T> TryMut<T> for Block
where
    ImplBlock: TryMut<T>,
{
    fn try_mut(&mut self) -> Option<&mut T> {
        Rc::get_mut(&mut self.payload).and_then(|impl_block| impl_block.try_mut())
    }
}

impl BlockId {
    fn new(id: U128Id) -> Self {
        Self { id }
    }

    pub fn to_jsvalue(&self) -> JsValue {
        self.id.to_jsvalue()
    }

    pub fn to_id(&self) -> U128Id {
        U128Id::clone(&self.id)
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            id: U128Id::clone(&this.id),
        }
    }
}

impl ArenaBlock {
    fn new(timestamp: f64, block: Block) -> Self {
        Self {
            timestamp: timestamp,
            payload: block,
        }
    }

    async fn pack(&self) -> JsValue {
        let (payload, type_name) = match self.payload.as_ref() {
            ImplBlock::World(world) => (world.pack().await, "World"),
            ImplBlock::None => (object! {}.into(), "None"),
        };

        (object! {
            type_name: type_name,
            timestamp: self.timestamp,
            payload: payload
        })
        .into()
    }

    async fn unpack(val: JsValue) -> Option<Self> {
        let val = unwrap_option!(val.dyn_into::<JsObject>().ok());

        let type_name = unwrap_option!(val.get("type_name").and_then(|x| x.as_string()));
        let timestamp = unwrap_option!(val.get("timestamp").and_then(|x| x.as_f64()));
        let payload = unwrap_option!(val.get("payload").map(|x| {
            let x: js_sys::Object = x.into();
            let x: JsValue = x.into();
            x
        }));

        let payload = match type_name.as_str() {
            "World" => world::World::unpack(payload)
                .await
                .map(|x| ImplBlock::World(x)),
            "None" => Some(ImplBlock::None),
            _ => None,
        };

        let payload = unwrap_option!(payload);

        Some(Self {
            timestamp,
            payload: Block::new(payload),
        })
    }
}

impl Arena {
    pub fn new() -> Self {
        Self {
            table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            table: Arc::clone(&this.table),
        }
    }

    async fn get(&self, block_id: &BlockId) -> Option<Arc<Mutex<ArenaBlock>>> {
        let table = self.table.lock().await;
        table.get(block_id).map(|x| Arc::clone(x))
    }

    pub async fn assign_arena_block(&mut self, block_id: BlockId, new_arena_block: ArenaBlock) {
        if let Some(arena_block) = self.table.lock().await.get(&block_id) {
            let mut arena_block = arena_block.lock().await;
            if arena_block.timestamp < new_arena_block.timestamp {
                arena_block.timestamp = new_arena_block.timestamp;
                arena_block.payload = new_arena_block.payload;
            }
        } else {
            self.table
                .lock()
                .await
                .insert(block_id, Arc::new(Mutex::new(new_arena_block)));
        }
    }

    pub async fn timestamp_of(&self, block_id: &BlockId) -> Option<Timestamp> {
        let arena_block = unwrap_option!(self.get(block_id).await);
        let arena_block = arena_block.lock().await;
        Some(arena_block.timestamp)
    }

    pub async fn map<T, U, A>(&self, block_id: &BlockId, f: impl FnOnce(&T) -> A) -> Option<U>
    where
        Block: TryRef<T>,
        A: Future<Output = U>,
    {
        let arena_block = unwrap_option!(self.get(&block_id).await);
        let arena_block = arena_block.lock().await;
        let block = unwrap_option!(arena_block.payload.try_ref());
        Some(f(block).await)
    }

    pub async fn iter_map_with_ids<T, U>(
        &self,
        block_ids: impl Iterator<Item = BlockId>,
        mut f: impl FnMut(BlockId, &T) -> U,
    ) -> impl Iterator<Item = U>
    where
        Block: TryRef<T>,
    {
        let mut blocks = vec![];
        {
            let table = self.table.lock().await;
            for block_id in block_ids {
                if let Some(arena_block) = table.get(&block_id) {
                    blocks.push((block_id, Arc::clone(arena_block)));
                }
            }
        }
        let mut mapped = vec![];
        for (block_id, arena_block) in blocks {
            if let Some(block) = arena_block.lock().await.payload.try_ref() {
                mapped.push(f(block_id, block));
            }
        }
        mapped.into_iter()
    }

    pub async fn iter_map<T, U>(&self, f: impl FnMut(BlockId, &T) -> U) -> impl Iterator<Item = U>
    where
        Block: TryRef<T>,
    {
        let keys = self
            .table
            .lock()
            .await
            .keys()
            .map(|x| BlockId::clone(x))
            .collect::<Vec<_>>();
        self.iter_map_with_ids(keys.into_iter(), f).await
    }
}
