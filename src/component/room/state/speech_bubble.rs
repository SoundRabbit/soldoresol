use crate::block::BlockId;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

pub struct Item {
    texture_id: Option<BlockId>,
    message: String,
    position: [f64; 2],
}

pub struct State(VecDeque<Item>);

impl State {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn enqueue(&mut self, item: Item) {
        self.0.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<Item> {
        self.0.pop_front()
    }
}

impl Deref for State {
    type Target = VecDeque<Item>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
