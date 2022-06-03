use crate::arena::{resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Texture {
    Image(Rc<three::Texture>),
}

pub struct TextureTable {
    data: HashMap<U128Id, Texture>,
}

impl TextureTable {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn load_image(
        &mut self,
        image: BlockRef<resource::ImageData>,
    ) -> Option<Rc<three::Texture>> {
        let image_id = image.id();
        if let Some(Texture::Image(texture)) = self.data.get(&image_id) {
            return Some(Rc::clone(texture));
        }

        let texture = image.map(|image| Rc::new(three::Texture::from_image(image.element())));
        texture.map(|texture| {
            self.data
                .insert(image_id, Texture::Image(Rc::clone(&texture)));
            texture
        })
    }
}
