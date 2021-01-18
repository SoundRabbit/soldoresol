pub mod message;
pub mod tab;

use super::BlockId;

pub struct Chat {
    tabs: Vec<BlockId>,
}

impl Chat {
    pub fn new(tabs: Vec<BlockId>) -> Self {
        Self { tabs: tabs }
    }

    pub fn tabs(&self) -> &Vec<BlockId> {
        &self.tabs
    }
}
