#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::resource::BlockTexture;
use super::util::Pack;
use super::BlockMut;
use super::BlockRef;
use crate::libs::color::Pallet;
use crate::libs::random_id::U128Id;
use std::collections::HashSet;

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
    [pub Boxblock(constructor, pack, component)]
    (is_bind_to_grid): bool;
    origin: BlockMut<Component> = BlockMut::<Component>::none();
    size: [f64; 3] = [1.0, 1.0, 1.0];
    position: [f64; 3] = [0.0, 0.0, 0.0];
    shape: Shape = Shape::Cube;
    color: Pallet = Pallet::blue(5);
    texture: Option<BlockRef<BlockTexture>> = None;
    name: String = String::from("ブロック");
    display_name: (String, String) = (String::from(""), String::from(""));
    is_fixed_position: bool = false;
}

impl Boxblock {
    pub fn origin(&self) -> &BlockMut<Component> {
        &self.origin
    }

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

    pub fn texture(&self) -> Option<&BlockRef<BlockTexture>> {
        self.texture.as_ref()
    }

    pub fn set_texture(&mut self, texture: Option<BlockRef<BlockTexture>>) {
        self.texture = texture;
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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
    pub fn set_is_fixed_position(&mut self, is_fixed_position: bool) {
        self.is_fixed_position = is_fixed_position;
    }
    pub fn is_fixed_position(&self) -> bool {
        self.is_fixed_position
    }
    pub fn set_is_bind_to_grid(&mut self, is_bind_to_grid: bool) {
        self.is_bind_to_grid = is_bind_to_grid;
    }
    pub fn is_bind_to_grid(&self) -> bool {
        self.is_bind_to_grid
    }
}

impl BlockMut<Component> {
    pub fn create_clone(&self) -> Option<Boxblock> {
        self.map(|component| {
            let mut cloned = component.origin.clone();
            cloned.origin = BlockMut::clone(self);
            cloned
        })
    }
}

impl Clone for Boxblock {
    fn clone(&self) -> Self {
        Self {
            origin: BlockMut::none(),
            is_bind_to_grid: self.is_bind_to_grid,
            position: self.position.clone(),
            size: self.size.clone(),
            shape: self.shape.clone(),
            color: self.color.clone(),
            texture: self
                .texture
                .as_ref()
                .map(|texture| BlockRef::clone(&texture)),
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            is_fixed_position: self.is_fixed_position,
        }
    }
}
