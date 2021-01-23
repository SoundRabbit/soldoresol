use super::super::BlockId;
use std::rc::Rc;

pub enum Sender {
    Player { client_id: Rc<String> },
}

impl Sender {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::Player { client_id } => Self::Player {
                client_id: Rc::clone(client_id),
            },
        }
    }
}

pub struct Message {
    sender: Sender,
    text: Rc<String>,
    replies: Vec<BlockId>,
}

impl Message {
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
            text: Rc::new(String::from("")),
            replies: vec![],
        }
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            sender: Sender::clone(&this.sender),
            text: Rc::clone(&this.text),
            replies: this
                .replies
                .iter()
                .map(|b_id| BlockId::clone(b_id))
                .collect(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Rc::new(text);
    }

    pub fn text(&self) -> Rc<String> {
        Rc::clone(&self.text)
    }
}
