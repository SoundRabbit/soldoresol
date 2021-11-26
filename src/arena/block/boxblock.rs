uses! {}

use super::super::resource::BlockTexture;
use super::util::Pack;
use super::BlockMut;
use crate::libs::color::Pallet;

#[derive(Clone, Copy)]
pub enum Shape {
    Cube,
    Cyliner,
    Sphere,
}

#[async_trait(?Send)]
impl Pack for Shape {
    async fn pack(&self, _: bool) -> JsValue {
        match self {
            Self::Cube => JsValue::from("Cube"),
            Self::Cyliner => JsValue::from("Cyliner"),
            Self::Sphere => JsValue::from("Sphere"),
        }
    }
}

block! {
    [pub Boxblock(constructor, pack)]
    size: [f64; 3] = [1.0, 1.0, 1.0];
    position: [f64; 3] = [0.0, 0.0, 0.0];
    shape: Shape = Shape::Cube;
    color: Pallet = Pallet::blue(5);
    texture: Option<BlockMut<BlockTexture>> = None;
}

impl Boxblock {
    pub fn size(&self) -> &[f64; 3] {
        &self.size
    }

    pub fn set_size(&mut self, size: [f64; 3]) {
        self.size = size;
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn color(&self) -> &Pallet {
        &self.color
    }

    pub fn set_color(&mut self, color: Pallet) {
        self.color = color;
    }

    pub fn texture(&self) -> Option<&BlockMut<BlockTexture>> {
        self.texture.as_ref()
    }

    pub fn set_texture(&mut self, texture: Option<BlockMut<BlockTexture>>) {
        self.texture = texture;
    }
}
