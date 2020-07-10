use super::{Block, Field};
use crate::Color;
use crate::JsObject;
use crate::Promise;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub enum Type {
    Line(f64),
    Rounded,
}

#[derive(Clone)]
pub struct Area {
    origin: [f32; 3],
    vector: [f32; 3],
    color_1: Color,
    color_2: Color,
    type_: Type,
}

impl Type {
    pub fn is_line(&self) -> bool {
        match self {
            Self::Line(..) => true,
            _ => false,
        }
    }

    pub fn is_rounded(&self) -> bool {
        match self {
            Self::Rounded => true,
            _ => false,
        }
    }

    pub fn from_jsobject(val: JsObject) -> Option<Self> {
        let type_ = val
            .get("type")
            .and_then(|t| t.as_string())
            .unwrap_or(String::new());

        if type_ == "Line" {
            if let Some(w) = val.get("payload").and_then(|p| p.as_f64()) {
                Some(Self::Line(w))
            } else {
                None
            }
        } else if type_ == "Rounded" {
            Some(Self::Rounded)
        } else {
            None
        }
    }
}

impl Area {
    pub fn new(
        origin: [f32; 3],
        vector: [f32; 3],
        color_1: Color,
        color_2: Color,
        type_: Type,
    ) -> Self {
        Self {
            origin: origin,
            vector: vector,
            color_1: color_1,
            color_2: color_2,
            type_: type_,
        }
    }

    pub fn vec(&self) -> &[f32; 3] {
        &self.vector
    }

    pub fn set_vec(&mut self, vec: [f32; 3]) {
        self.vector = vec;
    }

    pub fn org(&self) -> &[f32; 3] {
        &self.origin
    }

    pub fn set_org(&mut self, org: [f32; 3]) {
        self.origin = org;
    }

    pub fn color_1(&self) -> &Color {
        &self.color_1
    }

    pub fn set_color_1(&mut self, color: Color) {
        self.color_1 = color;
    }

    pub fn color_2(&self) -> &Color {
        &self.color_2
    }

    pub fn set_color_2(&mut self, color: Color) {
        self.color_2 = color;
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn set_type(&mut self, type_: Type) {
        self.type_ = type_;
    }
}

impl Block for Area {
    fn pack(&self) -> Promise<JsValue> {
        let origin = array![self.origin[0], self.origin[1], self.origin[2]];
        let vector = array![self.vector[0], self.vector[1], self.vector[2]];
        let color_1 = self.color_1.to_u32();
        let color_2 = self.color_2.to_u32();

        let type_ = match &self.type_ {
            Type::Line(w) => object! {
                type: "Line",
                payload: *w
            },
            Type::Rounded => object! {
                type: "Rounded"
            },
        };

        let data = object! {
            origin: origin,
            vector: vector,
            color_1: color_1,
            color_2: color_2,
            type: type_
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(_: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        Promise::new(|resolve| {
            let val = val.dyn_into::<JsObject>().unwrap();

            if let (Some(origin), Some(vector), Some(color_1), Some(color_2), Some(type_)) = (
                val.get("origin").map(|s| {
                    let s: js_sys::Object = s.into();
                    js_sys::Array::from(s.as_ref())
                }),
                val.get("vector").map(|p| {
                    let p: js_sys::Object = p.into();
                    js_sys::Array::from(p.as_ref())
                }),
                val.get("color_1")
                    .and_then(|c| c.as_f64().map(|c| c as u32)),
                val.get("color_2")
                    .and_then(|c| c.as_f64().map(|c| c as u32)),
                val.get("type").and_then(|t| Type::from_jsobject(t)),
            ) {
                let origin = if let (Some(x), Some(y), Some(z)) = (
                    origin.get(0).as_f64().map(|x| x as f32),
                    origin.get(1).as_f64().map(|x| x as f32),
                    origin.get(2).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y, z])
                } else {
                    None
                };

                let vector = if let (Some(x), Some(y), Some(z)) = (
                    vector.get(0).as_f64().map(|x| x as f32),
                    vector.get(1).as_f64().map(|x| x as f32),
                    vector.get(2).as_f64().map(|x| x as f32),
                ) {
                    Some([x, y, z])
                } else {
                    None
                };

                if let (Some(origin), Some(vector)) = (origin, vector) {
                    let tablemask = Self {
                        origin: origin,
                        vector: vector,
                        color_1: Color::from(color_1),
                        color_2: Color::from(color_2),
                        type_: type_,
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
}
