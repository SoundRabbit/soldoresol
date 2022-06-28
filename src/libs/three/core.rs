use wasm_bindgen::prelude::*;

use super::{Camera, Euler, Ray, Vector2, Vector3};

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new_with_f32array(
        array: &js_sys::Float32Array,
        item_size: u32,
        normalized: bool,
    ) -> BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new_with_u16array(
        array: &js_sys::Uint16Array,
        item_size: u32,
        normalized: bool,
    ) -> BufferAttribute;

    #[wasm_bindgen(method, getter, js_name = "array")]
    pub fn array_as_f32array(this: &BufferAttribute) -> js_sys::Float32Array;
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type BufferGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new() -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "getAttribute")]
    pub fn get_attribute(this: &BufferGeometry, name: &str) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "setAttribute")]
    pub fn set_attribute(
        this: &BufferGeometry,
        name: &str,
        attribute: &BufferAttribute,
    ) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "setIndex")]
    pub fn set_index(this: &BufferGeometry, index: &BufferAttribute) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "setFromPoints")]
    pub fn set_from_points(this: &BufferGeometry, points: &js_sys::Array) -> BufferGeometry;
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

    #[wasm_bindgen(method)]
    pub fn add(this: &Object3D, object: &Object3D);

    #[wasm_bindgen(method)]
    pub fn remove(this: &Object3D, object: &Object3D);

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &Object3D) -> js_sys::Array;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Object3D) -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn scale(this: &Object3D) -> Vector3;

    #[wasm_bindgen(method, setter, js_name = "renderOrder")]
    pub fn set_render_order(this: &Object3D, render_order: f64);

    #[wasm_bindgen(method, getter)]
    pub fn rotation(this: &Object3D) -> Euler;

    #[wasm_bindgen(method, getter, js_name = "userData")]
    pub fn user_data(this: &Object3D) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = "userData")]
    pub fn set_user_data(this: &Object3D, user_data: &JsValue);

    #[wasm_bindgen(method, setter, js_name = "visible")]
    pub fn set_visible(this: &Object3D, visible: bool);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    pub type Raycaster;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Raycaster;

    #[wasm_bindgen(method, js_name = "intersectObjects")]
    pub fn intersect_objects(this: &Raycaster, objects: &js_sys::Array) -> js_sys::Array;

    #[wasm_bindgen(method, js_name = "setFromCamera")]
    pub fn set_from_camera(this: &Raycaster, coords: &Vector2, camera: &Camera);

    #[wasm_bindgen(method, getter)]
    pub fn ray(this: &Raycaster) -> Ray;
}
