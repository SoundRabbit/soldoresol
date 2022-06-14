use crate::libs::js_object::Object;
use wasm_bindgen::prelude::*;

use super::{Color, EventDispatcher, Texture};

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Material)]
    pub type LineBasicMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new(parameters: &Object) -> LineBasicMaterial;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &LineBasicMaterial) -> Color;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = LineBasicMaterial)]
    pub type LineDashedMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new(parameters: &Object) -> LineDashedMaterial;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Material;

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn set_needs_update(this: &Material, needs_update: bool);

    #[wasm_bindgen(method, setter, js_name = "opacity")]
    pub fn set_opacity(this: &Material, opacity: f64);

    #[wasm_bindgen(method, setter, js_name = "stencilWrite")]
    pub fn set_stencil_write(this: &Material, stencil_write: bool);

    #[wasm_bindgen(method, setter, js_name = "stencilFunc")]
    pub fn set_stencil_func(this: &Material, stencil_func: u32);

    #[wasm_bindgen(method, setter, js_name = "stencilRef")]
    pub fn set_stencil_ref(this: &Material, stencil_func: u32);

    #[wasm_bindgen(method, setter, js_name = "stencilFail")]
    pub fn set_stencil_fail(this: &Material, stencil_func: u32);

    #[wasm_bindgen(method, setter, js_name = "stencilZFail")]
    pub fn set_stencil_z_fail(this: &Material, stencil_func: u32);

    #[wasm_bindgen(method, setter, js_name = "stencilZPass")]
    pub fn set_stencil_z_pass(this: &Material, stencil_func: u32);

    #[wasm_bindgen(method, setter, js_name = "transparent")]
    pub fn set_transparent(this: &Material, transparent: bool);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Material)]
    pub type MeshBasicMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new(parameters: &Object) -> MeshBasicMaterial;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &MeshBasicMaterial) -> Color;

    #[wasm_bindgen(method, setter, js_name = "map")]
    pub fn set_map(this: &MeshBasicMaterial, map: &Texture);
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
