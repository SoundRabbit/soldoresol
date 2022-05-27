use wasm_bindgen::prelude::*;

use super::{Color, EventDispatcher, Texture};

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Material;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Material)]
    pub type MeshToonMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshToonMaterial;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &MeshToonMaterial) -> Color;

    #[wasm_bindgen(method, setter, js_name = "map")]
    pub fn set_map(this: &MeshToonMaterial, map: &Texture);
}
