extern crate async_std;
extern crate async_trait;
extern crate base64;
extern crate bincode;
extern crate chrono;
extern crate downcast_rs;
extern crate futures;
extern crate hex;
extern crate indexmap;
extern crate isaribi;
extern crate kagura;
extern crate kanaria;
extern crate lazy_static;
extern crate ndarray;
extern crate nusa;
extern crate ordered_float;
extern crate peg;
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
mod table;

use component::{app, App};
use nusa::prelude::*;
use wasm_bindgen::prelude::*;

#[inline]
fn lazy<T>(x: T) -> T {
    x
}

#[wasm_bindgen(start)]
pub fn main() {
    let entry_point = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap();

    wasm_bindgen_futures::spawn_local(kagura::Runtime::run(nusa::dom_node::BasicDomNode::new(
        entry_point.into(),
        |this| vec![App::empty(this, None, app::Props {}, Sub::none())],
    )));
}
