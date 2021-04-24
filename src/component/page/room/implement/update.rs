use super::super::{
    super::util::State,
    children::room_modeless,
    model::table::{BoxblockTool, CharacterTool, ShapeTool, TableTool},
    renderer::{ObjectId, Renderer},
};
use super::{Cmd, Contextmenu, ContextmenuKind, Implement, Modal, ModelessContent, Msg, On};
use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self, ResourceId};
use crate::arena::Insert;
use crate::libs::clone_of::CloneOf;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod update_mouse;

impl Implement {
    pub fn update(&mut self, msg: Msg) -> Cmd<Msg, On> {
        match msg {
            Msg::NoOp => Cmd::none(),

            Msg::SetCanvasElement { canvas } => {
                let canvas = Rc::new(canvas);
                self.renderer = Some(Renderer::new(Rc::clone(&canvas)));

                let client_rect = canvas.get_bounding_client_rect();
                let client_left = client_rect.left() as f32;
                let client_top = client_rect.top() as f32;
                let client_width = client_rect.width() as f32;
                let client_height = client_rect.height() as f32;
                self.canvas_pos = [client_left, client_top];
                self.canvas_size = [client_width, client_height];

                self.canvas = Some(canvas);
                Cmd::task(move |resolve| {
                    let a = Closure::once(
                        Box::new(move || resolve(Msg::ResetCanvasSize)) as Box<dyn FnOnce()>
                    );
                    let _ = web_sys::window()
                        .unwrap()
                        .request_animation_frame(a.as_ref().unchecked_ref());
                    a.forget();
                })
            }

            Msg::ResetCanvasSize => {
                if let Some(canvas) = &self.canvas {
                    let client_rect = canvas.get_bounding_client_rect();
                    let client_left = client_rect.left() as f32;
                    let client_top = client_rect.top() as f32;
                    let client_width = client_rect.width() as f32;
                    let client_height = client_rect.height() as f32;
                    self.canvas_pos = [client_left, client_top];
                    self.canvas_size = [client_width, client_height];
                }
                if let Some(renderer) = &mut self.renderer {
                    renderer.reset_size();
                    self.gl_render_async()
                } else {
                    Cmd::none()
                }
            }

            Msg::RenderCanvas => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render(
                        &self.block_arena,
                        &self.local_block_arena,
                        &self.resource_arena,
                        &self.world_id,
                        &self.camera_matrix,
                        &self.grabbed_object_id,
                    );
                }
                Cmd::none()
            }

            Msg::UpdateMouseState { e } => {
                self.mouse_btn_state.update(&e);
                let page_x = e.page_x();
                let page_y = e.page_y();
                let focused_object_id = if let Some(renderer) = &self.renderer {
                    let x = page_x as f32 - self.canvas_pos[0];
                    let y = page_y as f32 - self.canvas_pos[1];
                    renderer.get_object_id(x, y)
                } else {
                    ObjectId::None
                };
                if self.mouse_btn_state.secondary.is_clicked {
                    crate::debug::log_1(format!("focused: {}", focused_object_id));
                    match focused_object_id {
                        ObjectId::Character(block_id) => {
                            self.contextmenu = Some(Contextmenu {
                                page_x: page_x,
                                page_y: page_y,
                                kind: ContextmenuKind::Character(block_id),
                            });
                        }
                        _ => {}
                    }
                    Cmd::none()
                } else {
                    if self.update_mouse() {
                        self.gl_render_async()
                    } else {
                        Cmd::none()
                    }
                }
            }

            Msg::UpdateKeyState { e, is_key_down } => {
                self.key_state.update(e, is_key_down);
                Cmd::none()
            }

            Msg::SetTableToolIdx { idx } => {
                self.table_tools.set_selected_idx(idx);
                Cmd::none()
            }

            Msg::SetSelectingTableTool { tool } => {
                if let Some(selecting_tool) = self.table_tools.selected_mut() {
                    *selecting_tool = tool;
                }
                Cmd::none()
            }

            Msg::OpenNewModal { modal } => {
                self.modal = modal;
                Cmd::none()
            }

            Msg::OpenNewModeless { content } => {
                self.modeless_list.push(ModelessContent {
                    content: State::new(SelectList::new(vec![content], 0)),
                    page_x: 0,
                    page_y: 0,
                    minimized: false,
                });
                Cmd::none()
            }

            Msg::OpenNewChatModeless => {
                let tabs = self
                    .block_arena
                    .map(&self.chat_id, |chat: &block::chat::Chat| {
                        chat.channels()
                            .iter()
                            .map(|channel_id| {
                                room_modeless::Content::ChatChannel(BlockId::clone(&channel_id))
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or(vec![]);
                self.modeless_list.push(ModelessContent {
                    content: State::new(SelectList::new(tabs, 0)),
                    page_x: 128,
                    page_y: 128,
                    minimized: false,
                });
                Cmd::none()
            }

            Msg::CloseModeless { modeless_id } => {
                self.modeless_list.remove(&modeless_id);
                Cmd::none()
            }

            Msg::MinimizeModeless { modeless_id } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.minimized = true;
                }
                Cmd::none()
            }

            Msg::RestoreModeless { modeless_id } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.minimized = false;
                }
                Cmd::none()
            }

            Msg::FocusModeless { modeless_id } => {
                self.modeless_list.focus(&modeless_id);
                Cmd::none()
            }

            Msg::SetModelessContainerElement { element } => {
                self.modeless_container_element = Some(State::new(element));
                Cmd::none()
            }

            Msg::SetDraggingModelessTab {
                modeless_id,
                tab_idx,
            } => {
                crate::debug::log_1("SetDraggingModelessTab");
                self.dragging_modeless_tab = Some((modeless_id, tab_idx));
                Cmd::none()
            }

            Msg::MoveModelessTab {
                modeless_id: to_id,
                tab_idx,
            } => {
                if let Some((from_id, from_idx)) = self.dragging_modeless_tab.take() {
                    let tab = if let Some(tab) = self
                        .modeless_list
                        .get_mut(&from_id)
                        .and_then(|c| c.content.remove(from_idx))
                    {
                        Some(tab)
                    } else {
                        None
                    };
                    if let Some((tab, to_content)) =
                        join_some!(tab, self.modeless_list.get_mut(&to_id))
                    {
                        if let Some(tab_idx) = tab_idx {
                            to_content.content.insert(tab_idx, tab);
                        } else {
                            to_content.content.push(tab);
                        }
                    }
                    if let Some(from_content) = self.modeless_list.get(&from_id) {
                        if from_content.content.len() < 1 {
                            self.modeless_list.remove(&from_id);
                        }
                    }
                }
                Cmd::none()
            }

            Msg::DropModelessTab { page_x, page_y } => {
                if let Some((from_id, from_idx)) = self.dragging_modeless_tab.take() {
                    let tab = if let Some(tab) = self
                        .modeless_list
                        .get_mut(&from_id)
                        .and_then(|c| c.content.remove(from_idx))
                    {
                        Some(tab)
                    } else {
                        None
                    };
                    if let Some(tab) = tab {
                        self.modeless_list.push(ModelessContent {
                            content: State::new(SelectList::new(vec![tab], 0)),
                            page_x,
                            page_y,
                            minimized: false,
                        });
                    }
                    if let Some(from_content) = self.modeless_list.get(&from_id) {
                        if from_content.content.len() < 1 {
                            self.modeless_list.remove(&from_id);
                        }
                    }
                }
                Cmd::none()
            }

            Msg::SelectModelessTab {
                modeless_id,
                tab_idx,
            } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.content.set_selected_idx(tab_idx);
                }
                Cmd::none()
            }

            Msg::SetOverlay { overlay } => {
                self.overlay = overlay;
                Cmd::none()
            }

            Msg::SetContextmenu { contextmenu } => {
                self.contextmenu = contextmenu;
                Cmd::none()
            }

            Msg::LoadFile { files, overlay } => {
                if let Some(overlay) = overlay {
                    self.overlay = overlay;
                }
                Cmd::task(move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        let mut tasks = vec![];
                        for file in files {
                            tasks.push(resource::Data::from_blob(file.into()));
                        }
                        let data = futures::future::join_all(tasks)
                            .await
                            .into_iter()
                            .filter_map(|x| x)
                            .collect::<Vec<_>>();
                        resolve(Msg::LoadResourceData { data })
                    })
                })
            }

            Msg::LoadResourceData { data } => {
                for a_data in data {
                    self.resource_arena.add(a_data);
                }
                Cmd::none()
            }

            Msg::CreateNewChannel {
                channel_name,
                channel_type,
            } => {
                let channel = block::chat::channel::Channel::new(channel_name, channel_type);
                let channel_id = self.block_arena.insert(channel);

                self.block_arena
                    .map_mut(&self.chat_id, move |chat: &mut block::chat::Chat| {
                        chat.push_channel(channel_id);
                    });

                self.modal = Modal::None;

                Cmd::none()
            }

            Msg::UpdateTableProps {
                table_id,
                size,
                grid_color,
                background_color,
                background_image,
            } => {
                let mut is_updated = self
                    .block_arena
                    .map_mut(&table_id, |table: &mut block::table::Table| {
                        let mut is_updated = false;

                        if let Some(grid_color) = grid_color {
                            table.set_grid_color(grid_color);
                            is_updated = true;
                        }

                        if let Some(background_color) = background_color {
                            table.set_background_color(background_color);
                            is_updated = true;
                        }

                        if let Some(background_image) = background_image {
                            table.set_background_texture_id(background_image);
                            is_updated = true;
                        }

                        is_updated
                    })
                    .unwrap_or(false);

                if let Some(size) = size {
                    let tex_size = [size[0] as f64, size[1] as f64];

                    let (drawing_texture_id, drawed_texture_id) = self
                        .block_arena
                        .map_mut(&table_id, |table: &mut block::table::Table| {
                            table.set_size(size);
                            (
                                BlockId::clone(table.drawing_texture_id()),
                                BlockId::clone(table.drawed_texture_id()),
                            )
                        })
                        .unwrap_or((BlockId::none(), BlockId::none()));

                    self.local_block_arena.map_mut(
                        &drawing_texture_id,
                        |tex: &mut block::texture::Texture| {
                            tex.set_size(tex_size.clone());
                        },
                    );

                    self.block_arena.map_mut(
                        &drawed_texture_id,
                        |tex: &mut block::texture::Texture| {
                            tex.set_size(tex_size.clone());
                        },
                    );

                    is_updated = true;
                }

                if is_updated {
                    self.gl_render_async()
                } else {
                    Cmd::none()
                }
            }

            Msg::SetCharacterTextureId {
                character_id,
                tex_idx,
                resource_id,
            } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        crate::debug::log_1("SetCharacterTextureId");
                        character.set_tex_id(tex_idx, resource_id);
                    },
                );

                self.gl_render_async()
            }

            Msg::AddCharacterTexture { character_id } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        character.add_tex_to_select();
                    },
                );

                self.gl_render_async()
            }

            Msg::RemoveCharacterTexture {
                character_id,
                tex_idx,
            } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        character.remove_tex(tex_idx);
                    },
                );

                self.gl_render_async()
            }

            Msg::SetCharacterTextureIdx {
                character_id,
                tex_idx,
            } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        character.set_current_tex_idx(tex_idx);
                    },
                );

                self.gl_render_async()
            }

            Msg::SetCharacterTextureName {
                character_id,
                tex_idx,
                tex_name,
            } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        character.set_tex_name(tex_idx, tex_name);
                    },
                );

                Cmd::none()
            }
        }
    }

    fn gl_render_async(&mut self) -> Cmd<Msg, On> {
        Cmd::task(move |resolve| {
            let a =
                Closure::once(Box::new(move || resolve(Msg::RenderCanvas)) as Box<dyn FnOnce()>);
            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(a.as_ref().unchecked_ref());
            a.forget();
        })
    }

    fn create_new_character(
        &mut self,
        size: Option<f32>,
        tex_scale: Option<f32>,
        tex_id: Option<ResourceId>,
        name: Option<String>,
        pos: Option<[f32; 3]>,
    ) {
        let mut character_block = block::character::Character::new();
        if let Some(size) = size {
            character_block.set_size(size);
        }
        if let Some(tex_scale) = tex_scale {
            character_block.set_tex_scale(0, tex_scale);
        }
        character_block.set_tex_id(0, tex_id);
        if let Some(name) = name {
            character_block.set_name(name);
        }
        if let Some(pos) = pos {
            character_block.set_position(pos);
        }

        let character_id = self.block_arena.insert(character_block);

        self.block_arena
            .map_mut(&self.world_id, |world: &mut block::world::World| {
                world.add_character(character_id);
            });
    }

    fn create_new_boxblock(&mut self, pos: [f32; 3], size: [f32; 3], color: Pallet) {
        let boxblock_block = block::boxblock::Boxblock::new(pos, size, color);
        let boxblock_id = self.block_arena.insert(boxblock_block);
        self.block_arena
            .map(&self.world_id, |world: &block::world::World| {
                BlockId::clone(world.selecting_table())
            })
            .map(|selecting_table_id| {
                self.block_arena
                    .map_mut(&selecting_table_id, |table: &mut block::table::Table| {
                        table.add_boxblock(boxblock_id);
                    });
            });
    }
}
