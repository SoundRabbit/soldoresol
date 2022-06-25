use super::atom::collapse::{self, Collapse};
use super::atom::common::Common;
use super::atom::marker::Marker;
use super::atom::text::Text;
use crate::arena::{block, ArenaMut, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub arena: ArenaMut,
    pub scene: BlockMut<block::Scene>,
}

pub enum Msg {}

pub enum On {}

pub struct TableList {
    arena: ArenaMut,
    scene: BlockMut<block::Scene>,
}

impl Component for TableList {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for TableList {}

impl Constructor for TableList {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            scene: props.scene,
        }
    }
}

impl Update for TableList {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.scene = props.scene;
        Cmd::none()
    }
}

impl Render<Html> for TableList {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::fragment(vec![
            self.scene
                .map(|scene| self.render_table(scene.master_table(), true))
                .unwrap_or(Html::none()),
            Html::fragment(
                self.scene
                    .map(|scene| {
                        scene
                            .tables()
                            .iter()
                            .map(|table| self.render_table(table, false))
                            .collect()
                    })
                    .unwrap_or(vec![]),
            ),
        ]))
    }
}

impl TableList {
    fn render_table(&self, table: &BlockMut<block::Table>, is_master: bool) -> Html {
        let table_id = table.id();
        table
            .map(|table| {
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
                                            vec![Html::text(if is_master {
                                                "マスター"
                                            } else {
                                                "テーブル"
                                            })],
                                        ),
                                        Html::input(
                                            Attributes::new().value(table.name()),
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
                                                .value(table_id.to_string()),
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

impl Styled for TableList {
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
