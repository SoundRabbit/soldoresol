use super::{Block, BlockId, Field};
use crate::Color;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Character {
    size: [f32; 3],
    position: [f32; 3],
    texture_id: Option<BlockId>,
    background_color: Color,
    name: String,
    property_id: BlockId,
}

pub struct Update {
    pub size: Option<[f32; 2]>,
    pub position: Option<[f32; 3]>,
    pub texture_id: Option<Option<BlockId>>,
    pub background_color: Option<Color>,
    pub name: Option<String>,
    pub property_id: Option<BlockId>,
}

impl Character {
    pub fn new(property_id: BlockId) -> Self {
        Self {
            size: [1.0, 0.0, 1.0],
            position: [0.0, 0.0, 0.0],
            texture_id: None,
            background_color: Color::from(0),
            name: "キャラクター".into(),
            property_id: property_id,
        }
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn property_id(&self) -> &BlockId {
        &self.property_id
    }

    pub fn update(&mut self, diff: Update) {
        if let Some(size) = diff.size {
            self.size = [size[0], 0.0, size[1]];
        }
        if let Some(position) = diff.position {
            self.position = position;
        }
        if let Some(texture_id) = diff.texture_id {
            self.texture_id = texture_id;
        }
        if let Some(background_color) = diff.background_color {
            self.background_color = background_color;
        }
        if let Some(name) = diff.name {
            self.name = name;
        }
        if let Some(property_id) = diff.property_id {
            self.property_id = property_id;
        }
    }
}

impl Block for Character {
    fn pack(&self, resolve: impl FnOnce(JsValue)) {}
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>)) {}
}
