use super::Block;
use super::BlockId;
use wasm_bindgen::prelude::*;

mod texture;

pub use texture::Texture;

pub struct Table {
    name: String,
    size: [f32; 2],
    pixel_ratio: [f32; 2],
    drawing_texture_id: BlockId,
    measure_texture_id: BlockId,
    image_texture_id: Option<BlockId>,
}

impl Table {}

impl Block for Table {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {}
    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {}
}
