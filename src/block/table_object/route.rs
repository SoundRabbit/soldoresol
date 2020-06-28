use super::{Block, BlockId, Field};
use crate::Promise;
use crate::{color_system, Color};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Route {
    width: f32,
    position: Vec<[f32; 2]>,
    z_rotation: f32,
    background_color: Color,
    size_is_binded: bool,
}

impl Route {
    pub fn new(property_id: BlockId) -> Self {
        Self {
            width: 1.0,
            position: vec![],
            z_rotation: 0.0,
            background_color: color_system::red((255.0 * 0.6) as u8, 5),
            size_is_binded: true,
        }
    }
}

impl Block for Route {
    fn pack(&self) -> Promise<JsValue> {
        unimplemented!();
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
