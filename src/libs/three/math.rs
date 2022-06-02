use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type Color;

    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64) -> Color;

    #[wasm_bindgen(method, js_name = "setRGB")]
    pub fn set_rgb(this: &Color, r: f64, g: f64, b: f64);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type Euler;

    #[wasm_bindgen(method, setter, js_name = "order")]
    pub fn set_order(this: &Euler, order: &str);

    #[wasm_bindgen(method, setter, js_name = "x")]
    pub fn set_x(this: &Euler, x: f64);

    #[wasm_bindgen(method, setter, js_name = "y")]
    pub fn set_y(this: &Euler, y: f64);

    #[wasm_bindgen(method, setter, js_name = "z")]
    pub fn set_z(this: &Euler, z: f64);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type Ray;

    #[wasm_bindgen(method, getter)]
    pub fn origin(this: &Ray) -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn direction(this: &Ray) -> Vector3;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type Vector2;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Vector2;

    #[wasm_bindgen(method, js_name = "set")]
    pub fn set(this: &Vector2, x: f64, y: f64);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(method, js_name = "set")]
    pub fn set(this: &Vector3, x: f64, y: f64, z: f64);

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f64;
}
