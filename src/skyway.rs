use crate::{
    model::{CharacterData, ResourceData, TablemaskData, WorldData},
    JsObject,
};
use js_sys::Array;
use std::{collections::BTreeSet, rc::Rc};
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
    DrawLineToTable([f64; 2], [f64; 2]),
    EraceLineToTable([f64; 2], [f64; 2]),
    SetTableSize([f64; 2]),
    SetTableImage(u128),
    CreateCharacterToTable(u128, CharacterData),
    CreateTablemaskToTable(u128, TablemaskData),
    SetCharacterImage(u128, u128),
    SetCharacterSize(u128, [f64; 2]),
    SetCharacterName(u128, String),
    SetCharacterProperty(u128, JsObject),
    SetTablemaskSizeWithStyle(u128, [f64; 2], bool),
    SetTablemaskColor(u128, u32),
    SetObjectPosition(u128, [f64; 3]),
    CloneObject(u128),
    BindObjectToTableGrid(u128),
    SetIsBindToGrid(bool),
    SetWorld(WorldData),
    SetResource(ResourceData),
    SetChat(JsObject),
    SetConnection(BTreeSet<String>),
    RemoveObject(u128),
    InsertChatItem(u32, JsObject),
    AddChatTab,
    SetChatTabName(u32, String),
    RemoveChatTab(u32),
    None,
}

impl Msg {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::DrawLineToTable(..) => "DrawLineToTable",
            Self::EraceLineToTable(..) => "EraceLineToTable",
            Self::SetTableSize(..) => "SetTableSize",
            Self::SetTableImage(..) => "SetTableImage",
            Self::CreateCharacterToTable(..) => "CreateCharacterToTable",
            Self::CreateTablemaskToTable(..) => "CreateTablemaskToTable",
            Self::SetCharacterImage(..) => "SetCharacterImage",
            Self::SetCharacterSize(..) => "SetCharacterSize",
            Self::SetCharacterName(..) => "SetCharacterName",
            Self::SetCharacterProperty(..) => "SetCharacterProperty",
            Self::SetTablemaskSizeWithStyle(..) => "SetTablemaskSizeWithStyle",
            Self::SetTablemaskColor(..) => "SetTablemaskColor",
            Self::SetObjectPosition(..) => "SetObjectPosition",
            Self::CloneObject(..) => "CloneObject",
            Self::BindObjectToTableGrid(..) => "BindObjectToTableGrid",
            Self::SetIsBindToGrid(..) => "SetIsBindToGrid",
            Self::SetWorld(..) => "SetWorld",
            Self::SetResource(..) => "SetResource",
            Self::SetChat(..) => "SetChat",
            Self::SetConnection(..) => "SetConnection",
            Self::RemoveObject(..) => "RemoveObject",
            Self::InsertChatItem(..) => "InsertChatItem",
            Self::AddChatTab => "AddChatTab",
            Self::SetChatTabName(..) => "SetChatTabName",
            Self::RemoveChatTab(..) => "RemoveChatTab",
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
            Self::SetTableSize(sz) => array![sz[0], sz[1]].into(),
            Self::SetTableImage(id) => JsValue::from(id.to_string()),
            Self::CreateCharacterToTable(id, character) => {
                let character: JsObject = character.into();
                let character: JsValue = character.into();
                array![id.to_string(), character].into()
            }
            Self::CreateTablemaskToTable(id, tablemask) => {
                let tablemask: JsObject = tablemask.into();
                let tablemask: JsValue = tablemask.into();
                array![id.to_string(), tablemask].into()
            }
            Self::SetObjectPosition(id, pos) => {
                array![id.to_string(), pos[0], pos[1], pos[2]].into()
            }
            Self::SetCharacterImage(c_id, d_id) => {
                array![c_id.to_string(), d_id.to_string()].into()
            }
            Self::SetCharacterSize(c_id, sz) => array![c_id.to_string(), sz[0], sz[1]].into(),
            Self::SetCharacterName(c_id, name) => array![c_id.to_string(), name].into(),
            Self::SetCharacterProperty(c_id, prop) => array![c_id.to_string(), prop].into(),
            Self::SetTablemaskSizeWithStyle(t_id, sz, r) => {
                array![t_id.to_string(), sz[0], sz[1], r].into()
            }
            Self::SetTablemaskColor(t_id, color) => array![t_id.to_string(), color].into(),
            Self::BindObjectToTableGrid(id) | Self::RemoveObject(id) | Self::CloneObject(id) => {
                JsValue::from(id.to_string())
            }
            Self::SetIsBindToGrid(f) => JsValue::from(f),
            Self::SetWorld(world_data) => {
                let world_data: JsObject = world_data.into();
                world_data.into()
            }
            Self::SetChat(chat) => chat.into(),
            Self::SetResource(resource_data) => resource_data.as_object().into(),
            Self::SetConnection(connection) => {
                let payload = array![];
                for peer_id in connection {
                    payload.push(&JsValue::from(peer_id));
                }
                payload.into()
            }
            Self::InsertChatItem(tab_idx, chat_item) => array![tab_idx, chat_item].into(),
            Self::AddChatTab => js_sys::Object::new().into(),
            Self::SetChatTabName(tab_idx, name) => array![tab_idx, name].into(),
            Self::RemoveChatTab(tab_idx) => JsValue::from(tab_idx),
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
                    Self::EraceLineToTable(
                        [args[0].as_f64().unwrap(), args[1].as_f64().unwrap()],
                        [args[2].as_f64().unwrap(), args[3].as_f64().unwrap()],
                    )
                }
                "SetTableSize" => {
                    let args = Array::from(&payload);
                    Self::SetTableSize([
                        args.get(0).as_f64().unwrap(),
                        args.get(1).as_f64().unwrap(),
                    ])
                }
                "SetTableImage" => {
                    Self::SetTableImage(payload.as_string().unwrap().parse().unwrap())
                }
                "CreateCharacterToTable" => {
                    let args = Array::from(&payload);
                    Self::CreateCharacterToTable(
                        args.get(0).as_string().unwrap().parse().unwrap(),
                        CharacterData::from(args.get(1).dyn_into::<JsObject>().unwrap()),
                    )
                }
                "CreateTablemaskToTable" => {
                    let args = Array::from(&payload);
                    Self::CreateTablemaskToTable(
                        args.get(0).as_string().unwrap().parse().unwrap(),
                        TablemaskData::from(args.get(0).dyn_into::<JsObject>().unwrap()),
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
                "SetCharacterProperty" => {
                    let args = Array::from(&payload);
                    Self::SetCharacterProperty(
                        args.get(0).as_string().unwrap().parse().unwrap(),
                        args.get(1).dyn_into::<JsObject>().unwrap(),
                    )
                }
                "SetTablemaskSizeWithStyle" => {
                    let args = payload.dyn_ref::<Array>().unwrap().to_vec();
                    Self::SetTablemaskSizeWithStyle(
                        args[0].as_string().unwrap().parse().unwrap(),
                        [args[1].as_f64().unwrap(), args[2].as_f64().unwrap()],
                        args[3].as_bool().unwrap(),
                    )
                }
                "SetTablemaskColor" => {
                    let args = Array::from(&payload);
                    Self::SetTablemaskColor(
                        args.get(0).as_string().unwrap().parse().unwrap(),
                        args.get(1).as_f64().unwrap() as u32,
                    )
                }
                "CloneObject" => Self::CloneObject(payload.as_string().unwrap().parse().unwrap()),
                "BindObjectToTableGrid" => {
                    Self::BindObjectToTableGrid(payload.as_string().unwrap().parse().unwrap())
                }
                "SetIsBindToGrid" => Self::SetIsBindToGrid(payload.as_bool().unwrap()),
                "SetWorld" => Self::SetWorld(WorldData::from(payload)),
                "SetResource" => Self::SetResource(ResourceData::from(payload)),
                "SetChat" => Self::SetChat(payload),
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
                "AddChatTab" => Self::AddChatTab,
                "SetChatTabName" => {
                    let args = Array::from(&payload);
                    Self::SetChatTabName(
                        args.get(0).as_f64().unwrap() as u32,
                        args.get(1).as_string().unwrap(),
                    )
                }
                "RemoveChatTab" => Self::RemoveChatTab(payload.as_f64().unwrap() as u32),
                "RemoveObject" => Self::RemoveObject(payload.as_string().unwrap().parse().unwrap()),
                _ => Self::None,
            }
        } else {
            Self::None
        }
    }
}
