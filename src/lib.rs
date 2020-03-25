extern crate kagura;
extern crate serde;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod component;
mod shader;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(component::app::new(), "app");
}
