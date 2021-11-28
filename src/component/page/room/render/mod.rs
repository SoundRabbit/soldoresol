use super::super::atom::{
    attr,
    btn::{self, Btn},
    card::{self, Card},
    dropdown::{self, Dropdown},
    file_catcher::{self, FileCatcher},
    header::{self, Header},
    heading::{self, Heading},
};
use super::super::organism::{
    modal_resource::{self, ModalResource},
    table_menu::{self, TableMenu},
    world_view::{self, WorldView},
};
use super::super::template::{
    basic_app::{self, BasicApp},
    common::Common,
};
use super::*;
use crate::arena::{block, Arena, ArenaMut, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Sub;

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
                FileCatcher::with_children(
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
                                    is_debug_mode: false,
                                    arena: ArenaMut::clone(&self.arena),
                                    world: BlockMut::clone(&self.world),
                                },
                                Sub::none(),
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
                                        _ => Msg::NoOp,
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
                                            room_modeless::On::UpdateBlocks { .. } => Msg::NoOp,
                                        },
                                    }),
                                    vec![Html::div(
                                        Attributes::new().class(Self::class("mouse-capture")),
                                        Events::new()
                                            .on_click(Msg::OnTableClicked)
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
                    Html::none()
                },
            ],
        ))
    }
}

impl Room {
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
                    attr::span(
                        Attributes::new()
                            .class(Dropdown::class("menu-heading"))
                            .class(Btn::class_name(&btn::Variant::DarkLikeMenu)),
                        "表示",
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::OpenChatModeless(None)),
                        vec![Html::text("全てのチャンネル")],
                    ),
                    Html::fragment(
                        self.chat
                            .map(|chat: &block::Chat| {
                                chat.channels()
                                    .iter()
                                    .filter_map(|channel| {
                                        let channel_id = channel.id();
                                        channel.map(|channel: &block::ChatChannel| {
                                            Btn::menu(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::OpenChatModeless(Some(channel_id))
                                                }),
                                                vec![Html::text(
                                                    String::from("#") + channel.name(),
                                                )],
                                            )
                                        })
                                    })
                                    .collect()
                            })
                            .unwrap_or(vec![]),
                    ),
                ],
            )],
        )
    }

    fn render_header_row_1_right(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("right")),
            Events::new(),
            vec![Dropdown::with_children(
                dropdown::Props {
                    text: dropdown::Text::Text(if self.is_2d_mode {
                        String::from("2Dモード（正射影）")
                    } else {
                        String::from("3Dモード（透視法）")
                    }),
                    direction: dropdown::Direction::Bottom,
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::Dark,
                },
                Sub::none(),
                vec![
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIs2dMode(true)),
                        vec![Html::text("2Dモード（正射影）")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIs2dMode(false)),
                        vec![Html::text("3Dモード（透視法）")],
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
