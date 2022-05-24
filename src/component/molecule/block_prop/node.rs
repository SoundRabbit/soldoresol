use super::super::atom::{
    btn::{self, Btn},
    dropdown::{self, Dropdown},
    fa,
    slider::{self, Slider},
    text,
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
    pub data: BlockMut<block::Property>,
    pub view_mode: ViewMode,
}

#[derive(Clone, Copy)]
pub enum ViewMode {
    View,
    Edit,
}

pub enum Msg {
    NoOp,
    Sub(On),
    PushChild,
    SetName(String),
    PushValue(usize),
    PushRow,
    SetPropertyView(block::property::PropertyView),
    SetDataView(block::property::DataView),
    SetValue(usize, usize, block::property::Value),
    RemoveChild(U128Id),
    RemoveValue(usize, usize),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
    RemoveNode {
        prop_id: U128Id,
    },
}

pub struct Node {
    arena: ArenaMut,
    prop: BlockMut<block::Property>,
    view_mode: ViewMode,
}

impl Component for Node {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Node {}

impl Constructor for Node {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            prop: props.data,
            view_mode: props.view_mode,
        }
    }
}

impl Update for Node {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.prop = props.data;
        self.view_mode = props.view_mode;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::PushChild => {
                let child_prop = block::Property::new();
                let child_prop = self.arena.insert(child_prop);
                let child_prop_id = child_prop.id();
                self.prop.update(|prop| {
                    prop.push_child(child_prop);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! { child_prop_id },
                    update: set! { self.prop.id() },
                })
            }
            Msg::SetName(name) => {
                self.prop.update(|prop| {
                    prop.set_name(name);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::PushValue(row_idx) => {
                let value = block::property::Value::Normal(String::from(""));

                self.prop.update(|prop| {
                    prop.data_mut().push_value(row_idx, value);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::PushRow => {
                self.prop.update(|prop| {
                    prop.data_mut().push_row();
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::SetPropertyView(prop_view) => {
                self.prop.update(|prop| {
                    prop.set_view(prop_view);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::SetDataView(data_view) => {
                self.prop.update(|prop| {
                    prop.data_mut().set_view(data_view);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::SetValue(row_idx, col_idx, value) => {
                self.prop.update(|prop| {
                    prop.data_mut().set_value(row_idx, col_idx, value);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::RemoveChild(block_id) => {
                self.prop.update(|prop| prop.remove_child(&block_id));

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
            Msg::RemoveValue(r, c) => {
                self.prop.update(|prop| {
                    prop.data_mut().remove_value2(r, c);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! {self.prop.id()},
                })
            }
        }
    }
}

impl Render<Html> for Node {
    type Children = (Attributes, Events);

    fn render(&self, (attrs, events): Self::Children) -> Html {
        Self::styled(Html::div(
            attrs.class("pure-form").class(Self::class("base")),
            events,
            self.prop
                .map(|prop| vec![self.render_node(prop), self.render_children(prop)])
                .unwrap_or_default(),
        ))
    }
}

impl Node {
    fn render_node(&self, prop: &block::Property) -> Html {
        Html::div(
            Attributes::new().class(Self::class("node")),
            Events::new(),
            vec![self.render_heading(prop), self.render_data(prop.data())],
        )
    }

    fn render_children(&self, prop: &block::Property) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("children"))
                .class(match prop.view() {
                    block::property::PropertyView::List => Self::class("children-list"),
                    block::property::PropertyView::Board => Self::class("children-board"),
                }),
            Events::new(),
            vec![
                prop.children()
                    .iter()
                    .map(|child_prop| {
                        Self::empty(
                            self,
                            None,
                            Props {
                                arena: ArenaMut::clone(&self.arena),
                                data: BlockMut::clone(&child_prop),
                                view_mode: self.view_mode,
                            },
                            Sub::map(|sub| match sub {
                                On::UpdateBlocks { insert, update } => {
                                    Msg::Sub(On::UpdateBlocks { insert, update })
                                }
                                On::RemoveNode { prop_id } => Msg::RemoveChild(prop_id),
                            }),
                        )
                    })
                    .collect::<Vec<_>>(),
                vec![match &self.view_mode {
                    ViewMode::Edit => Html::span(
                        Attributes::new()
                            .class(Self::class("dragger"))
                            .class(Self::class("controller")),
                        Events::new().on_click(self, |_| Msg::PushChild),
                        vec![fa::fas_i("fa-circle-plus")],
                    ),
                    ViewMode::View => Html::none(),
                }],
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    fn render_heading(&self, prop: &block::Property) -> Html {
        Html::div(
            Attributes::new().class(Self::class("heading")),
            Events::new(),
            vec![
                Html::span(
                    Attributes::new().class(Self::class("dragger")),
                    Events::new(),
                    vec![fa::fas_i("fa-circle-dot")],
                ),
                Html::input(
                    Attributes::new()
                        .value(prop.name())
                        .class(Self::class("name")),
                    Events::new().on_input(self, |name| Msg::SetName(name)),
                    vec![],
                ),
                Dropdown::new(
                    self,
                    None,
                    dropdown::Props {
                        direction: dropdown::Direction::BottomLeft,
                        variant: btn::Variant::TransparentLight,
                        toggle_type: dropdown::ToggleType::Click,
                    },
                    Sub::none(),
                    (
                        vec![fa::fas_i("fa-ellipsis-vertical")],
                        vec![
                            Btn::menu_as_light(
                                Attributes::new(),
                                Events::new().on_click(self, {
                                    let prop_id = self.prop.id();
                                    move |_| Msg::Sub(On::RemoveNode { prop_id })
                                }),
                                vec![Html::text("削除")],
                            ),
                            Html::span(
                                Attributes::new()
                                    .class(Dropdown::class("menu-heading"))
                                    .class(Btn::class_name(&btn::Variant::LightLikeMenu)),
                                Events::new(),
                                vec![text::span("要素の配置")],
                            ),
                            Btn::group(
                                Attributes::new(),
                                Events::new(),
                                vec![
                                    Btn::light(
                                        Attributes::new(),
                                        Events::new().on_click(self, |_| {
                                            Msg::SetPropertyView(
                                                block::property::PropertyView::List,
                                            )
                                        }),
                                        vec![Html::text("リスト")],
                                    ),
                                    Btn::light(
                                        Attributes::new(),
                                        Events::new().on_click(self, |_| {
                                            Msg::SetPropertyView(
                                                block::property::PropertyView::Board,
                                            )
                                        }),
                                        vec![Html::text("表")],
                                    ),
                                ],
                            ),
                            Html::span(
                                Attributes::new()
                                    .class(Dropdown::class("menu-heading"))
                                    .class(Btn::class_name(&btn::Variant::LightLikeMenu)),
                                Events::new(),
                                vec![text::span("値の配置")],
                            ),
                            Btn::group(
                                Attributes::new(),
                                Events::new(),
                                vec![
                                    Btn::light(
                                        Attributes::new(),
                                        Events::new().on_click(self, |_| {
                                            Msg::SetDataView(block::property::DataView::List)
                                        }),
                                        vec![Html::text("リスト")],
                                    ),
                                    Btn::light(
                                        Attributes::new(),
                                        Events::new().on_click(self, |_| {
                                            Msg::SetDataView(block::property::DataView::Tabular)
                                        }),
                                        vec![Html::text("表")],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ),
            ],
        )
    }

    fn render_data(&self, data: &block::property::Data) -> Html {
        match data.view() {
            block::property::DataView::List => self.render_data_as_list(data.values()),
            block::property::DataView::Tabular => self.render_data_as_tabular(data.values()),
        }
    }

    fn render_data_as_list(&self, values: &Vec<Vec<block::property::Value>>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("data"))
                .class(Self::class("view-list")),
            Events::new(),
            vec![
                values
                    .iter()
                    .enumerate()
                    .map(|(row_idx, values)| {
                        values.iter().enumerate().map(move |(col_idx, value)| {
                            self.render_value(Attributes::new(), row_idx, col_idx, value)
                        })
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
                vec![match &self.view_mode {
                    ViewMode::Edit => Html::span(
                        Attributes::new()
                            .class(Self::class("dragger"))
                            .class(Self::class("controller")),
                        Events::new().on_click(self, {
                            let row_num = values.len();
                            move |_| Msg::PushValue(row_num - 1)
                        }),
                        vec![fa::fas_i("fa-circle-plus")],
                    ),
                    ViewMode::View => Html::none(),
                }],
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    fn render_data_as_tabular(&self, rows: &Vec<Vec<block::property::Value>>) -> Html {
        let column_num = rows.iter().map(|columns| columns.len()).max().unwrap_or(0);

        Html::div(
            Attributes::new()
                .class(Self::class("data"))
                .class(Self::class("view-tabular"))
                .style(
                    "grid-template-columns",
                    format!(
                        "repeat({}, 1fr)",
                        match &self.view_mode {
                            ViewMode::Edit => column_num + 1,
                            ViewMode::View => column_num,
                        }
                    ),
                ),
            Events::new(),
            vec![
                rows.iter()
                    .enumerate()
                    .map(|(row_idx, columns)| {
                        let mut htmls = vec![];
                        let mut col_idx = 0;
                        for value in columns {
                            if col_idx == 0 {
                                htmls.push(self.render_value(
                                    Attributes::new().class(Self::class("first-col")),
                                    row_idx,
                                    col_idx,
                                    value,
                                ));
                            } else {
                                htmls.push(self.render_value(
                                    Attributes::new(),
                                    row_idx,
                                    col_idx,
                                    value,
                                ));
                            }
                            col_idx += 1;
                        }
                        while htmls.len() < column_num {
                            htmls.push(Html::div(Attributes::new(), Events::new(), vec![]));
                        }
                        match &self.view_mode {
                            ViewMode::Edit => htmls.push(Html::span(
                                Attributes::new()
                                    .class(Self::class("dragger"))
                                    .class(Self::class("controller")),
                                Events::new().on_click(self, move |_| Msg::PushValue(row_idx)),
                                vec![fa::fas_i("fa-circle-plus")],
                            )),
                            ViewMode::View => {}
                        }
                        htmls
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
                vec![match &self.view_mode {
                    ViewMode::Edit => Html::span(
                        Attributes::new()
                            .class(Self::class("dragger"))
                            .class(Self::class("controller")),
                        Events::new().on_click(self, move |_| Msg::PushRow),
                        vec![fa::fas_i("fa-circle-plus")],
                    ),
                    ViewMode::View => Html::none(),
                }],
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    fn render_value(
        &self,
        attrs: Attributes,
        row_idx: usize,
        col_idx: usize,
        value: &block::property::Value,
    ) -> Html {
        Html::div(
            attrs.class(Self::class("value")),
            Events::new(),
            vec![
                match value {
                    block::property::Value::Number(v) => {
                        self.render_value_as_number(row_idx, col_idx, v)
                    }
                    block::property::Value::NumberMinMax(v, i, a) => {
                        self.render_value_as_number_min_max(row_idx, col_idx, v, i, a)
                    }
                    block::property::Value::NumberMid(v, m) => {
                        self.render_value_as_number_mid(row_idx, col_idx, v, m)
                    }
                    block::property::Value::Normal(v) => {
                        self.render_value_as_normal(row_idx, col_idx, v)
                    }
                    block::property::Value::Note(v) => {
                        self.render_value_as_note(row_idx, col_idx, v)
                    }
                    block::property::Value::Check(v) => self.render_value_as_check(v),
                    block::property::Value::Select(v, l) => self.render_value_as_select(v, l),
                },
                match &self.view_mode {
                    ViewMode::Edit => Dropdown::new(
                        self,
                        None,
                        dropdown::Props {
                            direction: dropdown::Direction::BottomLeft,
                            variant: btn::Variant::TransparentLight,
                            toggle_type: dropdown::ToggleType::Click,
                        },
                        Sub::none(),
                        (
                            vec![fa::fas_i("fa-ellipsis-vertical")],
                            vec![
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, move |_| {
                                        Msg::RemoveValue(row_idx, col_idx)
                                    }),
                                    vec![Html::text("削除")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_number();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("数値")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_number_min_max();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("リソース")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_number_mid();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("カウンター")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_normal();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("テキスト")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_note();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("ノート")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_check();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("チェックボックス")],
                                ),
                                Btn::menu_as_light(
                                    Attributes::new(),
                                    Events::new().on_click(self, {
                                        let value = value.to_select();
                                        move |_| Msg::SetValue(row_idx, col_idx, value)
                                    }),
                                    vec![Html::text("セレクトリスト")],
                                ),
                            ],
                        ),
                    ),
                    ViewMode::View => Html::none(),
                },
            ],
        )
    }

    fn render_value_as_number(
        &self,
        row_idx: usize,
        col_idx: usize,
        value: &block::property::NumberValue,
    ) -> Html {
        Html::input(
            Attributes::new().type_("number").value(value.to_string()),
            Events::new().on_input(self, move |value| {
                Msg::SetValue(
                    row_idx,
                    col_idx,
                    value
                        .parse::<f64>()
                        .map(|value| block::property::Value::Number(value))
                        .unwrap_or_else(|_| block::property::Value::Number(0.0)),
                )
            }),
            vec![],
        )
    }

    fn render_value_as_number_min_max(
        &self,
        row_idx: usize,
        col_idx: usize,
        value: &block::property::NumberValue,
        min: &block::property::NumberMin,
        max: &block::property::NumberMax,
    ) -> Html {
        let value = *value;
        let min = *min;
        let max = *max;
        Slider::new(
            self,
            None,
            slider::Position::Linear {
                min: min,
                max: max,
                val: value,
                step: 1.0,
            },
            Sub::map(move |sub| match sub {
                slider::On::Input(value) => Msg::SetValue(
                    row_idx,
                    col_idx,
                    block::property::Value::NumberMinMax(value, min, max),
                ),
                slider::On::InputRange { min, max } => Msg::SetValue(
                    row_idx,
                    col_idx,
                    block::property::Value::NumberMinMax(value, min, max),
                ),
                slider::On::InputMid(..) => Msg::NoOp,
            }),
            slider::Props {
                range_is_editable: true,
                theme: slider::Theme::Light,
            },
        )
    }

    fn render_value_as_number_mid(
        &self,
        row_idx: usize,
        col_idx: usize,
        value: &block::property::NumberValue,
        mid: &block::property::NumberMid,
    ) -> Html {
        let value = *value;
        let mid = *mid;
        Slider::new(
            self,
            None,
            slider::Position::Inf {
                val: value,
                mid: mid,
                step: 1.0,
            },
            Sub::map(move |sub| match sub {
                slider::On::Input(value) => Msg::SetValue(
                    row_idx,
                    col_idx,
                    block::property::Value::NumberMid(value, mid),
                ),
                slider::On::InputMid(mid) => Msg::SetValue(
                    row_idx,
                    col_idx,
                    block::property::Value::NumberMid(value, mid),
                ),
                slider::On::InputRange { .. } => Msg::NoOp,
            }),
            slider::Props {
                range_is_editable: true,
                theme: slider::Theme::Light,
            },
        )
    }

    fn render_value_as_normal(&self, row_idx: usize, col_idx: usize, value: &String) -> Html {
        Html::input(
            Attributes::new().type_("text").value(value),
            Events::new().on_input(self, move |value| {
                Msg::SetValue(row_idx, col_idx, block::property::Value::Normal(value))
            }),
            vec![],
        )
    }

    fn render_value_as_note(&self, row_idx: usize, col_idx: usize, value: &String) -> Html {
        Html::textarea(
            Attributes::new().type_("text").value(value),
            Events::new().on_input(self, move |value| {
                Msg::SetValue(row_idx, col_idx, block::property::Value::Note(value))
            }),
            vec![],
        )
    }

    fn render_value_as_check(&self, value: &bool) -> Html {
        Html::div(Attributes::new(), Events::new(), vec![])
    }

    fn render_value_as_select(&self, value: &usize, list: &Vec<String>) -> Html {
        Html::div(Attributes::new(), Events::new(), vec![])
    }
}

impl Styled for Node {
    fn style() -> Style {
        style! {
            ".dragger" {
                "display": "inline-block";
                "line-height": "1.5";
                "padding": ".5em 1em";
                "align-self": "start";
            }

            ".dragger.controller:hover" {
                "cursor": "pointer";
            }

            ".data .dragger" {
                "color": format!("{}", crate::libs::color::Pallet::gray(5));
            }

            ".heading" {
                "display": "flex";
            }

            ".heading .name" {
                "flex-grow": "1";
                "border-radius": "0 !important";
                "border": "none !important";
                "box-shadow": "none !important";
            }

            ".heading .dragger" {
                "display": "none";
            }

            ".data.view-list" {
                "display": "flex";
                "flex-direction": "column";
            }

            ".data.view-tabular" {
                "display": "grid";
            }

            ".value" {
                "display": "flex";
            }

            ".value > *:first-child" {
                "flex-grow": "1";
            }

            ".children" {
                "display": "flex";
            }

            ".children.children-list" {
                "flex-direction": "column";
            }

            ".children.children-board" {
                "flex-direction": "row";
            }

            ".children.children-board > .base" {
                "flex-grow": "1";
            }

            ".children .heading .dragger" {
                "display": "inline-block";
            }

            ".children .data" {
                "padding-left": "3em";
            }

            ".children .children" {
                "margin-left": "1.4em";
                "padding-left": "1.4em";
                "border-left": format!("0.2em solid {}", crate::libs::color::Pallet::gray(9));
            }
        }
    }
}
