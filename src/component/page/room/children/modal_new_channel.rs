use super::molecule::modal::{self, Modal};
use super::util::styled::{Style, Styled};
use super::util::{Prop, State};
use crate::arena::block;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

use block::chat::channel::ChannelType;

pub struct Props {
    pub client_id: Prop<String>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetChannelType(ChannelType),
}

pub enum On {
    Close,
}

pub struct ModalNewChannel {
    client_id: Prop<String>,
    channel_type: ChannelType,
}

impl Constructor for ModalNewChannel {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            channel_type: ChannelType::Public,
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
            Msg::SetChannelType(channel_type) => {
                self.channel_type = channel_type;
                Cmd::none()
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
                    .class(Self::class("base"))
                    .class(Self::class("item")),
                Events::new(),
                vec![
                    vec![
                        Html::label(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("チャンネル名")],
                        ),
                        Html::input(Attributes::new(), Events::new(), vec![]),
                    ],
                    vec![
                        Html::label(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("チャンネルタイプ")],
                        ),
                        Html::select(
                            Attributes::new(),
                            Events::new().on("change", 
                            {
                                let client_id = self.client_id.to_string();
                                move |e| {
                                    let target = unwrap_or!(e.target(); Msg::NoOp);
                                    let target = unwrap_or!(target.dyn_into::<web_sys::HtmlSelectElement>().ok(); Msg::NoOp);
                                    let value = target.value();

                                    if value == "public" {
                                        Msg::SetChannelType(ChannelType::Public)
                                    } else if value == "private" {
                                        Msg::SetChannelType(ChannelType::Private{client_id: client_id})
                                    } else {
                                        Msg::NoOp
                                    }
                                }
                            }),
                            vec![
                                Self::render_select_option(
                                    "public",
                                    "公開チャンネル",
                                ),
                                Self::render_select_option(
                                    "private",
                                    "非公開チャンネル",
                                ),
                            ],
                        ),
                    ],
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
        ))
    }
}

impl ModalNewChannel {
    fn render_select_option(value: impl Into<String>, text: impl Into<String>) -> Html {
        Html::option(
            Attributes::new().value(value),
            Events::new(),
            vec![Html::text(text)],
        )
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
            }

            "item" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-auto-rows": "max-content";
                "align-items": "center";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
            }
        }
    }
}
