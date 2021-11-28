use crate::arena::{block, ArenaMut, BlockMut};
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

mod render_boxblock;
mod render_chat_channel;

#[derive(Clone)]
pub struct Content {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
    pub data: ContentData,
}

#[derive(Clone)]
pub enum ContentData {
    ChatChannel(BlockMut<block::ChatChannel>),
    Boxblock(BlockMut<block::Boxblock>),
}

impl ContentData {
    fn id(&self) -> U128Id {
        match self {
            Self::ChatChannel(x) => x.id(),
            Self::Boxblock(x) => x.id(),
        }
    }
}

pub enum Msg {
    NoOp,
    SendInputingChatMessage(bool),
    SendWaitingChatMessage(Vec<String>),
    SetWaitingChatMessage(
        Option<(
            block::chat_message::Message,
            Rc<Vec<(String, String)>>,
            block::chat_message::Sender,
        )>,
    ),
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

pub struct RoomModeless {
    selecting_content_id: U128Id,
    element_id: ElementId,

    // chat
    is_showing_chat_pallet: bool,
    inputing_chat_channel_name: String,
    inputing_chat_message: Option<String>,
    waiting_chat_message: Option<(
        block::chat_message::Message,
        Rc<Vec<(String, String)>>,
        block::chat_message::Sender,
    )>,

    //boxblock
    inputing_boxblock_name: String,

    // test
    test_chatpallet: block::character::ChatPallet,
    test_chatpallet_selected_index: usize,
    test_chatpallet_selected_item: Option<usize>,
}

ElementId! {
    input_channel_name,
    input_boxblock_name
}

impl Component for RoomModeless {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModeless {
    fn constructor(_: &Content) -> Self {
        let mut test_chatpallet = block::character::ChatPallet::new();

        test_chatpallet.data_set(String::from(include_str!("./test_chatpallet.txt")));

        Self {
            selecting_content_id: U128Id::none(),
            element_id: ElementId::new(),

            // chat
            is_showing_chat_pallet: false,
            inputing_chat_channel_name: String::new(),
            inputing_chat_message: Some(String::new()),
            waiting_chat_message: None,

            //boxblock
            inputing_boxblock_name: String::new(),

            // test
            test_chatpallet,
            test_chatpallet_selected_index: 0,
            test_chatpallet_selected_item: None,
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
                ContentData::Boxblock(boxblock) => {
                    boxblock.map(|boxblock| {
                        self.inputing_boxblock_name = boxblock.name().clone();
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
                _ => Cmd::none(),
            },
            Msg::SendWaitingChatMessage(captured) => match &content.data {
                ContentData::ChatChannel(channel) => self.send_waitng_chat_message(
                    ArenaMut::clone(&content.arena),
                    BlockMut::clone(channel),
                    captured,
                ),
                _ => Cmd::none(),
            },
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
                    self.update(content, Msg::SendInputingChatMessage(false))
                } else {
                    self.test_chatpallet_selected_item = Some(item);
                    self.inputing_chat_message = Some(
                        self.test_chatpallet.index()[self.test_chatpallet_selected_index].1[item]
                            .clone(),
                    );
                    Cmd::none()
                }
            }

            Msg::SetWaitingChatMessage(wcm) => {
                self.waiting_chat_message = wcm;
                Cmd::none()
            }
        }
    }
}

impl RoomModeless {
    fn send_chat_message(
        &mut self,
        mut arena: ArenaMut,
        mut channel: BlockMut<block::ChatChannel>,
        name: String,
        client_id: &Rc<String>,
        message: &String,
    ) -> Cmd<Self> {
        let sender = block::chat_message::Sender::new(Rc::clone(&client_id), None, name);

        let message = block::chat_message::Message::new(message);

        let descriptions = Rc::new(RefCell::new(vec![]));
        let message = message.map(Self::map_message_token(&descriptions));
        let descriptions: Vec<_> = descriptions.borrow_mut().drain(..).collect();

        if descriptions.len() > 0 {
            self.waiting_chat_message = Some((message, Rc::new(descriptions), sender));
            return Cmd::none();
        }

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

    fn send_waitng_chat_message(
        &mut self,
        mut arena: ArenaMut,
        mut channel: BlockMut<block::ChatChannel>,
        captured: Vec<String>,
    ) -> Cmd<Self> {
        if let Some((message, _, sender)) = self.waiting_chat_message.take() {
            let message = message.map(Self::capture_message_token(captured));
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
        } else {
            Cmd::none()
        }
    }

    fn map_message_token(
        descriptions: &Rc<RefCell<Vec<(String, String)>>>,
    ) -> impl Fn(block::chat_message::MapToken) -> block::chat_message::Message + 'static {
        let var_nums: HashMap<String, Vec<usize>> = HashMap::new();
        let var_nums = Rc::new(RefCell::new(var_nums));
        let descriptions = Rc::clone(&descriptions);

        move |token| match token {
            block::chat_message::MapToken::Text(text) => {
                block::chat_message::Message::from(vec![block::chat_message::MessageToken::Text(
                    text,
                )])
            }
            block::chat_message::MapToken::Refer(refer) => block::chat_message::Message::from(
                vec![block::chat_message::MessageToken::Refer(refer.get())],
            ),
            block::chat_message::MapToken::CommandBlock(cmd, text) => {
                let cmd_name = cmd.name.get();
                let cmd_args: Vec<_> = cmd.args.into_iter().map(|x| x.get()).collect();

                if cmd_name.to_string() == "capture" {
                    let mut cap_names = vec![];

                    for args in cmd_args {
                        let args: Vec<_> = args.into();
                        for arg in args {
                            if let block::chat_message::MessageToken::CommandBlock(cap, desc) = arg
                            {
                                for cap_name in cap.args {
                                    let cap_name = cap_name.to_string();
                                    descriptions
                                        .borrow_mut()
                                        .push((cap.name.to_string(), desc.to_string()));
                                    let num = descriptions.borrow().len();
                                    let mut var_nums = var_nums.borrow_mut();
                                    if let Some(vars) = var_nums.get_mut(&cap_name) {
                                        vars.push(num);
                                    } else {
                                        var_nums.insert(cap_name.clone(), vec![num]);
                                    }
                                    cap_names.push(cap_name);
                                }
                            }
                        }
                    }

                    let text = text.get();

                    for cap_name in cap_names {
                        if let Some(vars) = var_nums.borrow_mut().get_mut(&cap_name) {
                            vars.pop();
                        }
                    }

                    text
                } else if cmd_name.to_string() == "ref" {
                    let cap_name = text.get().to_string();
                    let text = if let Some(num) =
                        var_nums.borrow().get(&cap_name).and_then(|x| x.last())
                    {
                        block::chat_message::Message::from(vec![
                            block::chat_message::MessageToken::Text(num.to_string()),
                        ])
                    } else {
                        block::chat_message::Message::from(vec![
                            block::chat_message::MessageToken::Text(cap_name),
                        ])
                    };
                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::CommandBlock(
                            block::chat_message::MessageCommand {
                                name: cmd_name,
                                args: cmd_args,
                            },
                            text,
                        ),
                    ])
                } else {
                    let text = text.get();
                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::CommandBlock(
                            block::chat_message::MessageCommand {
                                name: cmd_name,
                                args: cmd_args,
                            },
                            text,
                        ),
                    ])
                }
            }
        }
    }

    fn capture_message_token(
        captured: Vec<String>,
    ) -> impl Fn(block::chat_message::MapToken) -> block::chat_message::Message + 'static {
        move |token| match token {
            block::chat_message::MapToken::Text(text) => {
                block::chat_message::Message::from(vec![block::chat_message::MessageToken::Text(
                    text,
                )])
            }
            block::chat_message::MapToken::Refer(refer) => block::chat_message::Message::from(
                vec![block::chat_message::MessageToken::Refer(refer.get())],
            ),
            block::chat_message::MapToken::CommandBlock(cmd, text) => {
                let cmd_name = cmd.name.get();
                let cmd_args: Vec<_> = cmd.args.into_iter().map(|x| x.get()).collect();

                if cmd_name.to_string() == "ref" {
                    let cap_name = text.get().to_string();
                    let text = cap_name
                        .parse()
                        .ok()
                        .and_then(|x: usize| captured.get(x - 1).map(|x: &String| x.clone()))
                        .unwrap_or(String::from(""));

                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::Text(text),
                    ])
                } else {
                    let text = text.get();
                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::CommandBlock(
                            block::chat_message::MessageCommand {
                                name: cmd_name,
                                args: cmd_args,
                            },
                            text,
                        ),
                    ])
                }
            }
        }
    }
}

impl Render for RoomModeless {
    fn render(&self, content: &Content, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(
            match &content.data {
                ContentData::ChatChannel(chat_channel) => {
                    chat_channel.map(|cc| self.render_chat_channel(cc))
                }
                ContentData::Boxblock(boxblock) => boxblock.map(|bb| self.render_boxblock(bb)),
            }
            .unwrap_or(Html::none()),
        )
    }
}

impl Styled for RoomModeless {
    fn style() -> Style {
        style! {
            @extends Self::style_chat_channel();
            @extends Self::style_boxblock();

            ".banner" {
                "grid-column": "1 / -1";
            }

            ".common-label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            ".common-base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "row-gap": ".65rem";
                "padding-top": ".65rem";
                "padding-bottom": ".65rem";
                "height": "100%";
            }

            ".common-header" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "grid-auto-rows": "max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
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
        use super::atom::fa;
        match &content.data {
            ContentData::ChatChannel(chat_channel) => chat_channel
                .map(|cc| Html::text(String::from("#") + cc.name()))
                .unwrap_or(Html::none()),
            ContentData::Boxblock(boxblock) => boxblock
                .map(|bb| {
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![fa::i("fa-cube"), Html::text(bb.name())],
                    )
                })
                .unwrap_or(Html::none()),
        }
    }
}
