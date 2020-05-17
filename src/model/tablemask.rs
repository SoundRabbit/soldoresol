use super::Color;
use super::ColorSystem;

pub struct Tablemask {
    size: [f64; 2],
    position: [f64; 3],
    background_color: Color,
}

impl Tablemask {
    pub fn new() -> Self {
        Self {
            size: [8.0, 8.0],
            position: [0.0, 0.0, 0.0],
            background_color: Color::from([1.0, 0.0, 0.0, 0.5]),
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

    pub fn background_color(&self) -> &Color {
        &self.background_color
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
