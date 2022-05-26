use super::super::resource::BlockTexture;
use super::super::resource::ImageData;
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Pack;
use super::BlockMut;
use super::BlockRef;
use super::Character;
use super::Scene;

block! {
    [pub World(constructor, pack)]
    characters: Vec<BlockMut<Character>> = vec![];
    scenes: Vec<BlockMut<Scene>> = vec![];
    selecting_scene: BlockMut<Scene> = BlockMut::<Scene>::none();
    image_data_resources: Vec<BlockRef<ImageData>> = vec![];
    block_texture_resources: Vec<BlockRef<BlockTexture>> = vec![];
}

impl World {
    pub fn characters(&self) -> &Vec<BlockMut<Character>> {
        &self.characters
    }

    pub fn push_character(&mut self, character: BlockMut<Character>) {
        self.characters.push(character);
    }

    pub fn remove_character(&mut self, block_id: &U128Id) {
        if let Some(character_idx) = self
            .characters
            .iter()
            .position(|character| character.id() == *block_id)
        {
            self.characters.remove(character_idx);
        }
    }

    pub fn scenes(&self) -> &Vec<BlockMut<Scene>> {
        &self.scenes
    }

    pub fn selecting_scene(&self) -> &BlockMut<Scene> {
        &self.selecting_scene
    }

    pub fn push_scenes(&mut self, scene: BlockMut<Scene>) {
        if self.scenes.len() == 0 {
            self.selecting_scene = BlockMut::clone(&scene);
        }
        self.scenes.push(scene);
    }

    pub fn image_data_resources(&self) -> &Vec<BlockRef<ImageData>> {
        &self.image_data_resources
    }

    pub fn push_image_data_resource(&mut self, image_data: BlockRef<ImageData>) {
        self.image_data_resources.push(image_data);
    }

    pub fn block_texture_resources(&self) -> &Vec<BlockRef<BlockTexture>> {
        &self.block_texture_resources
    }

    pub fn push_block_texture_resource(&mut self, block_texture: BlockRef<BlockTexture>) {
        self.block_texture_resources.push(block_texture);
    }
}
