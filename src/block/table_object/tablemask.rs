use super::{Block, BlockId, Field};
use crate::resource::ResourceId;
use crate::Color;
use crate::Promise;
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Tablemask {
    size: [f32; 3],
    position: [f32; 3],
    color: Color,
    is_rounded: bool,
    is_inved: bool,
    is_fixed: bool,
}

impl Tablemask {
    pub fn new(size: &[f32; 2], color: Color, is_rounded: bool, is_inved: bool) -> Self {
        Self {
            size: [size[0], size[1], 0.0],
            position: [0.0, 0.0, 0.0],
            color: color,
            is_rounded: is_rounded,
            is_inved: is_inved,
            is_fixed: false,
        }
    }

    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: &[f32; 2]) {
        self.size = [size[0], size[1], 0.0];
    }

    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f32; 2]) {
        self.position = [position[0], position[1], 0.0];
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn is_rounded(&self) -> bool {
        self.is_rounded
    }

    pub fn set_is_rounded(&mut self, is_rounded: bool) {
        self.is_rounded = is_rounded;
    }

    pub fn is_inved(&self) -> bool {
        self.is_inved
    }

    pub fn set_is_inved(&mut self, is_inved: bool) {
        self.is_inved = is_inved;
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed;
    }
}

impl Block for Tablemask {
    fn pack(&self) -> Promise<JsValue> {
        let size = array![self.size[0], self.size[1], self.size[2]];
        let position = array![self.position[0], self.position[1], self.position[2]];
        let color = self.color.to_u32();

        let data = object! {
            size: size,
            position: position,
            color: color,
            is_inved: self.is_inved,
            is_rounded: self.is_rounded,
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

            if let (
                Some(size),
                Some(position),
                Some(color),
                Some(is_inved),
                Some(is_rounded),
                Some(is_fixed),
            ) = (
                val.get("size").map(|s| {
                    let s: js_sys::Object = s.into();
                    js_sys::Array::from(s.as_ref())
                }),
                val.get("position").map(|p| {
                    let p: js_sys::Object = p.into();
                    js_sys::Array::from(p.as_ref())
                }),
                val.get("color").and_then(|c| c.as_f64().map(|c| c as u32)),
                val.get("is_inved").and_then(|f| f.as_bool()),
                val.get("is_rounded").and_then(|f| f.as_bool()),
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
                        is_inved: is_inved,
                        is_rounded: is_rounded,
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

    fn resources(&self, _: &Field) -> HashSet<ResourceId> {
        set! {}
    }
}
