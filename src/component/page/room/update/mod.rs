use super::*;
use crate::arena::{block, user};
use kagura::component::Cmd;

mod task;

impl Update for Room {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        let mut chat = block::Chat::new();

        let mut chat_channel = block::ChatChannel::new();

        chat_channel.name_set(String::from("メイン"));

        let chat_channel = self.arena.insert(chat_channel);
        chat.channels_push(chat_channel.clone());
        self.arena.insert(chat);

        self.modeless_container.update(|this| {
            this.open_modeless(vec![room_modeless::Content {
                arena: ArenaMut::clone(&self.arena),
                client_id: Rc::clone(&props.client_id),
                data: room_modeless::ContentData::ChatChannel(chat_channel),
            }]);
        });

        Cmd::chain(Msg::NoOp)
    }
}
