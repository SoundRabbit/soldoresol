use super::super::molecule::tab_menu::{self, TabMenu};
use super::*;

impl RoomModelessChat {
    pub fn render_channel_container(&self, chat: &block::Chat) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-container")),
            Events::new(),
            vec![TabMenu::with_children(
                tab_menu::Props {
                    selected: self.selected_channel_idx,
                    tabs: chat
                        .channels()
                        .iter()
                        .map(|channel| {
                            channel
                                .map(|channel| format!("# {}", channel.name()))
                                .unwrap_or(String::from("# ???"))
                        })
                        .collect(),
                    controlled: true,
                },
                Sub::map(|sub| match sub {
                    tab_menu::On::ChangeSelectedTab(tab_idx) => Msg::SetSelectedChannelIdx(tab_idx),
                }),
                chat.channels()
                    .iter()
                    .enumerate()
                    .map(|(channel_idx, channel)| {
                        if channel_idx == self.selected_channel_idx {
                            channel
                                .map(|channel| self.render_channel(channel))
                                .unwrap_or(Html::none())
                        } else {
                            Html::none()
                        }
                    })
                    .collect(),
            )],
        )
    }

    fn render_channel(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel")),
            Events::new(),
            vec![
                self.render_channel_header(chat_channel),
                self.render_channel_main(chat_channel),
            ],
        )
    }

    fn render_channel_header(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![Html::input(
                Attributes::new()
                    .id(&self.element_id.input_channel_name)
                    .value(chat_channel.name()),
                Events::new(),
                vec![],
            )],
        )
    }

    fn render_channel_main(&self, chat_channel: &block::ChatChannel) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-main")),
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
                    Attributes::new().class(Self::class("channel-log")),
                    Events::new(),
                    chat_channel
                        .messages()
                        .iter()
                        .rev()
                        .take(50)
                        .rev()
                        .filter_map(|cm| {
                            cm.map(|cm: &block::ChatMessage| self.render_channel_message(cm))
                        })
                        .collect(),
                ),
            ],
        )
    }

    fn render_channel_message(&self, chat_message: &block::ChatMessage) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("channel-message")),
            Events::new(),
            vec![
                chat_message
                    .sender()
                    .icon()
                    .and_then(|icon| {
                        icon.map(|icon| {
                            Html::img(
                                Attributes::new()
                                    .draggable(false)
                                    .class(Self::class("channel-message-icon"))
                                    .src(icon.url().to_string()),
                                Events::new(),
                                vec![],
                            )
                        })
                    })
                    .unwrap_or_else(|| {
                        Html::div(
                            Attributes::new().class(Self::class("channel-message-icon")),
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
                        )
                    }),
                Html::div(
                    Attributes::new().class(Self::class("channel-message-heading")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("channel-message-heading-row")),
                            Events::new(),
                            vec![
                                attr::span(
                                    Attributes::new().class(Self::class("channel-message-sender")),
                                    chat_message.sender().name(),
                                ),
                                attr::span(
                                    Attributes::new()
                                        .class(Self::class("channel-message-timestamp")),
                                    chat_message
                                        .timestamp()
                                        .with_timezone(&chrono::Local)
                                        .format("%Y/%m/%d %H:%M:%S")
                                        .to_string(),
                                ),
                            ],
                        ),
                        attr::span(
                            Attributes::new().class(Self::class("channel-message-client")),
                            chat_message.sender().client_id().as_ref(),
                        ),
                    ],
                ),
                chat_message::div(
                    Attributes::new().class(Self::class("channel-message-content")),
                    Events::new(),
                    chat_message.message(),
                ),
            ],
        )
    }
}
