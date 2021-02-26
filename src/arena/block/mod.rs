use super::Insert;
use crate::libs::js_object::JsObject;
use crate::libs::random_id::U128Id;
use crate::libs::try_ref::{TryMut, TryRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod character;
pub mod chat;
pub mod table;
pub mod texture;
pub mod world;

pub enum Block {
    World(world::World),
    Table(table::Table),
    Texture(texture::Texture),
    Chat(chat::Chat),
    ChatChannel(chat::channel::Channel),
    ChatMessage(chat::message::Message),
    Character(character::Character),
    None,
}

impl Block {
    fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    fn clone(this: &Self) -> Self {
        match this {
            Self::World(block) => Self::World(world::World::clone(block)),
            Self::Table(block) => Self::Table(table::Table::clone(block)),
            Self::Texture(block) => Self::Texture(texture::Texture::clone(block)),
            Self::Chat(block) => Self::Chat(chat::Chat::clone(block)),
            Self::ChatChannel(block) => Self::ChatChannel(chat::channel::Channel::clone(block)),
            Self::ChatMessage(block) => Self::ChatMessage(chat::message::Message::clone(block)),
            Self::Character(block) => Self::Character(character::Character::clone(block)),
            Self::None => Self::None,
        }
    }

    async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    async fn unpack() -> Option<Self> {
        unimplemented!();
    }
}

macro_rules! try_ref {
    ($f:ty : $a:ident => $t:ty) => {
        impl TryRef<$t> for $f {
            fn try_ref(&self) -> Option<&$t> {
                match self {
                    Self::$a(x) => Some(x),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! try_mut {
    ($f:ty : $a:ident => $t:ty) => {
        impl TryMut<$t> for $f {
            fn try_mut(&mut self) -> Option<&mut $t> {
                match self {
                    Self::$a(x) => Some(x),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! try_ref_mut {
    ($f:ty : $a:ident => $t:ty) => {
        try_ref!($f : $a => $t);
        try_mut!($f : $a => $t);
    };
}

try_ref_mut!(Block: World => world::World);
try_ref_mut!(Block: Table => table::Table);
try_ref_mut!(Block: Texture => texture::Texture);
try_ref_mut!(Block: Chat => chat::Chat);
try_ref_mut!(Block: ChatChannel => chat::channel::Channel);
try_ref_mut!(Block: ChatMessage => chat::message::Message);
try_ref_mut!(Block: Character => character::Character);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    id: U128Id,
}

impl BlockId {
    fn new(id: U128Id) -> Self {
        Self { id }
    }

    pub fn none() -> Self {
        Self { id: U128Id::none() }
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

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

type Timestamp = f64;

struct ArenaBlock {
    timestamp: Timestamp,
    payload: Block,
}

impl ArenaBlock {
    fn new(timestamp: f64, block: Block) -> Self {
        Self {
            timestamp: timestamp,
            payload: block,
        }
    }

    fn clone(this: &Self) -> Self {
        Self {
            timestamp: this.timestamp,
            payload: Block::clone(&this.payload),
        }
    }

    async fn pack(&self) -> JsValue {
        let (payload, type_name) = match &self.payload {
            Block::World(x) => (x.pack().await, "World"),
            Block::Table(x) => (x.pack().await, "Table"),
            Block::Texture(x) => (x.pack().await, "Texture"),
            Block::Chat(x) => (object! {}.into(), "None"),
            Block::ChatChannel(x) => (object! {}.into(), "None"),
            Block::ChatMessage(x) => (object! {}.into(), "None"),
            Block::Character(x) => (object! {}.into(), "None"),
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
            payload: payload,
        })
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
    table: Rc<RefCell<HashMap<BlockId, ArenaBlock>>>,
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

    pub fn as_ref(&self) -> ArenaRef {
        ArenaRef {
            arena: Self::clone(self),
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

    pub fn iter_map_with_ids<T, U>(
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

    pub fn map_mut<T, U>(&mut self, block_id: &BlockId, f: impl FnOnce(&mut T) -> U) -> Option<U>
    where
        Block: TryMut<T>,
    {
        let mut table = self.table.borrow_mut();
        if let Some(arena_block) = table.get_mut(&block_id) {
            if let Some(block) = arena_block.payload.try_mut() {
                arena_block.timestamp = js_sys::Date::now();
                return Some(f(block));
            }
        }
        None
    }
}

macro_rules! insert {
    ($t:ty : $f:ty => $a:ident) => {
        impl Insert<$f> for $t {
            type Id = BlockId;
            fn insert(&mut self, block: $f) -> BlockId {
                self.insert(Block::$a(block))
            }
        }
    };
}

insert!(Arena: world::World => World);
insert!(Arena: table::Table => Table);
insert!(Arena: texture::Texture => Texture);
insert!(Arena: chat::Chat => Chat);
insert!(Arena: chat::channel::Channel => ChatChannel);
insert!(Arena: character::Character => Character);
