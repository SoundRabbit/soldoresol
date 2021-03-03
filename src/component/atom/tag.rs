use super::atom::fa;
use super::atom::text;
use super::util::styled::{Style, Styled};
use crate::arena::block::{self, BlockId};
use kagura::prelude::*;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub block_id: BlockId,
    pub removable: bool,
}

pub enum Msg {
    Sub(On),
}

pub enum On {
    Click,
}

pub struct Tag {
    block_arena: block::ArenaRef,
    block_id: BlockId,
    removable: bool,
}

impl Constructor for Tag {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            block_arena: props.block_arena,
            block_id: props.block_id,
            removable: props.removable,
        }
    }
}

impl Component for Tag {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.block_id = props.block_id;
        self.removable = props.removable;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(&self.block_id, |tag: &block::tag::Tag| self.render_tag(tag))
                .unwrap_or(Html::none()),
        )
    }
}

impl Tag {
    fn render_tag(&self, tag: &block::tag::Tag) -> Html {
        Html::span(
            Attributes::new()
                .class("pure-button")
                .class(Self::class("base"))
                .style("background-color", tag.color().to_string()),
            Events::new().on_click(|_| Msg::Sub(On::Click)),
            vec![
                text::span(tag.name()),
                if self.removable {
                    Html::span(
                        Attributes::new()
                            .class(Self::class("btn"))
                            .style("color", tag.color().to_string()),
                        Events::new(),
                        vec![fa::i("fa-times")],
                    )
                } else {
                    Html::none()
                },
            ],
        )
    }
}

impl Styled for Tag {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "align-items": "center";
            }

            "btn" {
                "background-color": format!("{}", crate::libs::color::Pallet::gray(0).a(25));
                "width": "1em";
                "height": "1em";
                "display": "grid";
                "align-items": "center";
                "justify-items": "center";
            }
        }
    }
}
