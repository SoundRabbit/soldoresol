mod item;

use super::Block;
use super::BlockId;

pub use item::Item;

#[derive(Clone)]
pub struct Memo {
    tags: Vec<String>,
    items: Vec<BlockId>,
}

impl Memo {
    pub fn new() -> Self {
        Self {
            tags: vec![],
            items: vec![],
        }
    }

    pub fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    pub fn add_tag(&mut self, tag_name: String) {
        self.tags.push(tag_name);
    }

    pub fn remove_tag(&mut self, tag_idx: usize) {
        self.tags.remove(tag_idx);
    }

    pub fn set_tag_name(&mut self, tag_idx: usize, tag_name: String) {
        if let Some(tag) = self.tags.get_mut(tag_idx) {
            *tag = tag_name;
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &BlockId> {
        self.items.iter()
    }

    pub fn add_item(&mut self, item: BlockId) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_idx: usize) {
        self.items.remove(item_idx);
    }
}
