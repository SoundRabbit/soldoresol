use super::peer_connection;
use crate::{indexed_db, random_id, Config};
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub fn new() -> Component<Msg, State, Sub> {
    let hostname = web_sys::window().unwrap().location().hostname().unwrap();
    let is_dev_mode = hostname == "localhost";

    let init = move || {
        let state = State {
            hostname: hostname,
            config: None,
            common_database: None,
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
    common_database: Option<Rc<web_sys::IdbDatabase>>,
}

pub enum Msg {
    SetConfig(Result<task::http::Response, JsValue>),
    SetCommonDatabase(web_sys::IdbDatabase),
}

pub enum Sub {}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::SetConfig(Ok(response)) => {
            if let Some(config) = response
                .text
                .and_then(|text| toml::from_str::<Config>(&text).ok())
            {
                let common_db_name = String::from("") + &config.client.db_prefix + ".common";
                state.config = Some(Rc::new(config));
                indexed_db::open_db(common_db_name.as_str(), |database| {
                    Msg::SetCommonDatabase(database)
                })
            } else {
                Cmd::none()
            }
        }
        Msg::SetCommonDatabase(common_database) => {
            let names = common_database.object_store_names();
            let mut has_client = false;
            let mut has_rooms = false;
            for i in 0..names.length() {
                if let Some(name) = names.item(i) {
                    if name == "client" {
                        has_client = true;
                    } else if name == "rooms" {
                        has_rooms = true;
                    }
                }
            }
            if !has_client {
                indexed_db::create_object_strage(&common_database, "client", |database| {
                    let object_store = database
                        .transaction_with_str_and_mode(
                            "client",
                            web_sys::IdbTransactionMode::Readwrite,
                        )
                        .unwrap()
                        .object_store("client")
                        .unwrap();
                    object_store
                        .add_with_key(
                            &JsValue::from(random_id::u128val().to_string()),
                            &JsValue::from("client_id"),
                        )
                        .unwrap();
                    Msg::SetCommonDatabase(database)
                })
            } else if !has_rooms {
                indexed_db::create_object_strage(&common_database, "rooms", |database| {
                    Msg::SetCommonDatabase(database)
                })
            } else {
                state.common_database = Some(Rc::new(common_database));
                Cmd::none()
            }
        }
        _ => Cmd::none(),
    }
}

fn render(state: &State) -> Html<Msg> {
    if let (Some(config), Some(database)) = (&state.config, &state.common_database) {
        Html::component(peer_connection::new(Rc::clone(config), Rc::clone(database)))
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
