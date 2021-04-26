use super::super::{
    super::atom::btn::{self, Btn},
    super::atom::dropdown::{self, Dropdown},
    super::atom::fa,
    super::atom::header::{self, Header},
    super::atom::text,
    super::template::basic_app::{self, BasicApp},
    super::util::styled::Styled,
    children::modal_imported_files::{self, ModalImportedFiles},
    children::modal_new_channel::{self, ModalNewChannel},
    children::room_modeless::{self, RoomModeless},
    children::side_menu::{self, SideMenu},
};
use super::{ContextmenuKind, Implement, Modal, Msg, Overlay};
use crate::arena::block::{self, BlockId};
use crate::libs::random_id::U128Id;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

impl Implement {
    pub fn render(&self, _: Vec<Html>) -> Html {
        let selecting_table_id = self
            .block_arena
            .map(&self.world_id, |world: &block::world::World| {
                BlockId::clone(world.selecting_table())
            })
            .unwrap_or(BlockId::none());

        BasicApp::with_children(
            basic_app::Props {},
            Subscription::new({
                let tab_is_dragging = self.dragging_modeless_tab.is_some();
                move |sub| match sub {
                    basic_app::On::DragLeave(_) => Msg::NoOp,
                    basic_app::On::DragOver(e) => {
                        if !tab_is_dragging {
                            e.prevent_default();
                            Msg::SetOverlay {
                                overlay: Overlay::DragFile,
                            }
                        } else {
                            Msg::NoOp
                        }
                    }
                    basic_app::On::Drop(e) => {
                        if !tab_is_dragging {
                            let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                            let file_list = unwrap_or!(data_transfer.files(); Msg::NoOp);

                            e.prevent_default();

                            let mut files = vec![];
                            let file_num = file_list.length();

                            for i in 0..file_num {
                                if let Some(file) = file_list.item(i) {
                                    files.push(file);
                                }
                            }

                            Msg::LoadFile {
                                files,
                                overlay: Some(Overlay::None),
                            }
                        } else {
                            Msg::NoOp
                        }
                    }
                }
            }),
            vec![
                Header::with_children(
                    header::Props::new(),
                    Subscription::none(),
                    vec![
                        self.render_header_row_0(),
                        self.render_header_controller_menu(),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Self::class("body")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("side-menu")),
                            Events::new(),
                            vec![SideMenu::empty(
                                side_menu::Props {
                                    tools: self.table_tools.as_prop(),
                                    block_arena: self.block_arena.as_ref(),
                                    resource_arena: self.resource_arena.as_ref(),
                                    selecting_table_id: BlockId::clone(&selecting_table_id),
                                },
                                Subscription::new(|sub| match sub {
                                    side_menu::On::ChangeSelectedIdx { idx } => {
                                        Msg::SetTableToolIdx { idx }
                                    }
                                    side_menu::On::SetSelectedTool { tool } => {
                                        Msg::SetSelectingTableTool { tool }
                                    }
                                    side_menu::On::ChangeTableProps {
                                        table_id,
                                        size,
                                        grid_color,
                                        background_color,
                                        background_image,
                                    } => Msg::UpdateTableProps {
                                        table_id,
                                        size,
                                        grid_color,
                                        background_color,
                                        background_image,
                                    },
                                }),
                            )],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("main")),
                            Events::new(),
                            vec![self.render_canvas(), self.render_modeless_container()],
                        ),
                    ],
                ),
                self.render_contextmenu(),
                self.render_modal(),
                self.render_overlay(),
            ],
        )
    }

    fn render_modal(&self) -> Html {
        match &self.modal {
            Modal::None => Html::none(),
            Modal::NewChannel => ModalNewChannel::empty(
                modal_new_channel::Props {
                    client_id: Rc::clone(&self.client_id),
                },
                Subscription::new(|sub| match sub {
                    modal_new_channel::On::Close => Msg::OpenNewModal { modal: Modal::None },
                    modal_new_channel::On::CreateNewChannel {
                        channel_name,
                        channel_type,
                    } => Msg::CreateNewChannel {
                        channel_name,
                        channel_type,
                    },
                }),
            ),
            Modal::ImportedFiles => ModalImportedFiles::empty(
                modal_imported_files::Props {
                    resource_arena: self.resource_arena.as_ref(),
                },
                Subscription::new(|sub| match sub {
                    modal_imported_files::On::Close => Msg::OpenNewModal { modal: Modal::None },
                    modal_imported_files::On::SelectFile(_) => Msg::NoOp,
                }),
            ),
        }
    }

    fn render_overlay(&self) -> Html {
        match &self.overlay {
            Overlay::None => Html::none(),
            Overlay::DragFile => Html::div(
                Attributes::new()
                    .class(Self::class("overlay"))
                    .class(Self::class("overlay-file-import")),
                Events::new().on_dragleave(|_| Msg::SetOverlay {
                    overlay: Overlay::None,
                }),
                vec![Html::div(
                    Attributes::new().class(Self::class("overlay-file-import-text")),
                    Events::new(),
                    vec![
                        fa::i("fa-file-import"),
                        Html::text("ファイルをドロップして追加"),
                    ],
                )],
            ),
        }
    }

    fn render_header_row_0(&self) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![self.render_header_row_0_left()],
        )
    }

    fn render_header_row_0_left(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("header-room-id")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(Self::class("label"))
                        .string("for", &self.element_id.header_room_id),
                    Events::new(),
                    vec![Html::text("ルームID")],
                ),
                Html::input(
                    Attributes::new()
                        .flag("readonly")
                        .class("pure-input")
                        .id(&self.element_id.header_room_id)
                        .value(self.room_id.as_ref()),
                    Events::new(),
                    vec![],
                ),
            ],
        )
    }

    fn render_header_controller_menu(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("header-controller-menu")),
            Events::new(),
            vec![
                self.render_header_controller_menu_chat(),
                self.render_header_controller_menu_file(),
            ],
        )
    }

    fn render_header_controller_menu_chat(&self) -> Html {
        let channel_names = self
            .block_arena
            .map(&self.chat_id, |chat: &block::chat::Chat| {
                let mut channels = vec![];

                for channel_id in chat.channels() {
                    let channel_name = self
                        .block_arena
                        .map(channel_id, |channel: &block::chat::channel::Channel| {
                            channel.name().clone()
                        })
                        .unwrap_or(String::new());
                    channels.push((BlockId::clone(&channel_id), channel_name));
                }

                channels
            })
            .unwrap_or(vec![]);

        Dropdown::with_children(
            dropdown::Props {
                text: String::from("チャット"),
                direction: dropdown::Direction::BottomRight,
                variant: btn::Variant::Dark,
                ..Default::default()
            },
            Subscription::none(),
            vec![
                Dropdown::with_children(
                    dropdown::Props {
                        text: String::from("チャンネル"),
                        direction: dropdown::Direction::RightBottom,
                        toggle_type: dropdown::ToggleType::Hover,
                        variant: btn::Variant::Menu,
                        ..Default::default()
                    },
                    Subscription::none(),
                    vec![
                        vec![Btn::with_children(
                            btn::Props {
                                variant: btn::Variant::Menu,
                            },
                            Subscription::new(|sub| match sub {
                                btn::On::Click => Msg::OpenNewChatModeless,
                            }),
                            vec![fa::i("fa-comments"), Html::text(" 全てのチャンネル")],
                        )],
                        channel_names
                            .into_iter()
                            .map(|(channel_id, channel_name)| {
                                Btn::with_children(
                                    btn::Props {
                                        variant: btn::Variant::Menu,
                                    },
                                    Subscription::new(|sub| match sub {
                                        btn::On::Click => Msg::OpenNewModeless {
                                            content: room_modeless::Content::ChatChannel(
                                                channel_id,
                                            ),
                                        },
                                    }),
                                    vec![
                                        fa::i("fa-comment"),
                                        Html::text(format!(" {}", channel_name)),
                                    ],
                                )
                            })
                            .collect(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                ),
                Btn::with_children(
                    btn::Props {
                        variant: btn::Variant::Menu,
                    },
                    Subscription::new(|sub| match sub {
                        btn::On::Click => Msg::OpenNewModal {
                            modal: Modal::NewChannel,
                        },
                    }),
                    vec![fa::i("fa-plus"), Html::text(" 新規チャンネル")],
                ),
                Btn::with_children(
                    btn::Props {
                        variant: btn::Variant::Menu,
                    },
                    Subscription::new(|sub| match sub {
                        btn::On::Click => Msg::OpenNewChatModeless,
                    }),
                    vec![fa::i("fa-cog"), Html::text(" チャンネル設定")],
                ),
            ],
        )
    }

    fn render_header_controller_menu_file(&self) -> Html {
        Dropdown::with_children(
            dropdown::Props {
                text: String::from("ファイル"),
                direction: dropdown::Direction::BottomRight,
                variant: btn::Variant::Dark,
                ..Default::default()
            },
            Subscription::none(),
            vec![Btn::with_children(
                btn::Props {
                    variant: btn::Variant::Menu,
                },
                Subscription::new(|sub| match sub {
                    btn::On::Click => Msg::OpenNewModal {
                        modal: Modal::ImportedFiles,
                    },
                }),
                vec![fa::i("fa-file"), Html::text(" 全てのファイル")],
            )],
        )
    }

    fn render_canvas(&self) -> Html {
        Html::canvas(
            Attributes::new().class(Self::class("canvas")),
            Events::new().rendered(if self.renderer.is_none() {
                Some(|el: web_sys::Element| {
                    let canvas =
                        unwrap_or!(el.dyn_into::<web_sys::HtmlCanvasElement>().ok(); Msg::NoOp);
                    Msg::SetCanvasElement { canvas: canvas }
                })
            } else {
                None
            }),
            vec![],
        )
    }

    fn render_modeless_container(&self) -> Html {
        if let Some(modeless_container_element) = self.modeless_container_element.as_ref() {
            Html::div(
                Attributes::new().class(Self::class("modeless-container")),
                Events::new()
                    .on("dragover", |e| {
                        e.prevent_default();
                        Msg::NoOp
                    })
                    .on("drop", move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                        let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                        let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                        if data == RoomModeless::tag_id() {
                            e.prevent_default();
                            e.stop_propagation();
                            let page_x = e.page_x();
                            let page_y = e.page_y();
                            Msg::DropModelessTab { page_x, page_y }
                        } else {
                            Msg::NoOp
                        }
                    })
                    .on("mousemove", move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                        Msg::UpdateMouseState { e }
                    })
                    .on("mousedown", move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                        Msg::UpdateMouseState { e }
                    })
                    .on("mouseup", move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                        Msg::UpdateMouseState { e }
                    })
                    .on("contextmenu", move |e| {
                        e.prevent_default();
                        Msg::NoOp
                    }),
                self.modeless_list
                    .iter()
                    .map(|m| {
                        if let Some((modeless_id, z_index, modeless)) = m.as_ref() {
                            RoomModeless::empty(
                                room_modeless::Props {
                                    z_index: *z_index,
                                    content: modeless.content.as_prop(),
                                    container_element: modeless_container_element.as_prop(),
                                    page_x: modeless.page_x,
                                    page_y: modeless.page_y,
                                    minimized: modeless.minimized,
                                    block_arena: self.block_arena.as_ref(),
                                    resource_arena: self.resource_arena.as_ref(),
                                },
                                Subscription::new({
                                    let modeless_id = U128Id::clone(&modeless_id);
                                    |sub| match sub {
                                        room_modeless::On::Focus => {
                                            Msg::FocusModeless { modeless_id }
                                        }
                                        room_modeless::On::Close => {
                                            Msg::CloseModeless { modeless_id }
                                        }
                                        room_modeless::On::Minimize => {
                                            Msg::MinimizeModeless { modeless_id }
                                        }
                                        room_modeless::On::Restore => {
                                            Msg::RestoreModeless { modeless_id }
                                        }
                                        room_modeless::On::DragTabStart { tab_idx } => {
                                            Msg::SetDraggingModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                        room_modeless::On::DropTab { tab_idx } => {
                                            Msg::MoveModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                        room_modeless::On::SelectTab { tab_idx } => {
                                            Msg::SelectModelessTab {
                                                modeless_id,
                                                tab_idx,
                                            }
                                        }
                                        room_modeless::On::SetCharacterTextureId {
                                            character_id,
                                            tex_idx,
                                            resource_id,
                                        } => Msg::SetCharacterTextureId {
                                            character_id,
                                            tex_idx,
                                            resource_id,
                                        },
                                        room_modeless::On::AddCharacterTexture { character_id } => {
                                            Msg::AddCharacterTexture { character_id }
                                        }
                                        room_modeless::On::RemoveCharacterTexture {
                                            character_id,
                                            tex_idx,
                                        } => Msg::RemoveCharacterTexture {
                                            character_id,
                                            tex_idx,
                                        },
                                        room_modeless::On::SetCharacterTextureIdx {
                                            character_id,
                                            tex_idx,
                                        } => Msg::SetCharacterTextureIdx {
                                            character_id,
                                            tex_idx,
                                        },
                                        room_modeless::On::SetCharacterTextureName {
                                            character_id,
                                            tex_idx,
                                            tex_name,
                                        } => Msg::SetCharacterTextureName {
                                            character_id,
                                            tex_idx,
                                            tex_name,
                                        },
                                        room_modeless::On::AddPropertyChild { block_id, name } => {
                                            Msg::AddPropertyChild { block_id, name }
                                        }
                                        room_modeless::On::AddPropertyValue { property_id } => {
                                            Msg::AddPropertyValue { property_id }
                                        }
                                        room_modeless::On::SetPropertyValue {
                                            property_id,
                                            idx,
                                            value,
                                        } => Msg::SetPropertyValue {
                                            property_id,
                                            idx,
                                            value,
                                        },
                                    }
                                }),
                            )
                        } else {
                            Html::div(Attributes::new(), Events::new(), vec![])
                        }
                    })
                    .collect(),
            )
        } else {
            Html::div(
                Attributes::new().class(Self::class("modeless-container")),
                Events::new()
                    .rendered(Some(|element| Msg::SetModelessContainerElement { element })),
                vec![],
            )
        }
    }

    fn close_context_menu_event(_: web_sys::Event) -> Msg {
        Msg::SetContextmenu { contextmenu: None }
    }

    fn render_contextmenu(&self) -> Html {
        if let Some(contextmenu) = &self.contextmenu {
            Html::div(
                Attributes::new().class(Self::class("overlay")),
                Events::new()
                    .on("click", Self::close_context_menu_event)
                    .on("contextmenu", move |e| {
                        e.prevent_default();
                        Msg::NoOp
                    }),
                vec![Html::div(
                    Attributes::new()
                        .class(Self::class("contextmenu"))
                        .style("left", format!("{}px", contextmenu.page_x))
                        .style("top", format!("{}px", contextmenu.page_y)),
                    Events::new(),
                    match &contextmenu.kind {
                        ContextmenuKind::Character(block_id) => {
                            self.render_contextmenu_character(block_id)
                        }
                    },
                )],
            )
        } else {
            Html::none()
        }
    }

    fn render_contextmenu_character(&self, block_id: &BlockId) -> Vec<Html> {
        vec![Btn::with_child(
            btn::Props {
                variant: btn::Variant::Menu,
            },
            Subscription::new({
                let block_id = BlockId::clone(&block_id);
                move |sub| match sub {
                    btn::On::Click => Msg::OpenNewModeless {
                        content: room_modeless::Content::Character(block_id),
                    },
                }
            }),
            Html::text("編集"),
        )]
    }
}
