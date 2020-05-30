use super::{Color, ColorSystem};
use crate::JsObject;

pub struct Character {
    size: [f64; 2],
    position: [f64; 3],
    image_id: Option<u128>,
    background_color: Color,
    hp: i32,
    mp: i32,
}

pub struct CharacterData {
    pub size: [f64; 2],
    pub position: [f64; 3],
    pub image_id: Option<u128>,
    pub hp: i32,
    pub mp: i32,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: [1.0, 1.0],
            position: [0.0, 0.0, 0.0],
            image_id: None,
            background_color: Color::from(0),
            hp: 0,
            mp: 0,
        }
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        self.size = size;
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_hp(&mut self, hp: i32) {
        self.hp = hp;
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn set_mp(&mut self, mp: i32) {
        self.mp = mp;
    }

    pub fn mp(&self) -> i32 {
        self.mp
    }

    pub fn bind_to_grid(&mut self) {
        let p = self.position;
        let p = [(p[0] * 2.0).round() / 2.0, (p[1] * 2.0).round() / 2.0];
        self.position = [p[0], p[1], self.position[2]];
    }

    pub fn texture_id(&self) -> Option<u128> {
        if let Some(texture) = self.image_id {
            Some(texture)
        } else {
            None
        }
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn set_is_focused(&mut self, is_focused: bool) {
        if is_focused {
            self.background_color = ColorSystem::gray_900(127);
        } else {
            self.background_color = Color::from(0);
        }
    }

    pub fn set_image_id(&mut self, data_id: u128) {
        self.image_id = Some(data_id);
    }

    pub fn rendered(&mut self) {
        self.set_is_focused(false);
    }

    pub fn to_data(&self) -> CharacterData {
        CharacterData {
            size: self.size.clone(),
            position: self.position.clone(),
            image_id: self.texture_id(),
            hp: self.hp,
            mp: self.mp,
        }
    }
}

impl Clone for Character {
    fn clone(&self) -> Self {
        let mut clone = Self::new();

        clone.set_size(self.size.clone());
        clone.set_position(self.position.clone());
        if let Some(image_id) = self.image_id {
            clone.set_image_id(image_id);
        }

        clone
    }
}

impl From<CharacterData> for Character {
    fn from(character_data: CharacterData) -> Self {
        Self {
            size: character_data.size,
            position: character_data.position,
            image_id: character_data.image_id,
            background_color: Color::from(0),
            hp: character_data.hp,
            mp: character_data.mp,
        }
    }
}

impl CharacterData {
    pub fn as_object(&self) -> JsObject {
        let image_id = self.image_id.map(|id| id.to_string());
        let hp: i32 = self.hp;
        let mp: i32 = self.mp;

        object! {
            size: array![self.size[0], self.size[1]],
            position: array![self.position[0], self.position[1], self.position[2]],
            image_id: image_id,
            hp: hp,
            mp: mp
        }
    }
}

impl From<JsObject> for CharacterData {
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

        let image_id = obj
            .get("image_id")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .ok();

        let hp = obj.get("hp").unwrap().as_f64().unwrap() as i32;
        let mp = obj.get("mp").unwrap().as_f64().unwrap() as i32;

        Self {
            size,
            position,
            image_id,
            hp,
            mp,
        }
    }
}
