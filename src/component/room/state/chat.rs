use crate::block::{self, BlockId};

#[derive(PartialEq, Eq)]
pub enum Sender {
    Player,
    Character(BlockId),
}

pub struct State {
    selecting_tab_idx: usize,
    selecting_sender_idx: usize,
    inputing_message: String,
    block_id: BlockId,
    take_num: usize,
    senders: Vec<Sender>,
}

impl Sender {
    pub fn as_character(&self) -> Option<&BlockId> {
        match self {
            Self::Character(c_id) => Some(c_id),
            _ => None,
        }
    }
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
            senders: vec![Sender::Player],
        }
    }

    pub fn selecting_tab_idx(&self) -> usize {
        self.selecting_tab_idx
    }

    pub fn selecting_sender_idx(&self) -> usize {
        self.selecting_sender_idx
    }

    pub fn inputing_message(&self) -> &String {
        &self.inputing_message
    }

    pub fn set_inputing_message(&mut self, msg: String) {
        self.inputing_message = msg;
    }

    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }

    pub fn take_num(&self) -> usize {
        self.take_num
    }

    pub fn senders(&self) -> &Vec<Sender> {
        &self.senders
    }
}
