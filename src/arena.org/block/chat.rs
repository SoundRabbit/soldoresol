pub mod channel;
pub mod message;

use super::BlockId;

#[derive(Clone)]
pub struct Chat {
    channels: Vec<BlockId>,
}

impl Chat {
    pub fn new(channels: Vec<BlockId>) -> Self {
        Self { channels: channels }
    }

    pub fn channels(&self) -> &Vec<BlockId> {
        &self.channels
    }

    pub fn push_channel(&mut self, block_id: BlockId) {
        self.channels.push(block_id);
    }
}
