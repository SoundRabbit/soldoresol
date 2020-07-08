use super::{Block, BlockId, Field};
use crate::{resource::ResourceId, Promise};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum Icon {
    None,
    Resource(ResourceId),
    DefaultUser,
}

#[derive(Clone)]
pub enum Sender {
    System,
    User,
    Character(BlockId),
}

#[derive(Clone)]
pub struct Item {
    peer_id: String,
    display_name: String,
    icon: Icon,
    sender: Sender,
    text: String,
    reply: Option<String>,
}

impl Item {
    pub fn new(
        peer_id: String,
        display_name: String,
        icon: Icon,
        sender: Sender,
        text: String,
        reply: Option<String>,
    ) -> Self {
        Self {
            peer_id,
            display_name,
            icon,
            sender,
            text,
            reply,
        }
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn peer_id(&self) -> &String {
        &self.peer_id
    }

    pub fn icon(&self) -> &Icon {
        &self.icon
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn reply(&self) -> Option<&String> {
        self.reply.as_ref()
    }
}

impl Block for Item {
    fn pack(&self) -> Promise<JsValue> {
        let data = object! {};
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
