use super::{Block, BlockId, Field};
use crate::{color_system, resource::ResourceId, Color, JsObject, Promise};
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Tag {
    name: String,
    color: Color,
}

impl Tag {
    pub fn new() -> Self {
        Self {
            name: String::from("新規タグ"),
            color: color_system::gray(255, 5),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Block for Tag {
    fn pack(&self) -> Promise<JsValue> {
        let data = object! {
            name: &self.name,
            color: self.color.to_u32()
        };

        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();

        Promise::new(|resolve| resolve(Some(data)))
    }

    fn unpack(_: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let name = val.get("name").and_then(|x| x.as_string());
            let color = val
                .get("color")
                .and_then(|x| x.as_f64())
                .map(|x| Color::from(x as u32));
            if let (Some(name), Some(color)) = (name, color) {
                Some(Box::new(Self { name, color }))
            } else {
                None
            }
        } else {
            None
        };
        Promise::new(move |resolve| resolve(self_))
    }

    fn dependents(&self, _: &Field) -> HashSet<BlockId> {
        set! {}
    }

    fn resources(&self, _: &Field) -> HashSet<ResourceId> {
        set! {}
    }
}
