use super::super::util::{Prop, State};
use crate::idb;
use crate::skyway::MeshRoom;
use crate::Config;
use futures::join;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub async fn initialize(
    config: Prop<Config>,
    common_db: Prop<web_sys::IdbDatabase>,
    room: MeshRoom,
    room_id: Prop<String>,
) -> Option<(web_sys::IdbDatabase, web_sys::IdbDatabase, State<MeshRoom>)> {
    let props = join!(
        initialize_room_db(config.clone(), room_id.clone()),
        initialize_table_db(config.clone()),
        initialize_room_connection(room)
    );

    if let Some((room_db, table_db, room)) = join_some!(props.0, props.1, props.2) {
        let room_meta_data: js_sys::Object = object! {
            last_access_time: js_sys::Date::now()
        }
        .into();
        idb::assign(
            &common_db,
            "rooms",
            &JsValue::from(room_id.as_str()),
            &room_meta_data,
        )
        .await;
        Some((room_db, table_db, room))
    } else {
        None
    }
}

async fn initialize_room_db(
    config: Prop<Config>,
    room_id: Prop<String>,
) -> Option<web_sys::IdbDatabase> {
    let room_db_name = format!("{}.room", config.client.db_prefix);

    let room_db = if let Some(room_db) = idb::open_db(&room_db_name).await {
        room_db
    } else {
        return None;
    };

    let room_db = if object_store_names(&room_db)
        .into_iter()
        .position(|name| name == *room_id.as_ref())
        .is_some()
    {
        room_db
    } else {
        if let Some(room_db) = idb::create_object_strage(&room_db, room_id.as_ref()).await {
            room_db
        } else {
            return None;
        }
    };

    Some(room_db)
}

async fn initialize_table_db(config: Prop<Config>) -> Option<web_sys::IdbDatabase> {
    let table_db_name = format!("{}.table", config.client.db_prefix);
    idb::open_db(&table_db_name).await
}

fn object_store_names(db: &web_sys::IdbDatabase) -> Vec<String> {
    let mut res = vec![];
    let names = db.object_store_names();

    for i in 0..names.length() {
        if let Some(name) = names.item(i) {
            res.push(name);
        }
    }

    res
}

pub async fn initialize_room_connection(room: MeshRoom) -> Option<State<MeshRoom>> {
    let room = State::new(room);

    JsFuture::from(Promise::new({
        let room = room.as_prop();
        &mut move |resolve, _| {
            let a = Closure::wrap(Box::new(move || {
                let _ = resolve.call1(&js_sys::global(), &JsValue::null());
            }) as Box<dyn FnMut()>);
            room.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    }))
    .await
    .ok()
    .map(move |_| room)
}
