use super::super::BlockId;

pub struct Tab {
    name: String,
    messages: Vec<BlockId>,
}

impl Tab {
    pub fn new(name: String) -> Self {
        Self {
            name,
            messages: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
