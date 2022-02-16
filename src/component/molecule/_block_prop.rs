use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::slider::{self, Slider};
use super::atom::text;
use crate::arena::block::{self, BlockId};
use crate::libs::select_list::SelectList;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

pub struct Props {
    pub root_prop: BlockId,
    pub block_arena: block::ArenaRef,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetIsEditable(bool),
    UpdateMappedList {
        prop_id: BlockId,
        idx: usize,
        update: Box<dyn FnOnce(&mut SelectList<(String, String)>)>,
    },
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
    RemoveProperty {
        property_id: BlockId,
        idx: usize,
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
    is_editable: bool,
}

impl Component for BlockProp {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for BlockProp {
    fn constructor(props: &Props) -> Self {
        Self { is_editable: false }
    }
}

impl Update for BlockProp {
    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::Sub(sub),
            Msg::SetIsEditable(is_editable) => {
                self.is_editable = is_editable;
                Cmd::none()
            }
            Msg::UpdateMappedList {
                prop_id,
                idx,
                update,
            } => props
                .block_arena
                .map(
                    &BlockId::clone(&prop_id),
                    move |prop: &block::property::Property| {
                        if let block::property::Value::MappedList(mapped_list) = prop.value(idx) {
                            let mut mapped_list = SelectList::clone(mapped_list);
                            update(&mut mapped_list);
                            Cmd::Sub(On::SetPropertyValue {
                                property_id: prop_id,
                                idx: idx,
                                value: block::property::Value::MappedList(mapped_list),
                            })
                        } else {
                            Cmd::none()
                        }
                    },
                )
                .unwrap_or(Cmd::none()),
        }
    }
}

impl Render for BlockProp {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Btn::dark(
                    Attributes::new(),
                    Events::new().on_click({
                        let is_editable = self.is_editable;
                        move |_| Msg::SetIsEditable(!is_editable)
                    }),
                    vec![Html::text(if self.is_editable {
                        "編集完了"
                    } else {
                        "編集開始"
                    })],
                ),
                props
                    .block_arena
                    .map(&props.root_prop, |root_prop: &block::property::Property| {
                        self.render_prop_children(
                            props,
                            BlockId::clone(&props.root_prop),
                            root_prop,
                        )
                    })
                    .unwrap_or(Html::none()),
            ],
        ))
    }
}

impl BlockProp {
    fn render_prop_children(
        &self,
        props: &Props,
        prop_id: BlockId,
        prop: &block::property::Property,
    ) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("prop-list")),
            Events::new(),
            {
                let mut children: Vec<_> = prop
                    .children()
                    .enumerate()
                    .map(|(p_idx, p_id)| self.render_prop(props, &prop_id, p_id, p_idx))
                    .flatten()
                    .collect();
                if self.is_editable {
                    children.push(self.render_btn_add_prop(prop_id));
                }
                children
            },
        )
    }

    fn render_prop(
        &self,
        props: &Props,
        parent_id: &BlockId,
        prop_id: &BlockId,
        self_idx: usize,
    ) -> Vec<Html<Self>> {
        props
            .block_arena
            .map(prop_id, |prop: &block::property::Property| {
                vec![
                    self.render_prop_name(prop_id, prop),
                    self.render_menu_prop(parent_id, prop_id, self_idx, prop.value_mode()),
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
                            vec![self.render_prop_children(props, BlockId::clone(prop_id), prop)],
                        )
                    } else {
                        Html::none()
                    },
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_prop_name(&self, prop_id: &BlockId, prop: &block::property::Property) -> Html<Self> {
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

    fn render_value_list(&self, prop_id: &BlockId, prop: &block::property::Property) -> Html<Self> {
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
    ) -> Vec<Html<Self>> {
        match value {
            block::property::Value::None => self.render_value_none(prop_id, idx, value),
            block::property::Value::Text(text) => self.render_value_text(prop_id, idx, value, text),
            block::property::Value::MultiLineText(text) => {
                self.render_value_multi_line_text(prop_id, idx, value, text)
            }
            block::property::Value::ResourceMinMax { min, val, max } => {
                self.render_value_resource_min_max(prop_id, idx, value, *min, *val, *max)
            }
            block::property::Value::MappedList(mapped_list) => {
                self.render_value_mapped_list(prop_id, idx, value, mapped_list)
            }
        }
    }

    fn render_value_none(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Vec<Html<Self>> {
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
    ) -> Vec<Html<Self>> {
        vec![
            Html::input(
                Attributes::new().value(text),
                Events::new().on_input({
                    let prop_id = BlockId::clone(prop_id);
                    move |text| {
                        Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::Text(text),
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
    ) -> Vec<Html<Self>> {
        vec![
            Html::textarea(
                Attributes::new().value(text).nut("rows", 4),
                Events::new().on_input({
                    let prop_id = BlockId::clone(prop_id);
                    move |text| {
                        Msg::Sub(On::SetPropertyValue {
                            property_id: prop_id,
                            idx,
                            value: block::property::Value::MultiLineText(text),
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
    ) -> Vec<Html<Self>> {
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
                Sub::map({
                    let prop_id = BlockId::clone(prop_id);
                    move |sub| match sub {
                        slider::On::Input(val) => Msg::Sub(On::SetPropertyValue {
                            property_id: BlockId::clone(&prop_id),
                            idx,
                            value: block::property::Value::ResourceMinMax { min, val, max },
                        }),
                        slider::On::InputRange { min, max } => Msg::Sub(On::SetPropertyValue {
                            property_id: BlockId::clone(&prop_id),
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

    fn render_value_mapped_list(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
        mapped_list: &SelectList<(String, String)>,
    ) -> Vec<Html<Self>> {
        vec![
            Html::div(
                Attributes::new().class(Self::class("prop-value--table-column")),
                Events::new(),
                vec![
                    Dropdown::with_children(
                        dropdown::Props {
                            direction: dropdown::Direction::Bottom,
                            text: mapped_list
                                .selected()
                                .map(|(a, b)| format!("{}: {}", b, a))
                                .unwrap_or(String::from("")),
                            toggle_type: dropdown::ToggleType::Click,
                            variant: btn::Variant::DarkLikeMenu,
                        },
                        Sub::none(),
                        {
                            let mut x: Vec<_> = mapped_list
                                .iter()
                                .enumerate()
                                .map(|(list_idx, (a, b))| {
                                    self.render_value_mapped_list_item(
                                        prop_id,
                                        idx,
                                        mapped_list,
                                        list_idx,
                                        a,
                                        b,
                                    )
                                })
                                .collect();

                            if self.is_editable {
                                x.push(Btn::dark(
                                    Attributes::new(),
                                    Events::new().on("click", {
                                        let prop_id = BlockId::clone(prop_id);
                                        move |e| {
                                            e.stop_propagation();
                                            Msg::UpdateMappedList {
                                                prop_id,
                                                idx,
                                                update: Box::new(|mapped_list| {
                                                    let len = mapped_list.len();
                                                    mapped_list.push((
                                                        format!("{}", len),
                                                        format!("選択肢[{}]", len),
                                                    ));
                                                }),
                                            }
                                        }
                                    }),
                                    vec![Html::text("追加")],
                                ));
                            }

                            x
                        },
                    ),
                    self.render_value_mapped_list_selected(prop_id, idx, mapped_list),
                ],
            ),
            self.render_btn_set_value_type(prop_id, idx, value),
            self.render_btn_remove_value(BlockId::clone(prop_id), idx),
        ]
    }

    fn render_value_mapped_list_item(
        &self,
        prop_id: &BlockId,
        idx: usize,
        mapped_list: &SelectList<(String, String)>,
        list_idx: usize,
        a: &String,
        b: &String,
    ) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("prop-value--value-key")),
            Events::new(),
            vec![
                Btn::menu(
                    Attributes::new(),
                    Events::new().on_click({
                        let prop_id = BlockId::clone(prop_id);
                        move |_| Msg::UpdateMappedList {
                            prop_id,
                            idx,
                            update: Box::new(move |mapped_list| {
                                mapped_list.set_selected_idx(list_idx);
                            }),
                        }
                    }),
                    vec![Html::text(format!("{}: {}", b, a))],
                ),
                if self.is_editable {
                    Btn::danger(
                        Attributes::new(),
                        Events::new().on("click", {
                            let prop_id = BlockId::clone(prop_id);
                            move |e| {
                                e.stop_propagation();
                                Msg::UpdateMappedList {
                                    prop_id,
                                    idx,
                                    update: Box::new(move |mapped_list| {
                                        if mapped_list.len() > 1 {
                                            mapped_list.remove(list_idx);
                                        }
                                    }),
                                }
                            }
                        }),
                        vec![Html::text("削除")],
                    )
                } else {
                    Html::none()
                },
            ],
        )
    }

    fn render_value_mapped_list_selected(
        &self,
        prop_id: &BlockId,
        idx: usize,
        mapped_list: &SelectList<(String, String)>,
    ) -> Html<Self> {
        if self.is_editable {
            Html::div(
                Attributes::new().class(Self::class("prop-value--value-key-value")),
                Events::new(),
                vec![
                    Html::input(
                        Attributes::new().value(
                            mapped_list
                                .selected()
                                .map(|(_, b)| b.clone())
                                .unwrap_or(String::from("")),
                        ),
                        Events::new().on_input({
                            let prop_id = BlockId::clone(prop_id);
                            move |b| Msg::UpdateMappedList {
                                prop_id,
                                idx,
                                update: Box::new(|mapped_list| {
                                    if let Some(selected) = mapped_list.selected_mut() {
                                        selected.1 = b;
                                    }
                                }),
                            }
                        }),
                        vec![],
                    ),
                    text::span(":"),
                    Html::input(
                        Attributes::new().value(
                            mapped_list
                                .selected()
                                .map(|(a, _)| a.clone())
                                .unwrap_or(String::from("")),
                        ),
                        Events::new().on_input({
                            let prop_id = BlockId::clone(prop_id);
                            move |a| Msg::UpdateMappedList {
                                prop_id,
                                idx,
                                update: Box::new(|mapped_list| {
                                    if let Some(selected) = mapped_list.selected_mut() {
                                        selected.0 = a;
                                    }
                                }),
                            }
                        }),
                        vec![],
                    ),
                ],
            )
        } else {
            Html::none()
        }
    }

    fn render_btn_add_prop(&self, prop_id: BlockId) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("banner")),
            Events::new(),
            vec![Btn::secondary(
                Attributes::new(),
                Events::new().on_click(|_| {
                    Msg::Sub(On::AddPropertyChild {
                        property_id: prop_id,
                        name: String::from(""),
                    })
                }),
                vec![Html::text("追加")],
            )],
        )
    }

    fn render_menu_prop(
        &self,
        parent_id: &block::BlockId,
        prop_id: &BlockId,
        self_idx: usize,
        value_mode: &block::property::ValueMode,
    ) -> Html<Self> {
        if self.is_editable {
            Html::div(
                Attributes::new().class(Self::class("prop-menu")),
                Events::new(),
                vec![
                    self.render_btn_set_value_mode(prop_id, value_mode),
                    self.render_btn_remove_prop(BlockId::clone(parent_id), self_idx),
                ],
            )
        } else {
            Html::none()
        }
    }

    fn render_btn_set_value_mode(
        &self,
        prop_id: &BlockId,
        value_mode: &block::property::ValueMode,
    ) -> Html<Self> {
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
            Sub::none(),
            vec![
                Btn::menu_as_secondary(
                    Attributes::new(),
                    Events::new().on_click({
                        let prop_id = BlockId::clone(prop_id);
                        move |_| {
                            Msg::Sub(On::SetPropertyValueMode {
                                property_id: prop_id,
                                value_mode: block::property::ValueMode::List,
                            })
                        }
                    }),
                    vec![Html::text("リスト")],
                ),
                Btn::menu_as_secondary(
                    Attributes::new(),
                    Events::new().on_click({
                        let prop_id = BlockId::clone(prop_id);
                        move |sub| {
                            Msg::Sub(On::SetPropertyValueMode {
                                property_id: prop_id,
                                value_mode: block::property::ValueMode::Column,
                            })
                        }
                    }),
                    vec![Html::text("テーブル")],
                ),
            ],
        )
    }

    fn render_btn_remove_prop(&self, parent_id: BlockId, self_idx: usize) -> Html<Self> {
        Dropdown::with_children(
            dropdown::Props {
                direction: dropdown::Direction::BottomLeft,
                text: String::from("削除"),
                toggle_type: dropdown::ToggleType::Click,
                variant: btn::Variant::Danger,
            },
            Sub::none(),
            vec![Html::div(
                Attributes::new().class(Self::class("ok-cancel")),
                Events::new(),
                vec![
                    Btn::danger(
                        Attributes::new(),
                        Events::new().on_click(move |_| {
                            Msg::Sub(On::RemoveProperty {
                                property_id: parent_id,
                                idx: self_idx,
                            })
                        }),
                        vec![Html::text("OK")],
                    ),
                    Btn::primary(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("キャンセル")],
                    ),
                ],
            )],
        )
    }

    fn render_btn_add_value(&self, prop_id: BlockId) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("banner")),
            Events::new(),
            vec![Btn::dark(
                Attributes::new(),
                Events::new().on_click(move |_| {
                    Msg::Sub(On::AddPropertyValue {
                        property_id: prop_id,
                    })
                }),
                vec![Html::text("追加")],
            )],
        )
    }

    fn render_btn_set_value_type(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html<Self> {
        if self.is_editable {
            Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::BottomLeft,
                    text: String::from(match value {
                        block::property::Value::None => "未指定",
                        block::property::Value::Text(..) => "テキスト",
                        block::property::Value::MultiLineText(..) => "ノート",
                        block::property::Value::ResourceMinMax { .. } => "上限付きリソース",
                        block::property::Value::MappedList(..) => "選択肢",
                    }),
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::DarkLikeMenu,
                },
                Sub::none(),
                vec![
                    self.render_btn_set_value_type_as_text(prop_id, idx, value),
                    self.render_btn_set_value_type_as_muti_line_text(prop_id, idx, value),
                    self.render_btn_set_value_type_as_resource_min_max(prop_id, idx, value),
                    self.render_btn_set_value_type_as_mapped_list(prop_id, idx, value),
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
    ) -> Html<Self> {
        Btn::menu(
            Attributes::new(),
            Events::new().on_click({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |_| match value {
                    block::property::Value::Text(..) => Msg::NoOp,
                    block::property::Value::MultiLineText(x) => Msg::Sub(On::SetPropertyValue {
                        property_id: prop_id,
                        idx,
                        value: block::property::Value::Text(x),
                    }),
                    _ => Msg::Sub(On::SetPropertyValue {
                        property_id: prop_id,
                        idx,
                        value: block::property::Value::Text(String::new()),
                    }),
                }
            }),
            vec![Html::text("テキスト")],
        )
    }

    fn render_btn_set_value_type_as_muti_line_text(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html<Self> {
        Btn::menu(
            Attributes::new(),
            Events::new().on_click({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |_| match value {
                    block::property::Value::MultiLineText(..) => Msg::NoOp,
                    block::property::Value::Text(x) => Msg::Sub(On::SetPropertyValue {
                        property_id: prop_id,
                        idx,
                        value: block::property::Value::MultiLineText(x),
                    }),
                    _ => Msg::Sub(On::SetPropertyValue {
                        property_id: prop_id,
                        idx,
                        value: block::property::Value::MultiLineText(String::new()),
                    }),
                }
            }),
            vec![Html::text("ノート")],
        )
    }

    fn render_btn_set_value_type_as_resource_min_max(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html<Self> {
        Btn::menu(
            Attributes::new(),
            Events::new().on_click({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |_| match value {
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
                }
            }),
            vec![Html::text("上限付きリソース")],
        )
    }

    fn render_btn_set_value_type_as_mapped_list(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html<Self> {
        Btn::menu(
            Attributes::new(),
            Events::new().on_click({
                let prop_id = BlockId::clone(prop_id);
                let value = block::property::Value::clone(value);
                move |_| match value {
                    block::property::Value::MappedList { .. } => Msg::NoOp,
                    _ => Msg::Sub(On::SetPropertyValue {
                        property_id: prop_id,
                        idx,
                        value: block::property::Value::MappedList(SelectList::new(
                            vec![
                                (String::from("Yes"), String::from("1")),
                                (String::from("No"), String::from("0")),
                            ],
                            0,
                        )),
                    }),
                }
            }),
            vec![Html::text("選択肢")],
        )
    }

    fn render_btn_remove_value(&self, prop_id: BlockId, idx: usize) -> Html<Self> {
        if self.is_editable {
            Dropdown::with_children(
                dropdown::Props {
                    direction: dropdown::Direction::BottomLeft,
                    text: String::from("削除"),
                    toggle_type: dropdown::ToggleType::Click,
                    variant: btn::Variant::Danger,
                },
                Sub::none(),
                vec![Html::div(
                    Attributes::new().class(Self::class("ok-cancel")),
                    Events::new(),
                    vec![
                        Btn::danger(
                            Attributes::new(),
                            Events::new().on_click(move |_| {
                                Msg::Sub(On::RemovePropertyValue {
                                    property_id: prop_id,
                                    idx,
                                })
                            }),
                            vec![Html::text("OK")],
                        ),
                        Btn::primary(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("キャンセル")],
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
            ".base" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
                "width": "100%";
            }

            ".prop-list-container" {
                "grid-column-start": "1";
                "grid-column-end": "-1";
                "padding-left": "2rem";
            }

            ".prop-list" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "max-content 1fr";
                "align-items": "start";
            }

            ".prop-name" {
                "display": "grid";
                "row-gap": ".65em";
                "align-items": "start";
            }

            ".prop-menu" {
                "display": "grid";
                "column-gap": ".35em";
                "grid-template-columns": "1fr max-content";
            }

            ".prop-value-list" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
                "align-items": "start";
            }

            ".prop-value-list--editable" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "1fr max-content max-content";
                "align-items": "start";
            }

            ".prop-value-column" {
                "display": "grid";
                "column-gap": ".35em";
                "grid-auto-columns": "1fr";
                "grid-auto-flow": "column";
                "align-items": "start";
            }

            ".prop-value-column > *" {
                "width": "100%";
            }

            ".prop-value--table-column" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
                "grid-auto-flow": "row";
            }

            ".prop-value--value-key" {
                "display": "grid";
                "column-gap": ".15em";
                "grid-template-columns": "1fr max-content";
                "align-items": "center";
            }

            ".prop-value--value-key-value" {
                "display": "grid";
                "column-gap": ".35em";
                "grid-template-columns": "1fr max-content 1fr";
                "align-items": "center";
            }

            ".banner" {
                "grid-column-start": "1";
                "grid-column-end": "-1";
            }

            ".banner-2c" {
                "grid-column": "span 2";
            }

            ".banner > button" {
                "width": "100%";
            }

            ".banner > input" {
                "width": "100%";
            }

            ".banner-2c > button" {
                "width": "100%";
            }

            ".ok-cancel" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "column-gap": "0.05rem";
                "padding-left": "0.05rem";
                "padding-right": "0.05rem";
            }
        }
    }
}
