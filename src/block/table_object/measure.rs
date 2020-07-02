use super::{Block, Field};
use crate::Promise;
use crate::{color_system, Color};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Measure {
    origin: [f32; 3],
    vector: [f32; 3],
    color: Color,
}

impl Measure {
    pub fn new(origin: [f32; 3], vector: [f32; 3], color: Color) -> Self {
        Self {
            origin: origin,
            vector: vector,
            color: color,
        }
    }

    pub fn vec(&self) -> &[f32; 3] {
        &self.vector
    }

    pub fn set_vec(&mut self, vec: [f32; 3]) {
        self.vector = vec;
    }

    pub fn org(&self) -> &[f32; 3] {
        &self.origin
    }

    pub fn set_org(&mut self, org: [f32; 3]) {
        self.origin = org;
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Block for Measure {
    fn pack(&self) -> Promise<JsValue> {
        unimplemented!();
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
