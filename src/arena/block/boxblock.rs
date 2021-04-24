use crate::libs::color::Pallet;

pub struct Boxblock {
    position: [f32; 3],
    size: [f32; 3],
    color: Pallet,
    is_fixed: bool,
}

impl Boxblock {
    pub fn new(position: [f32; 3], size: [f32; 3], color: Pallet) -> Self {
        Self {
            position,
            size,
            color,
            is_fixed: true,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            size: self.size.clone(),
            color: self.color,
            is_fixed: self.is_fixed,
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

    pub fn color(&self) -> &Pallet {
        &self.color
    }

    pub fn set_color(&mut self, color: Pallet) {
        self.color = color;
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn set_is_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed;
    }
}
