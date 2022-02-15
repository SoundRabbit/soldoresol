use super::atom::attr;
use super::atom::btn::{self, Btn};
use super::atom::chat_message;
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::text;
use super::organism::modal_chat_capture::{self, ModalChatCapture};
use super::organism::modal_chatpallet::{self, ModalChatpallet};
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
mod chat_pallet;
mod controller;
mod send;

#[derive(Clone)]
pub enum ChatUser {
    Player(BlockMut<user::Player>),
    Character(BlockMut<block::Character>),
}

impl PartialEq for ChatUser {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ChatUser::Character(this), ChatUser::Character(other)) => this.id() == other.id(),
            (ChatUser::Player(this), ChatUser::Player(other)) => this.id() == other.id(),
            _ => false,
        }
    }
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

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChatPalletItem {
    Children(usize),
    SubSection(usize, usize),
    Section(usize, ChatPalletSectionItem),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChatPalletSectionItem {
    Children(usize),
    SubSection(usize, usize),
}

pub enum ShowingModal {
    None,
    ChatCapture(WaitingChatMessage),
    Chatpallet,
}

pub enum Msg {
    NoOp,
    SendInputingChatMessage(bool),
    SendWaitingChatMessage(Vec<String>),
    SetShowingModal(ShowingModal),
    SetInputingChatMessage(String),
    SetIsShowingChatPallet(bool),
    SetSelectedChannelIdx(usize),
    SetChatPallet(String),
    SetChatPalletSelectedSection(Option<usize>),
    SetChatPalletSelectedItem(ChatPalletItem),
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
    chat_user: ChatUser,

    is_showing_chat_pallet: bool,
    inputing_chat_message: Option<String>,
    selected_channel_idx: usize,
    showing_modal: ShowingModal,

    element_id: ElementId,

    chatpallet_selected_section: Option<usize>,
    chatpallet_selected_item: Option<ChatPalletItem>,
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
        Self {
            arena: ArenaMut::clone(&props.arena),
            chat: BlockMut::clone(&props.data),
            chat_user: ChatUser::clone(&props.user),

            is_showing_chat_pallet: false,
            inputing_chat_message: Some(String::new()),
            selected_channel_idx: 0,
            showing_modal: ShowingModal::None,

            element_id: ElementId::new(),

            chatpallet_selected_section: None,
            chatpallet_selected_item: None,
        }
    }
}

impl Update for RoomModelessChat {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.arena = ArenaMut::clone(&props.arena);
        self.chat = BlockMut::clone(&props.data);
        self.chat_user = ChatUser::clone(&props.user);

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
                            character.name().clone(),
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
                    self.send_chat_message(props, sender, channel, &message)
                } else {
                    Cmd::none()
                }
            }
            Msg::SendWaitingChatMessage(captured) => self.send_waitng_chat_message(&captured),
            Msg::SetInputingChatMessage(input) => {
                if self.inputing_chat_message.is_some() {
                    self.inputing_chat_message = Some(input);
                    self.chatpallet_selected_item = None;
                } else {
                    self.inputing_chat_message = Some(String::new());
                }
                Cmd::none()
            }
            Msg::SetIsShowingChatPallet(is_showing) => {
                self.is_showing_chat_pallet = is_showing;
                Cmd::none()
            }
            Msg::SetSelectedChannelIdx(idx) => {
                self.selected_channel_idx = idx;
                Cmd::none()
            }

            Msg::SetChatPallet(data) => {
                self.showing_modal = ShowingModal::None;

                if let ChatUser::Character(character) = &mut self.chat_user {
                    character.update(|character| {
                        character.set_chatpallet(data);
                    });

                    Cmd::sub(On::UpdateBlocks {
                        insert: set![],
                        update: set![character.id()],
                    })
                } else {
                    Cmd::none()
                }
            }

            Msg::SetChatPalletSelectedSection(idx) => {
                if self.chatpallet_selected_section != idx {
                    self.chatpallet_selected_section = idx;
                    self.chatpallet_selected_item = None;
                }
                Cmd::none()
            }

            Msg::SetChatPalletSelectedItem(item) => {
                if self
                    .chatpallet_selected_item
                    .map(|x| x == item)
                    .unwrap_or(false)
                {
                    self.chatpallet_selected_item = None;
                    return self.update(props, Msg::SendInputingChatMessage(false));
                }
                if let ChatUser::Character(character) = &props.user {
                    character.map(|character| {
                        if let Some(message) =
                            Self::get_chatpallet_item(character.chatpallet(), &item)
                        {
                            self.chatpallet_selected_item = Some(item);
                            self.inputing_chat_message = Some(message);
                        }
                    });
                }
                Cmd::none()
            }

            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
        }
    }
}

impl Render for RoomModelessChat {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_chat_pallet(props),
                self.chat
                    .map(|chat| self.render_channel_container(chat))
                    .unwrap_or(Html::none()),
                self.render_controller(),
                match &self.showing_modal {
                    ShowingModal::None => Html::none(),
                    ShowingModal::ChatCapture(waiting_chat_message) => ModalChatCapture::empty(
                        modal_chat_capture::Props {
                            vars: Rc::clone(&waiting_chat_message.descriptions),
                        },
                        Sub::map(|sub| match sub {
                            modal_chat_capture::On::Cancel => {
                                Msg::SetShowingModal(ShowingModal::None)
                            }
                            modal_chat_capture::On::Send(x) => Msg::SendWaitingChatMessage(x),
                        }),
                    ),
                    ShowingModal::Chatpallet => {
                        if let ChatUser::Character(character) = &self.chat_user {
                            character
                                .map(|character| {
                                    ModalChatpallet::empty(
                                        modal_chatpallet::Props {
                                            data: character.chatpallet().data().clone(),
                                        },
                                        Sub::map(|sub| match sub {
                                            modal_chatpallet::On::Close => {
                                                Msg::SetShowingModal(ShowingModal::None)
                                            }
                                            modal_chatpallet::On::Ok(data) => {
                                                Msg::SetChatPallet(data)
                                            }
                                        }),
                                    )
                                })
                                .unwrap_or_else(|| Html::none())
                        } else {
                            Html::none()
                        }
                    }
                },
            ],
        ))
    }
}

impl RoomModelessChat {
    fn get_chatpallet_item(
        chatpallet: &block::character::ChatPallet,
        item: &ChatPalletItem,
    ) -> Option<String> {
        match item {
            ChatPalletItem::Children(idx) => chatpallet.children().get(*idx).map(Clone::clone),
            ChatPalletItem::SubSection(s_idx, i_idx) => chatpallet
                .sub_sections()
                .get(*s_idx)
                .and_then(|ss| ss.children().get(*i_idx))
                .map(Clone::clone),
            ChatPalletItem::Section(idx, item) => chatpallet
                .sections()
                .get(*idx)
                .and_then(|s| Self::get_chatpallet_section_item(s, item)),
        }
    }

    fn get_chatpallet_section_item(
        chatpallet: &block::character::ChatPalletSection,
        item: &ChatPalletSectionItem,
    ) -> Option<String> {
        match item {
            ChatPalletSectionItem::Children(idx) => {
                chatpallet.children().get(*idx).map(Clone::clone)
            }
            ChatPalletSectionItem::SubSection(s_idx, i_idx) => chatpallet
                .sub_sections()
                .get(*s_idx)
                .and_then(|ss| ss.children().get(*i_idx))
                .map(Clone::clone),
        }
    }
}

impl Styled for RoomModelessChat {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "padding-top": ".65rem";
                "padding-bottom": ".65rem";
                "overflow": "hidden";
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "1fr max-content";
            }

            // チャパレ

            ".chatpallet-container" {
                "grid-column": "1 / 2";
                "grid-row": "1 / 3";
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
                "grid-template-rows": "max-content 1fr max-content";
                "row-gap": ".65rem";
            }

            ".chatpallet[data-is-showing='false']" {
                "width": "0";
            }

            ".chatpallet[data-is-showing='true']" {
                "min-width": "30ch";
                "max-width": "30ch";
                "padding-left": ".65rem";
            }

            ".chatpallet-section" {
                "overflow-y": "scroll";
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": ".65rem";
            }

            ".chatpallet-item" {
                "white-space": "pre-wrap";
                "font-size": "0.8rem";
            }

            // チャンネル

            ".channel-container" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "grid-column": "2 / 3";
                "grid-row": "1 / 2";
                "overflow": "hidden";
            }

            ".channel" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "overflow": "hidden";
            }

            ".channel-main" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "overflow": "hidden";
            }

            ".channel-log" {
                "height": "100%";
                "overflow-y": "scroll";
            }

            ".channel-message" {
                "border-top": format!(".35rem solid {}", crate::libs::color::Pallet::gray(3));
                "padding-top": ".35rem";
                "padding-bottom": ".35rem";
                "font-size": "0.8rem";
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "max-content max-content";
            }

            ".channel-message-icon" {
                "grid-row": "span 2";
                "width": "4.5rem";
                "height": "4.5rem";
                "align-self": "start";
                "line-height": "1.5";
                "font-size": "3rem";
                "text-align": "center";
                "align-self": "start";
                "object-fit": "cover";
                "object-position": "top";
            }

            ".channel-message-heading-row" {
                "grid-column": "2";
                "display": "flex";
                "justify-content": "space-between";
                "border-bottom": format!(".1rem solid {}", crate::libs::color::Pallet::gray(6));
            }

            ".channel-message-sender" {
                "font-size": "1.1em";
            }

            ".channel-message-timestamp" {
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".channel-message-client" {
                "text-align": "right";
                "font-size": "0.9em";
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".channel-message-content" {
                "overflow": "hidden";
                "white-space": "pre-wrap";
                "grid-column": "2";
            }

            // コントローラ

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
        }
    }
}
