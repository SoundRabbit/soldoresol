use kagura::prelude::*;
use std::collections::VecDeque;

pub struct CmdQueue<M, S> {
    payload: VecDeque<Cmd<M, S>>,
}

impl<M, S> CmdQueue<M, S> {
    pub fn new() -> Self {
        Self {
            payload: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, cmd: Cmd<M, S>) {
        self.payload.push_back(cmd);
    }

    pub fn dequeue(&mut self) -> Cmd<M, S> {
        self.payload.pop_front().unwrap_or(Cmd::none())
    }
}
