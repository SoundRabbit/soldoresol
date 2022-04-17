use super::molecule::tab_menu::{self, TabMenu};
use super::organism::modal_chat_capture::{self, ModalChatCapture};
use super::organism::modal_chatpallet::{self, ModalChatpallet};
use crate::arena::{block, user, ArenaMut, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

mod channel;
mod chat_pallet;
mod controller;
mod send;

use channel::Channel;
use chat_pallet::ChatPallet;
use controller::Controller;

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

pub enum ShowingModal {
    None,
    ChatCapture(WaitingChatMessage),
    Chatpallet,
}

pub enum Msg {
    NoOp,
    SendInputingChatMessage,
    SendWaitingChatMessage(Vec<String>),
    SetShowingModal(ShowingModal),
    SetSelectedChannelIdx(usize),
    SetChatPallet(String),
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
    client_id: Rc<String>,

    selected_channel_idx: usize,
    showing_modal: ShowingModal,

    shared_state: Rc<RefCell<SharedState>>,
}

struct SharedState {
    inputing_message: InputingMessage,
}

impl SharedState {
    fn new() -> Self {
        Self {
            inputing_message: InputingMessage::Text(String::new()),
        }
    }
}

enum InputingMessage {
    ChatPallet {
        text: String,
        index: ChatPalletIndex,
    },
    Text(String),
}

impl InputingMessage {
    fn take(&mut self) -> String {
        let message = self.to_string();
        *self = InputingMessage::Text(String::new());
        message
    }
}

impl std::fmt::Display for InputingMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatPallet { text, .. } => write!(f, "{}", text),
            Self::Text(text) => write!(f, "{}", text),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChatPalletIndex {
    Children(usize),
    SubSection(usize, usize),
    Section(usize, ChatPalletSectionIndex),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChatPalletSectionIndex {
    Children(usize),
    SubSection(usize, usize),
}

impl Component for RoomModelessChat {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomModelessChat {}

impl Constructor for RoomModelessChat {
    fn constructor(props: Props) -> Self {
        Self {
            arena: props.arena,
            chat: props.data,
            chat_user: props.user,
            client_id: props.client_id,

            selected_channel_idx: 0,
            showing_modal: ShowingModal::None,

            shared_state: Rc::new(RefCell::new(SharedState::new())),
        }
    }
}

impl Update for RoomModelessChat {
    fn on_load(self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.chat = props.data;
        self.chat_user = props.user;
        self.client_id = props.client_id;

        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SendInputingChatMessage => {
                let message = self.shared_state.borrow_mut().inputing_message.take();

                let sender = match &self.chat_user {
                    ChatUser::Player(player) => player.map(|player| {
                        block::chat_message::Sender::new(
                            Rc::clone(&self.client_id),
                            player.icon().map(|icon| BlockRef::clone(icon)),
                            player.name().clone(),
                        )
                    }),
                    ChatUser::Character(character) => character.map(|character| {
                        block::chat_message::Sender::new(
                            Rc::clone(&self.client_id),
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
                    self.send_chat_message(sender, channel, &message)
                } else {
                    Cmd::none()
                }
            }
            Msg::SendWaitingChatMessage(captured) => self.send_waitng_chat_message(&captured),
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

                    Cmd::submit(On::UpdateBlocks {
                        insert: set![],
                        update: set![character.id()],
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for RoomModelessChat {
    type Children = ();
    fn render(&self, _: ()) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![
                ChatPallet::empty(
                    self,
                    None,
                    chat_pallet::Props {
                        shared_state: Rc::clone(&self.shared_state),
                        chat_user: ChatUser::clone(&self.chat_user),
                    },
                    Sub::map(|sub| match sub {
                        chat_pallet::On::OpenModal(modal) => Msg::SetShowingModal(modal),
                        chat_pallet::On::SendInputingChatMessage => Msg::SendInputingChatMessage,
                    }),
                ),
                TabMenu::new(
                    self,
                    None,
                    tab_menu::Props {
                        selected: self.selected_channel_idx,
                        controlled: true,
                    },
                    Sub::map(|sub| match sub {
                        tab_menu::On::ChangeSelectedTab(channel_idx) => {
                            Msg::SetSelectedChannelIdx(channel_idx)
                        }
                    }),
                    (
                        Attributes::new().class(Self::class("channel-container")),
                        Events::new(),
                        self.chat
                            .map(|chat| {
                                chat.channels()
                                    .iter()
                                    .map(|channel| {
                                        (
                                            Html::text(
                                                channel
                                                    .map(|channel| format!("# {}", channel.name()))
                                                    .unwrap_or(String::from("# ???")),
                                            ),
                                            Channel::empty(
                                                self,
                                                None,
                                                channel::Props {
                                                    data: BlockMut::clone(&channel),
                                                },
                                                Sub::none(),
                                            ),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    ),
                ),
                Controller::empty(
                    self,
                    None,
                    controller::Props {
                        shared_state: Rc::clone(&self.shared_state),
                    },
                    Sub::map(|sub| match sub {
                        controller::On::SendInputingChatMessage => Msg::SendInputingChatMessage,
                    }),
                ),
                match &self.showing_modal {
                    ShowingModal::None => Html::none(),
                    ShowingModal::ChatCapture(waiting_chat_message) => ModalChatCapture::empty(
                        self,
                        None,
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
                                        self,
                                        None,
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

            ".channel-container" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "grid-column": "2 / 3";
                "grid-row": "1 / 2";
                "overflow": "hidden";
            }
        }
    }
}

fn get_chatpallet_item(
    chatpallet: &block::character::ChatPallet,
    item: &ChatPalletIndex,
) -> Option<String> {
    match item {
        ChatPalletIndex::Children(idx) => chatpallet.children().get(*idx).map(Clone::clone),
        ChatPalletIndex::SubSection(s_idx, i_idx) => chatpallet
            .sub_sections()
            .get(*s_idx)
            .and_then(|ss| ss.children().get(*i_idx))
            .map(Clone::clone),
        ChatPalletIndex::Section(idx, item) => chatpallet
            .sections()
            .get(*idx)
            .and_then(|s| get_chatpallet_section_item(s, item)),
    }
}

fn get_chatpallet_section_item(
    chatpallet: &block::character::ChatPalletSection,
    item: &ChatPalletSectionIndex,
) -> Option<String> {
    match item {
        ChatPalletSectionIndex::Children(idx) => chatpallet.children().get(*idx).map(Clone::clone),
        ChatPalletSectionIndex::SubSection(s_idx, i_idx) => chatpallet
            .sub_sections()
            .get(*s_idx)
            .and_then(|ss| ss.children().get(*i_idx))
            .map(Clone::clone),
    }
}
