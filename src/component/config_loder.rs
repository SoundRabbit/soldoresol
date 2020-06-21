use super::peer_connection;
use crate::Config;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub fn new() -> Component<Msg, State, Sub> {
    let hostname = web_sys::window().unwrap().location().hostname().unwrap();
    let is_dev_mode = hostname == "localhost";

    let init = move || {
        let state = State {
            hostname: hostname,
            config: None,
        };
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
    hostname: String,
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
            Attributes::new()
                .id("app")
                .class("centering")
                .class("fullscreen")
                .class("centering-a"),
            Events::new(),
            vec![Html::text("Loading...")],
        )
    }
}
