use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::color::Pallet;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Table {
    name: Rc<String>,
    boxblocks: Vec<BlockId>,
    pointlights: Vec<BlockId>,
    craftboards: Vec<BlockId>,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Rc::new(name.into()),
            boxblocks: vec![],
            pointlights: vec![],
            craftboards: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Rc::new(name);
    }
    pub fn boxblocks(&self) -> impl Iterator<Item = &BlockId> {
        self.boxblocks.iter()
    }

    pub fn add_boxblock(&mut self, boxblock_id: BlockId) {
        self.boxblocks.push(boxblock_id);
    }

    pub fn pointlights(&self) -> impl Iterator<Item = &BlockId> {
        self.pointlights.iter()
    }

    pub fn add_pointlight(&mut self, pointlight_id: BlockId) {
        self.pointlights.push(pointlight_id);
    }

    pub fn craftboards(&self) -> impl Iterator<Item = &BlockId> {
        self.craftboards.iter()
    }

    pub fn add_craftboard(&mut self, craftboard_id: BlockId) {
        self.craftboards.push(craftboard_id);
    }

    pub async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    pub async fn unpack(_val: JsValue) -> Self {
        unimplemented!();
    }
}
