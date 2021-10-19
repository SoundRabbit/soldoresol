use super::atom::align::Align;
use super::atom::text;
use super::molecule::block_prop::{self, BlockProp};
use super::molecule::tab_menu::{self, TabMenu};
use crate::arena::block::{self, BlockId};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

pub struct Props {
    pub prop_blocks: Vec<BlockId>,
    pub tabs: Vec<String>,
    pub block_arena: block::ArenaRef,
}

pub enum Msg {
    SetSelectedTabIdx(usize),
    Sub(On),
}

pub enum On {
    SetPropertyName {
        property_id: BlockId,
        name: String,
    },
    AddPropertyChild {
        property_id: Option<BlockId>,
        name: String,
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
    SetPropertyValueMode {
        property_id: BlockId,
        value_mode: block::property::ValueMode,
    },
    RemoveProperty {
        property_id: BlockId,
        idx: usize,
    },
}

pub struct BlockOption {
    selected_tab_idx: usize,
    element_id: ElementId,
}

struct ElementId {
    input_tab_name: String,
}

impl Component for BlockOption {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for BlockOption {
    fn constructor(_: &Props) -> Self {
        Self {
            selected_tab_idx: 0,
            element_id: ElementId {
                input_tab_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
        }
    }
}

impl Update for BlockOption {
    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::SetSelectedTabIdx(selected_tab_idx) => {
                self.selected_tab_idx = selected_tab_idx;
                Cmd::none()
            }

            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }
}

impl Render for BlockOption {
    fn render(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        let (tabs, contents) = {
            let mut tabs = props.tabs.clone();
            let mut contents = children;

            for prop_id in &props.prop_blocks {
                props
                    .block_arena
                    .map(prop_id, |prop_block: &block::property::Property| {
                        tabs.push(prop_block.name().clone());
                        contents.push(self.render_content(props, prop_id, prop_block));
                    });
            }

            tabs.push(String::from("[追加]"));

            (tabs, contents)
        };

        let tab_num = tabs.len() - 1;

        Self::styled(TabMenu::with_children(
            tab_menu::Props {
                selected: self.selected_tab_idx,
                tabs: tabs,
                controlled: true,
            },
            Sub::map(move |sub| match sub {
                tab_menu::On::ChangeSelectedTab(idx) => {
                    if idx < tab_num {
                        Msg::SetSelectedTabIdx(idx)
                    } else {
                        Msg::Sub(On::AddPropertyChild {
                            property_id: None,
                            name: String::from("新規タブ"),
                        })
                    }
                }
            }),
            contents,
        ))
    }
}

impl BlockOption {
    pub fn content_base(
        attrs: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Self>>,
    ) -> Html<Self> {
        Html::div(
            attrs.class(Self::class("content-base")).class("pure-form"),
            events,
            children,
        )
    }

    fn render_content(
        &self,
        props: &Props,
        prop_id: &BlockId,
        prop_block: &block::property::Property,
    ) -> Html<Self> {
        Self::content_base(
            Attributes::new(),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("root-prop")),
                Events::new(),
                vec![
                    Align::key_value(
                        Attributes::new(),
                        Events::new(),
                        vec![
                            text::label("タブ名", &self.element_id.input_tab_name),
                            Html::input(
                                Attributes::new()
                                    .value(prop_block.name())
                                    .id(&self.element_id.input_tab_name),
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
                            ),
                        ],
                    ),
                    BlockProp::empty(
                        block_prop::Props {
                            root_prop: BlockId::clone(prop_id),
                            block_arena: block::ArenaRef::clone(&props.block_arena),
                        },
                        Sub::map(|sub| match sub {
                            block_prop::On::AddPropertyChild { property_id, name } => {
                                Msg::Sub(On::AddPropertyChild {
                                    property_id: Some(property_id),
                                    name,
                                })
                            }
                            block_prop::On::AddPropertyValue { property_id } => {
                                Msg::Sub(On::AddPropertyValue { property_id })
                            }
                            block_prop::On::SetPropertyName { property_id, name } => {
                                Msg::Sub(On::SetPropertyName { property_id, name })
                            }
                            block_prop::On::SetPropertyValue {
                                property_id,
                                idx,
                                value,
                            } => Msg::Sub(On::SetPropertyValue {
                                property_id,
                                idx,
                                value,
                            }),
                            block_prop::On::RemovePropertyValue { property_id, idx } => {
                                Msg::Sub(On::RemovePropertyValue { property_id, idx })
                            }
                            block_prop::On::SetPropertyValueMode {
                                property_id,
                                value_mode,
                            } => Msg::Sub(On::SetPropertyValueMode {
                                property_id,
                                value_mode,
                            }),
                            block_prop::On::RemoveProperty { property_id, idx } => {
                                Msg::Sub(On::RemoveProperty { property_id, idx })
                            }
                        }),
                    ),
                ],
            )],
        )
    }
}

impl Styled for BlockOption {
    fn style() -> Style {
        style! {
            ".content-base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "row";
                "row-gap": ".65em";
                "overflow-y": "scroll";
                "overflow-x": "hidden";
                "max-height": "100%";
                "padding": "1.2ch 0 1.2ch 1.2ch";
            }

            ".root-prop" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
            }
        }
    }
}
