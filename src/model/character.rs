use super::color::Color;
use super::TexstureLayer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Character {
    size: [f64; 2],
    position: [f64; 3],
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: [1.0, 1.0],
            position: [0.0, 0.0, 0.0],
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }
}
