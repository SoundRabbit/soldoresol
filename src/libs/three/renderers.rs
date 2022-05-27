use super::{Camera, Object3D};
use crate::libs::js_object::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type WebGLRenderer;

    #[wasm_bindgen(constructor)]
    pub fn new(parameters: &Object) -> WebGLRenderer;

    #[wasm_bindgen(method, js_name = "setPixelRatio")]
    pub fn set_pixel_ratio(this: &WebGLRenderer, pixel_ratio: f64);

    #[wasm_bindgen(method, js_name = "setSize")]
    pub fn set_size(this: &WebGLRenderer, width: f64, height: f64);

    #[wasm_bindgen(method)]
    pub fn render(this: &WebGLRenderer, scene: &Object3D, camera: &Camera);
}
