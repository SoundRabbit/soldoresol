use super::super::{
    super::util::State,
    children::room_modeless,
    model::table::{ShapeTool, TableTool},
    renderer::Renderer,
};
use super::{Cmd, Implement, Modal, ModelessContent, Msg, On};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use crate::arena::Insert;
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
                    );
                }
                Cmd::none()
            }

            Msg::UpdateMouseState { e } => {
                self.mouse_state.update(e);
                if self.update_mouse() {
                    self.gl_render_async()
                } else {
                    Cmd::none()
                }
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

                    self.block_arena.map_mut(
                        &drawing_texture_id,
                        |tex: &mut block::table::texture::Texture| {
                            tex.set_size(tex_size.clone());
                        },
                    );

                    self.block_arena.map_mut(
                        &drawed_texture_id,
                        |tex: &mut block::table::texture::Texture| {
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
}
