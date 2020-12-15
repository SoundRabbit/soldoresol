use super::{Block, BlockId, Field};
use crate::resource::ResourceId;
use crate::Color;
use crate::Promise;
use std::collections::HashSet;
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
        unreachable!();
    }
    fn unpack(_: &mut Field, _: JsValue) -> Promise<Box<Self>> {
        unreachable!();
    }

    fn dependents(&self, _: &Field) -> HashSet<BlockId> {
        unreachable!();
    }

    fn resources(&self, _: &Field) -> HashSet<ResourceId> {
        unreachable!();
    }
}
