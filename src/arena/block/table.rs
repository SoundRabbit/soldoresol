#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
use super::BlockMut;
use super::Boxblock;
use super::Craftboard;
use super::Textboard;

block! {
    [pub Table(constructor, pack)]
    name: String = String::new();
    boxblocks: Vec<BlockMut<Boxblock>> = vec![];
    craftboards: Vec<BlockMut<Craftboard>> = vec![];
    textboards: Vec<BlockMut<Textboard>> = vec![];
    default_is_bind_to_grid: bool = true;
}

impl Table {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn boxblocks(&self) -> &Vec<BlockMut<Boxblock>> {
        &self.boxblocks
    }
    pub fn push_boxblock(&mut self, boxblock: BlockMut<Boxblock>) {
        self.boxblocks.push(boxblock);
    }
    pub fn remove_boxblock(&mut self, block_id: &U128Id) {
        if let Some(boxblock_idx) = self
            .boxblocks
            .iter()
            .position(|boxblock| boxblock.id() == *block_id)
        {
            self.boxblocks.remove(boxblock_idx);
        }
    }

    pub fn craftboards(&self) -> &Vec<BlockMut<Craftboard>> {
        &self.craftboards
    }
    pub fn push_craftboard(&mut self, craftboard: BlockMut<Craftboard>) {
        self.craftboards.push(craftboard);
    }
    pub fn remove_craftboard(&mut self, block_id: &U128Id) {
        if let Some(craftboard_idx) = self
            .craftboards
            .iter()
            .position(|craftboard| craftboard.id() == *block_id)
        {
            self.craftboards.remove(craftboard_idx);
        }
    }

    pub fn textboards(&self) -> &Vec<BlockMut<Textboard>> {
        &self.textboards
    }

    pub fn push_textboard(&mut self, textboard: BlockMut<Textboard>) {
        self.textboards.push(textboard);
    }

    pub fn remove_textboard(&mut self, block_id: &U128Id) {
        if let Some(textboard_idx) = self
            .textboards
            .iter()
            .position(|textboard| textboard.id() == *block_id)
        {
            self.textboards.remove(textboard_idx);
        }
    }

    pub fn default_is_bind_to_grid(&self) -> bool {
        self.default_is_bind_to_grid
    }

    pub fn create_child(&self) -> Self {
        Self {
            name: String::new(),
            boxblocks: self
                .boxblocks
                .iter()
                .map(|block| BlockMut::clone(block))
                .collect(),
            craftboards: self
                .craftboards
                .iter()
                .map(|block| BlockMut::clone(block))
                .collect(),
            textboards: self
                .textboards
                .iter()
                .map(|block| BlockMut::clone(block))
                .collect(),
            default_is_bind_to_grid: self.default_is_bind_to_grid,
        }
    }
}
