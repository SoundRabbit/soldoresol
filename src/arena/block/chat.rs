uses! {
    super::BlockMut;
    super::util::Pack;
    super::ChatChannel;
}

packable! {
    [pub Chat]
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
