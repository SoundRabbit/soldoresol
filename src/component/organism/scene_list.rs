use super::atom::collapse::{self, Collapse};
use super::atom::common::Common;
use super::atom::marker::Marker;
use super::atom::text::Text;
use super::molecule::table_list::{self, TableList};
use crate::arena::{block, ArenaMut, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
}

pub enum Msg {
    Sub(On),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct SceneList {
    arena: ArenaMut,
    world: BlockMut<block::World>,
}

impl Component for SceneList {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for SceneList {}

impl Constructor for SceneList {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            world: props.world,
        }
    }
}

impl Update for SceneList {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.world = props.world;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
        }
    }
}

impl Render<Html> for SceneList {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::fragment(
            self.world
                .map(|world| {
                    world
                        .scenes()
                        .iter()
                        .map(|scene| {
                            self.render_scene(scene, scene.id() == world.selecting_scene().id())
                        })
                        .collect()
                })
                .unwrap_or(vec![]),
        ))
    }
}

impl SceneList {
    fn render_scene(&self, scene: &BlockMut<block::Scene>, is_selected: bool) -> Html {
        let scene_id = scene.id();
        let scene_block = BlockMut::clone(scene);
        scene
            .map(|scene| {
                Collapse::new(
                    self,
                    None,
                    collapse::Props {
                        is_default_collapsed: false,
                        is_indented: false,
                    },
                    Sub::none(),
                    (
                        Html::div(
                            Attributes::new().class(Self::class("item")),
                            Events::new(),
                            vec![
                                Html::div(
                                    Attributes::new().class(Common::keyvalue()),
                                    Events::new(),
                                    vec![
                                        Self::render_marker(
                                            is_selected,
                                            Attributes::new(),
                                            Events::new(),
                                            vec![Html::text("シーン")],
                                        ),
                                        Html::input(
                                            Attributes::new().value(scene.name()),
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
                                                .value(scene_id.to_string()),
                                            Events::new(),
                                            vec![],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        vec![TableList::empty(
                            self,
                            None,
                            table_list::Props {
                                arena: ArenaMut::clone(&self.arena),
                                scene: scene_block,
                            },
                            Sub::map(|sub| match sub {
                                table_list::On::UpdateBlocks { insert, update } => {
                                    Msg::Sub(On::UpdateBlocks { insert, update })
                                }
                            }),
                        )],
                    ),
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_marker(
        is_selected: bool,
        attrs: Attributes,
        events: Events,
        children: Vec<Html>,
    ) -> Html {
        if is_selected {
            Marker::fill_purple(attrs, events, children)
        } else {
            Marker::purple(attrs, events, children)
        }
    }
}

impl Styled for SceneList {
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
