use super::Block;
use super::BlockId;
use super::TableObject;
use crate::model::{Color, ColorSystem};
use wasm_bindgen::prelude::*;

pub struct Tablemask {
    size: [f32; 3],
    position: [f32; 3],
    z_rotation: f32,
    background_color: Color,
    size_is_binded: bool,
    is_rounded: bool,
    is_fixed: bool,
    property_id: BlockId,
}

impl Tablemask {
    pub fn new(property_id: BlockId) -> Self {
        Self {
            size: [8.0, 8.0, 0.0],
            position: [0.0, 0.0, 0.0],
            z_rotation: 0.0,
            background_color: ColorSystem::red((255.0 * 0.6) as u8, 5),
            size_is_binded: true,
            is_rounded: true,
            is_fixed: false,
            property_id: property_id,
        }
    }
}

impl TableObject for Tablemask {
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

impl Block for Tablemask {
    fn pack(&self, resolve: impl FnOnce(JsValue)) {}
    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>)) {}
}
