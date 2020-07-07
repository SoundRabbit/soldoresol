use super::{Block, BlockId, Field};
use crate::Promise;
use crate::{color_system, Color};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Tablemask {
    size: [f32; 3],
    position: [f32; 3],
    z_rotation: f32,
    color: Color,
    size_is_binded: bool,
    is_rounded: bool,
    is_inved: bool,
    is_fixed: bool,
    property_id: BlockId,
}

impl Tablemask {
    pub fn new(
        property_id: BlockId,
        size: &[f32; 2],
        color: Color,
        is_rounded: bool,
        is_inved: bool,
    ) -> Self {
        Self {
            size: [size[0], size[1], 0.0],
            position: [0.0, 0.0, 0.0],
            z_rotation: 0.0,
            color: color,
            size_is_binded: true,
            is_rounded: is_rounded,
            is_inved: is_inved,
            is_fixed: false,
            property_id: property_id,
        }
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: &[f32; 2]) {
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

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn is_rounded(&self) -> bool {
        self.is_rounded
    }

    pub fn set_is_rounded(&mut self, is_rounded: bool) {
        self.is_rounded = is_rounded;
    }

    pub fn is_inved(&self) -> bool {
        self.is_inved
    }

    pub fn set_is_inved(&mut self, is_inved: bool) {
        self.is_inved = is_inved;
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed;
    }

    pub fn property_id(&self) -> &BlockId {
        &self.property_id
    }

    pub fn set_property_id(&mut self, property_id: BlockId) {
        self.property_id = property_id;
    }
}

impl Block for Tablemask {
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
