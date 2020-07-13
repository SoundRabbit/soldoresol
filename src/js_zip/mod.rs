use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../src/js_zip/js_zip.js")]
extern "C" {
    pub type JSZip;

    #[wasm_bindgen(constructor)]
    pub fn new() -> JSZip;

    #[wasm_bindgen(method, js_name = "loadAsync")]
    pub fn load_async(this: &JSZip, data: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn file(this: &JSZip, name: &str) -> Option<ZipObject>;

    #[wasm_bindgen(method, getter)]
    pub fn files(this: &JSZip) -> js_sys::Object;
}

#[wasm_bindgen]
extern "C" {
    pub type ZipObject;

    #[wasm_bindgen(method, js_name = "async")]
    pub fn load_async(this: &ZipObject, type_: &str) -> js_sys::Promise;
}
