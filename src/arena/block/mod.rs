use crate::libs::js_object::JsObject;
use crate::libs::random_id::U128Id;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod chat;
pub mod table;
pub mod world;

trait TryRef<T> {
    fn try_ref(&self) -> Option<&T>;
}

trait TryMut<T> {
    fn try_mut(&mut self) -> Option<&mut T>;
}

enum Block {
    World(world::World),
    Table(table::Table),
    TableTexture(table::texture::Texture),
    Chat(chat::Chat),
    ChatTab(chat::tab::Tab),
    ChatMessage(chat::message::Message),
    None,
}

impl Block {
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

impl TryRef<world::World> for Block {
    fn try_ref(&self) -> Option<&world::World> {
        match self {
            Self::World(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<table::Table> for Block {
    fn try_ref(&self) -> Option<&table::Table> {
        match self {
            Self::Table(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<table::texture::Texture> for Block {
    fn try_ref(&self) -> Option<&table::texture::Texture> {
        match self {
            Self::TableTexture(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<chat::Chat> for Block {
    fn try_ref(&self) -> Option<&chat::Chat> {
        match self {
            Self::Chat(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<chat::tab::Tab> for Block {
    fn try_ref(&self) -> Option<&chat::tab::Tab> {
        match self {
            Self::ChatTab(x) => Some(x),
            _ => None,
        }
    }
}

impl TryRef<chat::message::Message> for Block {
    fn try_ref(&self) -> Option<&chat::message::Message> {
        match self {
            Self::ChatMessage(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<world::World> for Block {
    fn try_mut(&mut self) -> Option<&mut world::World> {
        match self {
            Self::World(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<table::Table> for Block {
    fn try_mut(&mut self) -> Option<&mut table::Table> {
        match self {
            Self::Table(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<table::texture::Texture> for Block {
    fn try_mut(&mut self) -> Option<&mut table::texture::Texture> {
        match self {
            Self::TableTexture(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<chat::Chat> for Block {
    fn try_mut(&mut self) -> Option<&mut chat::Chat> {
        match self {
            Self::Chat(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<chat::tab::Tab> for Block {
    fn try_mut(&mut self) -> Option<&mut chat::tab::Tab> {
        match self {
            Self::ChatTab(x) => Some(x),
            _ => None,
        }
    }
}

impl TryMut<chat::message::Message> for Block {
    fn try_mut(&mut self) -> Option<&mut chat::message::Message> {
        match self {
            Self::ChatMessage(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    id: U128Id,
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

type Timestamp = f64;

struct ArenaBlock {
    timestamp: Timestamp,
    payload: Rc<Block>,
}

impl ArenaBlock {
    fn new(timestamp: f64, block: Block) -> Self {
        Self {
            timestamp: timestamp,
            payload: Rc::new(block),
        }
    }

    async fn pack(&self) -> JsValue {
        let payload = Rc::clone(&self.payload);
        let (payload, type_name) = match payload.as_ref() {
            Block::World(x) => (x.pack().await, "World"),
            Block::Table(x) => (x.pack().await, "Table"),
            Block::TableTexture(x) => (x.pack().await, "TableTexture"),
            Block::Chat(x) => (object! {}.into(), "None"),
            Block::ChatTab(x) => (object! {}.into(), "None"),
            Block::ChatMessage(x) => (object! {}.into(), "None"),
            Block::None => (object! {}.into(), "None"),
        };

        (object! {
            type_name: type_name,
            timestamp: self.timestamp,
            payload: payload
        })
        .into()
    }

    async fn unpack(val: JsValue) -> Option<Self> {
        let val = unwrap_or!(val.dyn_into::<JsObject>().ok(); None);

        let type_name = unwrap_or!(val.get("type_name").and_then(|x| x.as_string()); None);
        let timestamp = unwrap_or!(val.get("timestamp").and_then(|x| x.as_f64()); None);
        let payload = unwrap_or!(val.get("payload").map(|x| {
            let x: js_sys::Object = x.into();
            let x: JsValue = x.into();
            x
        }); None);

        let payload = match type_name.as_str() {
            "World" => world::World::unpack(payload).await.map(|x| Block::World(x)),
            "None" => Some(Block::None),
            _ => None,
        };

        let payload = unwrap_or!(payload; None);

        Some(Self {
            timestamp,
            payload: Rc::new(payload),
        })
    }
}

pub trait Insert<T> {
    fn insert(&mut self, block: T) -> BlockId;
}

pub struct Arena {
    table: Rc<RefCell<HashMap<BlockId, ArenaBlock>>>,
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

    fn insert(&mut self, block: Block) -> BlockId {
        let block_id = BlockId::new(U128Id::new());
        let arena_block = ArenaBlock::new(js_sys::Date::now(), block);

        self.assign_arena_block(BlockId::clone(&block_id), arena_block);

        block_id
    }

    fn assign_arena_block(&mut self, block_id: BlockId, new_arena_block: ArenaBlock) {
        crate::debug::log_1("assign arena block");
        let mut table = self.table.borrow_mut();
        if let Some(arena_block) = table.get_mut(&block_id) {
            if arena_block.timestamp < new_arena_block.timestamp {
                arena_block.timestamp = new_arena_block.timestamp;
                arena_block.payload = new_arena_block.payload;
            }
        } else {
            table.insert(block_id, new_arena_block);
        }
    }

    pub fn timestamp_of(&self, block_id: &BlockId) -> Option<Timestamp> {
        let table = self.table.borrow();
        let arena_block = unwrap_or!(table.get(block_id); None);
        Some(arena_block.timestamp)
    }

    pub fn map<T, U>(&self, block_id: &BlockId, f: impl FnOnce(&T) -> U) -> Option<U>
    where
        Block: TryRef<T>,
    {
        let table = self.table.borrow();
        let arena_block = unwrap_or!(table.get(block_id); None);
        let block = unwrap_or!(arena_block.payload.try_ref(); None);
        Some(f(block))
    }

    pub async fn iter_map_with_ids<T, U>(
        &self,
        block_ids: impl Iterator<Item = BlockId>,
        mut f: impl FnMut(BlockId, &T) -> U,
    ) -> impl Iterator<Item = U>
    where
        Block: TryRef<T>,
    {
        let mut mapped = vec![];
        {
            for block_id in block_ids {
                if let Some(block) = self
                    .table
                    .borrow()
                    .get(&block_id)
                    .and_then(|ab| ab.payload.try_ref())
                {
                    mapped.push(f(block_id, &block));
                }
            }
        }
        mapped.into_iter()
    }

    pub fn iter_map<T, U>(&self, mut f: impl FnMut(BlockId, &T) -> U) -> impl Iterator<Item = U>
    where
        Block: TryRef<T>,
    {
        self.table
            .borrow()
            .iter()
            .filter_map(move |(block_id, ab)| {
                ab.payload
                    .try_ref()
                    .map(|block| f(BlockId::clone(&block_id), block))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Insert<chat::Chat> for Arena {
    fn insert(&mut self, block: chat::Chat) -> BlockId {
        self.insert(Block::Chat(block))
    }
}

impl Insert<chat::tab::Tab> for Arena {
    fn insert(&mut self, block: chat::tab::Tab) -> BlockId {
        self.insert(Block::ChatTab(block))
    }
}
