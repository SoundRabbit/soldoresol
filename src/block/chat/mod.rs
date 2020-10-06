use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, resource::ResourceId, Promise};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

pub mod item;
pub mod tab;

pub use item::Item;
pub use tab::Tab;

#[derive(Clone)]
pub struct Chat {
    tabs: Vec<BlockId>,
}

impl Chat {
    pub fn new(tabs: Vec<BlockId>) -> Self {
        Self { tabs: tabs }
    }

    pub fn tabs(&self) -> &Vec<BlockId> {
        &self.tabs
    }
}

impl Block for Chat {
    fn pack(&self) -> Promise<JsValue> {
        let val = js_sys::Array::new();
        for tab_id in &self.tabs {
            val.push(&tab_id.to_jsvalue());
        }
        Promise::new(|resolve| resolve(Some(val.into())))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let val = js_sys::Array::from(&val).to_vec();
        let mut tabs = vec![];
        for tab in val {
            if let Some(tab_id) = U128Id::from_jsvalue(&tab) {
                tabs.push(field.block_id(tab_id));
            }
        }
        let chat = Self { tabs };
        Promise::new(|resolve| resolve(Some(Box::new(chat))))
    }
    fn dependents(&self, field: &Field) -> HashSet<BlockId> {
        let mut deps = HashSet::new();

        for block_id in &self.tabs {
            if let Some(block) = field.get::<Tab>(block_id) {
                let block_deps = block.dependents(field);
                for block_dep in block_deps {
                    deps.insert(block_dep);
                }
                deps.insert(block_id.clone());
            }
        }

        deps
    }

    fn resources(&self, field: &Field) -> HashSet<ResourceId> {
        let mut reses = set! {};

        for block_id in &self.tabs {
            if let Some(block) = field.get::<Tab>(block_id) {
                let block_reses = block.resources(field);
                for block_res in block_reses {
                    reses.insert(block_res);
                }
            }
        }

        reses
    }
}

impl Deref for Chat {
    type Target = Vec<BlockId>;
    fn deref(&self) -> &Self::Target {
        &self.tabs
    }
}

impl DerefMut for Chat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tabs
    }
}
