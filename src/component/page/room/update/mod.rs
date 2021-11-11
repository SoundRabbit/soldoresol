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
        self.chat = self.arena.insert(chat);

        self.modeless_container.update(|this| {
            this.open_modeless(vec![room_modeless::Content {
                arena: ArenaMut::clone(&self.arena),
                client_id: Rc::clone(&props.client_id),
                data: room_modeless::ContentData::ChatChannel(chat_channel),
            }]);
        });

        Cmd::chain(Msg::NoOp)
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::OpenChatModeless(channel_id) => {
                if let Some(channel_id) = channel_id {
                    if let Some(channel) = self.arena.get_mut(&channel_id) {
                        self.modeless_container.update(|this| {
                            Self::open_chat_modeless(
                                &props.client_id,
                                &self.arena,
                                this,
                                &vec![channel],
                            );
                        });
                    }
                } else {
                    self.chat.map(|chat: &block::Chat| {
                        self.modeless_container.update(|this| {
                            Self::open_chat_modeless(
                                &props.client_id,
                                &self.arena,
                                this,
                                chat.channels(),
                            );
                        });
                    });
                }

                Cmd::none()
            }
        }
    }
}

impl Room {
    fn open_chat_modeless(
        client_id: &Rc<String>,
        arena: &ArenaMut,
        modeless_container: &mut TabModelessContainer<RoomModeless, room_modeless::TabName>,
        channels: &Vec<BlockMut>,
    ) {
        modeless_container.open_modeless(
            channels
                .iter()
                .map(|channel| room_modeless::Content {
                    arena: ArenaMut::clone(arena),
                    client_id: Rc::clone(&client_id),
                    data: room_modeless::ContentData::ChatChannel(BlockMut::clone(&channel)),
                })
                .collect(),
        );
    }
}
