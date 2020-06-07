use super::{Color, ColorSystem};
use crate::JsObject;
use serde::{Deserialize, Serialize};

pub struct Tablemask {
    size: [f64; 2],
    position: [f64; 3],
    background_color: Color,
    size_is_binded: bool,
    is_rounded: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TablemaskData {
    pub size: [f64; 2],
    pub position: [f64; 3],
    pub background_color: u32,
    pub size_is_binded: bool,
    pub is_rounded: bool,
}

impl Tablemask {
    pub fn new() -> Self {
        Self {
            size: [8.0, 8.0],
            position: [0.0, 0.0, 0.0],
            background_color: ColorSystem::red_500(127),
            size_is_binded: true,
            is_rounded: true,
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

    pub fn bind_to_grid(&mut self) {
        let p = self.position;
        let p = [(p[0] * 2.0).round() / 2.0, (p[1] * 2.0).round() / 2.0];
        self.position = [p[0], p[1], self.position[2]];
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn to_data(&self) -> TablemaskData {
        TablemaskData {
            size: self.size.clone(),
            position: self.position.clone(),
            background_color: self.background_color.to_u32(),
            size_is_binded: self.size_is_binded,
            is_rounded: self.is_rounded,
        }
    }
}

impl Clone for Tablemask {
    fn clone(&self) -> Self {
        let mut clone = Self::new();

        clone.set_size(self.size.clone());
        clone.set_position(self.position.clone());
        clone
    }
}

impl From<TablemaskData> for Tablemask {
    fn from(tablemask_data: TablemaskData) -> Self {
        Self {
            size: tablemask_data.size,
            position: tablemask_data.position,
            background_color: Color::from(tablemask_data.background_color),
            size_is_binded: tablemask_data.size_is_binded,
            is_rounded: tablemask_data.is_rounded,
        }
    }
}

impl TablemaskData {
    pub fn as_object(&self) -> JsObject {
        let background_color: u32 = self.background_color;
        let size_is_binded: bool = self.size_is_binded;
        let is_rounded: bool = self.is_rounded;

        object! {
            size: array![self.size[0], self.size[1]],
            position: array![self.position[0], self.position[1], self.position[2]],
            background_color: background_color,
            size_is_binded: size_is_binded,
            is_rounded: is_rounded
        }
    }
}

impl From<JsObject> for TablemaskData {
    fn from(obj: JsObject) -> Self {
        use js_sys::Array;
        use wasm_bindgen::JsCast;

        let size = obj.get("size").unwrap().dyn_into::<Array>().ok().unwrap();
        let size = [size.get(0).as_f64().unwrap(), size.get(1).as_f64().unwrap()];

        let position = obj
            .get("position")
            .unwrap()
            .dyn_into::<Array>()
            .ok()
            .unwrap();
        let position = [
            position.get(0).as_f64().unwrap(),
            position.get(1).as_f64().unwrap(),
            position.get(2).as_f64().unwrap(),
        ];

        let background_color = obj
            .get("background_color")
            .and_then(|x| x.as_f64().map(|x| x as u32))
            .unwrap_or(0xFF000000);
        let size_is_binded = obj
            .get("size_is_binded")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        let is_rounded = obj
            .get("is_rounded")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);

        Self {
            size,
            position,
            background_color,
            size_is_binded,
            is_rounded,
        }
    }
}
