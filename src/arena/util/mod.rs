pub mod pack;

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
        use wasm_bindgen::JsValue;

        $(
            use $path;
        )*
    };
}

macro_rules! block {
    {
        [$($a:ident)? local $b_name:ident]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        $($a)? struct $b_name {
            $($p_c_name: $p_c_type,)*
            $($p_d_name: $p_d_type,)*
        }

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
        [pub $b_name:ident]
        $(($p_c_name:ident): $p_c_type:ty;)*
        $($p_d_name:ident: $p_d_type:ty = $p_default:expr;)*
    } => {
        block! {
            [pub local $b_name]
            $(($p_c_name): $p_c_type;)*
            $($p_d_name: $p_d_type = $p_default;)*
        }

        #[async_trait(?Send)]
        impl Pack for $b_name {
            async fn pack(&self) -> JsValue {
                let object = object! {};

                $(
                    object.set(stringify!($p_c_name), &self.$p_c_name.pack().await);
                )*

                $(
                    object.set(stringify!($p_d_name), &self.$p_d_name.pack().await);
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
        }

        $(use $m::$b;)*

        enum BlockData {
            None,
            $($b($b),)*
        }

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
            async fn pack(&self) -> JsValue {
                match self {
                    Self::None => JsValue::null(),
                    $(
                        Self::$b(data) => {
                            data.pack().await
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
            async fn pack(&self) -> JsValue {
                self.data.pack().await
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

            fn as_mut(&self) -> BlockMut {
                BlockMut {
                    data: Rc::downgrade(&self.data)
                }
            }

            fn as_ref(&self) -> BlockRef {
                let data = BlockMut {
                    data: Rc::downgrade(&self.data)
                };
                BlockRef {
                    data
                }
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for Block {
            async fn pack(&self) -> JsValue {
                self.data.pack().await
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

        #[derive(Clone)]
        pub struct BlockMut {
            data: Weak<RefCell<AnnotBlockData>>
        }

        impl BlockMut {
            pub fn none() -> Self {
                Block::none().as_mut()
            }

            pub fn update<T>(&mut self, f: impl FnOnce(&mut T)) where Self: Access<T> {
                Access::update(self, f);
            }

            pub fn map<T, U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> where Self: Access<T> {
                Access::map(self, f)
            }

            pub fn id(&self) -> U128Id {
                if let Some(data) = self.data.upgrade() {
                    U128Id::clone(&data.borrow().block_id)
                } else {
                    U128Id::none()
                }
            }

            pub fn kind(&self) -> BlockKind {
                if let Some(data) = self.data.upgrade() {
                    data.borrow().data.kind()
                } else {
                    BlockKind::None
                }
            }

            pub fn as_ref(&self) -> BlockRef {
                BlockRef {
                    data: Self {
                        data: Weak::clone(&self.data)
                    }
                }
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for BlockMut {
            async fn pack(&self) -> JsValue {
                if let Some(data) = self.data.upgrade() {
                    data.pack().await
                } else {
                    JsValue::null()
                }
            }
        }

        $(
            impl Access<$b> for BlockMut {
                fn update(&mut self, f: impl FnOnce(&mut $b)) {
                    if let Some(self_data) = self.data.upgrade() {
                        let mut borrow = self_data.borrow_mut();
                        if let BlockData::$b(data) = &mut borrow.data {
                            f(data);
                            let timesamp = borrow.timestamp.max(js_sys::Date::now());
                            borrow.timestamp = timesamp;
                        }
                    }
                }

                fn map<U>(&self, f: impl FnOnce(&$b) -> U) -> Option<U> {
                    if let Some(self_data) = self.data.upgrade() {
                        if let BlockData::$b(data) = &self_data.borrow().data {
                            return Some(f(data));
                        }
                    }
                    None
                }
            }
        )*

        #[derive(Clone)]
        pub struct BlockRef {
            data: BlockMut
        }

        impl std::ops::Deref for BlockRef {
            type Target = BlockMut;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        #[async_trait(?Send)]
        impl util::Pack for BlockRef {
            async fn pack(&self) -> JsValue {
                self.data.pack().await
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

            fn get_mut(&mut self, block_id: &U128Id) -> Option<BlockMut> {
                if let Some(data) = self.data.get_mut(block_id) {
                    Some(data.as_mut())
                } else {
                    None
                }
            }

            fn get(&self, block_id: &U128Id) -> Option<BlockRef> {
                if let Some(data) = self.data.get(block_id) {
                    Some(data.as_ref())
                } else {
                    None
                }
            }

            fn insert(&mut self, block: Block) -> BlockMut {
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

            pub fn get_mut(&mut self, block_id: &U128Id) -> Option<BlockMut> {
                self.data.borrow_mut().get_mut(&block_id)
            }

            pub fn get(&self, block_id: &U128Id) -> Option<BlockRef> {
                self.data.borrow_mut().get(&block_id)
            }

            pub fn insert<T>(&mut self, block: T) -> BlockMut where Self: Insert<T> {
                Insert::insert(self, block)
            }
        }

        pub trait Insert<T> {
            fn insert(&mut self, block: T) -> BlockMut;
        }

        $(
            impl Insert<$b> for Arena {
                fn insert(&mut self, block: $b) -> BlockMut {
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

            pub fn get_mut(&mut self, block_id: &U128Id) -> Option<BlockMut> {
                self.data.upgrade().and_then(|data| data.borrow_mut().get_mut(&block_id))
            }

            pub fn get(&self, block_id: &U128Id) -> Option<BlockRef> {
                self.data.upgrade().and_then(|data| data.borrow_mut().get(&block_id))
            }

            pub fn insert<T>(&mut self, block: T) -> BlockMut where Self: Insert<T> {
                Insert::insert(self, block)
            }
        }

        $(
            impl Insert<$b> for ArenaMut {
                fn insert(&mut self, block: $b) -> BlockMut {
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
