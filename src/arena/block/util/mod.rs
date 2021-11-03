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
        [pub $b_name:ident]
        $($p_name:ident: $p_type:ty = $p_default:expr;)*
    } => {
        pub struct $b_name {
            $($p_name: $p_type,)*
        }

        impl $b_name {
            pub fn new() -> Self {
                Self {
                    $($p_name: $p_default,)*
                }
            }
        }

        #[async_trait(?Send)]
        impl Pack for $b_name {
            async fn pack(&self) -> JsValue {
                let object = object! {};

                $(
                    object.set(stringify!($p_name), &self.$p_name.pack().await);
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
        uses! {
            std::collections::HashMap;
            std::cell::Cell;
            std::rc::Weak;
        }

        $(pub mod $m;)*
        $(pub use $m::$b;)*

        enum BlockData {
            BlockId(U128Id),
            $($b($b),)*
        }

        struct AnnotBlockData {
            timestamp: f64,
            data: BlockData,
        }

        pub struct Block {
            data: Rc<RefCell<AnnotBlockData>>
        }

        impl Block {
            pub fn update<T>(&mut self, f: impl FnOnce(&mut T)) where Self: Access<T> {
                Access::update(self, f);
            }

            pub fn map<T, U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> where Self: Access<T> {
                Access::map(self, f)
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

        $(
            impl From<$b> for Block {
                fn from(data: $b) -> Self {
                    Self {
                        data: Rc::new(RefCell::new(
                            AnnotBlockData {
                                timestamp: js_sys::Date::now(),
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
                    if let BlockData::$b(data) = &mut self.data.borrow_mut().data {
                        f(data);
                        self.data.borrow_mut().timestamp = self.data.borrow().timestamp.max(js_sys::Date::now());
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

        pub struct BlockMut {
            data: Weak<RefCell<AnnotBlockData>>
        }

        impl BlockMut {
            pub fn update<T>(&mut self, f: impl FnOnce(&mut T)) where Self: Access<T> {
                Access::update(self, f);
            }

            pub fn map<T, U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> where Self: Access<T> {
                Access::map(self, f)
            }
        }

        $(
            impl Access<$b> for BlockMut {
                fn update(&mut self, f: impl FnOnce(&mut $b)) {
                    if let Some(self_data) = self.data.upgrade() {
                        if let BlockData::$b(data) = &mut self_data.borrow_mut().data {
                            f(data);
                            self_data.borrow_mut().timestamp = self_data.borrow().timestamp.max(js_sys::Date::now());
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

        pub struct BlockRef {
            data: BlockMut
        }

        impl std::ops::Deref for BlockRef {
            type Target = BlockMut;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        pub struct Arena {
            data: Rc<RefCell<HashMap<U128Id, Block>>>,
            is_origin: bool,
        }

        impl Arena {
            pub fn new() -> Self {
                Self {
                    data: Rc::new(RefCell::new(map! {})),
                    is_origin: true
                }
            }

            pub fn as_ref(&self) -> ArenaRef {
                ArenaRef {
                    data: Self {
                        data: Rc::clone(&self.data),
                        is_origin: false
                    }
                }
            }

            pub fn get_mut(&mut self, block_id: &U128Id) -> Option<BlockMut> {
                if let Some(data) = self.data.borrow().get(block_id) {
                    Some(data.as_mut())
                } else {
                    None
                }
            }

            pub fn get(&self, block_id: &U128Id) -> Option<BlockRef> {
                if let Some(data) = self.data.borrow().get(block_id) {
                    Some(data.as_ref())
                } else {
                    None
                }
            }
        }

        impl std::ops::Drop for Arena {
            fn drop(&mut self) {
                if self.is_origin {
                    self.data.borrow_mut().clear();
                }
            }
        }

        pub struct ArenaRef {
            data: Arena
        }

        impl std::ops::Deref for ArenaRef {
            type Target = Arena;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }
    }
}
