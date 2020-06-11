use super::peer_connection;
use crate::Config;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub fn new() -> Component<Msg, State, Sub> {
    let is_dev_mode = web_sys::window().unwrap().location().hostname().unwrap() == "localhost";

    let init = move || {
        let state = State { config: None };
        let config_url = if is_dev_mode {
            "/config.dev.toml"
        } else {
            "./config.toml"
        };
        let task = Cmd::task(task::http::get(config_url, task::http::Props::new(), |r| {
            Msg::SetConfig(r)
        }));
        (state, task)
    };
    Component::new(init, update, render)
}

pub struct State {
    config: Option<Rc<Config>>,
}

pub enum Msg {
    SetConfig(Result<task::http::Response, JsValue>),
}

pub enum Sub {}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::SetConfig(Ok(response)) => {
            if let Some(config) = response
                .text
                .and_then(|text| toml::from_str::<Config>(&text).ok())
            {
                state.config = Some(Rc::new(config));
            }
            Cmd::none()
        }
        _ => Cmd::none(),
    }
}

fn render(state: &State) -> Html<Msg> {
    if let Some(config) = &state.config {
        Html::component(peer_connection::new(Rc::clone(config)))
    } else {
        Html::div(
            Attributes::new().class("app").id("app"),
            Events::new(),
            vec![],
        )
    }
}
