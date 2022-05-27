use wasm_bindgen::prelude::*;

use super::{Color, Object3D};

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Scene;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;

    #[wasm_bindgen(method, setter, js_name = "background")]
    pub fn set_background(this: &Scene, color: &Color);
}
