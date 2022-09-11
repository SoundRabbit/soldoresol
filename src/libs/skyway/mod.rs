use super::{color::Color, js_object::Object, random_id::U128Id};
use js_sys::Array;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(raw_module = "../src/libs/skyway/skyway.js")]
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

#[wasm_bindgen(module = "skyway-js")]
extern "C" {
    pub type DataConnection;

    #[wasm_bindgen(method, js_name = "send")]
    pub fn send(this: &DataConnection, data: &JsValue);

    #[wasm_bindgen(method)]
    pub fn on(this: &DataConnection, event: &str, listener: Option<&js_sys::Function>);

    #[wasm_bindgen(method)]
    pub fn close(this: &DataConnection, fource_close: bool);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DataConnection) -> String;
}

#[wasm_bindgen(module = "skyway-js")]
extern "C" {
    pub type MeshRoom;

    #[wasm_bindgen(method)]
    pub fn send(this: &MeshRoom, data: &JsValue);

    #[wasm_bindgen(method)]
    pub fn on(this: &MeshRoom, event: &str, listener: Option<&js_sys::Function>);

    #[wasm_bindgen(method)]
    pub fn close(this: &MeshRoom);

    #[wasm_bindgen(method, js_name = "getLog")]
    pub fn get_log(this: &MeshRoom);
}

pub enum Msg {
    None,
    PostArenaIds {
        world: U128Id,
        chat: U128Id,
        blocks: HashSet<U128Id>,
    },
    PostBlock(JsValue),
    GetBlock(U128Id),
    GetBlockResponse(JsValue),
}

impl DataConnection {
    pub fn send_msg(&self, msg: Msg) {
        let msg: Object = msg.into();
        self.send(&msg);
    }
}

impl Msg {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::PostArenaIds { .. } => "PostArenaIds",
            Self::PostBlock { .. } => "PostBlock",
            Self::GetBlock { .. } => "GetBlock",
            Self::GetBlockResponse { .. } => "GetBlockResponse",
        }
    }
}

impl Into<Object> for Msg {
    fn into(self) -> Object {
        let type_name = self.type_name();
        let payload: JsValue = match self {
            Self::None => JsValue::NULL,
            Self::PostArenaIds {
                world,
                chat,
                blocks,
            } => {
                let world = world.to_jsvalue();
                let chat = chat.to_jsvalue();
                let blocks = blocks
                    .iter()
                    .fold(js_sys::Array::new(), |blocks, block_id| {
                        blocks.push(&block_id.to_jsvalue());
                        blocks
                    });
                (object! {
                    "world": world,
                    "chat": chat,
                    "blocks": blocks
                })
                .into()
            }
            Self::PostBlock(block_data) => block_data,
            Self::GetBlock(block_id) => block_id.to_jsvalue(),
            Self::GetBlockResponse(block_data) => block_data,
        };
        object! {
            "type": type_name,
            "payload": payload
        }
    }
}

impl From<&JsValue> for Msg {
    fn from(data: &JsValue) -> Self {
        if let Some(data) = data.dyn_ref::<Object>() {
            Self::from(data)
        } else {
            Self::None
        }
    }
}

impl From<&Object> for Msg {
    fn from(obj: &Object) -> Self {
        if let (Some(msg_type), Some(payload)) = (
            obj.get("type").and_then(|t| t.as_string()),
            obj.get("payload"),
        ) {
            match msg_type.as_str() {
                "PostArenaIds" => parse_post_arena(&payload),
                "PostBlock" => Self::PostBlock(payload.into()),
                "GetBlock" => U128Id::from_jsvalue(&payload)
                    .map(|block_id| Msg::GetBlock(block_id))
                    .unwrap_or(Msg::None),
                "GetBlockResponse" => Self::GetBlockResponse(payload.into()),
                _ => Self::None,
            }
        } else {
            Self::None
        }
    }
}

fn parse_post_arena(payload: &Object) -> Msg {
    let world = unwrap!(payload.get("world"); Msg::None);
    let world = unwrap!(U128Id::from_jsvalue(&world); Msg::None);

    let chat = unwrap!(payload.get("chat"); Msg::None);
    let chat = unwrap!(U128Id::from_jsvalue(&chat); Msg::None);

    let blocks = unwrap!(payload.get("blocks"); Msg::None);
    let blocks = js_sys::Array::from(&blocks).to_vec().iter().fold(
        HashSet::new(),
        |mut blocks, block_id| {
            if let Some(block_id) = U128Id::from_jsvalue(&block_id) {
                blocks.insert(block_id);
            }
            blocks
        },
    );

    Msg::PostArenaIds {
        world,
        chat,
        blocks,
    }
}
