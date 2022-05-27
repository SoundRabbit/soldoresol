use wasm_bindgen::prelude::*;

use super::{BufferGeometry, Material, Object3D};

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Mesh;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &BufferGeometry, material: &Material) -> Mesh;

    #[wasm_bindgen(method, getter)]
    pub fn material(this: &Mesh) -> Material;

    #[wasm_bindgen(method, getter)]
    pub fn geometry(this: &Mesh) -> BufferGeometry;

    #[wasm_bindgen(method, setter, js_name = "geometry")]
    pub fn set_geometry(this: &Mesh, geometry: &BufferGeometry);
}
