use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::tab_btn::{self, TabBtn};
use super::molecule::modeless::{self, Modeless};
use super::util::styled::{Style, Styled};
use super::util::Prop;
use crate::arena::block::{self, BlockId};
use crate::libs::color::color_system;
use crate::libs::random_id::U128Id;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub mod chat_channel;

use chat_channel::ChatChannel;

pub enum Content {
    ChatChannel(BlockId),
}

pub struct Props {
    pub content: Prop<SelectList<Content>>,
    pub z_index: usize,
    pub container_element: Prop<web_sys::Element>,
    pub page_x: i32,
    pub page_y: i32,
    pub block_arena: block::Arena,
}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    Close,
    Focus,
    DragTabStart { tab_idx: usize },
    DropTab { tab_idx: Option<usize> },
    SelectTab { tab_idx: usize },
}

pub struct RoomModeless {
    content: Prop<SelectList<Content>>,
    z_index: usize,
    container_element: Prop<web_sys::Element>,
    page_x: i32,
    page_y: i32,
    block_arena: block::Arena,
}

impl Constructor for RoomModeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            content: props.content,
            z_index: props.z_index,
            container_element: props.container_element,
            page_x: props.page_x,
            page_y: props.page_y,
            block_arena: props.block_arena,
        }
    }
}

impl Component for RoomModeless {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.content = props.content;
        self.z_index = props.z_index;
        self.container_element = props.container_element;
        self.page_x = props.page_x;
        self.page_y = props.page_y;
        self.block_arena = props.block_arena;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => {
                crate::debug::log_1("Msg::Sub");
                Cmd::sub(sub)
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Modeless::with_child(
            modeless::Props {
                z_index: self.z_index,
                container_element: Some(Prop::clone(&self.container_element)),
                page_x: self.page_x,
                page_y: self.page_y,
                ..Default::default()
            },
            Subscription::new(|sub| match sub {
                modeless::On::Focus => Msg::Sub(On::Focus),
            }),
            Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![self.render_header(), self.render_content()],
            ),
        ))
    }
}

impl RoomModeless {
    pub fn tag_id() -> String {
        use std::any;
        any::type_name::<Self>().to_string()
    }

    fn render_header(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("header")),
            Events::new()
                .on("dragover", |e| {
                    e.prevent_default();
                    Msg::NoOp
                })
                .on("drop", |e| {
                    let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                    let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                    if data == Self::tag_id() {
                        e.prevent_default();
                        e.stop_propagation();
                        Msg::Sub(On::DropTab { tab_idx: None })
                    } else {
                        Msg::NoOp
                    }
                }),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("header-tab-container")),
                    Events::new(),
                    self.render_header_tabs(),
                ),
                Btn::with_child(
                    btn::Props {
                        variant: btn::Variant::Secondary,
                    },
                    Subscription::new(|sub| match sub {
                        btn::On::Click => Msg::Sub(On::Close),
                    }),
                    fa::i("fa-times"),
                ),
            ],
        )
    }

    fn render_header_tabs(&self) -> Vec<Html> {
        let mut tabs = self.content
            .iter()
            .enumerate()
            .map(|(tab_idx, a_content)| {
                let is_selected = tab_idx == self.content.selected_idx();
                let tab_heading = match a_content {
                    Content::ChatChannel(channel_id) => vec![
                        fa::i("fa-comment"), 
                        Html::text(self.block_arena.map(channel_id, |channel: &block::chat::channel::Channel| {
                            format!(" {}", channel.name())
                        }).unwrap_or(String::new()))
                    ]
                };
                Html::div(
                    Attributes::new().class(Self::class("header-tab-btn")),
                    Events::new(),
                    vec![TabBtn::with_children(
                        tab_btn::Props {
                            is_selected,
                            data: Self::tag_id(),
                        },
                        Subscription::new(move |sub| match sub {
                            tab_btn::On::Click => Msg::Sub(On::SelectTab { tab_idx }),
                            tab_btn::On::DragStart => Msg::Sub(On::DragTabStart { tab_idx }),
                            tab_btn::On::Drop(e) => {
                                let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                                let data = unwrap_or!(data_transfer.get_data("text/plain").ok(); Msg::NoOp);
                                if data == Self::tag_id() {
                                    e.prevent_default();
                                    e.stop_propagation();
                                    Msg::Sub(On::DropTab { tab_idx: Some(tab_idx) })
                                } else {
                                    Msg::NoOp
                                }
                            },
                        }),
                        tab_heading,
                    )],
                )
            })
            .collect::<Vec<_>>();

        tabs.push(Dropdown::with_children(
            dropdown::Props {
                text: String::from("追加"),
                variant: btn::Variant::TransparentDark,
                ..Default::default()
            },
            Subscription::none(),
            vec![Btn::with_children(
                btn::Props {
                    variant: btn::Variant::Menu,
                },
                Subscription::none(),
                vec![fa::i("fa-chat"), Html::text(" 新規チャンネル")],
            )],
        ));

        tabs
    }

    fn render_content(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("body")),
            Events::new(),
            vec![match self.content.selected() {
                Some(Content::ChatChannel(channel_id)) => ChatChannel::empty(
                    chat_channel::Props {
                        block_arena: block::Arena::clone(&self.block_arena),
                        channel_id: BlockId::clone(channel_id),
                    },
                    Subscription::none(),
                ),
                _ => Html::none(),
            }],
        )
    }
}

impl Styled for RoomModeless {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "align-items": "stretch";
                "width": "100%";
                "height": "100%";
            }

            "header" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "background-color": format!("{}", color_system::gray(255, 8));
            }

            "header-tab-container" {
                "display": "flex";
                "flex-wrap": "wrap";
            }

            "body" {
                "background-color": format!("{}", color_system::gray(255, 0));
            }
        }
    }
}
