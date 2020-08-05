use std::collections::HashSet;

#[derive(Clone)]
pub struct Item {
    name: String,
    text: String,
    tags: HashSet<String>,
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
        tag_index: impl Iterator<Item = &'a String>,
    ) -> impl Iterator<Item = String> {
        let mut tags = vec![];
        for tag_name in tag_index {
            if self.has(tag_name) {
                tags.push(tag_name.clone());
            }
        }
        tags.into_iter()
    }

    pub fn has(&self, tag_name: &String) -> bool {
        self.tags.get(tag_name).is_some()
    }

    pub fn add_tag(&mut self, tag_name: String) {
        self.tags.insert(tag_name);
    }

    pub fn remove_tag(&mut self, tag_name: &String) {
        self.tags.remove(tag_name);
    }
}
