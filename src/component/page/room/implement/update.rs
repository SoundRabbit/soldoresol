use super::super::{
    super::util::State,
    children::room_modeless,
    model::table::{BoxblockTool, CharacterTool, PointlightTool, ShapeTool, TableTool},
    renderer::{ObjectId, Renderer},
};
use super::{Cmd, Contextmenu, ContextmenuKind, Implement, Modal, ModelessContent, Msg, On};
use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self, ResourceId};
use crate::arena::Insert;
use crate::libs::clone_of::CloneOf;
use crate::libs::color::Pallet;
use crate::libs::select_list::SelectList;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

mod update_mouse;

impl Implement {
    pub fn update(&mut self, msg: Msg) -> Cmd<Msg, On> {
        match msg {
            Msg::NoOp => {}

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

                // デバッグ用に大量のブロックを設置
                for x in 0..20 {
                    for y in 0..20 {
                        for z in 0..10 {
                            let p = [x as f32 - 9.5, y as f32 - 9.5, z as f32 + 0.5];
                            let s = [1.0, 1.0, 1.0];

                            self.create_new_boxblock(
                                p,
                                s,
                                crate::libs::color::Pallet::blue(5).a(100),
                                block::boxblock::Shape::Sphere,
                            );
                        }
                    }
                }

                self.cmds.push(Cmd::task(move |resolve| {
                    let a = Closure::once(
                        Box::new(move || resolve(Msg::ResetCanvasSize)) as Box<dyn FnOnce()>
                    );
                    let _ = web_sys::window()
                        .unwrap()
                        .request_animation_frame(a.as_ref().unchecked_ref());
                    a.forget();
                }));
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
                    self.gl_render_async();
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
                    match focused_object_id {
                        ObjectId::Character(block_id, _) => {
                            self.contextmenu = Some(Contextmenu {
                                page_x: page_x,
                                page_y: page_y,
                                kind: ContextmenuKind::Character(block_id),
                            });
                        }
                        ObjectId::Boxblock(block_id, _) => {
                            self.contextmenu = Some(Contextmenu {
                                page_x: page_x,
                                page_y: page_y,
                                kind: ContextmenuKind::Boxblock(block_id),
                            });
                        }
                        _ => {}
                    }
                } else {
                    if self.update_mouse() {
                        self.gl_render_async();
                    }
                }
            }

            Msg::UpdateKeyState { e, is_key_down } => {
                self.key_state.update(e, is_key_down);
            }

            Msg::SetTableToolIdx { idx } => {
                self.table_tools.set_selected_idx(idx);
            }

            Msg::SetSelectingTableTool { tool } => {
                if let Some(selecting_tool) = self.table_tools.selected_mut() {
                    *selecting_tool = tool;
                }
            }

            Msg::OpenNewModal { modal } => {
                self.modal = modal;
            }

            Msg::OpenNewModeless { content } => {
                self.modeless_list.push(ModelessContent {
                    content: State::new(SelectList::new(vec![content], 0)),
                    page_x: 0,
                    page_y: 0,
                    minimized: false,
                });
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
            }

            Msg::CloseModeless { modeless_id } => {
                self.modeless_list.remove(&modeless_id);
            }

            Msg::MinimizeModeless { modeless_id } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.minimized = true;
                }
            }

            Msg::RestoreModeless { modeless_id } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.minimized = false;
                }
            }

            Msg::FocusModeless { modeless_id } => {
                self.modeless_list.focus(&modeless_id);
            }

            Msg::SetModelessContainerElement { element } => {
                self.modeless_container_element = Some(State::new(element));
            }

            Msg::SetDraggingModelessTab {
                modeless_id,
                tab_idx,
            } => {
                self.dragging_modeless_tab = Some((modeless_id, tab_idx));
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
            }

            Msg::SelectModelessTab {
                modeless_id,
                tab_idx,
            } => {
                if let Some(modeless) = self.modeless_list.get_mut(&modeless_id) {
                    modeless.content.set_selected_idx(tab_idx);
                }
            }

            Msg::SetOverlay { overlay } => {
                self.overlay = overlay;
            }

            Msg::SetContextmenu { contextmenu } => {
                self.contextmenu = contextmenu;
            }

            Msg::LoadFile { files, overlay } => {
                if let Some(overlay) = overlay {
                    self.overlay = overlay;
                }
                self.cmds.push(Cmd::task(move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        let mut resource_tasks = vec![];
                        let mut block_tasks = vec![];
                        let is_toml = regex::Regex::new(r"^.*\.toml$").unwrap();
                        for file in files {
                            // MIMEタイプで判別できないので、拡張子で判別
                            if is_toml.is_match(&file.name()) {
                                block_tasks.push(JsFuture::from(Promise::new(
                                    &mut move |resolve, _| {
                                        let reader = Rc::new(web_sys::FileReader::new().unwrap());
                                        let a = Closure::wrap(Box::new({
                                            let reader = Rc::clone(&reader);
                                            move || {
                                                let _ = resolve.call1(
                                                    &js_sys::global(),
                                                    &reader.result().unwrap_or(JsValue::NULL),
                                                );
                                            }
                                        })
                                            as Box<dyn FnMut()>);
                                        reader.set_onload(Some(&a.as_ref().unchecked_ref()));
                                        let _ = reader.read_as_text(&file);
                                        a.forget();
                                    },
                                )));
                            } else {
                                resource_tasks.push(resource::Data::from_blob(file.into()));
                            }
                        }
                        let resources = futures::future::join_all(resource_tasks)
                            .await
                            .into_iter()
                            .filter_map(|x| x)
                            .collect::<Vec<_>>();
                        let toml_blocks = futures::future::join_all(block_tasks)
                            .await
                            .into_iter()
                            .filter_map(|x| {
                                x.ok()
                                    .and_then(|x| x.as_string())
                                    .and_then(|x| toml::from_str(&x).ok())
                            })
                            .collect::<Vec<toml::Value>>();
                        let mut block_tasks = vec![];
                        for toml_block in toml_blocks {
                            block_tasks.push(async {
                                crate::debug::log_1("start to create ab");
                                block::Arena::unpack_from_toml(toml_block).await
                            });
                        }
                        let blocks = futures::future::join_all(block_tasks)
                            .await
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>();

                        resolve(Msg::LoadData { blocks, resources })
                    })
                }));
            }

            Msg::LoadData { blocks, resources } => {
                self.cmds.push_msg(Msg::LoadArenaBlocks { blocks });
                self.cmds
                    .push_msg(Msg::LoadResourceData { data: resources });
            }

            Msg::LoadResourceData { data } => {
                for a_data in data {
                    self.resource_arena.add(a_data);
                }
            }

            Msg::LoadArenaBlocks { blocks } => {
                for (block_id, block) in blocks {
                    if block.is::<block::character::Character>() {
                        let block_id = BlockId::clone(&block_id);
                        self.block_arena.map_mut(
                            &self.world_id,
                            |world: &mut block::world::World| {
                                world.add_character(block_id);
                            },
                        );
                    }

                    self.block_arena.assign_arena_block(block_id, block);
                }
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
            }

            Msg::UpdateTableProps {
                table_id,
                size,
                grid_color,
                background_color,
                background_image,
                env_light_intensity,
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

                        if let Some(env_light_intensity) = env_light_intensity {
                            table.set_env_light_intensity(env_light_intensity);
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
                    self.gl_render_async();
                }
            }

            Msg::SetCharacterCommonProps {
                character_id,
                name,
                display_name,
                description,
                name_color,
            } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        if let Some(name) = name {
                            character.set_name(name);
                        }

                        if let Some(display_name) = display_name {
                            character.set_display_name(display_name);
                        }

                        if let Some(description) = description {
                            character.set_description(description);
                        }

                        if let Some(name_color) = name_color {
                            character.set_name_color(name_color);
                        }
                    },
                );

                self.gl_render_async();
            }

            Msg::SetCharacterTextureId {
                character_id,
                tex_idx,
                resource_id,
            } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        character.set_tex_id(tex_idx, resource_id);
                    },
                );

                self.gl_render_async();
            }

            Msg::AddCharacterTexture { character_id } => {
                self.block_arena.map_mut(
                    &character_id,
                    |character: &mut block::character::Character| {
                        character.add_tex_to_select();
                    },
                );

                self.gl_render_async();
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

                self.gl_render_async();
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

                self.gl_render_async();
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
            }

            Msg::SetBoxblockCommonProps {
                boxblock_id,
                display_name,
                name,
                color,
                size,
            } => {
                self.block_arena.map_mut(
                    &boxblock_id,
                    |character: &mut block::boxblock::Boxblock| {
                        if let Some(name) = name {
                            character.set_name(name);
                        }

                        if let Some(display_name) = display_name {
                            character.set_display_name(display_name);
                        }

                        if let Some(color) = color {
                            character.set_color(color);
                        }

                        if let Some(size) = size {
                            character.set_size(size);
                        }
                    },
                );

                self.gl_render_async();
            }

            Msg::SetPropertyName { property_id, name } => {
                self.block_arena
                    .map_mut(&property_id, |prop: &mut block::property::Property| {
                        prop.set_name(name);
                    });
            }

            Msg::AddPropertyChild { block_id, name } => {
                let property = block::property::Property::new(name);
                let property_id = self.block_arena.insert(property);

                if self
                    .block_arena
                    .map_mut(&block_id, |character: &mut block::character::Character| {
                        character.add_property(BlockId::clone(&property_id));
                    })
                    .is_some()
                {
                } else if self
                    .block_arena
                    .map_mut(&block_id, |character: &mut block::property::Property| {
                        character.add_child(BlockId::clone(&property_id));
                    })
                    .is_some()
                {
                } else {
                    self.block_arena.free(&property_id);
                }
            }

            Msg::AddPropertyValue { property_id } => {
                self.block_arena
                    .map_mut(&property_id, |prop: &mut block::property::Property| {
                        prop.add_value(block::property::Value::None);
                    });
            }

            Msg::SetPropertyValue {
                property_id,
                idx,
                value,
            } => {
                self.block_arena
                    .map_mut(&property_id, |prop: &mut block::property::Property| {
                        prop.set_value(idx, value);
                    });
            }

            Msg::RemovePropertyValue { property_id, idx } => {
                self.block_arena
                    .map_mut(&property_id, |prop: &mut block::property::Property| {
                        prop.remove_value(idx);
                    });
            }

            Msg::SetPropertyValueMode {
                property_id,
                value_mode,
            } => {
                self.block_arena
                    .map_mut(&property_id, |prop: &mut block::property::Property| {
                        prop.set_value_mode(value_mode);
                    });
            }

            Msg::RemoveProperty { property_id, idx } => {
                self.block_arena
                    .map_mut(&property_id, |prop: &mut block::property::Property| {
                        prop.remove_child(idx);
                    });
            }
        }

        self.cmds.pop()
    }

    fn gl_render_async(&mut self) {
        self.cmds.push(Cmd::task(move |resolve| {
            let a =
                Closure::once(Box::new(move || resolve(Msg::RenderCanvas)) as Box<dyn FnOnce()>);
            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(a.as_ref().unchecked_ref());
            a.forget();
        }));
    }

    fn create_new_character(
        &mut self,
        size: Option<f32>,
        height: Option<f32>,
        tex_id: Option<ResourceId>,
        name: Option<String>,
        pos: Option<[f32; 3]>,
    ) {
        let mut character_block = block::character::Character::new();
        if let Some(size) = size {
            character_block.set_size(size);
        }
        if let Some(height) = height {
            character_block.set_tex_height(0, height);
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

    fn create_new_boxblock(
        &mut self,
        pos: [f32; 3],
        size: [f32; 3],
        color: Pallet,
        shape: block::boxblock::Shape,
    ) {
        let boxblock_block = block::boxblock::Boxblock::new(pos, size, color, shape);
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

    fn create_new_pointlight(
        &mut self,
        pos: [f32; 3],
        light_intensity: f32,
        light_attenation: f32,
        color: Pallet,
    ) {
        let mut pointlight_block = block::pointlight::Pointlight::new(pos);
        pointlight_block.set_light_intensity(light_intensity);
        pointlight_block.set_light_attenation(light_attenation);
        pointlight_block.set_color(color);
        let pointlight_id = self.block_arena.insert(pointlight_block);
        self.block_arena
            .map(&self.world_id, |world: &block::world::World| {
                BlockId::clone(world.selecting_table())
            })
            .map(|selecting_table_id| {
                self.block_arena
                    .map_mut(&selecting_table_id, |table: &mut block::table::Table| {
                        table.add_pointlight(pointlight_id);
                    });
            });
    }
}
