use super::peer_connection;
use crate::{idb, random_id, Config};
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
            common_database: None,
            client_id: Rc::new("".into()),
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
    client_id: Rc<String>,
}

pub enum Msg {
    NoOp,
    SetConfig(Result<task::http::Response, JsValue>),
    TryToSetCommonDatabase(Rc<web_sys::IdbDatabase>),
    SetCommonDatabaseWithClientId(Rc<web_sys::IdbDatabase>, String),
    AddClientId(Rc<web_sys::IdbDatabase>),
    PutClientId(Rc<web_sys::IdbDatabase>),
}

pub enum Sub {}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetConfig(Ok(response)) => {
            if let Some(config) = response
                .text
                .and_then(|text| toml::from_str::<Config>(&text).ok())
            {
                let common_db_name = String::from("") + &config.client.db_prefix + ".common";
                state.config = Some(Rc::new(config));
                idb::open_db(common_db_name.as_str(), |database| {
                    Msg::TryToSetCommonDatabase(Rc::new(database))
                })
            } else {
                Cmd::none()
            }
        }
        Msg::TryToSetCommonDatabase(common_database) => {
            let names = common_database.object_store_names();
            let mut has_client = false;
            let mut has_rooms = false;
            let mut has_resources = false;
            let mut has_characters = false;
            for i in 0..names.length() {
                if let Some(name) = names.item(i) {
                    if name == "client" {
                        has_client = true;
                    } else if name == "rooms" {
                        has_rooms = true;
                    } else if name == "resources" {
                        has_resources = true;
                    } else if name == "characters" {
                        has_characters = true;
                    }
                }
            }
            if !has_client {
                idb::create_object_strage(&common_database, "client", |database| {
                    Msg::TryToSetCommonDatabase(Rc::new(database))
                })
            } else if !has_rooms {
                idb::create_object_strage(&common_database, "rooms", |database| {
                    Msg::TryToSetCommonDatabase(Rc::new(database))
                })
            } else if !has_resources {
                idb::create_object_strage(&common_database, "resources", |database| {
                    Msg::TryToSetCommonDatabase(Rc::new(database))
                })
            } else if !has_characters {
                idb::create_object_strage(&common_database, "characters", |database| {
                    Msg::TryToSetCommonDatabase(Rc::new(database))
                })
            } else {
                idb::query(
                    &common_database,
                    "client",
                    idb::Query::Get(&JsValue::from("client_id")),
                    {
                        let common_database = Rc::clone(&common_database);
                        move |client_id| {
                            if let Some(client_id) = client_id.as_string() {
                                Msg::SetCommonDatabaseWithClientId(common_database, client_id)
                            } else {
                                Msg::PutClientId(common_database)
                            }
                        }
                    },
                    {
                        let common_database = Rc::clone(&common_database);
                        |_| Msg::AddClientId(common_database)
                    },
                )
            }
        }
        Msg::SetCommonDatabaseWithClientId(common_database, client_id) => {
            state.common_database = Some(common_database);
            state.client_id = Rc::new(client_id);
            Cmd::none()
        }
        Msg::AddClientId(common_database) => idb::query(
            &common_database,
            "client",
            idb::Query::Add(
                &JsValue::from("client_id"),
                &JsValue::from(random_id::base64url()),
            ),
            {
                let common_database = Rc::clone(&common_database);
                move |_| Msg::TryToSetCommonDatabase(common_database)
            },
            |_| Msg::NoOp,
        ),
        Msg::PutClientId(common_database) => idb::query(
            &common_database,
            "client",
            idb::Query::Put(
                &JsValue::from("client_id"),
                &JsValue::from(random_id::base64url()),
            ),
            {
                let common_database = Rc::clone(&common_database);
                move |_| Msg::TryToSetCommonDatabase(common_database)
            },
            |_| Msg::NoOp,
        ),
        _ => Cmd::none(),
    }
}

fn render(state: &State) -> Html {
    if let (Some(config), Some(database)) = (&state.config, &state.common_database) {
        Html::component(
            peer_connection::new(
                Rc::clone(config),
                Rc::clone(&state.client_id),
                Rc::clone(database),
            )
            .subscribe(|sub| match sub {
                peer_connection::Sub::Reconnect => Msg::NoOp,
            }),
        )
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
