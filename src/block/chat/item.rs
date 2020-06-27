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

impl Block for Item {
    fn pack(&self) -> Promise<JsValue, ()> {
        unimplemented!();
    }
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()> {
        unimplemented!();
    }
}
