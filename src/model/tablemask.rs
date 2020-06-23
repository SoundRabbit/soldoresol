use super::{Color, ColorSystem};
use crate::JsObject;
use std::ops::Deref;

pub struct Tablemask {
    size: [f64; 2],
    position: [f64; 3],
    z_rotation: f64,
    background_color: Color,
    size_is_binded: bool,
    is_rounded: bool,
    is_fixed: bool,
}

pub struct TablemaskData(JsObject);

impl Tablemask {
    pub fn new() -> Self {
        Self {
            size: [8.0, 8.0],
            position: [0.0, 0.0, 0.0],
            z_rotation: 0.0,
            background_color: ColorSystem::red((255.0 * 0.6) as u8, 5),
            size_is_binded: true,
            is_rounded: true,
            is_fixed: false,
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }
    pub fn set_size_is_binded(&mut self, is_binded: bool) {
        self.size_is_binded = is_binded;
    }

    pub fn size_is_binded(&self) -> bool {
        self.size_is_binded
    }

    pub fn set_is_rounded(&mut self, is_rounded: bool) {
        self.is_rounded = is_rounded;
    }

    pub fn is_rounded(&self) -> bool {
        self.is_rounded
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_z_rotation(&mut self, z_rotation: f64) {
        self.z_rotation = z_rotation;
    }

    pub fn z_rotation(&self) -> f64 {
        self.z_rotation
    }

    pub fn bind_to_grid(&mut self) {
        let p = self.position;
        let p = [(p[0] * 2.0).round() / 2.0, (p[1] * 2.0).round() / 2.0];
        self.position = [p[0], p[1], self.position[2]];
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn as_data(&self) -> TablemaskData {
        TablemaskData(object! {
            size: array![self.size[0], self.size[1]],
            position: array![self.position[0],self.position[1],self.position[2]],
            z_rotation: self.z_rotation,
            background_color: self.background_color.to_u32(),
            size_is_binded: self.size_is_binded,
            is_rounded: self.is_rounded,
            is_fixed: self.is_fixed
        })
    }
}

impl Clone for Tablemask {
    fn clone(&self) -> Self {
        let mut clone = Self::new();

        clone.set_size(self.size.clone());
        clone.set_position(self.position.clone());
        clone.set_z_rotation(self.z_rotation);
        clone.set_background_color(Color::from(self.background_color().to_u32()));
        clone.set_is_rounded(self.is_rounded());
        clone.set_is_fixed(self.is_fixed());

        clone
    }
}

impl Into<Tablemask> for TablemaskData {
    fn into(self) -> Tablemask {
        use js_sys::Array;

        let obj = self.0;

        let size = Array::from(&obj.get("size").unwrap());
        let size = [size.get(0).as_f64().unwrap(), size.get(1).as_f64().unwrap()];

        let position = Array::from(&obj.get("position").unwrap());
        let position = [
            position.get(0).as_f64().unwrap(),
            position.get(1).as_f64().unwrap(),
            position.get(2).as_f64().unwrap(),
        ];

        let background_color = obj
            .get("background_color")
            .and_then(|x| x.as_f64().map(|x| x as u32))
            .unwrap_or(0xFF000000);
        let background_color = Color::from(background_color);
        let size_is_binded = obj
            .get("size_is_binded")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        let is_rounded = obj
            .get("is_rounded")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        let z_rotation = obj
            .get("z_rotation")
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0);
        let is_fixed = obj
            .get("is_fixed")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);

        Tablemask {
            size,
            position,
            z_rotation,
            background_color,
            size_is_binded,
            is_rounded,
            is_fixed,
        }
    }
}

impl Into<JsObject> for TablemaskData {
    fn into(self) -> JsObject {
        self.0
    }
}

impl From<JsObject> for TablemaskData {
    fn from(obj: JsObject) -> Self {
        Self(obj)
    }
}

impl Deref for TablemaskData {
    type Target = JsObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
