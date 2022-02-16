#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::Pack;
use super::BlockMut;
use super::ChatChannel;

block! {
    [pub Chat(constructor, pack)]
    channels: Vec<BlockMut<ChatChannel>> = vec![];
}

impl Chat {
    pub fn channels(&self) -> &Vec<BlockMut<ChatChannel>> {
        &self.channels
    }

    pub fn channels_push(&mut self, channel: BlockMut<ChatChannel>) {
        self.channels.push(channel);
    }
}
