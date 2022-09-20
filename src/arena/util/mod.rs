pub mod cubebox;
pub mod pack;

pub use cubebox::Cubebox;
pub use pack::{Pack, PackDepth};

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::super::ArenaMut;
    #[allow(unused_imports)]
    pub use super::super::BlockMut;
    #[allow(unused_imports)]
    pub use crate::libs::random_id::U128Id;
    #[allow(unused_imports)]
    pub use async_trait::async_trait;
    #[allow(unused_imports)]
    pub use std::cell::RefCell;
    #[allow(unused_imports)]
    pub use std::collections::HashSet;
    #[allow(unused_imports)]
    pub use std::rc::Rc;
    #[allow(unused_imports)]
    pub use wasm_bindgen::{prelude::*, JsCast};
}

macro_rules! block {
    {
        [pub $b_name:ident($($options:ident),*)]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        pub struct $b_name {
            $($p_c_name: $p_c_type,)*
            $($p_d_name: $p_d_type,)*
        }

        block! {
            [impl $b_name($($options),*)]
            $(($p_c_name): $p_c_type;)*
            $($p_d_name: $p_d_type = $p_default;)*
        }
    };

    {
        [impl $b_name:ident()]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {};

    {
        [impl $b_name:ident($option:ident$(,$options:ident)+)]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        block! {
            [impl $b_name($option)]
            $(($p_c_name): $p_c_type;)*
            $($p_d_name: $p_d_type = $p_default;)*
        }

        block! {
            [impl $b_name($($options),+)]
            $(($p_c_name): $p_c_type;)*
            $($p_d_name: $p_d_type = $p_default;)*
        }
    };

    {
        [impl $b_name:ident(constructor)]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        impl $b_name {
            pub fn new($($p_c_name: $p_c_type,)*) -> Self {
                $(let $p_d_name = $p_default;)*
                Self {
                    $($p_c_name,)*
                    $($p_d_name,)*
                }
            }
        }
    };

    {
        [impl $b_name:ident(pack)]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        #[async_trait(?Send)]
        impl Pack for $b_name {
            #[allow(unused_variables)]
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                let object = object! {};

                $(
                    object.set(stringify!($p_c_name), &self.$p_c_name.pack(pack_depth).await);
                )*

                $(
                    object.set(stringify!($p_d_name), &self.$p_d_name.pack(pack_depth).await);
                )*

                object.into()
            }

            async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
                if let Some(data) = data.dyn_ref::<crate::libs::js_object::Object>() {
                    $(
                        let $p_c_name = if let Some(item) = data.get(stringify!($p_c_name)) {
                            <$p_c_type as Pack>::unpack(&item, ArenaMut::clone(&arena)).await
                        } else {
                            None
                        };
                        let $p_c_name = unwrap!($p_c_name; None);
                        let $p_c_name = *$p_c_name;
                    )*
                    $(
                        let $p_d_name = if let Some(item) = data.get(stringify!($p_d_name)) {
                            <$p_d_type as Pack>::unpack(&item, ArenaMut::clone(&arena)).await
                        } else {
                            None
                        };
                        let $p_d_name = unwrap!($p_d_name; None);
                        let $p_d_name = *$p_d_name;
                    )*
                    let this = Self {
                        $($p_c_name,)*
                        $($p_d_name,)*
                    };
                    Some(Box::new(this))
                } else {
                    None
                }
            }
        }
    };

    {
        [impl $b_name:ident(component)]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        block! {
            [pub Component(constructor, pack)]
            (origin): $b_name;
            children: Vec<BlockMut<$b_name>> = vec![];
        }

        impl Component {
            #[allow(unused_variables)]
            pub fn update(&mut self, mut f: impl FnMut(&mut $b_name)) -> HashSet<U128Id> {
                let mut update_blocks = HashSet::new();

                f(&mut self.origin);
                for child in &mut self.children {
                    child.update(&mut f);
                    update_blocks.insert(child.id());
                }

                update_blocks
            }

            #[allow(unused_variables)]
            pub fn push(&mut self, child: BlockMut<$b_name>) {
                self.children.push(child);
            }

            #[allow(unused_variables)]
            pub fn remove(&mut self, block_id: &U128Id) {

            }
        }

        impl std::ops::Deref for Component {
            type Target = $b_name;
            fn deref(&self) -> &Self::Target {
                &self.origin
            }
        }

        impl std::ops::DerefMut for Component {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.origin
            }
        }

        pub enum Block {
            Block(BlockMut<$b_name>),
            Component(BlockMut<Component>)
        }

        impl Block {
            pub fn id(&self) -> U128Id {
                match self {
                    Self::Block(x) => x.id(),
                    Self::Component(x) => x.id()
                }
            }

            pub fn map<T>(&self, f: impl FnOnce(&$b_name) -> T) -> Option<T> {
                match self {
                    Self::Block(x) => x.map(f),
                    Self::Component(x) => x.map(|x| f(x))
                }
            }

            pub fn update(&mut self, f: impl FnMut(&mut $b_name)) -> HashSet<U128Id> {
                let mut update_ids = HashSet::new();

                match self {
                    Self::Block(x) => {
                        if x.update(f) {
                            update_ids.insert(x.id());
                        }
                    }
                    Self::Component(x) => {
                        update_ids.insert(x.id());
                        if x.update(|x| {
                            update_ids.extend(x.update(f));
                        }) {
                            update_ids.insert(x.id());
                        }
                    }
                }

                update_ids
            }
        }

        impl Clone for Block {
            fn clone(&self) -> Self {
                match self {
                    Self::Block(x) => Self::Block(BlockMut::clone(x)),
                    Self::Component(x) => Self::Component(BlockMut::clone(x)),
                }
            }
        }
    };
}

macro_rules! mods {
    {
        $(pub $m:ident::$b:ident;)*
    } => {
        #[allow(unused_imports)]
        use super::util;
        #[allow(unused_imports)]
        use super::BlockRef;
        #[allow(unused_imports)]
        use super::BlockMut;

        $(pub mod $m;)*
        $(pub use $m::$b;)*
    };
}

macro_rules! arena {
    {
        $(pub $m:ident::$b:ident;)*
    } => {
        use std::collections::HashMap;
        use std::rc::Weak;
        use std::marker::PhantomData;

        $(use $m::$b;)*

        enum BlockData {
            None,
            $($b($b),)*
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum BlockKind {
            None,
            $($b,)*
        }

        impl BlockData {
            fn kind(&self) -> BlockKind {
                match self {
                    Self::None => BlockKind::None,
                    $(Self::$b(..) => BlockKind::$b,)*
                }
            }

            fn kind_of(data: &JsValue) -> BlockKind {
                    data
                        .dyn_ref::<crate::libs::js_object::Object>()
                        .and_then(|data| data
                            .get("_tag")
                            .and_then(|x| x.as_string())
                            .map(|x| match x.as_str() {
                                $(
                                    stringify!($b) => { BlockKind::$b }
                                )*
                                _ => BlockKind::None
                            })
                        )
                        .unwrap_or(BlockKind::None)
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for BlockData {
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                match self {
                    Self::None => (object! {
                        "_tag": "None",
                        "_val": JsValue::null()
                    }).into(),
                    $(
                        Self::$b(data) => (object!{
                            "_tag": stringify!($b),
                            "_val": data.pack(pack_depth).await
                        }).into(),
                    )*
                }
            }

            async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
                if let Some(data) = data.dyn_ref::<crate::libs::js_object::Object>() {
                    let tag = data.get("_tag").and_then(|x| x.as_string());
                    let val = data.get("_val");

                    if let Some((tag, val)) = join_some!(tag, val) {
                        match tag.as_str() {
                            "None" => {
                                return Some(Box::new(Self::None));
                            }
                            $(
                                stringify!($b) => {
                                    let val = <$b as Pack>::unpack(&val, ArenaMut::clone(&arena)).await;
                                    if let Some(val) = val {
                                        return Some(Box::new(Self::$b(*val)));
                                    }
                                }
                            )*
                            _ => {}
                        }
                    }
                }
                None
            }
        }

        struct AnnotBlockData {
            timestamp: f64,
            block_id: U128Id,
            data: BlockData,
        }

        impl AnnotBlockData {
            fn kind_of(data: &JsValue) -> BlockKind {
                BlockData::kind_of(data)
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for AnnotBlockData {
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                match pack_depth {
                    PackDepth::Recursive => (object!{
                        "timestamp": self.timestamp,
                        "block_id": self.block_id.pack(PackDepth::Recursive).await,
                        "data": self.data.pack(PackDepth::Recursive).await
                    }).into(),
                    PackDepth::FirstBlock => (object!{
                        "timestamp": self.timestamp,
                        "block_id": self.block_id.pack(PackDepth::OnlyId).await,
                        "data": self.data.pack(PackDepth::OnlyId).await
                    }).into(),
                    PackDepth::OnlyId => self.block_id.pack(PackDepth::OnlyId).await
                }
            }

            async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
                if let Some(data) = data.dyn_ref::<crate::libs::js_object::Object>() {
                    let timestamp = data.get("timestamp").and_then(|x| x.as_f64());
                    let block_id = if let Some(block_id) = data.get("block_id") {
                        U128Id::unpack(&block_id, ArenaMut::clone(&arena)).await
                    } else {
                        None
                    };
                    let data = if let Some(data) = data.get("block_id") {
                        BlockData::unpack(&data, ArenaMut::clone(&arena)).await
                    } else {
                        None
                    };

                    if let Some((timestamp, block_id, data)) = join_some!(timestamp, block_id, data) {
                        let this = Self {
                            timestamp: timestamp,
                            block_id: *block_id,
                            data: *data,
                        };
                        return Some(Box::new(this));
                    }
                }
                None
            }
        }

        pub struct Untyped();
        pub struct NoData();

        pub struct Block {
            data: Rc<RefCell<AnnotBlockData>>
        }

        impl Block {
            fn none() -> Block {
                Self {
                    data: Rc::new(RefCell::new(
                        AnnotBlockData {
                            timestamp: js_sys::Date::now(),
                            block_id: U128Id::none(),
                            data: BlockData::None
                        }
                    ))
                }
            }

            #[allow(unused)]
            pub fn update<T>(&mut self, f: impl FnOnce(&mut T)) -> bool where Self: Access<T> {
                Access::update(self, f)
            }

            #[allow(unused)]
            pub fn map<T, U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> where Self: Access<T> {
                Access::map(self, f)
            }

            #[allow(unused)]
            pub fn timestamp(&self) -> f64 {
                self.data.borrow().timestamp
            }

            pub fn id(&self) -> U128Id {
                U128Id::clone(&self.data.borrow().block_id)
            }

            pub fn kind(&self) -> BlockKind {
                self.data.borrow().data.kind()
            }

            pub fn kind_of(data: &JsValue) -> BlockKind {
                AnnotBlockData::kind_of(data)
            }

            fn as_mut<T>(&self) -> BlockMut<T> {
                BlockMut {
                    data: Rc::downgrade(&self.data),
                    phantom_data: PhantomData
                }
            }

            fn as_untyped_mut(&self) -> BlockMut<Untyped> {
                BlockMut {
                    data: Rc::downgrade(&self.data),
                    phantom_data: PhantomData
                }
            }

            fn as_ref<T>(&self) -> BlockRef<T> {
                BlockRef {
                    data: self.as_mut()
                }
            }

            fn as_untyped_ref(&self) -> BlockRef<Untyped> {
                BlockRef {
                    data: self.as_untyped_mut()
                }
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for Block {
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                self.data.pack(pack_depth).await
            }

            async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
                let annot_block = AnnotBlockData::unpack(data, ArenaMut::clone(&arena)).await;
                if let Some(annot_block) = annot_block {
                    let this = Self {
                        data: Rc::new(RefCell::new(*annot_block))
                    };
                    Some(Box::new(this))
                } else {
                    None
                }
            }
        }

        $(
            impl From<$b> for Block {
                fn from(data: $b) -> Self {
                    Self {
                        data: Rc::new(RefCell::new(
                            AnnotBlockData {
                                timestamp: js_sys::Date::now(),
                                block_id: U128Id::new(),
                                data: BlockData::$b(data)
                            }
                        ))
                    }
                }
            }
        )*

        impl From<NoData> for Block {
            fn from(_data: NoData) -> Self {
                Self::none()
            }
        }

        pub trait Access<T> {
            fn update(&mut self, f: impl FnOnce(&mut T)) -> bool;
            fn map<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U>;
        }

        $(
            impl Access<$b> for Block {
                fn update(&mut self, f: impl FnOnce(&mut $b)) -> bool {
                    let mut borrow = self.data.borrow_mut();
                    if let BlockData::$b(data) = &mut borrow.data {
                        f(data);
                        let timestamp = borrow.timestamp.max(js_sys::Date::now());
                        borrow.timestamp = timestamp;
                        true
                    } else {
                        false
                    }
                }

                fn map<U>(&self, f: impl FnOnce(&$b) -> U) -> Option<U> {
                    match &self.data.borrow().data {
                        BlockData::$b(data) => Some(f(data)),
                        _ => None
                    }
                }
            }
        )*

        pub struct BlockMut<T> {
            data: Weak<RefCell<AnnotBlockData>>,
            phantom_data: PhantomData<T>,
        }

        impl<T> Clone for BlockMut<T> {
            fn clone(&self) -> Self {
                Self {
                    data: Weak::clone(&self.data),
                    phantom_data: PhantomData
                }
            }
        }

        impl<T> std::default::Default for BlockMut<T> {
            fn default() -> Self {
                Self::none()
            }
        }

        impl<T> BlockMut<T> {
            pub fn none() -> Self {
                Block::none().as_mut::<T>()
            }

            pub fn timestamp(&self) -> f64 {
                if let Some(data) = self.data.upgrade() {
                    data.borrow().timestamp
                } else {
                    0.0
                }
            }

            pub fn id(&self) -> U128Id {
                if let Some(data) = self.data.upgrade() {
                    U128Id::clone(&data.borrow().block_id)
                } else {
                    U128Id::none()
                }
            }

            pub fn untyped(self) -> BlockMut<Untyped> {
                BlockMut {
                    data: self.data,
                    phantom_data: PhantomData
                }
            }

            #[allow(unused)]
            pub fn as_ref(&self) -> BlockRef<T> {
                BlockRef {
                    data: BlockMut::clone(self)
                }
            }

            pub fn kind(&self) -> BlockKind {
                if let Some(data) = self.data.upgrade() {
                    data.borrow().data.kind()
                } else {
                    BlockKind::None
                }
            }
        }

        impl BlockMut<Untyped> {
            pub fn type_as<T>(&self) -> BlockMut<T> {
                BlockMut {
                    data: Weak::clone(&self.data),
                    phantom_data: PhantomData
                }
            }
        }

        $(
            impl BlockMut<$b> {
                #[allow(unused)]
                pub fn update(&mut self, f: impl FnOnce(&mut $b)) -> bool {
                    if let Some(self_data) = self.data.upgrade() {
                        let mut borrow = self_data.borrow_mut();
                        if let BlockData::$b(data) = &mut borrow.data {
                            f(data);
                            let timesamp = borrow.timestamp.max(js_sys::Date::now());
                            borrow.timestamp = timesamp;
                            return true;
                        }
                    }
                    false
                }

                #[allow(unused)]
                pub fn map<T>(&self, f: impl FnOnce(& $b) -> T) -> Option<T> {
                    if let Some(self_data) = self.data.upgrade() {
                        if let BlockData::$b(data) = &self_data.borrow().data {
                            return Some(f(data));
                        }
                    }
                    None
                }
            }

            impl Access<$b> for BlockMut<$b> {
                fn update(&mut self, f: impl FnOnce(&mut $b)) -> bool {
                    self.update(f)
                }

                fn map<U>(&self, f: impl FnOnce(&$b) -> U) -> Option<U> {
                    self.map(f)
                }
            }
        )*

        #[async_trait(?Send)]
        impl<T> util::Pack for BlockMut<T> where Block: From<T>{
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                if let Some(data) = self.data.upgrade() {
                    data.pack(pack_depth).await
                } else {
                    JsValue::null()
                }
            }

            async fn unpack(data: &JsValue, mut arena: ArenaMut) -> Option<Box<Self>> {
                if let Some(block) = Block::unpack(&data, ArenaMut::clone(&arena)).await {
                    if let Some(arena) = arena.data.upgrade() {
                        if let Some(prev_block) = arena.borrow().get(&block.data.borrow().block_id) {
                            if prev_block.data.borrow().timestamp < block.data.borrow().timestamp {
                                let mut data = BlockData::None;
                                std::mem::swap(&mut data, &mut block.data.borrow_mut().data);
                                prev_block.data.borrow_mut().timestamp = block.data.borrow().timestamp;
                                prev_block.data.borrow_mut().data = data;
                            }
                            return Some(Box::new(prev_block.as_mut()));
                        }
                        return Some(Box::new(arena.borrow_mut().get_insert(*block)));
                    }
                } else if let Some(block_id) = U128Id::unpack(&data, ArenaMut::clone(&arena)).await {
                    return arena.get_mut::<T>(&block_id).map(|x| Box::new(x));
                }

                None
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for BlockMut<Untyped> {
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                match self.kind() {
                    $(
                        BlockKind::$b => { self.type_as::<$b>().pack(pack_depth).await }
                    )*
                    BlockKind::None => { self.type_as::<NoData>().pack(pack_depth).await }
                }
            }

            async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
                match Block::kind_of(data) {
                    $(
                        BlockKind::$b => {
                            BlockMut::<$b>::unpack(data, ArenaMut::clone(&arena)).await.map(|x| Box::new(x.untyped()))
                        }
                    )*
                    BlockKind::None => {
                        BlockMut::<NoData>::unpack(data, ArenaMut::clone(&arena)).await.map(|x| Box::new(x.untyped()))
                    }
                }
            }
        }

        pub struct BlockRef<T> {
            data: BlockMut<T>
        }

        impl<T> Clone for BlockRef<T> {
            fn clone(&self) -> Self {
                Self {
                    data: BlockMut::clone(&self.data),
                }
            }
        }

        impl<T> std::default::Default for BlockRef<T> {
            fn default() -> Self {
                Self::none()
            }
        }

        impl<T> BlockRef<T> {
            pub fn none() -> Self{
                Self {
                    data: BlockMut::none()
                }
            }
        }

        $(
            impl std::ops::Deref for BlockRef<$b> {

                type Target = BlockMut<$b>;

                fn deref(&self) -> &Self::Target {
                    &self.data
                }
            }
        )*

        #[async_trait(?Send)]
        impl<T> util::Pack for BlockRef<T> where BlockMut<T>: util::Pack {
            async fn pack(&self, pack_depth: PackDepth) -> JsValue {
                self.data.pack(pack_depth).await
            }

            async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
                BlockMut::<T>::unpack(data, ArenaMut::clone(&arena))
                    .await
                    .map(|x| Box::new(BlockMut::<T>::as_ref(&x)))
            }
        }

        struct ArenaData {
            data: HashMap<U128Id, Block>
        }

        impl ArenaData {
            fn new() -> Self {
                Self {
                    data: map![]
                }
            }

            pub fn ids<'a>(&'a self) -> impl Iterator<Item = &'a U128Id> {
                self.data.keys()
            }

            pub fn kind_of(&self, block_id: &U128Id) -> BlockKind {
                self.data.get(block_id).map(|block| block.kind()).unwrap_or(BlockKind::None)
            }

            fn get(&self, block_id: &U128Id) -> Option<&Block> {
                self.data.get(block_id)
            }

            fn get_mut<T>(&mut self, block_id: &U128Id) -> Option<BlockMut<T>> where Block: From<T>{
                if let Some(data) = self.data.get_mut(block_id) {
                    Some(data.as_mut())
                } else {
                    None
                }
            }

            fn get_ref<T>(&self, block_id: &U128Id) -> Option<BlockRef<T>> where Block: From<T> {
                if let Some(data) = self.data.get(block_id) {
                    Some(data.as_ref())
                } else {
                    None
                }
            }

            fn get_insert<T>(&mut self, block: Block) -> BlockMut<T> where Block: From<T>{
                let block_id = block.id();
                let block_mut = block.as_mut();

                self.data.insert(block_id, block);

                block_mut
            }

            pub fn remove(&mut self, block_id: U128Id) {
                self.data.insert(block_id, Block::none());
            }

            fn get_untyped(&self, block_id: &U128Id) -> Option<BlockRef<Untyped>> {
                if let Some(data) = self.data.get(block_id) {
                    Some(data.as_untyped_ref())
                } else {
                    None
                }
            }
        }

        pub struct Arena {
            data: Rc<RefCell<ArenaData>>,
        }

        impl Arena {
            pub fn new() -> Self {
                Self {
                    data: Rc::new(RefCell::new(ArenaData::new())),
                }
            }

            pub fn ids(&self) -> impl Iterator<Item =U128Id> {
                self.data.borrow().ids().map(|id| U128Id::clone(id)).collect::<Vec<_>>().into_iter()
            }

            pub fn as_mut(&self) -> ArenaMut {
                ArenaMut {
                    data: Rc::downgrade(&self.data),
                }
            }

            pub fn as_ref(&self) -> ArenaRef {
                ArenaRef {
                    data: ArenaMut {
                        data: Rc::downgrade(&self.data),
                    }
                }
            }

            pub fn get_mut<T>(&mut self, block_id: &U128Id) -> Option<BlockMut<T>> where Block: From<T> {
                self.data.borrow_mut().get_mut(&block_id)
            }

            pub fn get<T>(&self, block_id: &U128Id) -> Option<BlockRef<T>> where Block: From<T>  {
                self.data.borrow_mut().get_ref(&block_id)
            }

            pub fn insert<T>(&mut self, block: T) -> BlockMut<T> where Self: Insert<T> {
                Insert::insert(self, block)
            }

            pub fn remove(&mut self, block_id: U128Id) {
                self.data.borrow_mut().remove(block_id);
            }

            pub fn get_untyped(&self, block_id: &U128Id) -> Option<BlockRef<Untyped>> {
                self.data.borrow_mut().get_untyped(&block_id)
            }
        }

        pub trait Insert<T> {
            fn insert(&mut self, block: T) -> BlockMut<T>;
        }

        $(
            impl Insert<$b> for Arena {
                fn insert(&mut self, block: $b) -> BlockMut<$b> {
                    let block = Block::from(block);
                    self.data.borrow_mut().get_insert(block)
                }
            }
        )*

        #[derive(Clone)]
        pub struct ArenaMut {
            data: Weak<RefCell<ArenaData>>
        }

        impl ArenaMut {
            pub fn as_ref(&self) -> ArenaRef {
                ArenaRef {
                    data: ArenaMut {
                        data: Weak::clone(&self.data),
                    }
                }
            }

            pub fn kind_of(&self, block_id: &U128Id) -> BlockKind {
                self.data.upgrade().map(|data| data.borrow_mut().kind_of(&block_id)).unwrap_or(BlockKind::None)
            }

            pub fn get_mut<T>(&mut self, block_id: &U128Id) -> Option<BlockMut<T>>  where Block: From<T> {
                self.data.upgrade().and_then(|data| data.borrow_mut().get_mut(&block_id))
            }

            pub fn get<T>(&self, block_id: &U128Id) -> Option<BlockRef<T>> where Block: From<T> {
                self.data.upgrade().and_then(|data| data.borrow_mut().get_ref(&block_id))
            }

            pub fn insert<T>(&mut self, block: T) -> BlockMut<T> where Self: Insert<T> {
                Insert::insert(self, block)
            }

            pub fn remove(&mut self, block_id: U128Id) {
                self.data.upgrade().map(|data| data.borrow_mut().remove(block_id));
            }
        }

        $(
            impl Insert<$b> for ArenaMut {
                fn insert(&mut self, block: $b) -> BlockMut<$b> {
                    if let Some(data) = self.data.upgrade() {
                        let block = Block::from(block);
                        data.borrow_mut().get_insert(block)
                    } else {
                        Block::none().as_mut()
                    }
                }
            }
        )*

        #[derive(Clone)]
        pub struct ArenaRef {
            data: ArenaMut
        }

        impl std::ops::Deref for ArenaRef {
            type Target = ArenaMut;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }
    }
}
