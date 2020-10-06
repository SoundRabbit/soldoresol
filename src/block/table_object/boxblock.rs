use super::{Block, BlockId, Field};
use crate::Color;
use crate::Promise;
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Boxblock {
    position: [f32; 3],
    size: [f32; 3],
    color: Color,
    is_fixed: bool,
}

impl Boxblock {
    pub fn new(position: [f32; 3], size: [f32; 3], color: Color) -> Self {
        Self {
            position,
            size,
            color,
            is_fixed: true,
        }
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.position = pos;
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f32; 3]) {
        self.size = size;
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed;
    }
}

impl Block for Boxblock {
    fn pack(&self) -> Promise<JsValue> {
        let size = array![self.size[0], self.size[1], self.size[2]];
        let position = array![self.position[0], self.position[1], self.position[2]];
        let color = self.color.to_u32();

        let data = object! {
            size: size,
            position: position,
            color: color,
            is_fixed: self.is_fixed
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(_: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        Promise::new(|resolve| {
            use crate::JsObject;

            let val = val.dyn_into::<JsObject>().unwrap();

            if let (Some(size), Some(position), Some(color), Some(is_fixed)) = (
                val.get("size").map(|s| {
                    let s: js_sys::Object = s.into();
                    js_sys::Array::from(s.as_ref())
                }),
                val.get("position").map(|p| {
                    let p: js_sys::Object = p.into();
                    js_sys::Array::from(p.as_ref())
                }),
                val.get("color").and_then(|c| c.as_f64().map(|c| c as u32)),
                val.get("is_fixed").and_then(|f| f.as_bool()),
            ) {
                let size = if let (Some(x), Some(y), Some(z)) = (
                    size.get(0).as_f64().map(|x| x as f32),
                    size.get(1).as_f64().map(|x| x as f32),
                    size.get(2).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y, z])
                } else {
                    None
                };

                let position = if let (Some(x), Some(y), Some(z)) = (
                    position.get(0).as_f64().map(|x| x as f32),
                    position.get(1).as_f64().map(|x| x as f32),
                    position.get(2).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y, z])
                } else {
                    None
                };

                if let (Some(size), Some(position)) = (size, position) {
                    let tablemask = Self {
                        size: size,
                        position: position,
                        color: Color::from(color),
                        is_fixed: is_fixed,
                    };
                    resolve(Some(Box::new(tablemask)));
                } else {
                    resolve(None);
                }
            } else {
                resolve(None);
            }
        })
    }
    fn dependents(&self, _: &Field) -> HashSet<BlockId> {
        set! {}
    }
}
