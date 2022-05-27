use wasm_bindgen::prelude::*;

use super::{Euler, Vector3};

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type BufferGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new() -> BufferGeometry;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type EventDispatcher;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EventDispatcher;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Object3D;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Object3D;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Object3D) -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn rotation(this: &Object3D) -> Euler;

    #[wasm_bindgen(method)]
    pub fn add(this: &Object3D, object: &Object3D);

    #[wasm_bindgen(method)]
    pub fn remove(this: &Object3D, object: &Object3D);
}
