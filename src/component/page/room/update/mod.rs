use super::*;
use crate::arena::{block, BlockKind};
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

        let craftboard = block::Craftboard::new([0.0, 0.0, 0.0]);

        let mut table = block::Table::new();
        table.push_craftboard(self.arena.insert(craftboard));

        let mut scene = block::Scene::new();
        scene.tables_push(self.arena.insert(table));

        let mut world = block::World::new();
        world.push_scenes(self.arena.insert(scene));
        self.world = self.arena.insert(world);

        let me = user::Player::new();
        self.me = self.arena.insert(me);

        self.chat_users
            .push(ChatUser::Player(BlockMut::clone(&self.me)));

        self.modeless_container.update(|modeless_container| {
            Self::open_modeless(
                &props.client_id,
                &self.arena,
                &self.world,
                modeless_container,
                room_modeless::ContentData::Chat {
                    data: BlockMut::clone(&self.chat),
                    user: ChatUser::Player(BlockMut::clone(&self.me)),
                },
            );
        });

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
            Msg::OpenCharacterModeless(character_id) => {
                if let Some(character) = self.arena.get_mut(&character_id) {
                    self.modeless_container.update(|modeless_container| {
                        Self::open_modeless(
                            &props.client_id,
                            &self.arena,
                            &self.world,
                            modeless_container,
                            room_modeless::ContentData::Character(character),
                        );
                    });
                }

                Cmd::none()
            }
            Msg::OpenCraftboardModeless(craftboard_id) => {
                if let Some(craftboard) = self.arena.get_mut(&craftboard_id) {
                    self.modeless_container.update(|modeless_container| {
                        Self::open_modeless(
                            &props.client_id,
                            &self.arena,
                            &self.world,
                            modeless_container,
                            room_modeless::ContentData::Craftboard(craftboard),
                        );
                    });
                }

                Cmd::none()
            }
            Msg::OpenChatModeless(chat_user) => {
                self.modeless_container.update(|modeless_container| {
                    Self::open_modeless(
                        &props.client_id,
                        &self.arena,
                        &self.world,
                        modeless_container,
                        room_modeless::ContentData::Chat {
                            data: BlockMut::clone(&self.chat),
                            user: chat_user,
                        },
                    );
                });

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
            Msg::SetShowingModal(modal) => {
                self.showing_modal = modal;
                Cmd::none()
            }
            Msg::CloseModalChatUser(selected) => {
                self.chat_users = self.chat_users.drain(0..1).collect();

                for character in selected {
                    self.chat_users.push(ChatUser::Character(character));
                }

                self.showing_modal = ShowingModal::None;

                Cmd::none()
            }
            Msg::OnTableWheel(e) => {
                if e.target() == e.current_target() {
                    self.table.update(|table| {
                        table.on_wheel(e, &self.table_tool);
                    });
                }
                Cmd::none()
            }
            Msg::OnTableClick(e) => {
                if e.target() == e.current_target() {
                    self.table.update(|table| {
                        table.on_click(e, &self.table_tool);
                    });
                }
                Cmd::none()
            }
            Msg::OnTableMousedown(e) => {
                if e.target() == e.current_target() {
                    self.table.update(|table| {
                        table.on_mousedown(e, &self.table_tool);
                    });
                }
                Cmd::none()
            }
            Msg::OnTableMouseup(e) => {
                if e.target() == e.current_target() {
                    self.table.update(|table| {
                        table.on_mouseup(e, &self.table_tool);
                    });
                }
                Cmd::none()
            }
            Msg::OnTableMousemove(e) => {
                if e.target() == e.current_target() {
                    self.table.update(|table| {
                        table.on_mousemove(e, &self.table_tool);
                    });
                }
                Cmd::none()
            }
            Msg::OnTableContextmenu(e) => {
                if e.target() == e.current_target() {
                    e.prevent_default();
                    let (block_kind, block_id) = self
                        .table
                        .map(|table| table.focused_block(e.page_x() as f64, e.page_y() as f64));
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
                        BlockKind::Character => {
                            if let Some(block) = self.arena.get_mut::<block::Character>(&block_id) {
                                self.showing_contextmenu = Some(ShowingContextmenu {
                                    page_x: e.page_x() as f64,
                                    page_y: e.page_y() as f64,
                                    data: ShowingContextmenuData::Character(block),
                                });
                            }
                        }
                        BlockKind::Craftboard => {
                            if let Some(block) = self.arena.get_mut::<block::Craftboard>(&block_id)
                            {
                                self.showing_contextmenu = Some(ShowingContextmenu {
                                    page_x: e.page_x() as f64,
                                    page_y: e.page_y() as f64,
                                    data: ShowingContextmenuData::Craftboard(block),
                                });
                            }
                        }
                        _ => {}
                    }
                }
                Cmd::none()
            }
            Msg::AddResourceImageData(image_data) => {
                let image_data = self.arena.insert(image_data).as_ref();
                self.world.update(|world| {
                    world.push_image_data_resource(image_data);
                });
                Cmd::none()
            }
            Msg::SetIs2dMode(is_2d_mode, is_debug_mode) => {
                self.is_2d_mode = is_2d_mode;
                self.is_debug_mode = is_debug_mode;
                Cmd::none()
            }
        }
    }
}

impl Room {
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
