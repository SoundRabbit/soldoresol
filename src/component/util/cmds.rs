use kagura::prelude::*;
use std::collections::VecDeque;

pub struct Cmds<Msg: 'static, Sub> {
    cmds: VecDeque<Cmd<Msg, Sub>>,
}

impl<Msg, Sub> Cmds<Msg, Sub> {
    pub fn new() -> Self {
        Self {
            cmds: VecDeque::new(),
        }
    }

    pub fn pop(&mut self) -> Cmd<Msg, Sub> {
        if let Some(cmd) = self.cmds.pop_front() {
            cmd
        } else {
            Cmd::none()
        }
    }

    pub fn push(&mut self, cmd: Cmd<Msg, Sub>) {
        self.cmds.push_back(cmd);
    }

    pub fn push_msg(&mut self, msg: Msg) {
        self.cmds.push_back(Cmd::task(move |resolve| resolve(msg)));
    }
}
