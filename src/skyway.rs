use crate::JsObject;
use js_sys::Object;
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
    pub fn data(this: &ReceiveData) -> JsObject;
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
        self.payload.send(&msg.as_object())
    }
}

pub enum Msg {
    DrawLineToTable([f64; 2], [f64; 2]),
    EraceLineToTable([f64; 2], [f64; 2]),
    CreateCharacterToTable(u128, [f64; 3]),
    SetCharacterImage(u128, u128),
    SetObjectPosition(u128, [f64; 3]),
    SetIsBindToGrid(bool),
    SetWorld(crate::model::WorldData),
    SetResource(crate::model::ResourceData),
    SetConnection(std::collections::BTreeSet<String>),
    AddResource(u128, crate::model::resource::DataString),
    RemoveObject(u128),
    None,
}

impl Msg {
    pub fn type_name(&self) -> String {
        format!("{}", self)
    }

    pub fn as_object(&self) -> JsObject {
        match self {
            Self::DrawLineToTable(b, e) | Self::EraceLineToTable(b, e) => object! {
                type: &self.type_name(),
                payload: array![array![b[0], b[1]], array![e[0], e[1]]]
            },
            Self::CreateCharacterToTable(id, pos) | Self::SetObjectPosition(id, pos) => object! {
                type: &self.type_name(),
                payload: array![id.to_string(), array![pos[0], pos[1], pos[2]]]
            },
            Self::SetCharacterImage(c_id, d_id) => object! {
                type: &self.type_name(),
                payload: array![c_id.to_string(), d_id.to_string()]
            },
            Self::SetIsBindToGrid(f) => object! {
                type: &self.type_name(),
                payload: *f
            },
            Self::SetWorld(world_data) => {
                let payload: JsObject = world_data.as_object();
                object! {
                    type: &self.type_name(),
                    payload: payload
                }
            }
            Self::SetResource(resource_data) => {
                let payload: JsObject = resource_data.as_object();
                object! {
                    type: &self.type_name(),
                    payload: payload
                }
            }
            Self::SetConnection(connection) => {
                let payload = array![];

                for peer_id in connection {
                    payload.push(&JsValue::from(peer_id));
                }

                object! {
                    type: &self.type_name(),
                    payload: payload
                }
            }
            Self::AddResource(id, payload) => object! {
                type: &self.type_name(),
                payload: array![id.to_string(), payload.as_object()]
            },
            Self::RemoveObject(id) => object! {
                type: &self.type_name(),
                payload: id.to_string()
            },
            Self::None => object! {
                type: &self.type_name(),
                payload: JsValue::NULL
            },
        }
    }
}

impl std::fmt::Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DrawLineToTable(..) => write!(f, "DrawLineToTable"),
            Self::EraceLineToTable(..) => write!(f, "EraceLineToTable"),
            Self::CreateCharacterToTable(..) => write!(f, "CreateCharacterToTable"),
            Self::SetCharacterImage(..) => write!(f, "SetCharacterImage"),
            Self::SetObjectPosition(..) => write!(f, "SetObjectPosition"),
            Self::SetIsBindToGrid(..) => write!(f, "SetIsBindToGrid"),
            Self::SetWorld(..) => write!(f, "SetWorld"),
            Self::SetResource(..) => write!(f, "SetResource"),
            Self::SetConnection(..) => write!(f, "SetConnection"),
            Self::AddResource(..) => write!(f, "AddResource"),
            Self::RemoveObject(..) => write!(f, "RemoveObject"),
            Self::None => write!(f, "None"),
        }
    }
}

impl From<JsObject> for Msg {
    fn from(obj: JsObject) -> Self {
        if let (Some(msg_type), Some(payload)) = (
            obj.get("type").and_then(|t| t.as_string()),
            obj.get("payload"),
        ) {
            match msg_type {
                _ => Self::None,
            }
        } else {
            Self::None
        }
    }
}
