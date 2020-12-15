use super::BlockId;
use wasm_bindgen::{prelude::*, JsCast};

mod texture;

pub struct Table {
    name: String,
    size: [f32; 2],
    is_bind_to_grid: bool,
    is_showing_grid: bool,
    drawing_texture_id: BlockId,
    background_texture_id: Option<ResourceId>,
}

impl Table {
    pub fn new(drawing_texture_id: BlockId, size: [f32; 2], name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size,
            is_bind_to_grid: true,
            is_showing_grid: true,
            drawing_texture_id,
            background_texture_id: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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

    pub fn image_texture_id(&self) -> Option<&ResourceId> {
        self.image_texture_id.as_ref()
    }

    pub fn set_image_texture_id(&mut self, image_texture_id: Option<ResourceId>) {
        self.image_texture_id = image_texture_id
    }

    async fn pack(&self) -> JsValue {
        unimplemented!();
    }

    async fn unpack(_val: JsValue) -> Self {
        unimplemented!();
    }
}
