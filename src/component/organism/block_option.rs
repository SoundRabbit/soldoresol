use super::atom::align::Align;
use super::atom::text;
use super::molecule::block_prop::{self, BlockProp};
use super::molecule::tab_menu::{self, TabMenu};
use crate::arena::block::{self, BlockId};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub props: Vec<BlockId>,
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
    props: Vec<BlockId>,
    tabs: Vec<String>,
    block_arena: block::ArenaRef,
    selected_tab_idx: usize,
    element_id: ElementId,
}

struct ElementId {
    input_tab_name: String,
}

impl Constructor for BlockOption {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            props: props.props,
            tabs: props.tabs,
            block_arena: props.block_arena,
            selected_tab_idx: 0,
            element_id: ElementId {
                input_tab_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
        }
    }
}

impl Component for BlockOption {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.props = props.props;
        self.tabs = props.tabs;
        self.block_arena = props.block_arena;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::SetSelectedTabIdx(selected_tab_idx) => {
                self.selected_tab_idx = selected_tab_idx;
                Cmd::none()
            }

            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        let (tabs, contents) = {
            let mut tabs = self.tabs.clone();
            let mut contents = children;

            for prop_id in &self.props {
                self.block_arena
                    .map(prop_id, |prop: &block::property::Property| {
                        tabs.push(prop.name().clone());
                        contents.push(self.render_content(prop_id, prop));
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
            Subscription::new(move |sub| match sub {
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
    pub fn content_base(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Html::div(
            attrs.class(Self::class("content-base")).class("pure-form"),
            events,
            children,
        )
    }

    fn render_content(&self, prop_id: &BlockId, prop: &block::property::Property) -> Html {
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
                                    .value(prop.name())
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
                            block_arena: block::ArenaRef::clone(&self.block_arena),
                        },
                        Subscription::new(|sub| match sub {
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
