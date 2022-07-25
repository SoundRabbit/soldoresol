use super::RoomData;
use crate::libs::gapi::{gapi, GoogleResponse};
use crate::libs::idb;
use crate::libs::js_object::Object;
use crate::model::config::Config;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

pub async fn get_room_index(common_database: &web_sys::IdbDatabase) -> Option<Vec<RoomData>> {
    let room_ids = idb::query(common_database, "rooms", idb::Query::GetAllKeys).await;

    if let Some(room_ids) = room_ids {
        let room_ids = js_sys::Array::from(&room_ids)
            .to_vec()
            .into_iter()
            .filter_map(|x| x.as_string());
        let mut rooms = vec![];

        for room_id in room_ids {
            let room_data = idb::query(
                common_database,
                "rooms",
                idb::Query::Get(&JsValue::from(&room_id)),
            )
            .await;

            if let Some(room_data) = room_data.and_then(|x| x.dyn_into::<Object>().ok()) {
                let last_access_time = room_data.get("last_access_time").unwrap().as_f64().unwrap();
                let last_access_time = js_sys::Date::new(&JsValue::from(last_access_time));

                let room_name = room_data
                    .get("room_name")
                    .and_then(|x| x.as_string())
                    .unwrap_or(String::from("Nameless Room"));

                let room_description = room_data
                    .get("room_description")
                    .and_then(|x| x.as_string())
                    .unwrap_or(String::from("なし"));

                rooms.push(RoomData {
                    id: room_id,
                    name: room_name,
                    last_access_time: last_access_time,
                    description: room_description,
                });
            }
        }

        Some(rooms)
    } else {
        None
    }
}

pub async fn remove_room(
    config: Rc<Config>,
    room_id: &String,
    common_database: &web_sys::IdbDatabase,
) {
    let room_db_name = format!("{}.room", config.client.db_prefix);
    let room_db = unwrap!(idb::open_db(&room_db_name).await);
    idb::delete_object_store(&room_db, room_id.clone()).await;
    idb::query(
        common_database,
        "rooms",
        idb::Query::Delete(&JsValue::from(room_id.as_str())),
    )
    .await;
}

pub async fn initialize_google_drive() -> Option<Vec<RoomData>> {
    if let Some(rooms_id) = initialize_google_drive_rooms().await {
        let mut next_page_token = None;
        let mut rooms = vec![];

        loop {
            let (npt, mut roomes_seg) =
                get_room_index_from_drive(&rooms_id, next_page_token.as_ref()).await;
            rooms.append(&mut roomes_seg);

            if npt.is_some() {
                next_page_token = npt;
            } else {
                break;
            }
        }

        Some(rooms)
    } else {
        None
    }
}

async fn initialize_google_drive_rooms() -> Option<String> {
    JsFuture::from(Promise::new(&mut move |resolve, _| {
        let a = Closure::wrap(Box::new({
            let resolve = Rc::new(resolve);
            move |response: GoogleResponse| {
                let rooms = response
                    .result()
                    .get("files")
                    .and_then(|x| js_sys::Array::from(&x).to_vec().pop());
                if let Some(rooms) = rooms {
                    crate::debug::log_2("rooms?:", &JsValue::from(&rooms));
                    let _ = resolve.call1(&js_sys::global(), &rooms);
                } else {
                    crate::debug::log_2("response?", &JsValue::from(&response));
                    let a = Closure::wrap(Box::new({
                        let resolve = Rc::clone(&resolve);
                        move |err: JsValue, rooms: Object| {
                            crate::debug::log_2("err?:", &JsValue::from(&err));
                            crate::debug::log_2("rooms?:", &JsValue::from(&rooms));
                            let _ = resolve.call1(&js_sys::global(), &rooms);
                        }
                    })
                        as Box<dyn FnMut(JsValue, Object)>);
                    gapi.client()
                        .drive()
                        .files()
                        .create(
                            object! {
                                "resource": object! {
                                    "name": "rooms",
                                    "mimeType": "application/vnd.google-apps.folder",
                                    "parents": array!["appDataFolder"]
                                },
                                "fields": "id"
                            }
                            .as_ref(),
                        )
                        .then(Some(a.as_ref().unchecked_ref()), None);
                    a.forget();
                }
            }
        }) as Box<dyn FnMut(_)>);
        gapi.client()
            .drive()
            .files()
            .list(
                object! {
                    "pageSize": 1,
                    "q": "mimeType = 'application/vnd.google-apps.folder' and name = 'rooms'",
                    "fields": "files(id)",
                    "scope": array!["appDataFolder"]
                }
                .as_ref(),
            )
            .then(Some(a.as_ref().unchecked_ref()), None);
        a.forget();
    }))
    .await
    .ok()
    .and_then(|x| {
        crate::debug::log_2("rooms:", x.as_ref());
        js_sys::Reflect::get(&x, &JsValue::from("id")).ok()
    })
    .and_then(|x| x.as_string())
}

async fn get_room_index_from_drive(
    rooms_id: &String,
    next_page_token: Option<&String>,
) -> (Option<String>, Vec<RoomData>) {
    if let Ok(responce) = JsFuture::from(Promise::new(&mut move |resolve, _| {
        let a = Closure::wrap(Box::new({
            let resolve = Rc::new(resolve);
            move |responce: GoogleResponse| {
                let _ = resolve.call1(&js_sys::global(), &responce);
            }
        }) as Box<dyn FnMut(_)>);
        gapi.client()
            .drive()
            .files()
            .list(
                object! {
                    "pageSize": 1,
                    "fields": "nextPageToken, files(id, webContentLink)",
                    "scope": array![rooms_id],
                    "pageToken": next_page_token.map(|x| JsValue::from(x)).unwrap_or(JsValue::null())
                }
                .as_ref(),
            )
            .then(Some(a.as_ref().unchecked_ref()), None);
        a.forget();
    })).await {
        let next_page_token = js_sys::Reflect::get(&responce, &JsValue::from("nextPageToken")).ok().and_then(|x| x.as_string());
        let rooms = js_sys::Reflect::get(&responce, &JsValue::from("files"))
        .ok()
        .and_then(|x| if js_sys::Array::is_array(&x) {
            Some(js_sys::Array::from(&x).to_vec())
        } else {
            None
        }).unwrap_or(vec![])
        .into_iter()
        .filter_map(|room|
            join_some!(
                js_sys::Reflect::get(&room, &JsValue::from("id")).ok().and_then(|x| x.as_string()),
                js_sys::Reflect::get(&room, &JsValue::from("webContentLink")).ok().and_then(|x| x.as_string())
            )
        ).collect::<Vec<_>>();

        let mut room_data_list = vec![];

        for (room_id, web_content_link) in rooms {
            if let Some(room_data) = get_room_from_drive(&room_id, &web_content_link).await {
                room_data_list.push(room_data);
            }
        }
        (next_page_token, room_data_list)
    } else {
        (None, vec![])
    }
}

async fn get_room_from_drive(room_id: &String, web_content_link: &String) -> Option<RoomData> {
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(web_content_link, &opts).unwrap();
    let response = JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request))
        .await
        .unwrap()
        .dyn_into::<web_sys::Response>()
        .unwrap();

    let data: Option<toml::Value> = toml::from_str(
        &JsFuture::from(response.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap(),
    )
    .ok();

    if let Some(toml::Value::Table(data)) = data {
        let last_access_time = data
            .get("last_access_time")
            .and_then(|x| {
                if let toml::Value::Datetime(x) = x {
                    Some(x)
                } else {
                    None
                }
            })
            .unwrap()
            .to_string();
        let last_access_time = js_sys::Date::new(&JsValue::from(last_access_time));

        let room_name = data
            .get("name")
            .and_then(|x| {
                if let toml::Value::String(x) = x {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .unwrap_or(String::from("Nameless Room"));

        let room_description = data
            .get("description")
            .and_then(|x| {
                if let toml::Value::String(x) = x {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .unwrap_or(String::from("なし"));

        Some(RoomData {
            id: room_id.clone(),
            name: room_name,
            last_access_time: last_access_time,
            description: room_description,
        })
    } else {
        None
    }
}
