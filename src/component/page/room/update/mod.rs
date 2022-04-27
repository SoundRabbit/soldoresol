use super::super::organism::{room_modeless, room_modeless_chat::ChatUser};
use super::{Msg, Room, ShowingContextmenu, ShowingContextmenuData, ShowingModal};
use crate::arena::{block, ArenaMut, BlockKind, BlockMut};
use kagura::prelude::*;
use nusa::prelude::*;

mod task;

impl Update for Room {
    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
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
                    self.table.borrow_mut().reserve_rendering();
                }

                Cmd::none()
            }
            Msg::OpenBoxblockModeless(boxblock_id) => {
                if let Some(boxblock) = self.arena.get_mut(&boxblock_id) {
                    super::open_modeless(
                        &self.client_id,
                        &self.arena,
                        &self.world,
                        &self.modeless_container,
                        room_modeless::ContentData::Boxblock(boxblock),
                    );
                }

                Cmd::none()
            }
            Msg::OpenCharacterModeless(character_id) => {
                if let Some(character) = self.arena.get_mut(&character_id) {
                    super::open_modeless(
                        &self.client_id,
                        &self.arena,
                        &self.world,
                        &self.modeless_container,
                        room_modeless::ContentData::Character(character),
                    );
                }

                Cmd::none()
            }
            Msg::OpenCraftboardModeless(craftboard_id) => {
                if let Some(craftboard) = self.arena.get_mut(&craftboard_id) {
                    super::open_modeless(
                        &self.client_id,
                        &self.arena,
                        &self.world,
                        &self.modeless_container,
                        room_modeless::ContentData::Craftboard(craftboard),
                    );
                }

                Cmd::none()
            }
            Msg::OpenChatModeless(chat_user) => {
                if !self.chat_users.iter().any(|user| *user == chat_user) {
                    self.chat_users.push(ChatUser::clone(&chat_user));
                }

                super::open_modeless(
                    &self.client_id,
                    &self.arena,
                    &self.world,
                    &self.modeless_container,
                    room_modeless::ContentData::Chat {
                        data: BlockMut::clone(&self.chat),
                        user: chat_user,
                    },
                );

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
                self.table.borrow_mut().on_wheel(e, &self.table_tool);
                Cmd::none()
            }
            Msg::OnTableClick(e) => {
                self.table.borrow_mut().on_click(
                    ArenaMut::clone(&self.arena),
                    BlockMut::clone(&self.world),
                    e,
                    &self.table_tool,
                );
                Cmd::none()
            }
            Msg::OnTableMousedown(e) => {
                self.table.borrow_mut().on_mousedown(
                    ArenaMut::clone(&self.arena),
                    BlockMut::clone(&self.world),
                    e,
                    &self.table_tool,
                );
                Cmd::none()
            }
            Msg::OnTableMouseup(e) => {
                self.table.borrow_mut().on_mouseup(e, &self.table_tool);
                Cmd::none()
            }
            Msg::OnTableMousemove(e) => {
                self.table.borrow_mut().on_mousemove(
                    ArenaMut::clone(&self.arena),
                    BlockMut::clone(&self.world),
                    e,
                    &self.table_tool,
                );
                Cmd::none()
            }
            Msg::OnTableContextmenu(e) => {
                e.prevent_default();
                let (block_kind, block_id) = self
                    .table
                    .borrow()
                    .focused_block(e.page_x() as f64, e.page_y() as f64);
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
                        if let Some(block) = self.arena.get_mut::<block::Craftboard>(&block_id) {
                            self.showing_contextmenu = Some(ShowingContextmenu {
                                page_x: e.page_x() as f64,
                                page_y: e.page_y() as f64,
                                data: ShowingContextmenuData::Craftboard(block),
                            });
                        }
                    }
                    _ => {}
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
            Msg::SetBlockIsFixedPosition(block, is_fixed_position) => {
                trys! {
                    block.type_as::<block::Boxblock>().update(|boxblock| {
                        boxblock.set_is_fixed_position(is_fixed_position);
                    });
                    block.type_as::<block::Character>().update(|character| {
                        character.set_is_fixed_position(is_fixed_position);
                    });
                    block.type_as::<block::Craftboard>().update(|craftboard| {
                        craftboard.set_is_fixed_position(is_fixed_position);
                    });
                }
                Cmd::none()
            }
            Msg::SetBlockIsBindToGrid(block, is_bind_to_grid) => {
                trys! {
                    block.type_as::<block::Boxblock>().update(|boxblock| {
                        boxblock.set_is_bind_to_grid(is_bind_to_grid);
                    });
                    block.type_as::<block::Character>().update(|character| {
                        character.set_is_bind_to_grid(is_bind_to_grid);
                    });
                    block.type_as::<block::Craftboard>().update(|craftboard| {
                        craftboard.set_is_bind_to_grid(is_bind_to_grid);
                    });
                }
                Cmd::none()
            }
        }
    }
}
