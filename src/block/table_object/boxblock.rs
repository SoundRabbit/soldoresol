use super::{Block, Field};
use crate::Color;
use crate::Promise;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Boxblock {
    position: [f32; 3],
    size: [f32; 3],
    color: Color,
    is_fixed: bool,
}

impl Boxblock {
    pub fn new(position: [f32; 3], size: [f32; 3], color: Color) -> Self {
        Self {
            position,
            size,
            color,
            is_fixed: true,
        }
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.position = pos;
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 3]) {
        self.size = size;
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed;
    }
}

impl Block for Boxblock {
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
