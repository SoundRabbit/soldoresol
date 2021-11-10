use super::*;
use super::{
    super::atom::attr,
    super::atom::btn::{self, Btn},
    super::atom::dropdown::{self, Dropdown},
    super::atom::fa,
    super::atom::text,
    super::organism::modal_chat_capture::{self, ModalChatCapture},
};

mod message_style;

impl RoomModeless {
    pub fn render_chat_channel(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("channel-base"))
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
                self.render_header(chat_channel),
                self.render_main(chat_channel),
            ],
        )
    }

    fn render_header(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(Self::class("channel-label"))
                        .string("for", &self.element_id.input_channel_name),
                    Events::new(),
                    vec![Html::text("チャンネル名")],
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
            Attributes::new().class(Self::class("channel-main")),
            Events::new(),
            vec![
                self.render_chat_panel(),
                Html::div(
                    Attributes::new().class(Self::class("channel-main-chat")),
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
                            Attributes::new().class(Self::class("channel-main-log")),
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
            Attributes::new().class(Self::class("channel-chatpallet-container")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new()
                        .string("data-is-showing", self.is_showing_chat_pallet.to_string())
                        .class(Self::class("channel-chatpallet")),
                    Events::new(),
                    vec![
                        Dropdown::with_children(
                            dropdown::Props {
                                text: self
                                    .test_chatpallet
                                    .index()
                                    .get(self.test_chatpallet_selected_index)
                                    .map(|(name, _)| name.clone())
                                    .unwrap_or(String::from("")),
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
                            Attributes::new().class(Self::class("channel-chatpallet-index")),
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
                                                    .class(Self::class("channel-chatpallet-item")),
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
            Attributes::new().class(Self::class("channel-main-message")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("channel-main-message-icon")),
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
                    Attributes::new().class(Self::class("channel-main-message-heading")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new()
                                .class(Self::class("channel-main-message-heading-row")),
                            Events::new(),
                            vec![
                                attr::span(
                                    Attributes::new()
                                        .class(Self::class("channel-main-message-sender")),
                                    chat_message.sender().name(),
                                ),
                                attr::span(
                                    Attributes::new()
                                        .class(Self::class("channel-main-message-timestamp")),
                                    chat_message
                                        .timestamp()
                                        .with_timezone(&chrono::Local)
                                        .format("%Y/%m/%d %H:%M:%S")
                                        .to_string(),
                                ),
                            ],
                        ),
                        attr::span(
                            Attributes::new().class(Self::class("channel-main-message-client")),
                            chat_message.sender().client_id().as_ref(),
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Self::class("channel-main-message-content")),
                    Events::new(),
                    self.render_main_chat_content(chat_message.message()),
                ),
            ],
        )
    }

    fn render_main_chat_content(&self, message: &block::chat_message::Message) -> Vec<Html<Self>> {
        message
            .iter()
            .map(|message_token| self.render_main_chat_token(message_token))
            .collect()
    }

    fn render_main_chat_token(
        &self,
        message_token: &block::chat_message::MessageToken,
    ) -> Html<Self> {
        match message_token {
            block::chat_message::MessageToken::Text(text) => Html::text(text),
            block::chat_message::MessageToken::Refer(text) => Html::text(format!("{{{}}}", text)),
            block::chat_message::MessageToken::CommandBlock(cmd, message) => {
                let cmd_name = cmd.name.to_string();
                if cmd_name == "gr" {
                    let mut cols = vec![];
                    for col in &cmd.args {
                        let col = col.to_string();
                        if col == "k" {
                            cols.push(String::from("max-content"));
                        } else {
                            cols.push(String::from("1fr"));
                        }
                    }
                    Html::span(
                        Attributes::new()
                            .string("data-cmd", cmd_name)
                            .style("grid-template-columns", cols.join(" ")),
                        Events::new(),
                        self.render_main_chat_content(message),
                    )
                } else if cmd_name == "block" {
                    let mut cmds: Vec<_> = cmd
                        .args
                        .iter()
                        .map(block::chat_message::Message::to_string)
                        .collect();
                    cmds.push(String::from("block"));
                    let cmds = cmds.join(" ");
                    Html::span(
                        Attributes::new().string("data-cmd", cmds),
                        Events::new(),
                        self.render_main_chat_content(message),
                    )
                } else if cmd_name == "fas" || cmd_name == "far" || cmd_name == "fab" {
                    let args: Vec<_> = cmd
                        .args
                        .iter()
                        .map(block::chat_message::Message::to_string)
                        .collect();
                    let args = args.join(" ");
                    Html::i(
                        Attributes::new().class(cmd_name).class(args),
                        Events::new(),
                        self.render_main_chat_content(message),
                    )
                } else if cmd_name == "rb" {
                    Html::ruby(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            Html::fragment(self.render_main_chat_content(message)),
                            Html::rp(Attributes::new(), Events::new(), vec![Html::text("《")]),
                            Html::rt(
                                Attributes::new(),
                                Events::new(),
                                cmd.args
                                    .iter()
                                    .map(|msg| Html::fragment(self.render_main_chat_content(msg)))
                                    .collect(),
                            ),
                            Html::rp(Attributes::new(), Events::new(), vec![Html::text("》")]),
                        ],
                    )
                } else {
                    Html::span(
                        Attributes::new().string("data-cmd", cmd_name),
                        Events::new(),
                        self.render_main_chat_content(message),
                    )
                }
            }
        }
    }

    fn render_controller(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-footer")),
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
                    Attributes::new().class(Self::class("channel-footer-guide")),
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

    pub fn style_chat_channel() -> Style {
        style! {
            @extends Self::message_style();

            ".channel-label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }

            ".channel-base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "row-gap": ".65rem";
                "padding-top": ".65rem";
                "padding-bottom": ".65rem";
                "height": "100%";
            }

            ".channel-header" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "grid-auto-rows": "max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
            }

            ".channel-main" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "1fr";
                "overflow": "hidden";
                "column-gap": ".35rem";
            }

            ".channel-chatpallet-container" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "grid-template-rows": "1fr";
            }

            ".channel-chatpallet-container > button" {
                "padding-left": ".35em";
                "padding-right": ".35em";
            }

            ".channel-chatpallet" {
                "overflow": "hidden";
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "row-gap": ".65rem";
            }

            ".channel-chatpallet[data-is-showing='false']" {
                "width": "0";
            }

            ".channel-chatpallet[data-is-showing='true']" {
                "min-width": "30ch";
                "max-width": "30ch";
            }

            ".channel-chatpallet-index" {
                "overflow-y": "scroll";
                "display": "grid";
                "grid-auto-rows": "max-content";
                "row-gap": ".65rem";
            }

            ".channel-chatpallet-item" {
                "white-space": "pre-wrap";
            }

            ".channel-main-chat" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr max-content";
                "overflow": "hidden";
            }

            ".channel-main-log" {
                "overflow-y": "scroll";
            }

            ".channel-main-message" {
                "border-top": format!(".65rem solid {}", crate::libs::color::Pallet::gray(3));
                "padding-top": ".35rem";
                "padding-bottom": ".35rem";

                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "max-content max-content";
            }

            ".channel-main-message-icon" {
                "grid-row": "span 2";
                "width": "4.5rem";
                "height": "4.5rem";
                "align-self": "start";
                "line-height": "1.5";
                "font-size": "3rem";
                "text-align": "center";
                "align-self": "start";
            }

            ".channel-main-message-heading-row" {
                "grid-column": "2";
                "display": "flex";
                "justify-content": "space-between";
                "border-bottom": format!(".1rem solid {}", crate::libs::color::Pallet::gray(6));
            }

            ".channel-main-message-sender" {
                "font-size": "1.1em";
            }

            ".channel-main-message-timestamp" {
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".channel-main-message-client" {
                "text-align": "right";
                "font-size": "0.9em";
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".channel-main-message-content" {
                "overflow-x": "hidden";
                "white-space": "pre-wrap";
                "grid-column": "2";
            }

            ".channel-footer" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "height": "10rem";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
            }

            ".channel-footer textarea" {
                "grid-column": "1 / -1";
                "resize": "none";
            }

            ".channel-footer-guide" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "align-items": "center";
            }
        }
    }
}
