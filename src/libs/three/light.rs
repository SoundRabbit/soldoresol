use wasm_bindgen::prelude::*;

use super::{Color, Object3D};

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type AmbientLight;

    #[wasm_bindgen(constructor)]
    pub fn new() -> AmbientLight;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Light;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Light;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &Light) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn intensity(this: &Light) -> f64;

    #[wasm_bindgen(method, setter, js_name = "intensity")]
    pub fn set_intensity(this: &Light, intensity: f64);
}
