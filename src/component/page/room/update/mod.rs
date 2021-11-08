use super::*;
use crate::arena::{block, user};
use kagura::component::Cmd;

mod task;

impl Update for Room {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        let mut chat = block::Chat::new();

        let mut chat_channel = block::ChatChannel::new();

        chat_channel.name_set(String::from("メイン"));
        for i in 0..50 {
            let sender = block::chat_message::Sender::new(
                Rc::clone(&props.client_id),
                None,
                String::from("system"),
            );
            let message = block::chat_message::EvalutedMessage::new(
                &String::from(""),
                |refer| refer,
                |cmd, msg| {
                    block::chat_message::EvalutedMessage::from(vec![
                        block::chat_message::EvalutedMessageToken::CommandBlock(cmd, msg),
                    ])
                },
            );
            let chat_message = block::ChatMessage::new(sender, chrono::Utc::now(), message);

            chat_channel.messages_push(self.arena.insert(chat_message));
        }

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
