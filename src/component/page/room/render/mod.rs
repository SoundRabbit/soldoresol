use super::super::atom::{
    attr,
    btn::{self, Btn},
    dropdown::{self, Dropdown},
    file_catcher::{self, FileCatcher},
    header::{self, Header},
    text,
};
use super::super::organism::modal_chat_user::{self, ModalChatUser};
use super::super::organism::{
    table_menu::{self, TableMenu},
    world_view::{self, WorldView},
};
use super::super::template::{
    basic_app::{self, BasicApp},
    common::Common,
};
use super::*;
use crate::arena::{block, ArenaMut, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Sub;
use wasm_bindgen::JsCast;

mod contextmenu;

impl Render for Room {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(BasicApp::with_children(
            basic_app::Props {},
            Sub::none(),
            vec![
                Header::with_children(
                    header::Props::new(),
                    Sub::none(),
                    vec![self.render_header_row_0(), self.render_header_row_1()],
                ),
                FileCatcher::<Self>::with_children(
                    file_catcher::Props {
                        attributes: Attributes::new().class(Common::layered()),
                        ok_to_catch_file: self.ok_to_catch_file,
                    },
                    Sub::map(|sub| match sub {
                        file_catcher::On::LoadImageData(data) => Msg::AddResourceImageData(data),
                    }),
                    vec![
                        Html::div(
                            Attributes::new().class(Common::layered_item()),
                            Events::new(),
                            vec![self.table.with_children(
                                table::Props {
                                    is_2d_mode: self.is_2d_mode,
                                    is_debug_mode: self.is_debug_mode,
                                    arena: ArenaMut::clone(&self.arena),
                                    world: BlockMut::clone(&self.world),
                                },
                                Sub::map(|sub| match sub {
                                    table::On::UpdateBlocks { insert, update } => {
                                        Msg::UpdateBlocks { insert, update }
                                    }
                                }),
                                vec![],
                            )],
                        ),
                        Html::div(
                            Attributes::new()
                                .class(Common::layered_item())
                                .class(Self::class("main")),
                            Events::new(),
                            vec![
                                TableMenu::empty(
                                    table_menu::Props {
                                        arena: ArenaMut::clone(&self.arena),
                                        world: BlockMut::clone(&self.world),
                                    },
                                    Sub::map(|sub| match sub {
                                        table_menu::On::SelectTool(tool) => {
                                            Msg::SetSelectedTableTool(tool)
                                        }
                                        table_menu::On::UpdateBlocks { insert, update } => {
                                            Msg::UpdateBlocks { insert, update }
                                        }
                                    }),
                                ),
                                self.modeless_container.with_children(
                                    tab_modeless_container::Props {},
                                    Sub::map(|sub| match sub {
                                        tab_modeless_container::On::StartDragTab => {
                                            Msg::SetOkToCatchFile(false)
                                        }
                                        tab_modeless_container::On::EndDragTab => {
                                            Msg::SetOkToCatchFile(true)
                                        }
                                        tab_modeless_container::On::Sub(sub) => match sub {
                                            room_modeless::On::UpdateBlocks { insert, update } => {
                                                Msg::UpdateBlocks { insert, update }
                                            }
                                        },
                                    }),
                                    vec![Html::div(
                                        Attributes::new().class(Self::class("mouse-capture")),
                                        Events::new()
                                            .on("wheel", |e| {
                                                if let Ok(e) = e.dyn_into::<web_sys::WheelEvent>() {
                                                    Msg::OnTableWheel(e)
                                                } else {
                                                    Msg::NoOp
                                                }
                                            })
                                            .on_click(Msg::OnTableClick)
                                            .on_mousedown(Msg::OnTableMousedown)
                                            .on_mouseup(Msg::OnTableMouseup)
                                            .on_mousemove(Msg::OnTableMousemove)
                                            .on_contextmenu(Msg::OnTableContextmenu),
                                        vec![],
                                    )],
                                ),
                                WorldView::empty(
                                    world_view::Props {
                                        arena: ArenaMut::clone(&self.arena),
                                        world: BlockMut::clone(&self.world),
                                    },
                                    Sub::none(),
                                ),
                            ],
                        ),
                    ],
                ),
                if let Some(contextmenu) = &self.showing_contextmenu {
                    self.render_contextmenu(contextmenu)
                } else {
                    Common::none()
                },
                self.render_moadl(),
            ],
        ))
    }
}

impl Room {
    fn render_moadl(&self) -> Html<Self> {
        match &self.showing_modal {
            ShowingModal::None => Html::none(),
            ShowingModal::ChatUser => ModalChatUser::empty(
                modal_chat_user::Props {
                    world: self.world.as_ref(),
                    selected: self
                        .chat_users
                        .iter()
                        .filter_map(|user| match user {
                            ChatUser::Character(character) => Some(BlockMut::clone(&character)),
                            _ => None,
                        })
                        .collect(),
                },
                Sub::map(|sub| match sub {
                    modal_chat_user::On::Cancel => Msg::SetShowingModal(ShowingModal::None),
                    modal_chat_user::On::Select(selected) => Msg::CloseModalChatUser(selected),
                }),
            ),
        }
    }

    fn render_header_row_0(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_header_row_0_left(),
                Html::div(
                    Attributes::new().class(Self::class("right")),
                    Events::new(),
                    vec![],
                ),
            ],
        )
    }

    fn render_header_row_0_left(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("view-room-id")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new().class(Self::class("label")),
                    Events::new(),
                    vec![Html::text("ルームID")],
                ),
                Html::input(Attributes::new().flag("readonly"), Events::new(), vec![]),
            ],
        )
    }

    fn render_header_row_1(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("header-row"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_header_row_1_left(),
                self.render_header_row_1_right(),
            ],
        )
    }

    fn render_header_row_1_left(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("view-room-id")),
            Events::new(),
            vec![Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::BottomRight,
                    text: dropdown::Text::Text(String::from("チャット")),
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::Dark,
                },
                Sub::none(),
                vec![
                    Html::fragment(
                        self.chat_users
                            .iter()
                            .map(|user| self.render_header_row_1_left_userbtn(user))
                            .collect(),
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetShowingModal(ShowingModal::ChatUser)),
                        vec![Html::text("チャットで使用するキャラクターを設定")],
                    ),
                ],
            )],
        )
    }

    fn render_header_row_1_left_userbtn(&self, user: &ChatUser) -> Html<Self> {
        Btn::menu(
            Attributes::new().class(if let ChatUser::Player(..) = user {
                Self::class("chatuser-player")
            } else {
                Self::class("chatuser-character")
            }),
            Events::new().on_click({
                let user = ChatUser::clone(&user);
                move |_| Msg::OpenChatModeless(user)
            }),
            vec![Html::text(match user {
                ChatUser::Character(user) => user
                    .map(|user| user.name().clone())
                    .unwrap_or(String::from("")),
                ChatUser::Player(user) => user
                    .map(|user| user.name().clone())
                    .unwrap_or(String::from("")),
            })],
        )
    }

    fn render_header_row_1_right(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("right")),
            Events::new(),
            vec![Dropdown::with_children(
                dropdown::Props {
                    text: dropdown::Text::Text(if self.is_debug_mode {
                        String::from("3Dモード（デバッグ用）")
                    } else {
                        if self.is_2d_mode {
                            String::from("2Dモード（正射影）")
                        } else {
                            String::from("3Dモード（透視法）")
                        }
                    }),
                    direction: dropdown::Direction::Bottom,
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::Dark,
                },
                Sub::none(),
                vec![
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIs2dMode(true, false)),
                        vec![Html::text("2Dモード（正射影）")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIs2dMode(false, false)),
                        vec![Html::text("3Dモード（透視法）")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIs2dMode(false, true)),
                        vec![Html::text("3Dモード（デバッグ用）")],
                    ),
                ],
            )],
        )
    }
}

impl Styled for Room {
    fn style() -> Style {
        style! {
            ".header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            ".view-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "column-gap": "0.65em";
            }

            ".label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            ".chatuser-player" {
                "color": crate::libs::color::Pallet::yellow(5);
            }

            ".main" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
            }

            ".mouse-capture" {
                "position": "absolute";
                "left": "0";
                "top": "0";
                "width": "100%";
                "height": "100%";
            }

            ".contextmenu-mask" {
                "position": "fixed";
                "left": "0";
                "top": "0";
                "width": "100%";
                "height": "100%";
                "z-index": super::super::constant::z_index::MASK;
            }

            ".contextmenu" {
                "position": "absolute";
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "row";
            }
        }
    }
}
