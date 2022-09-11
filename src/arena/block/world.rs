use super::super::component::{BoxblockComponent, CraftboardComponent, TextboardComponent};
use super::super::resource::{BlockTexture, ImageData};
use super::super::Access;
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
use super::{BlockMut, BlockRef};
use super::{Character, Scene, TerranTexture};
use crate::libs::random_id::U128Id;
use std::collections::HashSet;

block! {
    [pub Components(constructor, pack)]
    boxblocks: Vec<BlockMut<BoxblockComponent>> = vec![];
    craftboards: Vec<BlockMut<CraftboardComponent>> = vec![];
    textboards: Vec<BlockMut<TextboardComponent>> = vec![];
}

impl Components {
    pub fn boxblocks(&self) -> &Vec<BlockMut<BoxblockComponent>> {
        &self.boxblocks
    }

    pub fn craftboards(&self) -> &Vec<BlockMut<CraftboardComponent>> {
        &self.craftboards
    }

    pub fn textboards(&self) -> &Vec<BlockMut<TextboardComponent>> {
        &self.textboards
    }
}

block! {
    [pub World(constructor, pack)]
    characters: Vec<BlockMut<Character>> = vec![];
    scenes: Vec<BlockMut<Scene>> = vec![];
    selecting_scene: BlockMut<Scene> = BlockMut::<Scene>::none();
    components: Components = Components::new();
    image_data_resources: Vec<BlockRef<ImageData>> = vec![];
    block_texture_resources: Vec<BlockRef<BlockTexture>> = vec![];
    terran_texture_blocks: Vec<BlockMut<TerranTexture>> = vec![];
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

    pub fn components(&self) -> &Components {
        &self.components
    }

    pub fn push_boxblock_as_component(&mut self, component: BlockMut<BoxblockComponent>) {
        self.components.boxblocks.push(component);
    }

    pub fn push_craftboard_as_component(&mut self, component: BlockMut<CraftboardComponent>) {
        self.components.craftboards.push(component);
    }

    pub fn push_textboard_as_component(&mut self, component: BlockMut<TextboardComponent>) {
        self.components.textboards.push(component);
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

    pub fn terran_texture_blocks(&self) -> &Vec<BlockMut<TerranTexture>> {
        &self.terran_texture_blocks
    }

    pub fn push_terran_texture_block(&mut self, block_texture: BlockMut<TerranTexture>) {
        self.terran_texture_blocks.push(block_texture);
    }
}
