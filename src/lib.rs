extern crate bincode;
extern crate hex;
extern crate kagura;
extern crate ndarray;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

mod component;
mod model;
mod random_id;
mod renderer;
mod skyway;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(component::connection::new(), "app");
}
