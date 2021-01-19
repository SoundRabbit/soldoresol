use super::super::BlockId;

pub struct Channel {
    name: String,
    messages: Vec<BlockId>,
}

impl Channel {
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
