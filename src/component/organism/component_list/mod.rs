use super::atom::text::Text;
use super::molecule::tab_menu::{self, TabMenu};
use crate::arena::{block, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

mod boxblock_list;
use boxblock_list::BoxblockList;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub selecting: U128Id,
}

pub enum Msg {}

pub enum On {}

pub struct ComponentList {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    selecting: U128Id,
}

impl Component for ComponentList {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ComponentList {}

impl Constructor for ComponentList {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            world: props.world,
            selecting: props.selecting,
        }
    }
}

impl Update for ComponentList {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.world = props.world;
        self.selecting = props.selecting;
        Cmd::none()
    }
}

impl Render<Html> for ComponentList {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Self::styled(TabMenu::new(
            self,
            None,
            tab_menu::Props {
                selected: 0,
                controlled: false,
            },
            Sub::none(),
            (
                Attributes::new(),
                Events::new(),
                vec![(
                    Text::condense_75("ブロック"),
                    Html::div(
                        Attributes::new()
                            .class(Self::class("padding"))
                            .class(Self::class("scroll"))
                            .class(Self::class("list")),
                        Events::new(),
                        vec![BoxblockList::empty(
                            self,
                            None,
                            boxblock_list::Props {
                                arena: ArenaMut::clone(&self.arena),
                                world: BlockMut::clone(&self.world),
                                selecting: U128Id::clone(&self.selecting),
                            },
                            Sub::none(),
                        )],
                    ),
                )],
            ),
        ))
    }
}

impl Styled for ComponentList {
    fn style() -> Style {
        style! {
            ".padding" {
                "padding": ".35rem";
            }

            ".scroll" {
                "overflow-y": "scroll";
                "height": "100%";
            }

            ".list" {
                "display": "grid";
                "grid-auto-flow": "row";
                "row-gap": ".35rem";
            }
        }
    }
}
