use super::atom::{
    btn::{self, Btn},
    collapse::{self, Collapse},
    common::Common,
    dropdown::{self, Dropdown},
    fa,
    marker::Marker,
    text::Text,
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

pub struct Props {
    pub arena: ArenaMut,
    pub scene: BlockMut<block::Scene>,
}

pub enum Msg {
    AddTable,
    SetSelectingTable(U128Id),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

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

    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::AddTable => {
                let table = self
                    .scene
                    .map(|scene| scene.selecting_table().map(|table| table.create_child()))
                    .unwrap_or(None);

                if let Some(table) = table {
                    let table = self.arena.insert(table);
                    let table_id = table.id();
                    self.scene.update(|scene| {
                        scene.tables_push(table);
                    });
                    Cmd::submit(On::UpdateBlocks {
                        insert: set! { table_id },
                        update: set! { self.scene.id() },
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::SetSelectingTable(table_id) => {
                self.scene.update(|scene| {
                    scene.set_selecting_table(&table_id);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.scene.id() },
                })
            }
        }
    }
}

impl Render<Html> for TableList {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::fragment(vec![
            self.scene
                .map(|scene| {
                    self.render_table(
                        scene.master_table(),
                        true,
                        scene.selecting_table().id() == scene.master_table().id(),
                    )
                })
                .unwrap_or(Html::none()),
            Html::fragment(
                self.scene
                    .map(|scene| {
                        scene
                            .tables()
                            .iter()
                            .map(|table| {
                                self.render_table(
                                    table,
                                    false,
                                    scene.selecting_table().id() == table.id(),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or(vec![]),
            ),
            Marker::blue(
                Attributes::new()
                    .class(Self::class("btn"))
                    .class(Self::class("indent")),
                Events::new().on_click(self, |_| Msg::AddTable),
                vec![Html::text("テーブルを追加")],
            ),
        ]))
    }
}

impl TableList {
    fn render_table(
        &self,
        table: &BlockMut<block::Table>,
        is_master: bool,
        is_selected: bool,
    ) -> Html {
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
                                if is_master {
                                    self.render_master(table, U128Id::clone(&table_id), is_selected)
                                } else {
                                    self.render_child(table, U128Id::clone(&table_id), is_selected)
                                },
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

    fn render_master(&self, table: &block::Table, table_id: U128Id, is_selected: bool) -> Html {
        Html::div(
            Attributes::new().class(Common::keyvalue()),
            Events::new(),
            vec![
                Self::render_marker(
                    is_selected,
                    Attributes::new(),
                    Events::new().on_click(self, move |_| Msg::SetSelectingTable(table_id)),
                    vec![Text::condense_75("マスター")],
                ),
                Html::input(Attributes::new().value(table.name()), Events::new(), vec![]),
            ],
        )
    }

    fn render_child(&self, table: &block::Table, table_id: U128Id, is_selected: bool) -> Html {
        Html::div(
            Attributes::new().class(Common::keyvaluekey()),
            Events::new(),
            vec![
                Self::render_marker(
                    is_selected,
                    Attributes::new(),
                    Events::new().on_click(self, move |_| Msg::SetSelectingTable(table_id)),
                    vec![Text::condense_75("テーブル")],
                ),
                Html::input(Attributes::new().value(table.name()), Events::new(), vec![]),
                Dropdown::new(
                    self,
                    None,
                    dropdown::Props {
                        direction: dropdown::Direction::BottomLeft,
                        toggle_type: dropdown::ToggleType::Click,
                        variant: btn::Variant::TransparentDark,
                    },
                    Sub::none(),
                    (
                        vec![fa::fas_i("fa-ellipsis-vertical")],
                        vec![Btn::menu_as_secondary(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("削除")],
                        )],
                    ),
                ),
            ],
        )
    }

    fn render_marker(
        is_selected: bool,
        attrs: Attributes,
        events: Events,
        children: Vec<Html>,
    ) -> Html {
        if is_selected {
            Marker::fill_blue(attrs, events, children)
        } else {
            Marker::blue(attrs, events, children)
        }
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
                "padding-bottom": ".65rem";
            }

            ".btn:hover" {
                "cursor": "pointer";
            }

            ".indent" {
                "margin-left": "2.5rem";
            }
        }
    }
}
