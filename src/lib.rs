extern crate base64;
extern crate bincode;
extern crate hex;
extern crate indexmap;
extern crate kagura;
extern crate kanaria;
extern crate ndarray;
extern crate ordered_float;
extern crate regex;
extern crate sainome;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate xmltree;

#[macro_use]
mod js_object;
#[macro_use]
mod util;
mod block;
mod color;
mod color_system;
mod component;
mod config;
mod debug;
mod dicebot;
mod idb;
mod js_zip;
mod model;
mod promise;
mod random_id;
mod renderer;
mod resource;
mod skyway;
mod udonarium;

use color::Color;
use config::Config;
use js_object::JsObject;
use js_zip::JSZip;
use promise::Promise;
use resource::Resource;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(component::config_loder::new(), "app");
}
