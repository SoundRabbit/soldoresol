use crate::libs::skyway::{MeshRoom, Peer};
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub async fn close_room_connection(room: Rc<MeshRoom>) {
    let _ = JsFuture::from(Promise::new({
        let room = Rc::clone(&room);
        &mut move |resolve, _| {
            let a = Closure::wrap(Box::new(move || {
                let _ = resolve.call1(&js_sys::global(), &JsValue::null());
            }) as Box<dyn FnMut()>);
            room.on("open", Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    }))
    .await;
}

pub async fn try_to_open_room_connection(
    peer: Rc<Peer>,
    room_id: Rc<String>,
) -> Option<Rc<MeshRoom>> {
    let room = Rc::new(peer.join_room(&room_id));

    JsFuture::from(Promise::new({
        let room = Rc::clone(&room);
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
