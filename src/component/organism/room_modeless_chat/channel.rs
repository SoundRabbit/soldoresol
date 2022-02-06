use super::*;

impl RoomModelessChat {
    pub fn render_channel(&self, channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-base")),
            Events::new(),
            vec![
                self.render_channel_header(channel),
                self.render_channel_main(channel),
            ],
        )
    }

    fn render_channel_header(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
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
                        .value(chat_channel.name()),
                    Events::new(),
                    vec![],
                ),
            ],
        )
    }

    fn render_channel_main(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
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
                                    cm.map(|cm: &block::ChatMessage| {
                                        self.render_channel_message(cm)
                                    })
                                })
                                .collect(),
                        ),
                    ],
                ),
            ],
        )
    }

    fn render_channel_message(&self, chat_message: &block::ChatMessage) -> Html<Self> {
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
}
