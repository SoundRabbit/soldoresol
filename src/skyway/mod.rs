use crate::{random_id::U128Id, JsObject};
use js_sys::Array;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../src/skyway/skyway.js")]
extern "C" {
    pub type Peer;

    #[wasm_bindgen(constructor)]
    pub fn new(key: &str) -> Peer;

    #[wasm_bindgen(method)]
    pub fn on(this: &Peer, event: &str, listener: Option<&js_sys::Function>);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Peer) -> String;

    #[wasm_bindgen(method)]
    pub fn connect(this: &Peer, peer_id: &str) -> DataConnection;

    #[wasm_bindgen(method, js_name = "joinRoom")]
    pub fn join_room(this: &Peer, room_id: &str) -> MeshRoom;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Peer);

    #[wasm_bindgen(method)]
    pub fn reconnect(this: &Peer);
}

#[wasm_bindgen]
extern "C" {
    pub type DataConnection;

    #[wasm_bindgen(method, js_name = "send")]
    pub fn send(this: &DataConnection, data: &JsValue);

    #[wasm_bindgen(method)]
    pub fn on(this: &DataConnection, event: &str, listener: Option<&js_sys::Function>);

    #[wasm_bindgen(method)]
    pub fn close(this: &DataConnection, fource_close: bool);

    pub type MeshRoom;

    #[wasm_bindgen(method)]
    pub fn send(this: &MeshRoom, data: &JsValue);

    #[wasm_bindgen(method)]
    pub fn on(this: &MeshRoom, event: &str, listener: Option<&js_sys::Function>);

    #[wasm_bindgen(method)]
    pub fn close(this: &MeshRoom);

    #[wasm_bindgen(method, js_name = "getLog")]
    pub fn get_log(this: &MeshRoom);

    pub type ReceiveData;

    #[wasm_bindgen(method, getter)]
    pub fn src(this: &ReceiveData) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &ReceiveData) -> Option<JsObject>;
}

pub struct Room {
    pub id: Rc<String>,
    pub payload: MeshRoom,
}

impl Room {
    pub fn new(payload: MeshRoom, id: Rc<String>) -> Self {
        Self { id, payload }
    }

    pub fn send(&self, msg: Msg) {
        let msg: JsObject = msg.into();
        self.payload.send(&msg)
    }
}

pub enum Msg {
    None,
    SetContext { world: U128Id, chat: U128Id },
    SetBlockPacks(HashMap<U128Id, JsValue>),
    SetResourcePacks(HashMap<U128Id, JsValue>),
    InsertChatItem(U128Id, U128Id, f64),
}

impl Msg {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SetContext { .. } => "SetContext",
            Self::SetBlockPacks(..) => "SetBlockPacks",
            Self::SetResourcePacks(..) => "SetResourcePacks",
            Self::InsertChatItem(..) => "InsertChatItem",
        }
    }
}

impl Into<JsObject> for Msg {
    fn into(self) -> JsObject {
        let type_name = self.type_name();
        let payload: JsValue = match self {
            Self::None => JsValue::NULL,
            Self::SetContext { chat, world } => {
                let payload = object! {
                    chat: chat.to_jsvalue(),
                    world: world.to_jsvalue()
                };
                let payload: js_sys::Object = payload.into();
                payload.into()
            }
            Self::SetBlockPacks(packs) | Self::SetResourcePacks(packs) => {
                let payload = Array::new();
                for (id, pack) in packs {
                    payload.push(array![id.to_jsvalue(), pack].as_ref());
                }
                payload.into()
            }
            Self::InsertChatItem(tab_id, item, timestamp) => {
                array![tab_id.to_jsvalue(), item.to_jsvalue(), timestamp].into()
            }
        };
        object! {
            type: type_name,
            payload: payload
        }
    }
}

impl From<JsObject> for Msg {
    fn from(obj: JsObject) -> Self {
        if let (Some(msg_type), Some(payload)) = (
            obj.get("type").and_then(|t| t.as_string()),
            obj.get("payload"),
        ) {
            match msg_type.as_str() {
                "SetContext" => {
                    if let (Some(chat), Some(world)) = (
                        payload.get("chat").and_then(|x| U128Id::from_jsvalue(&x)),
                        payload.get("world").and_then(|x| U128Id::from_jsvalue(&x)),
                    ) {
                        Self::SetContext { chat, world }
                    } else {
                        Self::None
                    }
                }
                "SetBlockPacks" => {
                    let payload: js_sys::Object = payload.into();
                    let payload = Array::from(payload.as_ref()).to_vec();
                    let mut packs = HashMap::new();
                    for row in payload {
                        let cols = Array::from(row.as_ref()).to_vec();
                        if let (Some(id), Some(data)) =
                            (U128Id::from_jsvalue(&cols[0]), cols.get(1))
                        {
                            packs.insert(id, data.clone());
                        }
                    }
                    Msg::SetBlockPacks(packs)
                }
                "SetResourcePacks" => {
                    let payload: js_sys::Object = payload.into();
                    let payload = Array::from(payload.as_ref()).to_vec();
                    let mut packs = HashMap::new();
                    for row in payload {
                        let cols = Array::from(row.as_ref()).to_vec();
                        if let (Some(id), Some(data)) =
                            (U128Id::from_jsvalue(&cols[0]), cols.get(1))
                        {
                            packs.insert(id, data.clone());
                        }
                    }
                    Msg::SetResourcePacks(packs)
                }
                "InsertChatItem" => {
                    let payload: js_sys::Object = payload.into();
                    let payload = Array::from(payload.as_ref());
                    let tab_id = U128Id::from_jsvalue(&payload.get(0));
                    let item = U128Id::from_jsvalue(&payload.get(1));
                    let timestamp = payload.get(2).as_f64();
                    if let (Some(tab_id), Some(item), Some(timestamp)) = (tab_id, item, timestamp) {
                        Self::InsertChatItem(tab_id, item, timestamp)
                    } else {
                        Self::None
                    }
                }
                _ => Self::None,
            }
        } else {
            Self::None
        }
    }
}
