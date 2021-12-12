uses! {}

use super::super::resource::BlockTexture;
use super::util::Pack;
use super::BlockMut;
use crate::libs::color::Pallet;

#[derive(Clone, Copy)]
pub enum Shape {
    Cube,
    Cylinder,
    Sphere,
    Slope,
}

#[async_trait(?Send)]
impl Pack for Shape {
    async fn pack(&self, _: bool) -> JsValue {
        match self {
            Self::Cube => JsValue::from("Cube"),
            Self::Cylinder => JsValue::from("Cylinder"),
            Self::Sphere => JsValue::from("Sphere"),
            Self::Slope => JsValue::from("Slope"),
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
    name: String = String::from("ブロック");
    display_name: (String, String) = (String::from(""), String::from(""));
    is_fixed_position: bool = false;
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

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn display_name(&self) -> &(String, String) {
        &self.display_name
    }

    pub fn set_display_name(&mut self, display_name: (Option<String>, Option<String>)) {
        if let Some(main) = display_name.0 {
            self.display_name.0 = main;
        }
        if let Some(sub) = display_name.1 {
            self.display_name.1 = sub;
        }
    }
}
