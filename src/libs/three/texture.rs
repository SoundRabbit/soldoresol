use wasm_bindgen::prelude::*;

use super::EventDispatcher;

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = EventDispatcher)]
    pub type Texture;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Texture;

    #[wasm_bindgen(constructor)]
    pub fn new_with_image(image: &web_sys::HtmlImageElement) -> Texture;

    #[wasm_bindgen(constructor)]
    pub fn new_with_canvas(image: &web_sys::HtmlCanvasElement) -> Texture;

    #[wasm_bindgen(method)]
    pub fn dispose(this: &Texture);

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn set_needs_update(this: &Texture, needs_update: bool);

    #[wasm_bindgen(method, setter, js_name = "wrapS")]
    pub fn set_wrap_s(this: &Texture, wrap_s: i32);
}

#[wasm_bindgen(module = "three")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type CanvasTexture;
}
