use super::super::organism::{room_modeless, room_modeless_chat::ChatUser};
use super::{Msg, Room, ShowingContextmenu, ShowingContextmenuData, ShowingModal};
use crate::arena::{block, component, ArenaMut, BlockKind, BlockMut};
use crate::table::Table;
use kagura::prelude::*;
use nusa::prelude::*;
use std::rc::Rc;

mod task;

impl Update for Room {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        let bcdice_loader = Rc::clone(&self.bcdice_loader);
        Cmd::task(async move {
            bcdice_loader
                .dynamic_load("DiceBot")
                .await
                .map(|game_system_class| Cmd::chain(Msg::SetGameSystemClass(game_system_class)))
                .unwrap_or(Cmd::none())
        })
    }

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
                            | BlockKind::Textboard
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
                        room_modeless::ContentData::Boxblock(block::boxblock::Block::Block(
                            boxblock,
                        )),
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
                        room_modeless::ContentData::Craftboard(block::craftboard::Block::Block(
                            craftboard,
                        )),
                    );
                }

                Cmd::none()
            }
            Msg::OpenTextboardModeless(textboard_id) => {
                if let Some(textboard) = self.arena.get_mut(&textboard_id) {
                    super::open_modeless(
                        &self.client_id,
                        &self.arena,
                        &self.world,
                        &self.modeless_container,
                        room_modeless::ContentData::Textboard(block::textboard::Block::Block(
                            textboard,
                        )),
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
                        game_system_class: Rc::clone(&self.game_system_class),
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
                let (block_kind, block_id) = self.table.borrow().focused_block(
                    e.page_x() as f64,
                    e.page_y() as f64,
                    self.arena.as_ref(),
                );
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
                    BlockKind::Textboard => {
                        if let Some(block) = self.arena.get_mut::<block::Textboard>(&block_id) {
                            self.showing_contextmenu = Some(ShowingContextmenu {
                                page_x: e.page_x() as f64,
                                page_y: e.page_y() as f64,
                                data: ShowingContextmenuData::Textboard(block),
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
                self.table.borrow_mut().reserve_rendering();
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
            Msg::SetGameSystemClass(game_system_class) => {
                *self.game_system_class.borrow_mut() = Some(game_system_class);
                Cmd::none()
            }

            Msg::RemoveCharacter(charcater_id) => {
                self.world.update(|world| {
                    world.remove_character(&charcater_id);
                });

                Cmd::chain(Msg::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.world.id(), charcater_id },
                })
            }

            Msg::RemoveBoxblock(boxblock_id) => {
                let scene = unwrap!(self.world.map(|world| BlockMut::clone(world.selecting_scene())); Cmd::none());
                let table = unwrap!(scene.map(|secene| BlockMut::clone(secene.selecting_table())); Cmd::none());
                let mut updated_blocks = Table::update_table(scene.as_ref(), table, |table| {
                    table.remove_boxblock(&boxblock_id);
                });
                updated_blocks.insert(boxblock_id);

                Cmd::chain(Msg::UpdateBlocks {
                    insert: set! {},
                    update: updated_blocks,
                })
            }

            Msg::RemoveCraftboard(craftboard_id) => {
                let scene = unwrap!(self.world.map(|world| BlockMut::clone(world.selecting_scene())); Cmd::none());
                let mut table = unwrap!(scene.map(|secene| BlockMut::clone(secene.selecting_table())); Cmd::none());
                table.update(|table| {
                    table.remove_craftboard(&craftboard_id);
                });

                Cmd::chain(Msg::UpdateBlocks {
                    insert: set! {},
                    update: set! { table.id(), craftboard_id },
                })
            }

            Msg::RemoveTextboard(textboard_id) => {
                let mut scene = unwrap!(self.world.map(|world| BlockMut::clone(world.selecting_scene())); Cmd::none());
                scene.update(|scene| {
                    scene.textboards_remove(&textboard_id);
                });

                Cmd::chain(Msg::UpdateBlocks {
                    insert: set! {},
                    update: set! { scene.id(), textboard_id },
                })
            }

            Msg::CreateComponent(origin) => {
                let mut component_id = None;

                trys! {
                    origin.type_as::<block::Boxblock>().map(|boxblock| {
                        let component = component::BoxblockComponent::new(boxblock.clone());
                        let component = self.arena.insert(component);
                        component_id = Some(component.id());
                        self.world.update(|world| {
                            world.push_boxblock_as_component(component);
                        });
                    }).is_some();
                    origin.type_as::<block::Craftboard>().map(|craftboard|{
                        let component = component::CraftboardComponent::new(craftboard.clone());
                        let component = self.arena.insert(component);
                        component_id = Some(component.id());
                        self.world.update(|world| {
                            world.push_craftboard_as_component(component);
                        });
                    }).is_some();
                }

                if let Some(component_id) = component_id {
                    Cmd::chain(Msg::UpdateBlocks {
                        insert: set! {},
                        update: set! { self.world.id(), component_id },
                    })
                } else {
                    Cmd::none()
                }
            }
        }
    }
}
