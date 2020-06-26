use super::{Block, BlockId, Field};
use wasm_bindgen::prelude::*;

mod texture;

pub use texture::Texture;

#[derive(Clone)]
pub struct Table {
    name: String,
    size: [f32; 2],
    drawing_texture_id: BlockId,
    measure_texture_id: BlockId,
    image_texture_id: Option<BlockId>,
    tablemasks: Vec<BlockId>,
}

impl Table {
    pub fn new(field: &Field) -> Self {
        let texture_width = 4096;
        let texture_height = 4096;

        let size = [20.0, 20.0];
        let drawing_texture_id = field.add(Texture::new(&[4096, 4096], [20.0, 20.0]));
        let measure_texture_id = field.add(Texture::new(&[4096, 4096], [20.0, 20.0]));

        Self {
            name: "テーブル".into(),
            size,
            drawing_texture_id,
            measure_texture_id,
            image_texture_id: None,
            tablemasks: vec![],
        }
    }
}

impl Block for Table {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {}
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {}
}
