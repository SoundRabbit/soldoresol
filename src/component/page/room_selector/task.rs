use crate::idb;
use crate::JsObject;
use wasm_bindgen::{prelude::*, JsCast};

pub async fn get_room_index(
    common_database: &web_sys::IdbDatabase,
) -> Option<Vec<(String, js_sys::Date)>> {
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

            if let Some(room_data) = room_data.and_then(|x| x.dyn_into::<JsObject>().ok()) {
                let last_access_time = room_data.get("last_access_time").unwrap().as_f64().unwrap();
                let last_access_time = js_sys::Date::new(&JsValue::from(last_access_time));
                rooms.push((room_id, last_access_time));
            }
        }

        Some(rooms)
    } else {
        None
    }
}
