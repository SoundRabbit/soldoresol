use super::Msg;
use super::On;
use super::Room;
use kagura::prelude::*;

impl Room {
    pub fn update(&mut self, msg: Msg) -> Cmd<Msg, On> {
        Cmd::none()
    }
}
