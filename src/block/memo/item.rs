use super::BlockId;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Item {
    name: String,
    text: String,
    tags: HashSet<BlockId>,
}

impl Item {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            text: String::new(),
            tags: HashSet::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn tags<'a>(
        &self,
        tag_index: impl Iterator<Item = &'a BlockId>,
    ) -> impl Iterator<Item = BlockId> {
        let mut tags = vec![];
        for tag_id in tag_index {
            if self.has(tag_id) {
                tags.push(tag_id.clone());
            }
        }
        tags.into_iter()
    }

    pub fn has(&self, tag_id: &BlockId) -> bool {
        self.tags.get(tag_id).is_some()
    }

    pub fn add_tag(&mut self, tag_id: BlockId) {
        self.tags.insert(tag_id);
    }

    pub fn remove_tag(&mut self, tag_name: &BlockId) {
        self.tags.remove(tag_name);
    }
}
