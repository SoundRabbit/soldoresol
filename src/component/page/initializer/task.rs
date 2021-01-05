use crate::libs::idb;
use crate::libs::random_id;
use crate::libs::skyway::Peer;
use crate::model::config::Config;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub async fn initialize() -> Option<(Config, web_sys::IdbDatabase, String, Peer, String)> {
    let config = if let Some(config) = load_config().await {
        config
    } else {
        return None;
    };

    let db_name = format!("{}.common", config.client.db_prefix);

    let (common_db, client_id) = if let Some(db_props) = initialize_common_db(&db_name).await {
        db_props
    } else {
        return None;
    };

    let peer = if let Some(peer) = initialize_peer_connection(&config.skyway.key).await {
        peer
    } else {
        return None;
    };

    let peer_id = peer.id();

    Some((config, common_db, client_id, peer, peer_id))
}

async fn load_config() -> Option<Config> {
    crate::debug::log_1("start to load config");

    let hostname = web_sys::window().unwrap().location().hostname().unwrap();
    let is_dev_mode = hostname == "localhost";

    let config_url = if is_dev_mode {
        "/config.dev.toml"
    } else {
        "./config.toml"
    };

    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::SameOrigin);

    let request = web_sys::Request::new_with_str_and_init(config_url, &opts).unwrap();
    let response = JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request))
        .await
        .unwrap()
        .dyn_into::<web_sys::Response>()
        .unwrap();

    toml::from_str::<Config>(
        &JsFuture::from(response.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap(),
    )
    .ok()
}

async fn initialize_common_db(db_name: &str) -> Option<(web_sys::IdbDatabase, String)> {
    let database = idb::open_db(db_name).await;
    if let Some(database) = database {
        let client_id = initialize_object_store(&database).await;
        if let Some(client_id) = client_id {
            return Some((database, client_id));
        } else {
            crate::debug::log_1("faild to get client_id");
        }
    } else {
        crate::debug::log_1(format!("faild to open db: {}", db_name));
    }
    None
}

async fn initialize_object_store(database: &web_sys::IdbDatabase) -> Option<String> {
    loop {
        let names = database.object_store_names();
        let mut has_client = false;
        let mut has_rooms = false;
        let mut has_resources = false;
        let mut has_tables = false;
        let mut has_characters = false;
        for i in 0..names.length() {
            if let Some(name) = names.item(i) {
                if name == "client" {
                    has_client = true;
                } else if name == "rooms" {
                    has_rooms = true;
                } else if name == "resources" {
                    has_resources = true;
                } else if name == "tables" {
                    has_tables = true;
                } else if name == "characters" {
                    has_characters = true;
                }
            }
        }

        if has_client && has_rooms && has_resources && has_characters && has_tables {
            break;
        } else {
            if !has_client {
                idb::create_object_strage(&database, "client").await;
            } else if !has_rooms {
                idb::create_object_strage(&database, "rooms").await;
            } else if !has_resources {
                idb::create_object_strage(&database, "resources").await;
            } else if !has_tables {
                idb::create_object_strage(&database, "tables").await;
            } else {
                idb::create_object_strage(&database, "characters").await;
            }
        }
    }
    let client_id = idb::query(
        &database,
        "client",
        idb::Query::Get(&JsValue::from("client_id")),
    )
    .await;
    if let Some(client_id) = client_id.and_then(|x| x.as_string()) {
        Some(client_id)
    } else {
        assign_client_id(database).await
    }
}

async fn assign_client_id(database: &web_sys::IdbDatabase) -> Option<String> {
    let client_id = random_id::base64url();
    if idb::assign(
        database,
        "client",
        &JsValue::from("client_id"),
        &JsValue::from(&client_id),
    )
    .await
    .is_some()
    {
        Some(client_id)
    } else {
        None
    }
}

async fn initialize_peer_connection(key: &str) -> Option<Peer> {
    let peer = Rc::new(Peer::new(key));

    JsFuture::from(Promise::new(&mut move |resolve, _| {
        let a = Closure::once(Box::new({
            let peer = Rc::clone(&peer);
            move || {
                let _ = resolve.call1(&js_sys::global(), &peer);
            }
        }));
        peer.on("open", Some(a.as_ref().unchecked_ref()));
        a.forget();
    }))
    .await
    .ok()
    .and_then(|x| x.dyn_into::<Peer>().ok())
}
