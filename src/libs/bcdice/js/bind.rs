use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "bcdice")]
extern "C" {
    pub type DynamicLoader;

    #[wasm_bindgen(constructor)]
    pub fn new() -> DynamicLoader;

    #[wasm_bindgen(method, js_name = "listAvailableGameSystems")]
    pub fn list_available_game_systems(this: &DynamicLoader) -> js_sys::Array;

    #[wasm_bindgen(method, js_name = "dynamicLoad")]
    pub fn dynamic_load(this: &DynamicLoader, id: &str) -> js_sys::Promise;
}
