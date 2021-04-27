use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::slider::{self, Slider};
use super::util::styled::{Style, Styled};
use crate::arena::block::{self, BlockId};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub root_prop: BlockId,
    pub block_arena: block::ArenaRef,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetIsEditable(bool),
}

pub enum On {
    AddPropertyChild {
        property_id: BlockId,
        name: String,
    },
    SetPropertyName {
        property_id: BlockId,
        name: String,
    },
    SetPropertyValueMode {
        property_id: BlockId,
        value_mode: block::property::ValueMode,
    },
    AddPropertyValue {
        property_id: BlockId,
    },
    SetPropertyValue {
        property_id: BlockId,
        idx: usize,
        value: block::property::Value,
    },
    RemovePropertyValue {
        property_id: BlockId,
        idx: usize,
    },
}

pub struct BlockProp {
    root_prop: BlockId,
    block_arena: block::ArenaRef,
    is_editable: bool,
}

impl Constructor for BlockProp {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            root_prop: props.root_prop,
            block_arena: props.block_arena,
            is_editable: false,
        }
    }
}

impl Component for BlockProp {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.root_prop = props.root_prop;
        self.block_arena = props.block_arena;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::SetIsEditable(is_editable) => {
                self.is_editable = is_editable;
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Btn::with_child(
                    btn::Props {
                        variant: btn::Variant::Dark,
                    },
                    Subscription::new({
                        let is_editable = self.is_editable;
                        move |sub| match sub {
                            btn::On::Click => Msg::SetIsEditable(!is_editable),
                        }
                    }),
                    Html::text(if self.is_editable {
                        "編集完了"
                    } else {
                        "編集開始"
                    }),
                ),
                self.block_arena
                    .map(&self.root_prop, |root_prop: &block::property::Property| {
                        self.render_prop_children(BlockId::clone(&self.root_prop), root_prop)
                    })
                    .unwrap_or(Html::none()),
            ],
        ))
    }
}

impl BlockProp {
    fn render_prop_children(&self, prop_id: BlockId, prop: &block::property::Property) -> Html {
        Html::div(
            Attributes::new().class(Self::class("prop-list")),
            Events::new(),
            {
                let mut children: Vec<_> = prop
                    .children()
                    .map(|p_id| self.render_prop(p_id))
                    .flatten()
                    .collect();
                if self.is_editable {
                    children.push(self.render_btn_add_prop(prop_id));
                }
                children
            },
        )
    }

    fn render_prop(&self, prop_id: &BlockId) -> Vec<Html> {
        self.block_arena
            .map(prop_id, |prop: &block::property::Property| {
                vec![
                    self.render_prop_name(prop_id, prop),
                    self.render_btn_set_value_mode(prop_id, prop.value_mode()),
                    if self.is_editable {
                        Html::div(Attributes::new(), Events::new(), vec![])
                    } else {
                        Html::none()
                    },
                    if self.is_editable || prop.values().count() > 0 {
                        self.render_value_list(prop_id, prop)
                    } else {
                        Html::none()
                    },
                    if self.is_editable || prop.children().count() > 0 {
                        Html::div(
                            Attributes::new().class(Self::class("prop-list-container")),
                            Events::new(),
                            vec![self.render_prop_children(BlockId::clone(prop_id), prop)],
                        )
                    } else {
                        Html::none()
                    },
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_prop_name(&self, prop_id: &BlockId, prop: &block::property::Property) -> Html {
        let attr = Attributes::new().class(Self::class("prop-name"));
        let attr = if prop.values().count() > 0 || self.is_editable {
            attr
        } else {
            attr.class(Self::class("banner"))
        };
        Html::div(
            attr,
            Events::new(),
            vec![Html::input(
                Attributes::new().value(prop.name()),
                Events::new().on_input({
                    let prop_id = BlockId::clone(prop_id);
                    move |name| {
                        Msg::Sub(On::SetPropertyName {
                            property_id: prop_id,
                            name,
                        })
                    }
                }),
                vec![],
            )],
        )
    }

    fn render_value_list(&self, prop_id: &BlockId, prop: &block::property::Property) -> Html {
        Html::div(
            if self.is_editable {
                Attributes::new().class(Self::class("prop-value-list--editable"))
            } else {
                match prop.value_mode() {
                    block::property::ValueMode::List => {
                        Attributes::new().class(Self::class("prop-value-list"))
                    }
                    block::property::ValueMode::Column => {
                        Attributes::new().class(Self::class("prop-value-column"))
                    }
                }
            },
            Events::new(),
            {
                let mut values: Vec<_> = prop
                    .values()
                    .enumerate()
                    .map(|(idx, value)| self.render_value(prop_id, idx, value))
                    .flatten()
                    .collect();
                if self.is_editable {
                    values.push(self.render_btn_add_value(BlockId::clone(prop_id)));
                }
                values
            },
        )
    }

    fn render_value(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Vec<Html> {
        match value {
            block::property::Value::None => self.render_value_none(prop_id, idx, value),
            block::property::Value::Text(text) => {
                self.render_value_text(prop_id, idx, value, text.as_ref())
            }
            block::property::Value::MultiLineText(text) => {
                self.render_value_multi_line_text(prop_id, idx, value, text.as_ref())
            }
            block::property::Value::ResourceMinMax { min, val, max } => {
                self.render_value_resource_min_max(prop_id, idx, value, *min, *val, *max)
            }
        }
    }

    fn render_value_none(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Vec<Html> {
        if self.is_editable {
            vec![
                Html::div(
                    Attributes::new().class(Self::class("banner-2c")),
                    Events::new(),
                    vec![self.render_btn_set_value_type(prop_id, idx, value)],
                ),
                self.render_btn_remove_value(BlockId::clone(prop_id), idx),
            ]
        } else {
            vec![]
        }
    }

    fn render_value_text(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
        text: &String,
    ) -> Vec<Html> {
        vec![
            Html::input(
                Attributes::new().value(text),
                Events::new().on_input({
                    let prop_id = BlockId::clone(prop_id);
                    move |text| {
                        Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::Text(Rc::new(text)),
                        })
                    }
                }),
                vec![],
            ),
            self.render_btn_set_value_type(prop_id, idx, value),
            self.render_btn_remove_value(BlockId::clone(prop_id), idx),
        ]
    }

    fn render_value_multi_line_text(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
        text: &String,
    ) -> Vec<Html> {
        vec![
            Html::textarea(
                Attributes::new().value(text),
                Events::new().on_input({
                    let prop_id = BlockId::clone(prop_id);
                    move |text| {
                        Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::MultiLineText(Rc::new(text)),
                        })
                    }
                }),
                vec![],
            ),
            self.render_btn_set_value_type(prop_id, idx, value),
            self.render_btn_remove_value(BlockId::clone(prop_id), idx),
        ]
    }

    fn render_value_resource_min_max(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
        min: f64,
        val: f64,
        max: f64,
    ) -> Vec<Html> {
        vec![
            Slider::empty(
                slider::Props {
                    position: slider::Position::Linear {
                        min,
                        val,
                        max,
                        step: 1.0,
                    },
                    range_is_editable: true,
                },
                Subscription::new({
                    let prop_id = BlockId::clone(prop_id);
                    move |sub| match sub {
                        slider::On::Input(val) => Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::ResourceMinMax { min, val, max },
                        }),
                        slider::On::InputRange { min, max } => Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::ResourceMinMax { min, val, max },
                        }),
                        _ => Msg::NoOp,
                    }
                }),
            ),
            self.render_btn_set_value_type(prop_id, idx, value),
            self.render_btn_remove_value(BlockId::clone(prop_id), idx),
        ]
    }

    fn render_btn_add_prop(&self, prop_id: BlockId) -> Html {
        Html::div(
            Attributes::new().class(Self::class("banner")),
            Events::new(),
            vec![Btn::with_child(
                btn::Props {
                    variant: btn::Variant::Secondary,
                },
                Subscription::new(move |sub| match sub {
                    btn::On::Click => Msg::Sub(On::AddPropertyChild {
                        property_id: prop_id,
                        name: String::from(""),
                    }),
                }),
                Html::text("追加"),
            )],
        )
    }

    fn render_btn_set_value_mode(
        &self,
        prop_id: &BlockId,
        value_mode: &block::property::ValueMode,
    ) -> Html {
        if self.is_editable {
            Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::BottomLeft,
                    text: String::from(match value_mode {
                        block::property::ValueMode::List => "リスト",
                        block::property::ValueMode::Column => "テーブル",
                    }),
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::SecondaryLikeMenu,
                },
                Subscription::none(),
                vec![
                    Btn::with_child(
                        btn::Props {
                            variant: btn::Variant::MenuAsSecondary,
                        },
                        Subscription::new({
                            let prop_id = BlockId::clone(prop_id);
                            move |sub| match sub {
                                btn::On::Click => Msg::Sub(On::SetPropertyValueMode {
                                    property_id: prop_id,
                                    value_mode: block::property::ValueMode::List,
                                }),
                            }
                        }),
                        Html::text("リスト"),
                    ),
                    Btn::with_child(
                        btn::Props {
                            variant: btn::Variant::MenuAsSecondary,
                        },
                        Subscription::new({
                            let prop_id = BlockId::clone(prop_id);
                            move |sub| match sub {
                                btn::On::Click => Msg::Sub(On::SetPropertyValueMode {
                                    property_id: prop_id,
                                    value_mode: block::property::ValueMode::Column,
                                }),
                            }
                        }),
                        Html::text("テーブル"),
                    ),
                ],
            )
        } else {
            Html::none()
        }
    }

    fn render_btn_add_value(&self, prop_id: BlockId) -> Html {
        Html::div(
            Attributes::new().class(Self::class("banner")),
            Events::new(),
            vec![Btn::with_child(
                btn::Props {
                    variant: btn::Variant::Dark,
                },
                Subscription::new(move |sub| match sub {
                    btn::On::Click => Msg::Sub(On::AddPropertyValue {
                        property_id: prop_id,
                    }),
                }),
                Html::text("追加"),
            )],
        )
    }

    fn render_btn_set_value_type(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html {
        if self.is_editable {
            Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::BottomLeft,
                    text: String::from(match value {
                        block::property::Value::None => "未指定",
                        block::property::Value::Text(..) => "テキスト",
                        block::property::Value::MultiLineText(..) => "ノート",
                        block::property::Value::ResourceMinMax { .. } => "上限付きリソース",
                    }),
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::DarkLikeMenu,
                },
                Subscription::none(),
                vec![
                    self.render_btn_set_value_type_as_text(prop_id, idx, value),
                    self.render_btn_set_value_type_as_muti_line_text(prop_id, idx, value),
                    self.render_btn_set_value_type_as_resource_min_max(prop_id, idx, value),
                ],
            )
        } else {
            Html::none()
        }
    }

    fn render_btn_set_value_type_as_text(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html {
        Btn::with_child(
            btn::Props {
                variant: btn::Variant::Menu,
            },
            Subscription::new({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |sub| match sub {
                    btn::On::Click => match value {
                        block::property::Value::Text(..) => Msg::NoOp,
                        block::property::Value::MultiLineText(x) => {
                            Msg::Sub(On::SetPropertyValue {
                                property_id: prop_id,
                                idx,
                                value: block::property::Value::Text(x),
                            })
                        }
                        _ => Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::Text(Rc::new(String::new())),
                        }),
                    },
                }
            }),
            Html::text("テキスト"),
        )
    }

    fn render_btn_set_value_type_as_muti_line_text(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html {
        Btn::with_child(
            btn::Props {
                variant: btn::Variant::Menu,
            },
            Subscription::new({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |sub| match sub {
                    btn::On::Click => match value {
                        block::property::Value::MultiLineText(..) => Msg::NoOp,
                        block::property::Value::Text(x) => Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::MultiLineText(x),
                        }),
                        _ => Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::MultiLineText(Rc::new(String::new())),
                        }),
                    },
                }
            }),
            Html::text("ノート"),
        )
    }

    fn render_btn_set_value_type_as_resource_min_max(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html {
        Btn::with_child(
            btn::Props {
                variant: btn::Variant::Menu,
            },
            Subscription::new({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |sub| match sub {
                    btn::On::Click => match value {
                        block::property::Value::ResourceMinMax { .. } => Msg::NoOp,
                        _ => Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::ResourceMinMax {
                                min: 0.0,
                                val: 50.0,
                                max: 100.0,
                            },
                        }),
                    },
                }
            }),
            Html::text("上限付きリソース"),
        )
    }

    fn render_btn_remove_value(&self, prop_id: BlockId, idx: usize) -> Html {
        if self.is_editable {
            Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::BottomLeft,
                    text: String::from("削除"),
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::Danger,
                },
                Subscription::none(),
                vec![Html::div(
                    Attributes::new().class(Self::class("ok-cancel")),
                    Events::new(),
                    vec![
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Danger,
                            },
                            Subscription::new(move |sub| match sub {
                                btn::On::Click => Msg::Sub(On::RemovePropertyValue {
                                    property_id: prop_id,
                                    idx,
                                }),
                            }),
                            Html::text("OK"),
                        ),
                        Btn::with_child(
                            btn::Props {
                                variant: btn::Variant::Primary,
                            },
                            Subscription::none(),
                            Html::text("キャンセル"),
                        ),
                    ],
                )],
            )
        } else {
            Html::none()
        }
    }
}

impl Styled for BlockProp {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
                "width": "100%";
            }

            "prop-list-container" {
                "grid-column-start": "1";
                "grid-column-end": "-1";
                "padding-left": "2rem";
            }

            "prop-list" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "max-content 1fr";
                "align-items": "start";
            }

            "prop-name" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "max-content";
                "align-items": "start";
            }

            "prop-value-list" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
                "align-items": "start";
            }

            "prop-value-list--editable" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "1fr max-content max-content";
                "align-items": "start";
            }

            "prop-value-column" {
                "display": "grid";
                "column-gap": ".35em";
                "grid-auto-columns": "1fr";
                "grid-auto-flow": "column";
                "align-items": "start";
            }

            "prop-value-column > *" {
                "width": "100%";
            }

            "banner" {
                "grid-column-start": "1";
                "grid-column-end": "-1";
            }

            "banner-2c" {
                "grid-column": "span 2";
            }

            "banner > button" {
                "width": "100%";
            }

            "banner-2c > button" {
                "width": "100%";
            }

            "ok-cancel" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "column-gap": "0.05rem";
                "padding-left": "0.05rem";
                "padding-right": "0.05rem";
            }
        }
    }
}
