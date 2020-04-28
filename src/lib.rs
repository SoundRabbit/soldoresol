extern crate kagura;
extern crate serde;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;
extern crate hex;
extern crate ndarray;
extern crate regex;
extern crate serde_json;

use wasm_bindgen::prelude::*;

mod component;
mod model;
mod random_id;
mod renderer;
mod shader;
mod table;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(component::app::new(), "app");
}
