use super::Insert;
use crate::libs::js_object::JsObject;
use crate::libs::random_id::U128Id;
use crate::libs::try_ref::{TryMut, TryRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod boxblock;
pub mod character;
pub mod chat;
pub mod pointlight;
pub mod property;
pub mod table;
pub mod tag;
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
    Tag(tag::Tag),
    Boxblock(boxblock::Boxblock),
    Property(property::Property),
    Pointlight(pointlight::Pointlight),
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
            Self::Tag(block) => Self::Tag(tag::Tag::clone(block)),
            Self::Boxblock(block) => Self::Boxblock(boxblock::Boxblock::clone(block)),
            Self::Property(block) => Self::Property(property::Property::clone(block)),
            Self::Pointlight(block) => Self::Pointlight(pointlight::Pointlight::clone(block)),
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
try_ref_mut!(Block: Tag => tag::Tag);
try_ref_mut!(Block: Boxblock => boxblock::Boxblock);
try_ref_mut!(Block: Property => property::Property);
try_ref_mut!(Block: Pointlight => pointlight::Pointlight);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    id: U128Id,
}

impl BlockId {
    fn new(id: U128Id) -> Self {
        Self { id }
    }

    fn from_str(id: &str) -> Option<Self> {
        U128Id::from_hex(id).map(|id| Self { id })
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

pub struct ArenaBlock {
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

    pub fn is<T>(&self) -> bool
    where
        Block: TryRef<T>,
    {
        TryRef::try_ref(&self.payload).is_some()
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
            Block::Tag(x) => (object! {}.into(), "None"),
            Block::Boxblock(x) => (object! {}.into(), "None"),
            Block::Property(x) => (object! {}.into(), "None"),
            Block::Pointlight(x) => (object! {}.into(), "None"),
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

    async fn pack_to_toml(&self) -> toml::Value {
        let mut packed = toml::value::Table::new();

        match &self.payload {
            Block::Character(x) => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("Character")),
                );
                packed.insert(String::from("payload"), x.pack_to_toml().await);
            }
            Block::Property(x) => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("Property")),
                );
                packed.insert(String::from("payload"), x.pack_to_toml().await);
            }
            _ => {
                packed.insert(
                    String::from("_type"),
                    toml::Value::String(String::from("None")),
                );
            }
        }

        toml::Value::Table(packed)
    }

    async fn unpack_from_toml(packed: toml::Value) -> Self {
        let mut unpacked = Self::new(js_sys::Date::now(), Block::None);

        if let toml::Value::Table(mut packed) = packed {
            if let Some(toml::Value::String(block_type)) = packed.remove("_type") {
                if block_type == "Character" {
                    if let Some(payload) = packed.remove("payload") {
                        unpacked.payload =
                            Block::Character(character::Character::unpack_from_toml(payload).await);
                    }
                } else if block_type == "Property" {
                    if let Some(payload) = packed.remove("payload") {
                        unpacked.payload =
                            Block::Property(property::Property::unpack_from_toml(payload).await);
                    }
                }
            }
        }

        unpacked
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

        self.assign_block(BlockId::clone(&block_id), block);

        block_id
    }

    fn assign_block(&mut self, block_id: BlockId, block: Block) {
        let arena_block = ArenaBlock::new(js_sys::Date::now(), block);
        self.assign_arena_block(BlockId::clone(&block_id), arena_block);
    }

    pub fn assign_arena_block(&mut self, block_id: BlockId, new_arena_block: ArenaBlock) {
        let mut table = self.table.borrow_mut();
        if let Some(arena_block) = table.get_mut(&block_id) {
            if arena_block.timestamp < new_arena_block.timestamp {
                arena_block.timestamp = new_arena_block.timestamp;
                arena_block.payload = new_arena_block.payload;
            }
        } else {
            crate::debug::log_1("assign arena block");
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

    pub fn free(&mut self, block_id: &BlockId) {
        self.assign_block(BlockId::clone(block_id), Block::None)
    }

    pub fn pack_to_toml(
        &self,
        block_ids: impl Iterator<Item = BlockId>,
    ) -> impl FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = toml::Value>>> {
        let mut blocks = vec![];
        for block_id in block_ids {
            if let Some(block) = self.table.borrow().get(&block_id) {
                blocks.push((block_id, ArenaBlock::clone(block)));
            }
        }
        move || {
            Box::pin(async move {
                let mut packed = toml::value::Array::new();

                for (block_id, block) in blocks {
                    let mut packed_block = toml::value::Table::new();
                    packed_block.insert(
                        String::from("_id"),
                        toml::Value::String(block_id.to_string()),
                    );
                    packed_block.insert(String::from("_payload"), block.pack_to_toml().await);
                    packed.push(toml::Value::Table(packed_block));
                }

                let array = toml::Value::Array(packed);
                let mut table = toml::value::Table::new();
                table.insert(String::from("block"), array);
                toml::Value::Table(table)
            })
        }
    }

    pub async fn unpack_from_toml(packed: toml::Value) -> Vec<(BlockId, ArenaBlock)> {
        let mut unpacked = vec![];

        if let toml::Value::Table(mut packed) = packed {
            if let Some(toml::Value::Array(packed)) = packed.remove("block") {
                for packed_block in packed {
                    if let toml::Value::Table(mut packed_block) = packed_block {
                        if let (Some(toml::Value::String(block_id)), Some(payload)) =
                            (packed_block.remove("_id"), packed_block.remove("_payload"))
                        {
                            if let Some(block_id) = BlockId::from_str(&block_id) {
                                let payload = ArenaBlock::unpack_from_toml(payload).await;
                                unpacked.push((block_id, payload));
                            }
                        }
                    }
                }
            }
        }

        unpacked
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
insert!(Arena: tag::Tag => Tag);
insert!(Arena: boxblock::Boxblock => Boxblock);
insert!(Arena: property::Property => Property);
insert!(Arena: pointlight::Pointlight => Pointlight);
