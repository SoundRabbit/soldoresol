use super::Color;
use super::ColorSystem;
use serde::{Deserialize, Serialize};

pub struct Character {
    size: [f64; 2],
    position: [f64; 3],
    image_id: Option<u128>,
    background_color: Color,
}

#[derive(Deserialize, Serialize)]
pub struct CharacterData {
    pub size: [f64; 2],
    pub position: [f64; 3],
    pub image_id: Option<u128>,
}

impl Character {
    pub fn new() -> Self {
        Self {
            size: [1.0, 0.0],
            position: [0.0, 0.0, 0.0],
            image_id: None,
            background_color: Color::from(0),
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
