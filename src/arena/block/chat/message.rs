use super::super::BlockId;
use std::rc::Rc;

#[derive(Clone)]
pub enum Sender {
    Player { client_id: Rc<String> },
}

#[derive(Clone)]
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

    pub fn set_text(&mut self, text: String) {
        self.text = Rc::new(text);
    }

    pub fn text(&self) -> Rc<String> {
        Rc::clone(&self.text)
    }
}
