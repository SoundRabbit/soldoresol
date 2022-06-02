use crate::libs::js_object::Object;
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
    pub type MeshStandardMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new(parameters: &Object) -> MeshStandardMaterial;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &MeshStandardMaterial) -> Color;

    #[wasm_bindgen(method, setter, js_name = "map")]
    pub fn set_map(this: &MeshStandardMaterial, map: &Texture);
}
