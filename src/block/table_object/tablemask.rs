use super::{Block, BlockId, Field};
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

pub struct Update {
    size: Option<[f32; 2]>,
    position: Option<[f32; 2]>,
    z_rotation: Option<f32>,
    background_color: Option<Color>,
    size_is_binded: Option<bool>,
    is_rounded: Option<bool>,
    is_fixed: Option<bool>,
    property_id: Option<BlockId>,
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

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn property_id(&self) -> &BlockId {
        &self.property_id
    }

    pub fn update(&mut self, diff: Update) {
        if let Some(size) = diff.size {
            self.size = [size[0], size[1], 0.0];
        }
        if let Some(position) = diff.position {
            self.position = [position[0], position[1], 0.0];
        }
        if let Some(z_rotation) = diff.z_rotation {
            self.z_rotation = z_rotation;
        }
        if let Some(background_color) = diff.background_color {
            self.background_color = background_color;
        }
        if let Some(is_rounded) = diff.is_rounded {
            self.is_rounded = is_rounded;
        }
        if let Some(is_fixed) = diff.is_fixed {
            self.is_fixed = is_fixed;
        }
        if let Some(property_id) = diff.property_id {
            self.property_id = property_id;
        }
    }
}

impl Block for Tablemask {
    fn pack(&self, resolve: impl FnOnce(JsValue)) {}
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>)) {}
}
