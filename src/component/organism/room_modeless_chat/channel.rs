use super::super::atom::{attr, btn::Btn, chat_message};
use crate::arena::{block, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    data: BlockMut<block::ChatChannel>,
}

pub enum Msg {}

pub enum On {}

pub struct Channel {
    data: BlockMut<block::ChatChannel>,
    element_id: ElementId,
}

ElementId! {
    input_channel_name
}

impl Component for Channel {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Channel {}

impl Constructor for Channel {
    fn constructor(props: Self::Props) -> Self {
        Self {
            data: props.data,
            element_id: ElementId::new(),
        }
    }
}

impl Update for Channel {
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.data = props.data;
        Cmd::none()
    }
}

impl Render<Html> for Channel {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(
            self.data
                .map(|channel| {
                    Html::div(
                        Attributes::new().class(Self::class("channel")),
                        Events::new(),
                        vec![self.render_header(channel), self.render_main(channel)],
                    )
                })
                .unwrap_or(Html::none()),
        )
    }
}

impl Channel {
    fn render_header(&self, chat_channel: &block::ChatChannel) -> Html {
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

    fn render_main(&self, chat_channel: &block::ChatChannel) -> Html {
        Html::div(
            Attributes::new().class(Self::class("channel-main")),
            Events::new(),
            vec![
                if chat_channel.messages().len() > 25 {
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
                        .filter_map(|cm| cm.map(|cm: &block::ChatMessage| self.render_message(cm)))
                        .collect(),
                ),
            ],
        )
    }

    fn render_message(&self, chat_message: &block::ChatMessage) -> Html {
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
                                    .draggable("false")
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

impl Styled for Channel {
    fn style() -> Style {
        style! {}
    }
}
