extern crate async_std;
extern crate async_trait;
extern crate base64;
extern crate bincode;
extern crate downcast_rs;
extern crate futures;
extern crate hex;
extern crate indexmap;
extern crate isaribi;
extern crate kagura;
extern crate kanaria;
extern crate lazy_static;
extern crate ndarray;
extern crate ordered_float;
extern crate regex;
extern crate sainome;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;
extern crate wasm_bindgen_futures;
extern crate web_sys;
extern crate xmltree;

#[macro_use]
extern crate wasm_bindgen;

#[macro_use]
mod libs;

mod arena;
mod component;
mod debug;
mod model;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    use component::{app, App};
    kagura::run::<App, _, _, _>("app", app::Props {}, vec![]);
}
