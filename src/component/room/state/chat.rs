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
    take: usize,
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
        let chat = block::Chat::new(vec![]);
        let block_id = block_field.add(chat);
        Self {
            selecting_tab_idx: 0,
            selecting_sender_idx: 0,
            inputing_message: "".into(),
            block_id,
            take: 64,
            senders: vec![Sender::Player],
        }
    }

    pub fn set_inputing_message(&mut self, msg: String) {
        self.inputing_message = msg;
    }
}
