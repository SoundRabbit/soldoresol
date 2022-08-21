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
    blocks: HashMap<[i32;3], TerranVoxel> = HashMap::new();
    texture: BlockMut<TerranTexture> = BlockMut::<TerranTexture>::none();
}

block! {
    [pub TerranVoxel(constructor, pack)]
    (tex_idx): u32;
}

impl Terran {
    pub fn blocks(&self) -> &HashMap<[i32; 3], TerranVoxel> {
        &self.blocks
    }
    pub fn insert_block(&mut self, position: [i32; 3], block: TerranVoxel) {
        self.blocks.insert(position, block);
    }
    pub fn remove_block(&mut self, position: &[i32; 3]) -> Option<TerranVoxel> {
        self.blocks.remove(position)
    }
    pub fn texture(&self) -> &BlockMut<TerranTexture> {
        &self.texture
    }
    pub fn set_texture(&mut self, texture: BlockMut<TerranTexture>) {
        self.texture = texture;
    }
    pub fn check_adjacent(&self, p: &[i32; 3]) -> [bool; 6] {
        [
            self.check_filled(&[p[0] + 1, p[1], p[2]]),
            self.check_filled(&[p[0], p[1] + 1, p[2]]),
            self.check_filled(&[p[0], p[1], p[2] + 1]),
            self.check_filled(&[p[0] - 1, p[1], p[2]]),
            self.check_filled(&[p[0], p[1] - 1, p[2]]),
            self.check_filled(&[p[0], p[1], p[2] - 1]),
        ]
    }
    pub fn check_filled(&self, p: &[i32; 3]) -> bool {
        self.blocks.contains_key(p)
    }
}

impl TerranVoxel {
    pub fn tex_idx(&self) -> u32 {
        self.tex_idx
    }
}
