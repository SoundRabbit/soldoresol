use crate::{
    model::{ResourceData, WorldData},
    JsObject,
};
use js_sys::Array;
use std::collections::BTreeSet;
use wasm_bindgen::{prelude::*, JsCast};

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
    pub id: String,
    pub payload: MeshRoom,
}

impl Room {
    pub fn new(payload: MeshRoom, id: String) -> Self {
        Self { id, payload }
    }

    pub fn send(&self, msg: Msg) {
        let msg: JsObject = msg.into();
        self.payload.send(&msg)
    }
}

pub enum Msg {
    DrawLineToTable([f64; 2], [f64; 2]),
    EraceLineToTable([f64; 2], [f64; 2]),
    CreateCharacterToTable(u128, [f64; 3]),
    SetCharacterImage(u128, u128),
    SetCharacterSize(u128, [f64; 2]),
    SetCharacterName(u128, String),
    SetObjectPosition(u128, [f64; 3]),
    SetIsBindToGrid(bool),
    SetWorld(WorldData),
    SetResource(ResourceData),
    SetConnection(BTreeSet<String>),
    RemoveObject(u128),
    InsertChatItem(u32, JsObject),
    None,
}

impl Msg {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::DrawLineToTable(..) => "DrawLineToTable",
            Self::EraceLineToTable(..) => "EraceLineToTable",
            Self::CreateCharacterToTable(..) => "CreateCharacterToTable",
            Self::SetCharacterImage(..) => "SetCharacterImage",
            Self::SetCharacterSize(..) => "SetCharacterSize",
            Self::SetCharacterName(..) => "SetCharacterName",
            Self::SetObjectPosition(..) => "SetObjectPosition",
            Self::SetIsBindToGrid(..) => "SetIsBindToGrid",
            Self::SetWorld(..) => "SetWorld",
            Self::SetResource(..) => "SetResource",
            Self::SetConnection(..) => "SetConnection",
            Self::RemoveObject(..) => "RemoveObject",
            Self::InsertChatItem(..) => "InsertChatItem",
            Self::None => "None",
        }
    }
}

impl Into<JsObject> for Msg {
    fn into(self) -> JsObject {
        let type_name = self.type_name();
        let payload: JsValue = match self {
            Self::DrawLineToTable(b, e) | Self::EraceLineToTable(b, e) => {
                array![b[0], b[1], e[0], e[1]].into()
            }
            Self::CreateCharacterToTable(id, pos) | Self::SetObjectPosition(id, pos) => {
                array![id.to_string(), pos[0], pos[1], pos[2]].into()
            }
            Self::SetCharacterImage(c_id, d_id) => {
                array![c_id.to_string(), d_id.to_string()].into()
            }
            Self::SetCharacterSize(c_id, sz) => array![c_id.to_string(), sz[0], sz[1]].into(),
            Self::SetCharacterName(c_id, name) => array![c_id.to_string(), name].into(),
            Self::SetIsBindToGrid(f) => JsValue::from(f),
            Self::SetWorld(world_data) => world_data.as_object().into(),
            Self::SetResource(resource_data) => resource_data.as_object().into(),
            Self::SetConnection(connection) => {
                let payload = array![];
                for peer_id in connection {
                    payload.push(&JsValue::from(peer_id));
                }
                payload.into()
            }
            Self::RemoveObject(id) => JsValue::from(id.to_string()),
            Self::InsertChatItem(tab_idx, chat_item) => array![tab_idx, chat_item].into(),
            Self::None => JsValue::NULL,
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
                "DrawLineToTable" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    Self::DrawLineToTable(
                        [args[0].as_f64().unwrap(), args[1].as_f64().unwrap()],
                        [args[2].as_f64().unwrap(), args[3].as_f64().unwrap()],
                    )
                }
                "EraceLineToTable" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    Self::DrawLineToTable(
                        [args[0].as_f64().unwrap(), args[1].as_f64().unwrap()],
                        [args[2].as_f64().unwrap(), args[3].as_f64().unwrap()],
                    )
                }
                "CreateCharacterToTable" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    Self::CreateCharacterToTable(
                        args[0].as_string().unwrap().parse().unwrap(),
                        [
                            args[1].as_f64().unwrap(),
                            args[2].as_f64().unwrap(),
                            args[3].as_f64().unwrap(),
                        ],
                    )
                }
                "SetObjectPosition" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    Self::SetObjectPosition(
                        args[0].as_string().unwrap().parse().unwrap(),
                        [
                            args[1].as_f64().unwrap(),
                            args[2].as_f64().unwrap(),
                            args[3].as_f64().unwrap(),
                        ],
                    )
                }
                "SetCharacterImage" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    Self::SetCharacterImage(
                        args[0].as_string().unwrap().parse().unwrap(),
                        args[1].as_string().unwrap().parse().unwrap(),
                    )
                }
                "SetCharacterSize" => {
                    let args = Array::from(&payload);
                    Self::SetCharacterSize(
                        args.get(0).as_string().unwrap().parse().unwrap(),
                        [args.get(1).as_f64().unwrap(), args.get(2).as_f64().unwrap()],
                    )
                }
                "SetCharacterName" => {
                    let args = Array::from(&payload);
                    Self::SetCharacterName(
                        args.get(0).as_string().unwrap().parse().unwrap(),
                        args.get(1).as_string().unwrap(),
                    )
                }
                "SetIsBindToGrid" => Self::SetIsBindToGrid(payload.as_bool().unwrap()),
                "SetWorld" => Self::SetWorld(WorldData::from(payload)),
                "SetResource" => Self::SetResource(ResourceData::from(payload)),
                "SetConnection" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    let peer_ids = args
                        .into_iter()
                        .map(|a| a.as_string().unwrap().parse().unwrap());

                    let mut connection = BTreeSet::new();

                    for peer_id in peer_ids {
                        connection.insert(peer_id);
                    }

                    Self::SetConnection(connection)
                }
                "InsertChatItem" => {
                    let args = Array::from(&payload);
                    Self::InsertChatItem(
                        args.get(0).as_f64().unwrap() as u32,
                        args.get(1).dyn_into::<JsObject>().unwrap(),
                    )
                }
                "RemoveObject" => Self::RemoveObject(payload.as_string().unwrap().parse().unwrap()),
                _ => Self::None,
            }
        } else {
            Self::None
        }
    }
}
