use crate::arena::{block, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::JsCast;

mod render_chat_channel;

#[derive(Clone)]
pub struct Content {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
    pub data: ContentData,
}

#[derive(Clone)]
pub enum ContentData {
    ChatChannel(BlockMut),
}

impl ContentData {
    fn id(&self) -> U128Id {
        match self {
            Self::ChatChannel(x) => x.id(),
        }
    }
}

pub enum Msg {
    NoOp,
    SendInputingChatMessage(bool),
    SetInputingChatMessage(String),
    SetIsShowingChatPallet(bool),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModeless {
    is_showing_chat_pallet: bool,
    inputing_chat_channel_name: String,
    inputing_chat_message: Option<String>,
    selecting_content_id: U128Id,
    element_id: ElementId,
}

ElementId! {
    input_channel_name
}

impl Component for RoomModeless {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModeless {
    fn constructor(_: &Content) -> Self {
        Self {
            is_showing_chat_pallet: false,
            inputing_chat_channel_name: String::new(),
            inputing_chat_message: Some(String::new()),
            selecting_content_id: U128Id::none(),
            element_id: ElementId::new(),
        }
    }
}

impl Update for RoomModeless {
    fn on_assemble(&mut self, content: &Content) -> Cmd<Self> {
        self.on_load(content)
    }

    fn on_load(&mut self, content: &Content) -> Cmd<Self> {
        let selecting_content_id = content.data.id();

        if self.selecting_content_id != selecting_content_id {
            match &content.data {
                ContentData::ChatChannel(channel) => {
                    channel.map(|channel: &block::ChatChannel| {
                        self.inputing_chat_channel_name = channel.name().clone();
                    });
                }
            }
        }

        Cmd::none()
    }

    fn update(&mut self, content: &Content, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SendInputingChatMessage(reset) => match &content.data {
                ContentData::ChatChannel(channel) => {
                    let message: String =
                        self.inputing_chat_message.take().unwrap_or(String::new());

                    if !reset {
                        self.inputing_chat_message = Some(String::new());
                    }

                    self.send_chat_message(
                        ArenaMut::clone(&content.arena),
                        BlockMut::clone(channel),
                        String::from("プレイヤー"),
                        &content.client_id,
                        &message,
                    )
                }
            },
            Msg::SetInputingChatMessage(input) => {
                if self.inputing_chat_message.is_some() {
                    self.inputing_chat_message = Some(input);
                } else {
                    self.inputing_chat_message = Some(String::new());
                }
                Cmd::none()
            }
            Msg::SetIsShowingChatPallet(is_showing) => {
                self.is_showing_chat_pallet = is_showing;
                Cmd::none()
            }
        }
    }
}

impl RoomModeless {
    fn send_chat_message(
        &mut self,
        mut arena: ArenaMut,
        mut channel: BlockMut,
        name: String,
        client_id: &Rc<String>,
        message: &String,
    ) -> Cmd<Self> {
        let sender = block::chat_message::Sender::new(Rc::clone(&client_id), None, name);
        let message = block::chat_message::EvalutedMessage::new(
            message,
            |refer| refer,
            |cmd, msg| {
                block::chat_message::EvalutedMessage::from(vec![
                    block::chat_message::EvalutedMessageToken::CommandBlock(cmd, msg),
                ])
            },
        );
        let chat_message = block::ChatMessage::new(sender, chrono::Utc::now(), message);
        let chat_message = arena.insert(chat_message);
        let chat_message_id = chat_message.id();
        channel.update(|channel: &mut block::ChatChannel| {
            channel.messages_push(chat_message);
        });
        let channel_id = channel.id();
        Cmd::sub(On::UpdateBlocks {
            insert: set! { chat_message_id },
            update: set! { channel_id },
        })
    }
}

impl Render for RoomModeless {
    fn render(&self, content: &Content, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(
            match &content.data {
                ContentData::ChatChannel(chat_channel) => {
                    chat_channel.map(|cc: &block::ChatChannel| self.render_chat_channel(cc))
                }
            }
            .unwrap_or(Html::none()),
        )
    }
}

impl Styled for RoomModeless {
    fn style() -> Style {
        style! {
            @extends Self::style_chat_channel();

            ".banner" {
                "grid-column": "1 / -1";
            }
        }
    }
}

pub struct TabName {}

impl Component for TabName {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for TabName {
    fn constructor(_: &Content) -> Self {
        Self {}
    }
}

impl Update for TabName {}

impl Render for TabName {
    fn render(&self, content: &Content, _children: Vec<Html<Self>>) -> Html<Self> {
        match &content.data {
            ContentData::ChatChannel(chat_channel) => chat_channel
                .map(|cc: &block::ChatChannel| Html::text(cc.name()))
                .unwrap_or(Html::none()),
        }
    }
}
