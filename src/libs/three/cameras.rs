use wasm_bindgen::prelude::*;

use super::Object3D;

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Camera;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Camera;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type PerspectiveCamera;

    #[wasm_bindgen(constructor)]
    pub fn new(fov: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera;

    #[wasm_bindgen(method, js_name = "updateProjectionMatrix")]
    pub fn update_projection_matrix(this: &PerspectiveCamera);

    #[wasm_bindgen(method, setter, js_name = "aspect")]
    pub fn set_aspect(this: &PerspectiveCamera, aspect: f64);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type OrthographicCamera;

    #[wasm_bindgen(constructor)]
    pub fn new(
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        near: f64,
        far: f64,
    ) -> OrthographicCamera;

    #[wasm_bindgen(method, js_name = "updateProjectionMatrix")]
    pub fn update_projection_matrix(this: &OrthographicCamera);

    #[wasm_bindgen(method, setter, js_name = "top")]
    pub fn set_top(this: &OrthographicCamera, top: f64);

    #[wasm_bindgen(method, setter, js_name = "left")]
    pub fn set_left(this: &OrthographicCamera, left: f64);

    #[wasm_bindgen(method, setter, js_name = "bottom")]
    pub fn set_bottom(this: &OrthographicCamera, bottom: f64);

    #[wasm_bindgen(method, setter, js_name = "right")]
    pub fn set_right(this: &OrthographicCamera, right: f64);
}
