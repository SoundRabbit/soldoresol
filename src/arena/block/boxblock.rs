use super::block_trait::DisplayNamed;
use crate::libs::color::Pallet;

#[derive(Clone)]
pub enum Shape {
    Cube,
    Sphere,
    Cyliner,
}

pub struct Boxblock {
    position: [f32; 3],
    size: [f32; 3],
    color: Pallet,
    is_fixed: bool,
    name: String,
    display_name: String,
    shape: Shape,
}

impl Shape {
    pub fn as_num(&self) -> i32 {
        match self {
            Self::Cube => 0,
            Self::Sphere => 1,
            Self::Cyliner => 2,
        }
    }
}

impl Boxblock {
    pub fn new(position: [f32; 3], size: [f32; 3], color: Pallet, shape: Shape) -> Self {
        Self {
            position,
            size,
            color,
            is_fixed: true,
            name: String::new(),
            display_name: String::new(),
            shape,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            size: self.size.clone(),
            color: self.color,
            is_fixed: self.is_fixed,
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            shape: self.shape.clone(),
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl DisplayNamed for Boxblock {
    fn display_name(&self) -> &String {
        self.name()
    }

    fn set_display_name(&mut self, name: String) {
        self.set_name(name)
    }
}
