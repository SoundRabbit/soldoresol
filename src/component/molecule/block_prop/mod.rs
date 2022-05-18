use super::atom::{
    btn::{self, Btn},
    dropdown::{self, Dropdown},
    fa,
    slider::{self, Slider},
};
use crate::arena::{block, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

mod node;
use node::Node;

pub use node::ViewMode;

pub struct Props {
    pub arena: ArenaMut,
    pub data: BlockMut<block::Property>,
}

pub enum Msg {
    Sub(On),
    SetViewMode(ViewMode),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct BlockProp {
    arena: ArenaMut,
    prop: BlockMut<block::Property>,
    view_mode: ViewMode,
}

impl Component for BlockProp {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for BlockProp {}

impl Constructor for BlockProp {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            prop: props.data,
            view_mode: ViewMode::View,
        }
    }
}

impl Update for BlockProp {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.prop = props.data;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetViewMode(view_mode) => {
                self.view_mode = view_mode;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for BlockProp {
    type Children = ();

    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Node::new(
                    self,
                    None,
                    node::Props {
                        arena: ArenaMut::clone(&self.arena),
                        data: BlockMut::clone(&self.prop),
                        view_mode: self.view_mode,
                    },
                    Sub::map(|sub| match sub {
                        node::On::UpdateBlocks { insert, update } => {
                            Msg::Sub(On::UpdateBlocks { insert, update })
                        }
                    }),
                    (Attributes::new().class(Self::class("prop")), Events::new()),
                ),
                self.render_menu(),
            ],
        ))
    }
}

impl BlockProp {
    fn render_menu(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("menu")),
            Events::new(),
            vec![match &self.view_mode {
                ViewMode::View => Btn::primary(
                    Attributes::new(),
                    Events::new().on_click(self, |_| Msg::SetViewMode(ViewMode::Edit)),
                    vec![Html::text("編集")],
                ),
                ViewMode::Edit => Btn::success(
                    Attributes::new(),
                    Events::new().on_click(self, |_| Msg::SetViewMode(ViewMode::View)),
                    vec![Html::text("閲覧")],
                ),
            }],
        )
    }
}

impl Styled for BlockProp {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "overflow": "hidden";
                "display": "flex";
                "flex-direction": "column";
            }

            ".prop" {
                "flex-grow": "1";
                "overflow-y": "scroll";
            }

            ".menu" {
                "display": "flex";
                "justify-content": "right";
            }
        }
    }
}
