pub mod channel;
pub mod message;

use super::BlockId;

pub struct Chat {
    channels: Vec<BlockId>,
}

impl Chat {
    pub fn new(channels: Vec<BlockId>) -> Self {
        Self { channels: channels }
    }

    pub fn clone(this: &Self) -> Self {
        let channels = this
            .channels
            .iter()
            .map(|b_id| BlockId::clone(b_id))
            .collect::<Vec<_>>();

        Self { channels }
    }

    pub fn channels(&self) -> &Vec<BlockId> {
        &self.channels
    }

    pub fn push_channel(&mut self, block_id: BlockId) {
        self.channels.push(block_id);
    }
}
