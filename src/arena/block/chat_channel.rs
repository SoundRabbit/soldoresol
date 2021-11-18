uses! {
    super::BlockMut;
    super::util::Pack;
    super::ChatMessage;
}

packable! {
    [pub ChatChannel]
    messages: Vec<BlockMut<ChatMessage>> = vec![];
    name: String = String::from("タブ");
}

impl ChatChannel {
    pub fn messages_push(&mut self, message: BlockMut<ChatMessage>) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<BlockMut<ChatMessage>> {
        &self.messages
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_set(&mut self, name: String) {
        self.name = name;
    }
}
