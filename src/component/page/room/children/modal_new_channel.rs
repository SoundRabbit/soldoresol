use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::text;
use super::molecule::modal::{self, Modal};
use super::util::styled::{Style, Styled};
use crate::arena::block;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use block::chat::channel::{ChannelPermission, ChannelType};

pub struct Props {
    pub client_id: Rc<String>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetChannelName(String),
    SetChannelType(ChannelType),
    CreateNewChannel,
}

pub enum On {
    Close,
    CreateNewChannel {
        channel_name: String,
        channel_type: ChannelType,
    },
}

pub struct ModalNewChannel {
    client_id: Rc<String>,
    channel_name: String,
    channel_type: ChannelType,
}

impl Constructor for ModalNewChannel {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            channel_type: ChannelType::Public,
            channel_name: String::from("新規チャンネル"),
            client_id: props.client_id,
        }
    }
}

impl Component for ModalNewChannel {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.client_id = props.client_id;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::SetChannelName(channel_name) => {
                self.channel_name = channel_name;
                Cmd::none()
            }
            Msg::SetChannelType(channel_type) => {
                self.channel_type = channel_type;
                Cmd::none()
            }
            Msg::CreateNewChannel => {
                let mut channel_name = String::from("");
                let mut channel_type = ChannelType::Public;
                std::mem::swap(&mut channel_name, &mut self.channel_name);
                std::mem::swap(&mut channel_type, &mut self.channel_type);
                Cmd::sub(On::CreateNewChannel {
                    channel_name,
                    channel_type,
                })
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Modal::with_child(
            modal::Props {
                header_title: String::from("新規チャンネル"),
                footer_message: String::from(""),
            },
            Subscription::new(|sub| match sub {
                modal::On::Close => Msg::Sub(On::Close),
            }),
            Html::div(
                Attributes::new()
                    .class("pure-form")
                    .class(Self::class("base")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("item")),
                        Events::new(),
                        vec![
                            self.render_channel_name_input(),
                            self.render_channel_type_select(),
                            self.render_channel_type_explanation(),
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("centering")),
                        Events::new(),
                        vec![Btn::primary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::CreateNewChannel),
                            vec![Html::text("新規チャンネルを作成")],
                        )],
                    ),
                ],
            ),
        ))
    }
}

impl ModalNewChannel {
    fn render_channel_name_input(&self) -> Vec<Html> {
        vec![
            Html::label(
                Attributes::new(),
                Events::new(),
                vec![Html::text("チャンネル名")],
            ),
            Html::input(
                Attributes::new().value(&self.channel_name),
                Events::new().on_input(Msg::SetChannelName),
                vec![],
            ),
        ]
    }

    fn render_channel_type_select(&self) -> Vec<Html> {
        vec![
            Html::label(Attributes::new(), Events::new(), vec![Html::text("権限")]),
            Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::Bottom,
                    text: match &self.channel_type {
                        ChannelType::Private { .. } => String::from("非公開チャンネル"),
                        ChannelType::Public => String::from("公開チャンネル"),
                    },
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::DarkLikeMenu,
                },
                Subscription::none(),
                vec![
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetChannelType(ChannelType::Public)),
                        vec![Html::text("公開チャンネル")],
                    ),
                    Btn::menu(
                        Attributes::new(),
                        Events::new().on_click({
                            let client_id = Rc::clone(&self.client_id);
                            move |_| {
                                Msg::SetChannelType(ChannelType::Private {
                                    client_id: client_id,
                                    read: ChannelPermission::EveryOne,
                                    write: ChannelPermission::EveryOne,
                                })
                            }
                        }),
                        vec![Html::text("非公開チャンネル")],
                    ),
                ],
            ),
        ]
    }

    fn render_channel_type_explanation(&self) -> Vec<Html> {
        vec![
            Html::div(Attributes::new(), Events::new(), vec![]),
            Html::div(
                Attributes::new().class(Self::class("item")),
                Events::new(),
                vec![
                    vec![
                        text::span("閲覧"),
                        text::span(match &self.channel_type {
                            ChannelType::Private { .. } => "許可されたプレイヤー",
                            ChannelType::Public => "全てのプレイヤー",
                        }),
                    ],
                    vec![
                        text::span("投稿"),
                        text::span(match &self.channel_type {
                            ChannelType::Private { .. } => "許可されたプレイヤー",
                            ChannelType::Public => "全てのプレイヤー",
                        }),
                    ],
                    vec![
                        text::span("権限の変更"),
                        text::span(match &self.channel_type {
                            ChannelType::Private { .. } => "このクライアントのみ可",
                            ChannelType::Public => "不可",
                        }),
                    ],
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
        ]
    }
}

impl Styled for ModalNewChannel {
    fn style() -> Style {
        style! {
            "base" {
                "width": "100%";
                "height": "100%";
                "overflow-y": "scroll";
                "padding": ".65rem";
                "display": "grid";
                "grid-template-rows": "1fr max-content";
                "grid-auto-flow": "row";
                "row-gap": ".65rem";
            }

            "item" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-auto-rows": "max-content";
                "align-items": "center";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
            }

            "centering" {
                "display": "grid";
                "justify-items": "center";
            }
        }
    }
}
