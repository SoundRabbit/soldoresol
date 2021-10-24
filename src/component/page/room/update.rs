use super::*;
use kagura::component::Cmd;

mod task;

impl Update for Room {
    fn on_assemble(&mut self, _: &Props) -> Cmd<Self> {
        crate::debug::log_1("assemble room");

        self.modeless_container.update(|modeless_container| {
            modeless_container.open_modeless(vec![room_modeless::Content::ChatChannel]);
        });

        Cmd::chain(Msg::NoOp)
    }
}
