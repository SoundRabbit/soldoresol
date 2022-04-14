use super::atom::collapse::{self, Collapse};
use super::atom::common::Common;
use super::atom::marker::Marker;
use super::atom::text;
use super::NoProps;
use crate::arena::{block, ArenaMut, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub enum Msg {}

pub enum On {}

pub struct SceneList {}

impl Component for SceneList {
    type Props = NoProps;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for SceneList {}

impl Constructor for SceneList {
    fn constructor(_: Self::Props) -> Self {
        Self {}
    }
}

impl Update for SceneList {}

impl Render<Html> for SceneList {
    type Children = (ArenaMut, BlockMut<block::World>);
    fn render(&self, (arena, world): Self::Children) -> Html {
        Self::styled(Html::fragment(
            world
                .map(|world| {
                    world
                        .scenes()
                        .iter()
                        .map(|scene| self.render_scene(scene))
                        .collect()
                })
                .unwrap_or(vec![]),
        ))
    }
}

impl SceneList {
    fn render_scene(&self, scene: &BlockMut<block::Scene>) -> Html {
        let scene_id = scene.id();
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
                                        Marker::purple(
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
                                        text::span("データID"),
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
                        vec![],
                    ),
                )
            })
            .unwrap_or(Html::none())
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
            }
        }
    }
}
