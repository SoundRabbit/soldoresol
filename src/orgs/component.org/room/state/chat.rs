use crate::block::{self, chat::item::Sender, BlockId};

pub struct State {
    selecting_tab_idx: usize,
    selecting_sender_idx: usize,
    inputing_message: String,
    block_id: BlockId,
    take_num: usize,
    senders: Vec<Sender>,
}

impl State {
    pub fn new(block_field: &mut block::Field) -> Self {
        let tab = block::chat::Tab::new("メイン");
        let block_id = block_field.add(tab);
        let chat = block::Chat::new(vec![block_id]);
        let block_id = block_field.add(chat);
        Self {
            selecting_tab_idx: 0,
            selecting_sender_idx: 0,
            inputing_message: "".into(),
            block_id,
            take_num: 64,
            senders: vec![Sender::User],
        }
    }

    pub fn selecting_tab_idx(&self) -> usize {
        self.selecting_tab_idx
    }

    pub fn set_selecting_tab_idx(&mut self, idx: usize) {
        self.selecting_tab_idx = idx;
    }

    pub fn selecting_sender_idx(&self) -> usize {
        self.selecting_sender_idx
    }

    pub fn set_selecting_sender_idx(&mut self, idx: usize) {
        if idx < self.senders.len() {
            self.selecting_sender_idx = idx;
        }
    }

    pub fn inputing_message(&self) -> &String {
        &self.inputing_message
    }

    pub fn set_inputing_message(&mut self, msg: String) {
        self.inputing_message = msg;
    }

    pub fn drain_inputing_message(&mut self) -> String {
        self.inputing_message.drain(..).collect()
    }

    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }

    pub fn set_block_id(&mut self, block_id: BlockId) {
        self.block_id = block_id;
    }

    pub fn take_num(&self) -> usize {
        self.take_num
    }

    pub fn senders(&self) -> &Vec<Sender> {
        &self.senders
    }

    pub fn add_sender(&mut self, character_id: BlockId) {
        if self
            .senders
            .iter()
            .position(|sender| match &sender {
                Sender::Character(c_id) => character_id == *c_id,
                _ => false,
            })
            .is_none()
        {
            self.senders.push(Sender::Character(character_id));
        }
    }

    pub fn remove_sender(&mut self, character_id: &BlockId) {
        if let Some(pos) = self.senders.iter().position(|sender| match &sender {
            Sender::Character(c_id) => *character_id == *c_id,
            _ => false,
        }) {
            self.senders.remove(pos);
        }
    }

    pub fn selecting_sender(&self) -> &Sender {
        if let Some(sender) = self.senders.get(self.selecting_sender_idx) {
            sender
        } else {
            &self.senders[self.senders.len() - 1]
        }
    }
}
