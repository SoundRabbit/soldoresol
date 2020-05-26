use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../src/skyway.js")]
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
}

#[wasm_bindgen]
extern "C" {
    pub type DataConnection;

    #[wasm_bindgen(method, js_name = "send")]
    pub fn send_str(this: &DataConnection, data: &str);

    #[wasm_bindgen(method)]
    pub fn on(this: &DataConnection, event: &str, listener: Option<&js_sys::Function>);

    #[wasm_bindgen(method)]
    pub fn close(this: &DataConnection, fource_close: bool);

    pub type MeshRoom;

    #[wasm_bindgen(method)]
    pub fn send(this: &MeshRoom, data: &str);

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
    pub fn data(this: &ReceiveData) -> String;

    pub type LogList;

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &LogList, index: usize) -> Option<String>;
}

impl DataConnection {
    pub fn send(&self, msg: &Msg) {
        if let Ok(data) = serde_json::to_string(msg) {
            self.send_str(&data);
        } else {
            web_sys::console::log_1(&JsValue::from(
                "some problems area occured in serializing message.",
            ));
        }
    }
}

pub struct Room {
    pub id: String,
    pub payload: MeshRoom,
}

impl Room {
    pub fn new(payload: MeshRoom, id: String) -> Self {
        Self { id, payload }
    }

    pub fn send(&self, msg: &Msg) {
        if let Ok(data) = serde_json::to_string(msg) {
            self.payload.send(&data);
        } else {
            web_sys::console::log_1(&JsValue::from(
                "some problems area occured in serializing message.",
            ));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    DrawLineToTable([f64; 2], [f64; 2]),
    EraceLineToTable([f64; 2], [f64; 2]),
    CreateCharacterToTable(u128, [f64; 3]),
    SetCharacterImage(u128, u128),
    SetObjectPosition(u128, [f64; 3]),
    SetIsBindToGrid(bool),
    SetWorld(crate::model::WorldData),
    SetResource(crate::model::ResourceData),
    AddResource(u128, crate::model::resource::DataString),
    RemoveObject(u128),
}

impl std::fmt::Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Msg::DrawLineToTable(..) => write!(f, "DrawLineToTable"),
            Msg::EraceLineToTable(..) => write!(f, "EraceLineToTable"),
            Msg::CreateCharacterToTable(..) => write!(f, "CreateCharacterToTable"),
            Msg::SetCharacterImage(..) => write!(f, "SetCharacterImage"),
            Msg::SetObjectPosition(..) => write!(f, "SetObjectPosition"),
            Msg::SetIsBindToGrid(..) => write!(f, "SetIsBindToGrid"),
            Msg::SetWorld(..) => write!(f, "SetWorld"),
            Msg::SetResource(..) => write!(f, "SetResource"),
            Msg::AddResource(..) => write!(f, "AddResource"),
            Msg::RemoveObject(..) => write!(f, "RemoveObject"),
        }
    }
}

#[derive(Deserialize)]
pub struct Log {
    #[serde(rename = "messageType")]
    pub message_type: String,

    pub message: Message,
}

#[derive(Deserialize)]
pub struct Message {
    pub src: String,
    pub data: Option<String>,
}
