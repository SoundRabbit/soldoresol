use super::BlockId;
use crate::arena::resource::ResourceId;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod texture;

pub struct Table {
    name: Rc<String>,
    size: [f32; 2],
    is_bind_to_grid: bool,
    is_showing_grid: bool,
    drawing_texture_id: BlockId,
    drawed_texture_id: BlockId,
    background_texture_id: Option<ResourceId>,
}

impl Table {
    pub fn new(
        drawing_texture_id: BlockId,
        drawed_texture_id: BlockId,
        size: [f32; 2],
        name: impl Into<String>,
    ) -> Self {
        Self {
            name: Rc::new(name.into()),
            size,
            is_bind_to_grid: true,
            is_showing_grid: true,
            drawing_texture_id,
            drawed_texture_id,
            background_texture_id: None,
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            name: Rc::clone(&this.name),
            size: this.size.clone(),
            is_bind_to_grid: this.is_bind_to_grid,
            is_showing_grid: this.is_showing_grid,
            drawing_texture_id: BlockId::clone(&this.drawing_texture_id),
            drawed_texture_id: BlockId::clone(&this.drawed_texture_id),
            background_texture_id: this
                .background_texture_id
                .as_ref()
                .map(|r_id| ResourceId::clone(r_id)),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Rc::new(name);
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

    pub fn set_is_bind_to_grid(&mut self, is_bind_to_grid: bool) {
        self.is_bind_to_grid = is_bind_to_grid;
    }

    pub fn is_showing_grid(&self) -> bool {
        self.is_showing_grid
    }

    pub fn set_is_showing_grid(&mut self, is_showing_grid: bool) {
        self.is_showing_grid = is_showing_grid;
    }

    pub fn drawing_texture_id(&self) -> &BlockId {
        &self.drawing_texture_id
    }

    pub fn drawed_texture_id(&self) -> &BlockId {
        &self.drawed_texture_id
    }

    pub fn background_texture_id(&self) -> Option<&ResourceId> {
        self.background_texture_id.as_ref()
    }

    pub fn set_background_texture_id(&mut self, background_texture_id: Option<ResourceId>) {
        self.background_texture_id = background_texture_id
    }

    pub async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    pub async fn unpack(_val: JsValue) -> Self {
        unimplemented!();
    }
}
