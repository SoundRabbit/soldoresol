use super::Block;
use super::BlockId;
use super::TableObject;
use crate::model::{Color, ColorSystem};
use wasm_bindgen::prelude::*;

pub struct Character {
    size: [f32; 3],
    position: [f32; 3],
    texture_id: Option<BlockId>,
    background_color: Color,
    name: String,
    property_id: BlockId,
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
}

impl TableObject for Character {
    fn size(&self) -> &[f32; 3] {
        &self.size
    }

    fn set_size(&mut self, size: [f32; 3]) {
        self.size = size
    }

    fn position(&self) -> &[f32; 3] {
        &self.position
    }

    fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    fn property_id(&self) -> &BlockId {
        &self.property_id
    }
}

impl Block for Character {
    fn pack(&self, resolve: impl FnOnce(JsValue)) {}
    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>)) {}
}
