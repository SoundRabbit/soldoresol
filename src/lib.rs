extern crate base64;
extern crate bincode;
extern crate hex;
extern crate kagura;
extern crate ndarray;
extern crate regex;
extern crate sainome;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate wasm_bindgen;
extern crate web_sys;

#[macro_use]
mod js_object;
mod component;
mod config;
mod dice_bot;
mod indexed_db;
mod model;
mod random_id;
mod renderer;
mod skyway;
mod util;
use config::Config;
use js_object::JsObject;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(component::config_loder::new(), "app");
}
