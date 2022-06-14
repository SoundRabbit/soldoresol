use crate::arena::{resource, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::three;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Texture {
    Image(Rc<three::Texture>),
    Block(Rc<three::Texture>),
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

        let texture = image.map(|image| {
            let texture = Rc::new(three::Texture::new_with_image(image.element()));
            texture.set_needs_update(true);
            texture
        });
        texture.map(|texture| {
            self.data
                .insert(image_id, Texture::Image(Rc::clone(&texture)));
            texture
        })
    }

    pub fn load_block(
        &mut self,
        block_texture: BlockRef<resource::BlockTexture>,
    ) -> Option<Rc<three::Texture>> {
        let texture_id = block_texture.id();
        if let Some(Texture::Image(texture)) = self.data.get(&texture_id) {
            return Some(Rc::clone(texture));
        }

        let texture = block_texture.map(|block_texture| {
            let texture = Rc::new(three::Texture::new_with_image(
                block_texture.data().element(),
            ));
            texture.set_wrap_s(three::RepeatWrapping);
            texture.set_needs_update(true);
            texture
        });
        texture.map(|texture| {
            self.data
                .insert(texture_id, Texture::Block(Rc::clone(&texture)));
            texture
        })
    }
}
