use super::*;
use crate::arena::{block, user, BlockKind};
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
                &self.world,
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
            Msg::UpdateBlocks { insert, update } => {
                let need_rendering =
                    insert
                        .iter()
                        .chain(update.iter())
                        .any(|b_id| match self.arena.kind_of(b_id) {
                            BlockKind::Boxblock
                            | BlockKind::CanvasTexture
                            | BlockKind::Character
                            | BlockKind::Craftboard
                            | BlockKind::LayerGroup
                            | BlockKind::Scene
                            | BlockKind::Table => true,
                            _ => false,
                        });

                if need_rendering {
                    self.table.update(|table| {
                        table.need_rendering();
                    });
                }

                Cmd::none()
            }
            Msg::OpenBoxblockModeless(boxblock_id) => {
                if let Some(boxblock) = self.arena.get_mut(&boxblock_id) {
                    self.modeless_container.update(|modeless_container| {
                        Self::open_modeless(
                            &props.client_id,
                            &self.arena,
                            &self.world,
                            modeless_container,
                            room_modeless::ContentData::Boxblock(boxblock),
                        );
                    });
                }

                Cmd::none()
            }
            Msg::OpenChatModeless(channel_id) => {
                if let Some(channel_id) = channel_id {
                    if let Some(channel) = self.arena.get_mut(&channel_id) {
                        self.modeless_container.update(|modeless_container| {
                            Self::open_chat_modeless(
                                &props.client_id,
                                &self.arena,
                                &self.world,
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
                                &self.world,
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
            Msg::SetShowingContextmenu(contextmenu) => {
                self.showing_contextmenu = contextmenu;
                Cmd::none()
            }
            Msg::OnTableClicked(e) => {
                if e.target() == e.current_target() {
                    self.table.update(|table| {
                        table.on_click(e, &self.table_tool);
                    });
                }
                Cmd::none()
            }
            Msg::OnTableContextmenu(e) => {
                if e.target() == e.current_target() {
                    e.prevent_default();
                    let (block_kind, block_id) = self.table.map(|table| {
                        let [px_x, px_y] = table.table_coord(&e);
                        table.focused_block(px_x, px_y)
                    });
                    match block_kind {
                        BlockKind::Boxblock => {
                            if let Some(block) = self.arena.get_mut::<block::Boxblock>(&block_id) {
                                self.showing_contextmenu = Some(ShowingContextmenu {
                                    page_x: e.page_x() as f64,
                                    page_y: e.page_y() as f64,
                                    data: ShowingContextmenuData::Boxblock(block),
                                });
                            }
                        }
                        _ => {}
                    }
                }
                Cmd::none()
            }
            Msg::AddResourceImageData(image_data) => {
                let image_data = self.arena.insert(image_data);
                self.world.update(|world| {
                    world.push_image_data_resource(image_data);
                });
                Cmd::none()
            }
            Msg::SetIs2dMode(is_2d_mode) => {
                self.is_2d_mode = is_2d_mode;
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
        world: &BlockMut<block::World>,
        modeless_container: &mut TabModelessContainer<RoomModeless, room_modeless::TabName>,
        channels: &Vec<BlockMut<block::ChatChannel>>,
    ) {
        modeless_container.open_modeless(
            channels
                .iter()
                .map(|channel| room_modeless::Content {
                    arena: ArenaMut::clone(arena),
                    world: BlockMut::clone(world),
                    client_id: Rc::clone(&client_id),
                    data: room_modeless::ContentData::ChatChannel(BlockMut::clone(&channel)),
                })
                .collect(),
        );
    }

    fn open_modeless(
        client_id: &Rc<String>,
        arena: &ArenaMut,
        world: &BlockMut<block::World>,
        modeless_container: &mut TabModelessContainer<RoomModeless, room_modeless::TabName>,
        content: room_modeless::ContentData,
    ) {
        modeless_container.open_modeless(vec![room_modeless::Content {
            arena: ArenaMut::clone(arena),
            world: BlockMut::clone(world),
            client_id: Rc::clone(&client_id),
            data: content,
        }]);
    }
}
