use super::Color;
use super::ColorSystem;
use serde::{Deserialize, Serialize};

pub struct Tablemask {
    size: [f64; 2],
    position: [f64; 3],
    background_color: Color,
    size_is_binded: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TablemaskData {
    pub size: [f64; 2],
    pub position: [f64; 3],
    pub background_color: u32,
    pub size_is_binded: bool,
}

impl Tablemask {
    pub fn new() -> Self {
        Self {
            size: [8.0, 8.0],
            position: [0.0, 0.0, 0.0],
            background_color: Color::from([1.0, 0.0, 0.0, 0.5]),
            size_is_binded: true,
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
