pub mod channel;
pub mod message;

use super::BlockId;

pub struct Chat {
    tabs: Vec<BlockId>,
}

impl Chat {
    pub fn new(tabs: Vec<BlockId>) -> Self {
        Self { tabs: tabs }
    }

    pub fn channels(&self) -> &Vec<BlockId> {
        &self.tabs
    }
}
