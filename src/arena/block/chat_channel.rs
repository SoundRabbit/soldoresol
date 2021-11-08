uses! {
    super::BlockMut;
    super::util::Pack;
}

block! {
    [pub ChatChannel]
    messages: Vec<BlockMut> = vec![];
    name: String = String::from("タブ");
}

impl ChatChannel {
    pub fn messages_push(&mut self, message: BlockMut) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<BlockMut> {
        &self.messages
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_set(&mut self, name: String) {
        self.name = name;
    }
}
