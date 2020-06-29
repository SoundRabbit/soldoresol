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
pub struct Item {
    display_name: String,
    peer_id: String,
    character_id: Option<BlockId>,
    icon: Icon,
    payload: String,
}

impl Item {
    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn peer_id(&self) -> &String {
        &self.peer_id
    }

    pub fn character_id(&self) -> Option<&BlockId> {
        self.character_id.as_ref()
    }

    pub fn icon(&self) -> &Icon {
        &self.icon
    }

    pub fn payload(&self) -> &String {
        &self.payload
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