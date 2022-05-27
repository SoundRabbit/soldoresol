use wasm_bindgen::prelude::*;

use super::EventDispatcher;

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Texture;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Texture;
}
