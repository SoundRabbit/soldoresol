use kagura::component::Cmd;
use kagura::prelude::*;
use std::collections::VecDeque;

pub struct Cmds<C: Component> {
    cmds: VecDeque<Cmd<C>>,
}

impl<C: Component> Cmds<C> {
    pub fn new() -> Self {
        Self {
            cmds: VecDeque::new(),
        }
    }

    pub fn pop(&mut self) -> Cmd<C> {
        if let Some(cmd) = self.cmds.pop_front() {
            cmd
        } else {
            Cmd::none()
        }
    }

    pub fn push(&mut self, cmd: Cmd<C>) {
        self.cmds.push_back(cmd);
    }

    pub fn push_msg(&mut self, msg: C::Msg) {
        self.cmds.push_back(Cmd::task(move |resolve| resolve(msg)));
    }
}
