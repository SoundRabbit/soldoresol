use wasm_bindgen::prelude::*;

use super::BufferGeometry;

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type BoxGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, depth: f64) -> BoxGeometry;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type CylinderGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new(
        radius_top: f64,
        radius_bottom: f64,
        height: f64,
        radial_segements: i32,
    ) -> CylinderGeometry;
}
