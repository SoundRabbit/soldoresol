use super::{Block, Field};
use crate::Promise;
use crate::{color_system, Color};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum Type {
    Line(f64),
    Rounded,
}

#[derive(Clone)]
pub struct Area {
    origin: [f32; 3],
    vector: [f32; 3],
    color_1: Color,
    color_2: Color,
    type_: Type,
}

impl Area {
    pub fn new(
        origin: [f32; 3],
        vector: [f32; 3],
        color_1: Color,
        color_2: Color,
        type_: Type,
    ) -> Self {
        Self {
            origin: origin,
            vector: vector,
            color_1: color_1,
            color_2: color_2,
            type_: type_,
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

    pub fn color_1(&self) -> &Color {
        &self.color_1
    }

    pub fn set_color_1(&mut self, color: Color) {
        self.color_1 = color;
    }

    pub fn color_2(&self) -> &Color {
        &self.color_2
    }

    pub fn set_color_2(&mut self, color: Color) {
        self.color_2 = color;
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn set_type(&mut self, type_: Type) {
        self.type_ = type_;
    }
}

impl Block for Area {
    fn pack(&self) -> Promise<JsValue> {
        unimplemented!();
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
