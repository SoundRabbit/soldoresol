use super::super::atom::{
    btn::{self, Btn},
    common::Common,
    dropdown::{self, Dropdown},
    file_catcher::{self, FileCatcher},
    header::{self, Header},
    marker::Marker,
    table::{self, Table},
    text::Text,
};
use super::super::organism::{
    modal_chat_user::{self, ModalChatUser},
    modal_dicebot::{self, ModalDicebot},
    modal_resource::{self, ModalResource},
    room_modeless::{self, RoomModeless},
    room_modeless_chat::ChatUser,
    tab_modeless_container::{self, TabModelessContainer},
    table_menu::{self, TableMenu},
    world_view::{self, WorldView},
};
use super::super::template::basic_app::{self, BasicApp};
use super::{Msg, Room, ShowingModal};
use crate::arena::{block, ArenaMut, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

mod contextmenu;

impl Render<Html> for Room {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(BasicApp::new(
            self,
            None,
            basic_app::Props {},
            Sub::none(),
            vec![
                Header::new(
                    self,
                    None,
                    header::Props {},
                    Sub::none(),
                    (Attributes::new(), Events::new(), vec![self.render_header_row_0(), self.render_header_row_1()]),
                ),
                FileCatcher::new(
                    self,
                    None,
                    file_catcher::Props {
                        ok_to_catch_file: self.ok_to_catch_file,
                    },
                    Sub::map(|sub| match sub {
                        file_catcher::On::LoadImageData(data) => Msg::AddResourceImageData(data),
                    }),
                    (
                        Attributes::new().class(Common::layered()),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Common::layered_item()),
                                Events::new(),
                                vec![
                                Table::empty(self, None, table::Props {
                                    table: Rc::clone(&self.table),
                                    world: BlockMut::clone(&self.world)
                                }, Sub::map(|sub| match sub {
                                    table::On::UpdateBlocks {update, insert} => Msg::UpdateBlocks {update, insert}
                                }))
                                ],
                            ),
                            Html::div(
                                Attributes::new()
                                    .class(Common::layered_item())
                                    .class(Self::class("main")),
                                Events::new(),
                                vec![
                                    TableMenu::empty(self,None,
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
                                    TabModelessContainer::<RoomModeless, room_modeless::TabName>::new(
                                        self ,None,
                                        tab_modeless_container::Props {
                                            modelesses: Rc::clone(&self.modeless_container)
                                        },
                                        Sub::map(|sub| match sub {
                                            tab_modeless_container::On::StartDragTab => {
                                                Msg::SetOkToCatchFile(false)
                                            }
                                            tab_modeless_container::On::EndDragTab => {
                                                Msg::SetOkToCatchFile(true)
                                            }
                                            tab_modeless_container::On::Sub(sub) => match sub {
                                                room_modeless::On::UpdateBlocks {
                                                    insert,
                                                    update,
                                                } => Msg::UpdateBlocks { insert, update },
                                            },
                                        }),
                                        vec![Html::div(
                                            Attributes::new().class(Self::class("mouse-capture")),
                                            Events::new()
                                                .on("wheel", self, |e| {
                                                    if let Ok(e) =
                                                        e.dyn_into::<web_sys::WheelEvent>()
                                                    {
                                                        Msg::OnTableWheel(e)
                                                    } else {
                                                        Msg::NoOp
                                                    }
                                                })
                                                .on_click(self, |e| {
                                                    let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                                                    Msg::OnTableClick(e)
                                                })
                                                .on_mousedown(self, |e|{
                                                    let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                                                    Msg::OnTableMousedown(e)
                                                })
                                                .on_mouseup(self, |e| {
                                                    let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                                                    Msg::OnTableMouseup(e)
                                                })
                                                .on_mousemove(self, |e| {
                                                    let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                                                    Msg::OnTableMousemove(e)
                                                })
                                                .on_contextmenu(self, |e| {
                                                    let e = unwrap!(e.dyn_into::<web_sys::MouseEvent>().ok(); Msg::NoOp);
                                                    Msg::OnTableContextmenu(e)
                                                }),
                                            vec![],
                                        )],
                                    ),
                                    WorldView::empty(self,None,
                                        world_view::Props {
                                            arena: ArenaMut::clone(&self.arena),
                                            world: BlockMut::clone(&self.world),
                                        },
                                        Sub::map(|sub| match sub {
                                            world_view::On::UpdateBlocks {insert, update} => Msg::UpdateBlocks{insert, update}
                                        }),
                                    ),
                                ],
                            ),
                        ],
                    ),
                ),
                if let Some(contextmenu) = &self.showing_contextmenu {
                    self.render_contextmenu(contextmenu)
                } else {
                    Common::none()
                },
                self.render_modal(),
            ],
        ))
    }
}

impl Room {
    fn render_modal(&self) -> Html {
        match &self.showing_modal {
            ShowingModal::None => Html::none(),
            ShowingModal::ChatUser => ModalChatUser::empty(
                self,
                None,
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
            ShowingModal::Dicebot => ModalDicebot::empty(
                self,
                None,
                modal_dicebot::Props {
                    bcdice_loader: Rc::clone(&self.bcdice_loader),
                    selected_game_system: self
                        .game_system_class
                        .borrow()
                        .as_ref()
                        .map(|game_system_class| game_system_class.id().clone()),
                },
                Sub::map(|sub| match sub {
                    modal_dicebot::On::Close => Msg::SetShowingModal(ShowingModal::None),
                    modal_dicebot::On::SelectGameSystem { game_system_class } => {
                        Msg::SetGameSystemClass(game_system_class)
                    }
                }),
            ),
            ShowingModal::Resource => ModalResource::empty(
                self,
                None,
                modal_resource::Props {
                    arena: ArenaMut::clone(&self.arena),
                    filter: set! {},
                    world: BlockMut::clone(&self.world),
                    is_selecter: false,
                    title: String::from(modal_resource::title::VIEW_ALL_RESOURCE),
                },
                Sub::map(|sub| match sub {
                    modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                    modal_resource::On::UpdateBlocks { insert, update } => {
                        Msg::UpdateBlocks { insert, update }
                    }
                    _ => Msg::NoOp,
                }),
            ),
        }
    }

    fn render_header_row_0(&self) -> Html {
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

    fn render_header_row_0_left(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("view-room-id")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new().class(Self::class("label")),
                    Events::new(),
                    vec![Html::text("ルームID")],
                ),
                Html::input(
                    Attributes::new()
                        .flag("readonly", true)
                        .value(self.annot_room_id.as_str()),
                    Events::new(),
                    vec![],
                ),
            ],
        )
    }

    fn render_header_row_1(&self) -> Html {
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

    fn render_header_row_1_left(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("left")),
            Events::new(),
            vec![
                Dropdown::new(
                    self,
                    None,
                    dropdown::Props {
                        direction: dropdown::Direction::BottomRight,
                        toggle_type: dropdown::ToggleType::Click,
                        variant: btn::Variant::Dark,
                    },
                    Sub::none(),
                    (
                        vec![Html::text("チャット")],
                        vec![
                            Html::fragment(
                                self.chat_users
                                    .iter()
                                    .map(|user| self.render_header_row_1_left_userbtn(user))
                                    .collect(),
                            ),
                            Btn::menu(
                                Attributes::new(),
                                Events::new().on_click(self, |_| {
                                    Msg::SetShowingModal(ShowingModal::ChatUser)
                                }),
                                vec![Html::text("キャラクター設定")],
                            ),
                            Btn::menu(
                                Attributes::new(),
                                Events::new().on_click(self, |_| {
                                    Msg::SetShowingModal(ShowingModal::Dicebot)
                                }),
                                vec![Html::text("ダイスボット設定")],
                            ),
                        ],
                    ),
                ),
                Btn::dark(
                    Attributes::new(),
                    Events::new().on_click(self, |_| Msg::SetShowingModal(ShowingModal::Resource)),
                    vec![Html::text("リソース")],
                ),
            ],
        )
    }

    fn render_header_row_1_left_userbtn(&self, user: &ChatUser) -> Html {
        Btn::menu(
            Attributes::new().class(if let ChatUser::Player(..) = user {
                Self::class("chatuser-player")
            } else {
                Self::class("chatuser-character")
            }),
            Events::new().on_click(self, {
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

    fn render_header_row_1_right(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("right")),
            Events::new(),
            vec![Dropdown::new(
                self,
                None,
                dropdown::Props {
                    direction: dropdown::Direction::Bottom,
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::Dark,
                },
                Sub::none(),
                (
                    vec![Html::text(if self.is_2d_mode {
                        "2Dモード（正射影）"
                    } else {
                        "3Dモード（透視法）"
                    })],
                    vec![
                        Btn::menu(
                            Attributes::new(),
                            Events::new().on_click(self, |_| Msg::SetIs2dMode(true)),
                            vec![Html::text("2Dモード（正射影）")],
                        ),
                        Btn::menu(
                            Attributes::new(),
                            Events::new().on_click(self, |_| Msg::SetIs2dMode(false)),
                            vec![Html::text("3Dモード（透視法）")],
                        ),
                    ],
                ),
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
                "grid-auto-flow": "column";
                "column-gap": "0.65em";
            }

            ".left" {
                "display": "grid";
                "grid-auto-columns": "max-content";
                "grid-auto-flow": "column";
                "column-gap": "0.65em";
            }

            ".label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            ".chatuser-character" {
                "color": crate::libs::color::Pallet::blue(5);
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
