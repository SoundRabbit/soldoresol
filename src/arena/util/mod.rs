pub mod cubebox;
pub mod pack;

pub use cubebox::Cubebox;
pub use pack::Pack;

macro_rules! uses {
    {$($path:path;)*} => {
        #[allow(unused_imports)]
        use crate::libs::random_id::U128Id;
        #[allow(unused_imports)]
        use async_trait::async_trait;
        #[allow(unused_imports)]
        use std::cell::RefCell;
        #[allow(unused_imports)]
        use std::rc::Rc;
        #[allow(unused_imports)]
        use wasm_bindgen::prelude::*;

        $(
            use $path;
        )*
    };
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
                Self {
                    $($p_c_name,)*
                    $($p_d_name: $p_default,)*
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
            async fn pack(&self, is_deep: bool) -> JsValue {
                let object = object! {};

                $(
                    object.set(stringify!($p_c_name), &self.$p_c_name.pack(is_deep).await);
                )*

                $(
                    object.set(stringify!($p_d_name), &self.$p_d_name.pack(is_deep).await);
                )*

                object.into()
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
        uses! {
            std::collections::HashMap;
            std::cell::Cell;
            std::rc::Weak;
            std::marker::PhantomData;
        }

        $(use $m::$b;)*

        enum BlockData {
            None,
            $($b($b),)*
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
        }

        #[async_trait(?Send)]
        impl util::Pack for BlockData {
            async fn pack(&self, is_deep: bool) -> JsValue {
                match self {
                    Self::None => JsValue::null(),
                    $(
                        Self::$b(data) => {
                            data.pack(is_deep).await
                        },
                    )*
                }
            }
        }

        struct AnnotBlockData {
            timestamp: f64,
            block_id: U128Id,
            data: BlockData,
        }

        #[async_trait(?Send)]
        impl util::Pack for AnnotBlockData {
            async fn pack(&self, is_deep: bool) -> JsValue {
                if is_deep {
                    self.data.pack(is_deep).await
                } else {
                    self.block_id.pack(is_deep).await
                }
            }
        }

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

            pub fn update<T>(&mut self, f: impl FnOnce(&mut T)) where Self: Access<T> {
                Access::update(self, f);
            }

            pub fn map<T, U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> where Self: Access<T> {
                Access::map(self, f)
            }

            pub fn id(&self) -> U128Id {
                U128Id::clone(&self.data.borrow().block_id)
            }

            pub fn kind(&self) -> BlockKind {
                self.data.borrow().data.kind()
            }

            fn as_mut<T>(&self) -> BlockMut<T> where Self: From<T> {
                BlockMut {
                    data: Rc::downgrade(&self.data),
                    phantom_data: PhantomData
                }
            }

            fn as_ref<T>(&self) -> BlockRef<T> where Self: From<T> {
                BlockRef {
                    data: self.as_mut()
                }
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for Block {
            async fn pack(&self, is_deep: bool) -> JsValue {
                self.data.pack(is_deep).await
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

        pub trait Access<T> {
            fn update(&mut self, f: impl FnOnce(&mut T));
            fn map<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U>;
        }

        $(
            impl Access<$b> for Block {
                fn update(&mut self, f: impl FnOnce(&mut $b)) {
                    let mut borrow = self.data.borrow_mut();
                    if let BlockData::$b(data) = &mut borrow.data {
                        f(data);
                        let timesamp = borrow.timestamp.max(js_sys::Date::now());
                        borrow.timestamp = timesamp;
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

        impl<T> BlockMut<T> {
            pub fn none() -> Self where Block: From<T>{
                Block::none().as_mut::<T>()
            }
        }

        $(
            impl BlockMut<$b> {
                pub fn update(&mut self, f: impl FnOnce(&mut $b)) {
                    if let Some(self_data) = self.data.upgrade() {
                        let mut borrow = self_data.borrow_mut();
                        if let BlockData::$b(data) = &mut borrow.data {
                            f(data);
                            let timesamp = borrow.timestamp.max(js_sys::Date::now());
                            borrow.timestamp = timesamp;
                        }
                    }
                }

                pub fn map<T>(&self, f: impl FnOnce(& $b) -> T) -> Option<T> {
                    if let Some(self_data) = self.data.upgrade() {
                        if let BlockData::$b(data) = &self_data.borrow().data {
                            return Some(f(data));
                        }
                    }
                    None
                }

                pub fn id(&self) -> U128Id {
                    if let Some(data) = self.data.upgrade() {
                        U128Id::clone(&data.borrow().block_id)
                    } else {
                        U128Id::none()
                    }
                }

                pub fn as_ref(&self) -> BlockRef<$b> {
                    BlockRef {
                        data: BlockMut::clone(self)
                    }
                }
            }

            #[async_trait(?Send)]
            impl util::Pack for BlockMut<$b> {
                async fn pack(&self, is_deep: bool) -> JsValue {
                    if let Some(data) = self.data.upgrade() {
                        data.pack(is_deep).await
                    } else {
                        JsValue::null()
                    }
                }
            }
        )*

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

        impl<T> BlockRef<T> {
            pub fn none() -> Self where Block: From<T>{
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

            #[async_trait(?Send)]
            impl util::Pack for BlockRef<$b> {
                async fn pack(&self, is_deep: bool) -> JsValue {
                    self.data.pack(is_deep).await
                }
            }
        )*

        struct ArenaData {
            data: HashMap<U128Id, Block>
        }

        impl ArenaData {
            fn new() -> Self {
                Self {
                    data: map![]
                }
            }

            pub fn kind_of(&self, block_id: &U128Id) -> BlockKind {
                self.data.get(block_id).map(|block| block.kind()).unwrap_or(BlockKind::None)
            }

            fn get_mut<T>(&mut self, block_id: &U128Id) -> Option<BlockMut<T>> where Block: From<T>{
                if let Some(data) = self.data.get_mut(block_id) {
                    Some(data.as_mut())
                } else {
                    None
                }
            }

            fn get<T>(&self, block_id: &U128Id) -> Option<BlockRef<T>> where Block: From<T> {
                if let Some(data) = self.data.get(block_id) {
                    Some(data.as_ref())
                } else {
                    None
                }
            }

            fn insert<T>(&mut self, block: Block) -> BlockMut<T> where Block: From<T>{
                let block_id = block.id();
                let block_mut = block.as_mut();

                self.data.insert(block_id, block);

                block_mut
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
                self.data.borrow_mut().get(&block_id)
            }

            pub fn insert<T>(&mut self, block: T) -> BlockMut<T> where Self: Insert<T> {
                Insert::insert(self, block)
            }
        }

        pub trait Insert<T> {
            fn insert(&mut self, block: T) -> BlockMut<T>;
        }

        $(
            impl Insert<$b> for Arena {
                fn insert(&mut self, block: $b) -> BlockMut<$b> {
                    let block = Block::from(block);
                    self.data.borrow_mut().insert(block)
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
                self.data.upgrade().and_then(|data| data.borrow_mut().get(&block_id))
            }

            pub fn insert<T>(&mut self, block: T) -> BlockMut<T> where Self: Insert<T> {
                Insert::insert(self, block)
            }
        }

        $(
            impl Insert<$b> for ArenaMut {
                fn insert(&mut self, block: $b) -> BlockMut<$b> {
                    if let Some(data) = self.data.upgrade() {
                        let block = Block::from(block);
                        data.borrow_mut().insert(block)
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
