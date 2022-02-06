use super::atom::attr;
use super::atom::btn::{self, Btn};
use super::atom::chat_message;
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::text;
use super::organism::modal_chat_capture::{self, ModalChatCapture};
use super::organism::room_modeless::RoomModeless;
use super::template::common::Common;
use crate::arena::{block, user, ArenaMut, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use wasm_bindgen::JsCast;

mod channel;
mod chat_panel;
mod controller;
mod send;

#[derive(Clone)]
pub enum ChatUser {
    Player(BlockMut<user::Player>),
    Character(BlockMut<block::Character>),
}

pub struct Props {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
    pub data: BlockMut<block::Chat>,
    pub user: ChatUser,
}

pub struct WaitingChatMessage {
    channel: BlockMut<block::ChatChannel>,
    message: block::chat_message::Message,
    descriptions: Rc<Vec<(String, String)>>,
    sender: block::chat_message::Sender,
}

pub enum Msg {
    NoOp,
    SendInputingChatMessage(bool),
    SendWaitingChatMessage(Vec<String>),
    SetWaitingChatMessage(Option<WaitingChatMessage>),
    SetInputingChatMessage(String),
    SetIsShowingChatPallet(bool),

    SetTestChatPalletSelectedIndex(usize),
    SetTestChatPalletSelectedItem(usize),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessChat {
    arena: ArenaMut,
    chat: BlockMut<block::Chat>,

    is_showing_chat_pallet: bool,
    inputing_chat_message: Option<String>,
    waiting_chat_message: Option<WaitingChatMessage>,
    selected_channel_idx: usize,

    element_id: ElementId,

    // test
    test_chatpallet: block::character::ChatPallet,
    test_chatpallet_selected_index: usize,
    test_chatpallet_selected_item: Option<usize>,
}

ElementId! {
    input_channel_name
}

impl Component for RoomModelessChat {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModelessChat {
    fn constructor(props: &Props) -> Self {
        let mut test_chatpallet = block::character::ChatPallet::new();
        test_chatpallet.set_data(String::from(include_str!("./test_chatpallet.txt")));

        Self {
            arena: ArenaMut::clone(&props.arena),
            chat: BlockMut::clone(&props.data),

            is_showing_chat_pallet: false,
            inputing_chat_message: Some(String::new()),
            waiting_chat_message: None,

            selected_channel_idx: 0,

            element_id: ElementId::new(),

            // test
            test_chatpallet,
            test_chatpallet_selected_index: 0,
            test_chatpallet_selected_item: None,
        }
    }
}

impl Update for RoomModelessChat {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.arena = ArenaMut::clone(&props.arena);
        self.chat = BlockMut::clone(&props.data);

        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SendInputingChatMessage(reset) => {
                let message: String = self.inputing_chat_message.take().unwrap_or(String::new());

                if !reset {
                    self.inputing_chat_message = Some(String::new());
                }

                let sender = match &props.user {
                    ChatUser::Player(player) => player.map(|player| {
                        block::chat_message::Sender::new(
                            Rc::clone(&props.client_id),
                            player.icon().map(|icon| BlockRef::clone(icon)),
                            player.name().clone(),
                        )
                    }),
                    ChatUser::Character(character) => character.map(|character| {
                        block::chat_message::Sender::new(
                            Rc::clone(&props.client_id),
                            character.selected_texture().and_then(|texture| {
                                texture.image().map(|image| BlockRef::clone(&image))
                            }),
                            character.display_name().0.clone(),
                        )
                    }),
                };

                let channel = self
                    .chat
                    .map(|chat| {
                        chat.channels()
                            .get(self.selected_channel_idx)
                            .map(BlockMut::clone)
                    })
                    .unwrap_or(None);

                if let Some((sender, channel)) = join_some!(sender, channel) {
                    self.send_chat_message(sender, channel, &message)
                } else {
                    Cmd::none()
                }
            }
            Msg::SendWaitingChatMessage(captured) => self.send_waitng_chat_message(&captured),
            Msg::SetInputingChatMessage(input) => {
                if self.inputing_chat_message.is_some() {
                    self.inputing_chat_message = Some(input);
                    self.test_chatpallet_selected_item = None;
                } else {
                    self.inputing_chat_message = Some(String::new());
                }
                Cmd::none()
            }
            Msg::SetIsShowingChatPallet(is_showing) => {
                self.is_showing_chat_pallet = is_showing;
                Cmd::none()
            }

            Msg::SetTestChatPalletSelectedIndex(idx) => {
                if self.test_chatpallet_selected_index != idx {
                    self.test_chatpallet_selected_index = idx;
                    self.test_chatpallet_selected_item = None;
                }
                Cmd::none()
            }

            Msg::SetTestChatPalletSelectedItem(item) => {
                if self
                    .test_chatpallet_selected_item
                    .map(|x| x == item)
                    .unwrap_or(false)
                {
                    self.test_chatpallet_selected_item = None;
                    self.update(props, Msg::SendInputingChatMessage(false))
                } else {
                    self.test_chatpallet_selected_item = Some(item);
                    self.inputing_chat_message = Some(
                        self.test_chatpallet.index()[self.test_chatpallet_selected_index].1[item]
                            .clone(),
                    );
                    Cmd::none()
                }
            }

            Msg::SetWaitingChatMessage(waiting_chat_message) => {
                self.waiting_chat_message = waiting_chat_message;
                Cmd::none()
            }
        }
    }
}

impl Render for RoomModelessChat {
    fn render(&self, _props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_controller(),
                ModalChatCapture::empty(
                    modal_chat_capture::Props {
                        is_showing: self.waiting_chat_message.is_some(),
                        vars: self
                            .waiting_chat_message
                            .as_ref()
                            .map(|x| Rc::clone(&x.descriptions))
                            .unwrap_or(Rc::new(vec![])),
                    },
                    Sub::map(|sub| match sub {
                        modal_chat_capture::On::Cancel => Msg::SetWaitingChatMessage(None),
                        modal_chat_capture::On::Send(x) => Msg::SendWaitingChatMessage(x),
                    }),
                ),
            ],
        ))
    }
}

impl RoomModelessChat {}

impl Styled for RoomModelessChat {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "padding-top": ".65rem";
                "padding-bottom": ".65rem";
                "overflow": "hidden";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "1fr max-content";
            }

            ".controller" {
                "grid-column": "2 / 3";
                "grid-row": "2 / 3";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "height": "10rem";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
            }

            ".controller textarea" {
                "grid-column": "1 / -1";
                "resize": "none";
            }

            ".controller-guide" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "align-items": "center";
            }

            // ----------

            ".main" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "1fr";
                "overflow": "hidden";
                "column-gap": ".35rem";
            }

            ".chatpallet-container" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "grid-template-rows": "1fr";
            }

            ".chatpallet-container > button" {
                "padding-left": ".35em";
                "padding-right": ".35em";
            }

            ".chatpallet" {
                "overflow": "hidden";
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "row-gap": ".65rem";
            }

            ".chatpallet[data-is-showing='false']" {
                "width": "0";
            }

            ".chatpallet[data-is-showing='true']" {
                "min-width": "30ch";
                "max-width": "30ch";
            }

            ".chatpallet-index" {
                "overflow-y": "scroll";
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": ".65rem";
            }

            ".chatpallet-item" {
                "white-space": "pre-wrap";
            }

            ".main-chat" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr max-content";
                "overflow": "hidden";
            }

            ".main-log" {
                "overflow-y": "scroll";
            }

            ".main-message" {
                "border-top": format!(".35rem solid {}", crate::libs::color::Pallet::gray(3));
                "padding-top": ".35rem";
                "padding-bottom": ".35rem";

                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "max-content max-content";
            }

            ".main-message-icon" {
                "grid-row": "span 2";
                "width": "4.5rem";
                "height": "4.5rem";
                "align-self": "start";
                "line-height": "1.5";
                "font-size": "3rem";
                "text-align": "center";
                "align-self": "start";
            }

            ".main-message-heading-row" {
                "grid-column": "2";
                "display": "flex";
                "justify-content": "space-between";
                "border-bottom": format!(".1rem solid {}", crate::libs::color::Pallet::gray(6));
            }

            ".main-message-sender" {
                "font-size": "1.1em";
            }

            ".main-message-timestamp" {
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".main-message-client" {
                "text-align": "right";
                "font-size": "0.9em";
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".main-message-content" {
                "overflow-x": "hidden";
                "white-space": "pre-wrap";
                "grid-column": "2";
            }

            ".footer" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "height": "10rem";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
            }

            ".footer textarea" {
                "grid-column": "1 / -1";
                "resize": "none";
            }

            ".footer-guide" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "align-items": "center";
            }
        }
    }
}
