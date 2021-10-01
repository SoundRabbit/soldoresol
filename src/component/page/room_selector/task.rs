use super::RoomData;
use crate::libs::idb;
use crate::libs::js_object::Object;
use wasm_bindgen::{prelude::*, JsCast};

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
