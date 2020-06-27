use super::{Block, BlockId, Field};
use crate::{resource::ResourceId, Promise};
use wasm_bindgen::prelude::*;

mod texture;

pub use texture::Texture;

#[derive(Clone)]
pub struct Table {
    name: String,
    size: [f32; 2],
    drawing_texture_id: BlockId,
    image_texture_id: Option<ResourceId>,
    tablemasks: Vec<BlockId>,
}

impl Table {
    pub fn new(drawing_texture_id: BlockId, size: [f32; 2], name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size,
            drawing_texture_id,
            image_texture_id: None,
            tablemasks: vec![],
        }
    }

    pub fn size(&self) -> &[f32; 2] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 2]) {
        self.size = size;
    }

    pub fn drawing_texture_id(&self) -> &BlockId {
        &self.drawing_texture_id
    }

    pub fn set_image_texture_id(&mut self, image_texture_id: Option<ResourceId>) {
        self.image_texture_id = image_texture_id
    }

    pub fn add_tablemask(&mut self, tablemask: BlockId) {
        self.tablemasks.push(tablemask);
    }

    pub fn remove_tablemask(&mut self, tablemask: &BlockId) {
        if let Some(idx) = self.tablemasks.iter().position(|x| x == tablemask) {
            self.tablemasks.remove(idx);
        }
    }
}

impl Block for Table {
    fn pack(&self) -> Promise<JsValue, ()> {
        unimplemented!();
    }
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()> {
        unimplemented!();
    }
}
