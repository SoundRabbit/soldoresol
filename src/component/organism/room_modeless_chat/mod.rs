use super::atom::attr;
use super::atom::btn::{self, Btn};
use super::atom::chat_message;
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::text;
use super::organism::modal_chat_capture::{self, ModalChatCapture};
use super::organism::room_modeless::RoomModeless;
use super::template::common::Common;
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

pub struct Props {
    pub arena: ArenaMut,
    pub client_id: Rc<String>,
    pub data: BlockMut<block::ChatChannel>,
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

pub struct RoomModelessChat {
    arena: ArenaMut,
    channel: BlockMut<block::ChatChannel>,

    is_showing_chat_pallet: bool,
    inputing_chat_channel_name: String,
    inputing_chat_message: Option<String>,
    waiting_chat_message: Option<(
        block::chat_message::Message,
        Rc<Vec<(String, String)>>,
        block::chat_message::Sender,
    )>,

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
        test_chatpallet.data_set(String::from(include_str!("./test_chatpallet.txt")));

        Self {
            arena: ArenaMut::clone(&props.arena),
            channel: BlockMut::clone(&props.data),

            is_showing_chat_pallet: false,
            inputing_chat_channel_name: props
                .data
                .map(|data| data.name().clone())
                .unwrap_or(String::new()),
            inputing_chat_message: Some(String::new()),
            waiting_chat_message: None,

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
        if self.channel.id() != props.data.id() {
            props.data.map(|channel| {
                self.inputing_chat_channel_name = channel.name().clone();
            });
        }

        self.arena = ArenaMut::clone(&props.arena);
        self.channel = BlockMut::clone(&props.data);

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

                self.send_chat_message(String::from("プレイヤー"), &props.client_id, &message)
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

impl RoomModelessChat {
    fn send_chat_message(
        &mut self,
        name: String,
        client_id: &Rc<String>,
        message: &String,
    ) -> Cmd<Self> {
        let sender = block::chat_message::Sender::new(Rc::clone(&client_id), None, name);

        let message = block::chat_message::Message::new(message);

        let mut descriptions = vec![];
        let mut var_nums = HashMap::new();
        let message = Self::map_message(&mut var_nums, &mut descriptions, message);

        if descriptions.len() > 0 {
            self.waiting_chat_message = Some((message, Rc::new(descriptions), sender));
            return Cmd::none();
        }

        let chat_message = block::ChatMessage::new(sender, chrono::Utc::now(), message);
        let chat_message = self.arena.insert(chat_message);
        let chat_message_id = chat_message.id();
        self.channel.update(|channel: &mut block::ChatChannel| {
            channel.messages_push(chat_message);
        });
        let channel_id = self.channel.id();
        Cmd::sub(On::UpdateBlocks {
            insert: set! { chat_message_id },
            update: set! { channel_id },
        })
    }

    fn send_waitng_chat_message(&mut self, captured: &Vec<String>) -> Cmd<Self> {
        if let Some((message, _, sender)) = self.waiting_chat_message.take() {
            let message = Self::capture_message(&captured, message);
            let chat_message = block::ChatMessage::new(sender, chrono::Utc::now(), message);
            let chat_message = self.arena.insert(chat_message);
            let chat_message_id = chat_message.id();
            self.channel.update(|channel: &mut block::ChatChannel| {
                channel.messages_push(chat_message);
            });
            let channel_id = self.channel.id();
            Cmd::sub(On::UpdateBlocks {
                insert: set! { chat_message_id },
                update: set! { channel_id },
            })
        } else {
            Cmd::none()
        }
    }

    fn map_message(
        var_nums: &mut HashMap<String, Vec<usize>>,
        descriptions: &mut Vec<(String, String)>,
        message: block::chat_message::Message,
    ) -> block::chat_message::Message {
        message.map(|token| Self::map_message_token(var_nums, descriptions, token))
    }

    fn map_message_token(
        var_nums: &mut HashMap<String, Vec<usize>>,
        descriptions: &mut Vec<(String, String)>,
        token: block::chat_message::MessageToken,
    ) -> block::chat_message::Message {
        match token {
            block::chat_message::MessageToken::Text(text) => block::chat_message::Message::from(
                vec![block::chat_message::MessageToken::Text(text)],
            ),
            block::chat_message::MessageToken::Refer(refer) => {
                block::chat_message::Message::from(vec![block::chat_message::MessageToken::Refer(
                    Self::map_message(var_nums, descriptions, refer),
                )])
            }
            block::chat_message::MessageToken::CommandBlock(cmd, text) => {
                let cmd_name = Self::map_message(var_nums, descriptions, cmd.name);
                let cmd_args: Vec<_> = cmd
                    .args
                    .into_iter()
                    .map(|x| Self::map_message(var_nums, descriptions, x))
                    .collect();

                if cmd_name.to_string() == "capture" {
                    let mut cap_names = vec![];

                    for args in cmd_args {
                        let args: Vec<_> = args.into();
                        for arg in args {
                            if let block::chat_message::MessageToken::CommandBlock(cap, desc) = arg
                            {
                                for cap_name in cap.args {
                                    let cap_name = cap_name.to_string();
                                    descriptions.push((cap.name.to_string(), desc.to_string()));
                                    let num = descriptions.len();
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

                    let text = Self::map_message(var_nums, descriptions, text);

                    for cap_name in cap_names {
                        if let Some(vars) = var_nums.get_mut(&cap_name) {
                            vars.pop();
                        }
                    }

                    text
                } else if cmd_name.to_string() == "ref" {
                    let cap_name = Self::map_message(var_nums, descriptions, text).to_string();
                    let text = if let Some(num) = var_nums.get(&cap_name).and_then(|x| x.last()) {
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
                    let text = Self::map_message(var_nums, descriptions, text);
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

    fn capture_message(
        captured: &Vec<String>,
        message: block::chat_message::Message,
    ) -> block::chat_message::Message {
        message.map(|token| Self::capture_message_token(captured, token))
    }

    fn capture_message_token(
        captured: &Vec<String>,
        token: block::chat_message::MessageToken,
    ) -> block::chat_message::Message {
        match token {
            block::chat_message::MessageToken::Text(text) => block::chat_message::Message::from(
                vec![block::chat_message::MessageToken::Text(text)],
            ),
            block::chat_message::MessageToken::Refer(refer) => {
                block::chat_message::Message::from(vec![block::chat_message::MessageToken::Refer(
                    Self::capture_message(captured, refer),
                )])
            }
            block::chat_message::MessageToken::CommandBlock(cmd, text) => {
                let cmd_name = Self::capture_message(captured, cmd.name);
                let cmd_args: Vec<_> = cmd
                    .args
                    .into_iter()
                    .map(|x| Self::capture_message(captured, x))
                    .collect();

                if cmd_name.to_string() == "ref" {
                    let cap_name = Self::capture_message(captured, text).to_string();
                    let text = cap_name
                        .parse()
                        .ok()
                        .and_then(|x: usize| captured.get(x - 1).map(|x: &String| x.clone()))
                        .unwrap_or(String::from(""));

                    block::chat_message::Message::from(vec![
                        block::chat_message::MessageToken::Text(text),
                    ])
                } else {
                    let text = Self::capture_message(captured, text);
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

impl Render for RoomModelessChat {
    fn render(&self, _props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                ModalChatCapture::empty(
                    modal_chat_capture::Props {
                        is_showing: self.waiting_chat_message.is_some(),
                        vars: self
                            .waiting_chat_message
                            .as_ref()
                            .map(|x| Rc::clone(&x.1))
                            .unwrap_or(Rc::new(vec![])),
                    },
                    Sub::map(|sub| match sub {
                        modal_chat_capture::On::Cancel => Msg::SetWaitingChatMessage(None),
                        modal_chat_capture::On::Send(x) => Msg::SendWaitingChatMessage(x),
                    }),
                ),
                self.channel
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.channel
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
            ],
        ))
    }
}

impl RoomModelessChat {
    fn render_header(&self, _chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_channel_name),
                    Events::new(),
                    vec![Html::text("#")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_channel_name)
                        .value(&self.inputing_chat_channel_name),
                    Events::new(),
                    vec![],
                ),
                Btn::primary(Attributes::new(), Events::new(), vec![Html::text("更新")]),
            ],
        )
    }

    fn render_main(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            vec![
                self.render_chat_panel(),
                Html::div(
                    Attributes::new().class(Self::class("main-chat")),
                    Events::new(),
                    vec![
                        if chat_channel.messages().len() > 50 {
                            Btn::secondary(
                                Attributes::new().class(Self::class("banner")),
                                Events::new(),
                                vec![Html::text("全チャットログを表示")],
                            )
                        } else {
                            Html::div(Attributes::new(), Events::new(), vec![])
                        },
                        Html::div(
                            Attributes::new().class(Self::class("main-log")),
                            Events::new(),
                            chat_channel
                                .messages()
                                .iter()
                                .rev()
                                .take(50)
                                .rev()
                                .filter_map(|cm| {
                                    cm.map(|cm: &block::ChatMessage| self.render_main_chat(cm))
                                })
                                .collect(),
                        ),
                        self.render_controller(),
                    ],
                ),
            ],
        )
    }

    fn render_chat_panel(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("chatpallet-container")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new()
                        .string("data-is-showing", self.is_showing_chat_pallet.to_string())
                        .class(Self::class("chatpallet")),
                    Events::new(),
                    vec![
                        Dropdown::with_children(
                            dropdown::Props {
                                text: dropdown::Text::Text(
                                    self.test_chatpallet
                                        .index()
                                        .get(self.test_chatpallet_selected_index)
                                        .map(|(name, _)| name.clone())
                                        .unwrap_or(String::from("")),
                                ),
                                direction: dropdown::Direction::Bottom,
                                toggle_type: dropdown::ToggleType::Click,
                                variant: btn::Variant::DarkLikeMenu,
                            },
                            Sub::none(),
                            self.test_chatpallet
                                .index()
                                .iter()
                                .enumerate()
                                .map(|(idx, (name, _))| {
                                    Btn::menu(
                                        Attributes::new(),
                                        Events::new().on_click(move |_| {
                                            Msg::SetTestChatPalletSelectedIndex(idx)
                                        }),
                                        vec![Html::text(name)],
                                    )
                                })
                                .collect(),
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("chatpallet-index")),
                            Events::new(),
                            self.test_chatpallet
                                .index()
                                .get(self.test_chatpallet_selected_index)
                                .map(|(_, items)| {
                                    items
                                        .iter()
                                        .enumerate()
                                        .map(|(idx, item)| {
                                            Btn::with_variant(
                                                btn::Variant::LightLikeMenu,
                                                Attributes::new()
                                                    .class(Self::class("chatpallet-item")),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTestChatPalletSelectedItem(idx)
                                                }),
                                                vec![Html::text(item)],
                                            )
                                        })
                                        .collect()
                                })
                                .unwrap_or(vec![]),
                        ),
                    ],
                ),
                Btn::light(
                    Attributes::new().title(if self.is_showing_chat_pallet {
                        "チャットパレットをしまう"
                    } else {
                        "チャットパレットを表示"
                    }),
                    Events::new().on_click({
                        let is_showing = self.is_showing_chat_pallet;
                        move |_| Msg::SetIsShowingChatPallet(!is_showing)
                    }),
                    vec![fa::i(if self.is_showing_chat_pallet {
                        "fa-caret-left"
                    } else {
                        "fa-caret-right"
                    })],
                ),
            ],
        )
    }

    fn render_main_chat(&self, chat_message: &block::ChatMessage) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("main-message")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("main-message-icon")),
                    Events::new(),
                    vec![Html::text(
                        chat_message
                            .sender()
                            .name()
                            .chars()
                            .nth(0)
                            .map(|x| String::from(x))
                            .unwrap_or(String::from("")),
                    )],
                ),
                Html::div(
                    Attributes::new().class(Self::class("main-message-heading")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("main-message-heading-row")),
                            Events::new(),
                            vec![
                                attr::span(
                                    Attributes::new().class(Self::class("main-message-sender")),
                                    chat_message.sender().name(),
                                ),
                                attr::span(
                                    Attributes::new().class(Self::class("main-message-timestamp")),
                                    chat_message
                                        .timestamp()
                                        .with_timezone(&chrono::Local)
                                        .format("%Y/%m/%d %H:%M:%S")
                                        .to_string(),
                                ),
                            ],
                        ),
                        attr::span(
                            Attributes::new().class(Self::class("main-message-client")),
                            chat_message.sender().client_id().as_ref(),
                        ),
                    ],
                ),
                chat_message::div(
                    Attributes::new().class(Self::class("main-message-content")),
                    Events::new(),
                    chat_message.message(),
                ),
            ],
        )
    }

    fn render_controller(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("footer")),
            Events::new(),
            vec![
                Html::textarea(
                    Attributes::new().value(
                        self.inputing_chat_message
                            .as_ref()
                            .map(String::as_str)
                            .unwrap_or(""),
                    ),
                    Events::new()
                        .on("input", {
                            let ignore_intput = self.inputing_chat_message.is_none();
                            move |e| {
                                if let Some(target) = e
                                    .target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
                                {
                                    if ignore_intput {
                                        target.set_value("");
                                    }
                                    Msg::SetInputingChatMessage(target.value())
                                } else {
                                    Msg::NoOp
                                }
                            }
                        })
                        .on_keydown(|e| {
                            if e.key() == "Enter" && !e.shift_key() {
                                Msg::SendInputingChatMessage(true)
                            } else {
                                Msg::NoOp
                            }
                        }),
                    vec![],
                ),
                Html::div(
                    Attributes::new().class(Self::class("footer-guide")),
                    Events::new(),
                    vec![
                        text::span("Shift＋Enterで改行できます。"),
                        Btn::primary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::SendInputingChatMessage(false)),
                            vec![Html::text("送信")],
                        ),
                    ],
                ),
            ],
        )
    }
}

impl Styled for RoomModelessChat {
    fn style() -> Style {
        style! {
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
                "border-top": format!(".65rem solid {}", crate::libs::color::Pallet::gray(3));
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
