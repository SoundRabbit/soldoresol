use super::{Block, BlockId, Field};
use crate::Promise;
use crate::{color_system, Color};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
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
            background_color: color_system::red((255.0 * 0.6) as u8, 5),
            size_is_binded: true,
            is_rounded: true,
            is_fixed: false,
            property_id: property_id,
        }
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 2]) {
        self.size = [size[0], size[1], 0.0];
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f32; 2]) {
        self.position = [position[0], position[1], 0.0];
    }

    pub fn z_rotation(&self) -> f32 {
        self.z_rotation
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn is_rounded(&self) -> bool {
        self.is_rounded
    }

    pub fn property_id(&self) -> &BlockId {
        &self.property_id
    }

    pub fn set_property_id(&mut self, property_id: BlockId) {
        self.property_id = property_id;
    }
}

impl Block for Tablemask {
    fn pack(&self) -> Promise<JsValue, ()> {
        unimplemented!();
    }
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()> {
        unimplemented!();
    }
}
