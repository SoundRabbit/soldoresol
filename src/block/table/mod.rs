use super::{Block, BlockId, Field};
use crate::{resource::ResourceId, Promise};
use wasm_bindgen::prelude::*;

mod texture;

pub use texture::Texture;

#[derive(Clone)]
pub struct Table {
    name: String,
    size: [f32; 2],
    is_bind_to_grid: bool,
    drawing_texture_id: BlockId,
    image_texture_id: Option<ResourceId>,
    tablemasks: Vec<BlockId>,
    areas: Vec<BlockId>,
}

impl Table {
    pub fn new(drawing_texture_id: BlockId, size: [f32; 2], name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size,
            is_bind_to_grid: true,
            drawing_texture_id,
            image_texture_id: None,
            tablemasks: vec![],
            areas: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn size(&self) -> &[f32; 2] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 2]) {
        self.size = size;
    }

    pub fn is_bind_to_grid(&self) -> bool {
        self.is_bind_to_grid
    }

    pub fn drawing_texture_id(&self) -> &BlockId {
        &self.drawing_texture_id
    }

    pub fn image_texture_id(&self) -> Option<&ResourceId> {
        self.image_texture_id.as_ref()
    }

    pub fn set_image_texture_id(&mut self, image_texture_id: Option<ResourceId>) {
        self.image_texture_id = image_texture_id
    }

    pub fn tablemasks(&self) -> impl Iterator<Item = &BlockId> {
        self.tablemasks.iter()
    }

    pub fn add_tablemask(&mut self, tablemask: BlockId) {
        self.tablemasks.push(tablemask);
    }

    pub fn remove_tablemask(&mut self, tablemask: &BlockId) {
        if let Some(idx) = self.tablemasks.iter().position(|x| x == tablemask) {
            self.tablemasks.remove(idx);
        }
    }

    pub fn areas(&self) -> impl Iterator<Item = &BlockId> {
        self.areas.iter()
    }

    pub fn add_area(&mut self, area: BlockId) {
        self.areas.push(area);
    }

    pub fn remove_area(&mut self, area: &BlockId) {
        if let Some(idx) = self.areas.iter().position(|x| x == area) {
            self.areas.remove(idx);
        }
    }
}

impl Block for Table {
    fn pack(&self) -> Promise<JsValue> {
        let data = object! {};
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
