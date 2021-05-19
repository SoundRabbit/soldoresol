use super::super::atom::btn::{self, Btn};
use super::super::util::styled::{Style, Styled};
use crate::arena::block;
use async_std::sync::{Arc, Mutex};
use kagura::prelude::*;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub channel_id: block::BlockId,
}

pub enum Msg {}

pub enum On {}

pub struct ChatChannel {
    block_arena: block::ArenaRef,
    channel_id: block::BlockId,
    element_id: ElementId,
}

struct ElementId {
    input_channel_name: String,
}

impl Constructor for ChatChannel {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self {
        Self {
            block_arena: props.block_arena,
            channel_id: props.channel_id,
            element_id: ElementId {
                input_channel_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
        }
    }
}

impl Component for ChatChannel {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.channel_id = props.channel_id;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(
                    &self.channel_id,
                    |channel: &block::chat::channel::Channel| self.render_channel(channel),
                )
                .unwrap_or(Html::none()),
        )
    }
}

impl ChatChannel {
    fn render_channel(&self, channel: &block::chat::channel::Channel) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![Html::fragment(vec![
                Html::label(
                    Attributes::new()
                        .class(Self::class("label"))
                        .string("for", &self.element_id.input_channel_name),
                    Events::new(),
                    vec![Html::text("チャンネル名")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_channel_name)
                        .value(channel.name()),
                    Events::new(),
                    vec![],
                ),
                Btn::primary(Attributes::new(), Events::new(), vec![Html::text("更新")]),
            ])],
        )
    }
}

impl Styled for ChatChannel {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "column";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }
            "label" {
                "display": "grid";
                "align-items": "center";
                "line-height": "1";
            }
        }
    }
}
