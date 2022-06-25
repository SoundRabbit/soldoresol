use super::super::atom::{common::Common, marker::Marker, text::Text};
use crate::arena::{block, component, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub selecting: U128Id,
}

pub enum Msg {}

pub enum On {}

pub struct BoxblockList {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    selecting: U128Id,
}

impl Component for BoxblockList {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for BoxblockList {}

impl Constructor for BoxblockList {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            world: props.world,
            selecting: props.selecting,
        }
    }
}

impl Update for BoxblockList {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.world = props.world;
        self.selecting = props.selecting;

        Cmd::none()
    }
}

impl Render<Html> for BoxblockList {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Self::styled(Html::fragment(
            self.world
                .map(|world| {
                    world
                        .components()
                        .boxblocks()
                        .iter()
                        .map(|boxblock| self.render_boxblock(boxblock))
                        .collect()
                })
                .unwrap_or(vec![]),
        ))
    }
}

impl BoxblockList {
    fn render_boxblock(&self, boxblock: &BlockMut<component::BoxblockComponent>) -> Html {
        let boxblock_id = boxblock.id();
        let is_selected = boxblock_id == self.selecting;
        boxblock
            .map(|boxblock| {
                Html::div(
                    Attributes::new().class(Self::class("item")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Common::keyvalue()),
                            Events::new(),
                            vec![
                                Self::render_marker(is_selected, Attributes::new(), Events::new()),
                                Html::input(
                                    Attributes::new().value(boxblock.name()),
                                    Events::new(),
                                    vec![],
                                ),
                            ],
                        ),
                        Html::div(
                            Attributes::new()
                                .class(Common::keyvalue())
                                .class(Self::class("item-content")),
                            Events::new(),
                            vec![
                                Text::span("データID"),
                                Html::input(
                                    Attributes::new()
                                        .flag("readonly", true)
                                        .value(boxblock_id.to_string()),
                                    Events::new(),
                                    vec![],
                                ),
                            ],
                        ),
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_marker(is_selected: bool, attrs: Attributes, events: Events) -> Html {
        if is_selected {
            Marker::fill_blue(attrs, events, vec![Html::text("選択中")])
        } else {
            Marker::blue(attrs, events, vec![Html::text("未選択")])
        }
    }
}

impl Styled for BoxblockList {
    fn style() -> Style {
        style! {
            ".item" {
                "width": "100%";
            }

            ".item-content" {
                "margin-left": "1rem";
                "border-left": format!(".35rem solid {}", crate::libs::color::Pallet::gray(0));
                "padding-left": ".65rem";
                "padding-top": ".65rem";
                "padding-bottom": ".65rem";
            }
        }
    }
}
