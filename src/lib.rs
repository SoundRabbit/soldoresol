extern crate base64;
extern crate bincode;
extern crate futures;
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
extern crate wasm_bindgen_futures;
extern crate web_sys;
extern crate xmltree;

#[macro_use]
mod js_object;
#[macro_use]
mod libs;
#[macro_use]
mod util;

#[allow(dead_code)]
mod block;
#[allow(dead_code)]
mod color;
#[allow(dead_code)]
mod color_system;
#[allow(dead_code)]
mod component;
#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod debug;
#[allow(dead_code)]
mod dicebot;
#[allow(dead_code)]
mod idb;
#[allow(dead_code)]
mod js_zip;
#[allow(dead_code)]
mod model;
#[allow(dead_code)]
mod promise;
#[allow(dead_code)]
mod random_id;
#[allow(dead_code)]
mod renderer;
#[allow(dead_code)]
mod resource;
#[allow(dead_code)]
mod skyway;
#[allow(dead_code)]
mod timestamp;
#[allow(dead_code)]
mod udonarium;

use color::Color;
use component::{app, App};
use config::Config;
use js_object::JsObject;
use js_zip::JSZip;
use promise::Promise;
use resource::Resource;
use timestamp::Timestamp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run::<component::App, _, _, _>("app", app::Props {}, vec![]);
}
