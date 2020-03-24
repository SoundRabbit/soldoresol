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

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init, update, render), "app");
}

struct State;

struct Msg;

struct Sub;

fn init() -> (State, Cmd<Msg, Sub>) {
    (State, Cmd::none())
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(_: &State) -> Html<Msg> {
    Html::h1(
        Attributes::new(),
        Events::new(),
        vec![Html::text("hello kagura")],
    )
}
