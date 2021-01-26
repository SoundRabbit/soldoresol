use super::super::{
    super::util::State, children::room_modeless, model::table::TableTool, renderer::Renderer,
};
use super::{Cmd, Implement, Modal, ModelessContent, Msg, On};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use crate::arena::Insert;
use crate::libs::select_list::SelectList;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

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
                        &self.resource_arena,
                        &self.world_id,
                        &self.camera_matrix,
                    );
                }
                Cmd::none()
            }

            Msg::UpdateMouseState { e } => {
                let mut need_update = false;
                let page_x = e.page_x() as f32;
                let page_y = e.page_y() as f32;
                let client_x = page_x - self.canvas_pos[0];
                let client_y = page_y - self.canvas_pos[1];
                let last_point = &self.mouse_state.last_point;

                match self.table_tools.selected() {
                    Some(TableTool::Pen(pen)) => {
                        if self.mouse_state.is_dragging {
                            let selecting_table_id = self.block_arena.map(
                                &self.world_id,
                                |world: &block::world::World| {
                                    BlockId::clone(&world.selecting_table())
                                },
                            );
                            let texture_id = selecting_table_id.and_then(|b_id| {
                                self.block_arena.map(&b_id, |table: &block::table::Table| {
                                    BlockId::clone(&table.drawing_texture_id())
                                })
                            });
                            if let Some(texture_id) = texture_id {
                                let a = self.camera_matrix.collision_point_on_xy_plane(
                                    &self.canvas_size,
                                    &[
                                        last_point[0] - self.canvas_pos[0],
                                        last_point[1] - self.canvas_pos[1],
                                    ],
                                );
                                let b = self.camera_matrix.collision_point_on_xy_plane(
                                    &self.canvas_size,
                                    &[client_x, client_y],
                                );
                                let p = self.block_arena.map_mut(
                                    &texture_id,
                                    |texture: &mut block::table::texture::Texture| {
                                        let a =
                                            texture.texture_position(&[a[0] as f64, a[1] as f64]);
                                        let b =
                                            texture.texture_position(&[b[0] as f64, b[1] as f64]);
                                        let context = texture.context();

                                        context.begin_path();
                                        context.set_stroke_style(
                                            &pen.pallet.color(pen.alpha).to_jsvalue(),
                                        );
                                        context.set_line_cap("round");
                                        context.set_line_width(pen.line_width);
                                        context.move_to(a[0], a[1]);
                                        context.line_to(b[0], b[1]);
                                        context.stroke();

                                        need_update = true;

                                        (a, b)
                                    },
                                );

                                if let Some((a, b)) = p {
                                    if self.mouse_state.is_changed_dragging_state {
                                        self.drawing_line = vec![a, b];
                                    } else {
                                        self.drawing_line.push(a);
                                    }
                                }
                            }
                        } else if self.mouse_state.is_changed_dragging_state
                            && self.drawing_line.len() >= 2
                        {
                            let mut points = self
                                .drawing_line
                                .drain(..)
                                .collect::<std::collections::VecDeque<_>>();

                            let selecting_table_id = self.block_arena.map(
                                &self.world_id,
                                |world: &block::world::World| {
                                    BlockId::clone(&world.selecting_table())
                                },
                            );
                            let drawing_texture_id = selecting_table_id.as_ref().and_then(|b_id| {
                                self.block_arena.map(&b_id, |table: &block::table::Table| {
                                    BlockId::clone(&table.drawing_texture_id())
                                })
                            });
                            let drawed_texture_id = selecting_table_id.as_ref().and_then(|b_id| {
                                self.block_arena.map(&b_id, |table: &block::table::Table| {
                                    BlockId::clone(&table.drawed_texture_id())
                                })
                            });

                            if let Some((drawing_texture_id, drawed_texture_id)) =
                                join_some!(drawing_texture_id, drawed_texture_id)
                            {
                                self.block_arena.map_mut(
                                    &drawing_texture_id,
                                    |texture: &mut block::table::texture::Texture| {
                                        let context = texture.context();
                                        let sz = texture.buffer_size();
                                        context.clear_rect(0.0, 0.0, sz[0], sz[1]);
                                    },
                                );

                                self.block_arena.map_mut(
                                    &drawed_texture_id,
                                    |texture: &mut block::table::texture::Texture| {
                                        let context = texture.context();

                                        let a = points.pop_front().unwrap();

                                        context.begin_path();
                                        context.set_stroke_style(
                                            &pen.pallet.color(pen.alpha).to_jsvalue(),
                                        );
                                        context.set_line_cap("round");
                                        context.set_line_width(pen.line_width);
                                        context.set_line_join("round");
                                        context.move_to(a[0], a[1]);

                                        for b in points {
                                            context.line_to(b[0], b[1]);
                                        }

                                        context.stroke();
                                    },
                                );

                                need_update = true;
                            }
                        }
                    }
                    _ => {}
                }

                self.mouse_state.update(e);

                if need_update {
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
