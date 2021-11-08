uses! {
    super::BlockMut;
    super::util::Pack;
}

block! {
    [pub Chat]
    channels: Vec<BlockMut> = vec![];
}

impl Chat {
    pub fn channels_push(&mut self, channel: BlockMut) {
        self.channels.push(channel);
    }
}
