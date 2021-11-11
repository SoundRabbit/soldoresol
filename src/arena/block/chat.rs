uses! {
    super::BlockMut;
    super::util::Pack;
}

block! {
    [pub Chat]
    channels: Vec<BlockMut> = vec![];
}

impl Chat {
    pub fn channels(&self) -> &Vec<BlockMut> {
        &self.channels
    }

    pub fn channels_push(&mut self, channel: BlockMut) {
        self.channels.push(channel);
    }
}
