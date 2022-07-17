use super::super::resource::ImageData;
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Cubebox;
use super::util::Pack;
use super::BlockMut;
use super::BlockRef;
use super::TerranTexture;
use crate::libs::color::Pallet;
use std::collections::{HashMap, HashSet};

block! {
    [pub Terran(constructor, pack)]
    blocks: HashMap<[i32;3], TerranBlock> = HashMap::new();
    texture: BlockMut<TerranTexture> = BlockMut::<TerranTexture>::none();
}

block! {
    [pub TerranBlock(constructor, pack)]
    (tex_idx): u32;
}

impl Terran {
    pub fn blocks(&self) -> &HashMap<[i32; 3], TerranBlock> {
        &self.blocks
    }
    pub fn insert_block(&mut self, position: [i32; 3], block: TerranBlock) {
        self.blocks.insert(position, block);
    }
    pub fn remove_block(&mut self, position: &[i32; 3]) -> Option<TerranBlock> {
        self.blocks.remove(position)
    }
    pub fn texture(&self) -> &BlockMut<TerranTexture> {
        &self.texture
    }
    pub fn set_texture(&mut self, texture: BlockMut<TerranTexture>) {
        self.texture = texture;
    }
}

impl TerranBlock {
    pub fn tex_idx(&self) -> u32 {
        self.tex_idx
    }
}
