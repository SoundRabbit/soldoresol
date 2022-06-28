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
    pub type CircleGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new(radius: f64, segments: i32) -> CircleGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new_with_theta(
        radius: f64,
        segments: i32,
        theta_start: f64,
        theta_length: f64,
    ) -> CircleGeometry;
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

    #[wasm_bindgen(constructor)]
    pub fn new_with_option(
        radius_top: f64,
        radius_bottom: f64,
        height: f64,
        radial_segements: i32,
        height_segements: i32,
        open_ended: bool,
        theta_start: f64,
        theta_length: f64,
    ) -> CylinderGeometry;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type IcosahedronGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new(radius: f64, detail: i32) -> IcosahedronGeometry;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type PlaneGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> PlaneGeometry;
}
