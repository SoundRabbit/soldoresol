use super::*;
use super::{super::atom::attr, super::atom::btn::Btn, super::atom::text};

mod message_style;

impl RoomModeless {
    pub fn render_chat_channel(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("channel-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.render_header(chat_channel),
                self.render_main(chat_channel),
                self.render_footer(),
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
                Btn::secondary(
                    Attributes::new().class(Self::class("banner")),
                    Events::new(),
                    vec![Html::text("全チャットログを表示")],
                ),
            ],
        )
    }

    fn render_main(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-main")),
            Events::new(),
            chat_channel
                .messages()
                .iter()
                .filter_map(|cm| cm.map(|cm: &block::ChatMessage| self.render_main_chat(cm)))
                .collect(),
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

    fn render_main_chat_content(
        &self,
        message: &block::chat_message::EvalutedMessage,
    ) -> Vec<Html<Self>> {
        message
            .iter()
            .map(|message_token| self.render_main_chat_token(message_token))
            .collect()
    }

    fn render_main_chat_token(
        &self,
        message_token: &block::chat_message::EvalutedMessageToken,
    ) -> Html<Self> {
        match message_token {
            block::chat_message::EvalutedMessageToken::Text(text) => Html::text(text),
            block::chat_message::EvalutedMessageToken::CommandBlock(cmd, message) => {
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
                        .map(block::chat_message::EvalutedMessage::to_string)
                        .collect();
                    cmds.push(String::from("block"));
                    let cmds = cmds.join(" ");
                    Html::span(
                        Attributes::new().string("data-cmd", cmds),
                        Events::new(),
                        self.render_main_chat_content(message),
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

    fn render_footer(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-footer")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("channel-footer-input")),
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
                                    if let Some(target) = e.target().and_then(|t| {
                                        t.dyn_into::<web_sys::HtmlTextAreaElement>().ok()
                                    }) {
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
                    Btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SendInputingChatMessage(false)),
                        vec![text::span("送"), text::span("信")],
                    ),
                ],
            )],
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
                "grid-template-rows": "max-content 1fr max-content";
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
                "overflow-y": "scroll";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
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
            }

            ".channel-main-message-heading" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "grid-auto-rows": "max-content";
                "grid-column": "2";
            }

            ".channel-main-message-heading-row" {
                "grid-column": "1";
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
                "grid-column": "1";
                "text-align": "right";
                "font-size": "0.9em";
                "font-color": format!("{}", crate::libs::color::Pallet::gray(7));
            }

            ".channel-main-message-content" {
                "grid-column": "2";
                "overflow-x": "hidden";
            }

            ".channel-footer" {
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "height": "7rem";
            }

            ".channel-footer-input" {
                "display": "grid";
                "grid-template-columns": "1fr min-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "height": "100%";
            }

            ".channel-footer-input textarea" {
                "resize": "none";
            }

            ".channel-footer-input button" {
                "display": "flex";
                "flex-direction": "column";
                "justify-content": "center";
            }
        }
    }
}
