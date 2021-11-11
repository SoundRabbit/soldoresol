use super::super::atom::{
    attr,
    btn::{self, Btn},
    card::{self, Card},
    dropdown::{self, Dropdown},
    header::{self, Header},
    heading::{self, Heading},
};
use super::super::template::basic_app::{self, BasicApp};
use super::*;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Sub;

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
                Html::div(
                    Attributes::new().class(Self::class("body")),
                    Events::new(),
                    vec![self.modeless_container.with_children(
                        tab_modeless_container::Props {},
                        Sub::map(|sub| match sub {
                            tab_modeless_container::On::Sub(..) => Msg::NoOp,
                        }),
                        vec![],
                    )],
                ),
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
                Html::div(
                    Attributes::new().class(Self::class("right")),
                    Events::new(),
                    vec![],
                ),
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
                    text: String::from("チャット"),
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
        }
    }
}
