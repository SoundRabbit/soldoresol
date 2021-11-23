use super::*;
use crate::arena::{block, user};
use kagura::component::Cmd;

mod task;

macro_rules! new_channel {
    ($arena:expr,$chat:expr,$name:expr) => {{
        let arena = &mut ($arena);
        let chat = &mut ($chat);

        let mut chat_channel = block::ChatChannel::new();

        chat_channel.name_set(String::from($name));

        let chat_channel = arena.insert(chat_channel);

        chat.update(|chat: &mut block::Chat| {
            chat.channels_push(chat_channel.clone());
        });

        chat_channel
    }};
}

impl Update for Room {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        self.chat = self.arena.insert(block::Chat::new());

        let chat_channel_main = new_channel!(self.arena, self.chat, "メイン");
        let chat_channel_sub = new_channel!(self.arena, self.chat, "サブ");

        self.modeless_container.update(|modeless_container| {
            Self::open_chat_modeless(
                &props.client_id,
                &self.arena,
                modeless_container,
                &vec![chat_channel_main, chat_channel_sub],
            );
        });

        let craftboard = block::Craftboard::new([0.0, 0.0, 0.0]);

        let mut table = block::Table::new();
        table.craftboards_push(self.arena.insert(craftboard));

        let mut scene = block::Scene::new();
        scene.tables_push(self.arena.insert(table));

        let mut world = block::World::new();
        world.scenes_push(self.arena.insert(scene));
        self.world = self.arena.insert(world);

        Cmd::chain(Msg::NoOp)
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::OpenChatModeless(channel_id) => {
                if let Some(channel_id) = channel_id {
                    if let Some(channel) = self.arena.get_mut(&channel_id) {
                        self.modeless_container.update(|modeless_container| {
                            Self::open_chat_modeless(
                                &props.client_id,
                                &self.arena,
                                modeless_container,
                                &vec![channel],
                            );
                        });
                    }
                } else {
                    self.chat.map(|chat: &block::Chat| {
                        self.modeless_container.update(|modeless_container| {
                            Self::open_chat_modeless(
                                &props.client_id,
                                &self.arena,
                                modeless_container,
                                chat.channels(),
                            );
                        });
                    });
                }

                Cmd::none()
            }
            Msg::SetOkToCatchFile(ok_to_catch_file) => {
                self.ok_to_catch_file = ok_to_catch_file;
                Cmd::none()
            }
            Msg::SetSelectedTableTool(tool) => {
                self.table_tool = tool;
                Cmd::none()
            }
            Msg::OnTableClicked(e) => {
                self.table.update(|table| {
                    table.on_click(e, &self.table_tool);
                });
                Cmd::none()
            }
        }
    }
}

impl Room {
    // fn modeless_content(
    //     &self,
    //     props: &Props,
    //     data: room_modeless::ContentData,
    // ) -> room_modeless::Content {
    //     room_modeless::Content {
    //         arena: ArenaMut::clone(&self.arena),
    //         client_id: Rc::clone(&props.client_id),
    //         data: data,
    //     }
    // }

    fn open_chat_modeless(
        client_id: &Rc<String>,
        arena: &ArenaMut,
        modeless_container: &mut TabModelessContainer<RoomModeless, room_modeless::TabName>,
        channels: &Vec<BlockMut<block::ChatChannel>>,
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
