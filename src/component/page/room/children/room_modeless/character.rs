use super::super::atom::btn::{self, Btn};
use super::super::atom::dropdown::{self, Dropdown};
use super::super::atom::text;
use super::super::modal_imported_files::{self, ModalImportedFiles};
use super::super::molecule::tab_menu::{self, TabMenu};
use super::super::util::styled::{Style, Styled};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use async_std::sync::{Arc, Mutex};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub resource_arena: resource::ArenaRef,
    pub character_id: block::BlockId,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetModal(Modal),
    SetSelectedTabIdx(usize),
    SetIsEditable(bool),
}

pub enum Modal {
    None,
    ImportedFiles,
}

pub enum On {
    SetTextureId {
        tex_idx: usize,
        resource_id: Option<resource::ResourceId>,
    },
    AddTexture,
    RemoveTexture {
        tex_idx: usize,
    },
    SetTextureIdx {
        tex_idx: usize,
    },
    SetTextureName {
        tex_idx: usize,
        tex_name: String,
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
}

pub struct Character {
    block_arena: block::ArenaRef,
    resource_arena: resource::ArenaRef,
    character_id: block::BlockId,
    element_id: ElementId,
    modal: Modal,
    selected_tab_idx: usize,
    is_editable: bool,
}

struct ElementId {
    input_character_name: String,
    input_display_name: String,
    input_tab_name: String,
}

impl Constructor for Character {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            block_arena: props.block_arena,
            resource_arena: props.resource_arena,
            character_id: props.character_id,
            element_id: ElementId {
                input_character_name: format!("{:X}", crate::libs::random_id::u128val()),
                input_display_name: format!("{:X}", crate::libs::random_id::u128val()),
                input_tab_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
            modal: Modal::None,
            selected_tab_idx: 0,
            is_editable: false,
        }
    }
}

impl Component for Character {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.block_arena = props.block_arena;
        self.character_id = props.character_id;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::SetModal(modal) => {
                self.modal = modal;
                Cmd::none()
            }
            Msg::SetSelectedTabIdx(idx) => {
                self.selected_tab_idx = idx;
                Cmd::none()
            }
            Msg::SetIsEditable(is_editable) => {
                self.is_editable = is_editable;
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(
                    &self.character_id,
                    |character: &block::character::Character| {
                        let prop_num = character.properties().count();

                        let mut prop_names = vec![String::from("[common]")];
                        let mut prop_tabs = vec![self.render_common(character)];

                        for prop_id in character.properties() {
                            self.block_arena
                                .map(prop_id, |prop: &block::property::Property| {
                                    prop_names.push(prop.name().clone());
                                });
                            prop_tabs.push(self.render_tab(prop_id));
                        }
                        prop_names.push(String::from("[追加]"));

                        TabMenu::with_children(
                            tab_menu::Props {
                                selected: self.selected_tab_idx,
                                tabs: prop_names,
                                controlled: true,
                            },
                            Subscription::new(move |sub| match sub {
                                tab_menu::On::ChangeSelectedTab(idx) => {
                                    if idx <= prop_num {
                                        Msg::SetSelectedTabIdx(idx)
                                    } else {
                                        Msg::Sub(On::AddPropertyChild {
                                            property_id: None,
                                            name: String::from("新規タブ"),
                                        })
                                    }
                                }
                            }),
                            prop_tabs,
                        )
                    },
                )
                .unwrap_or(Html::none()),
        )
    }
}

impl Character {
    fn render_common(&self, character: &block::character::Character) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("common")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("common-props")),
                            Events::new(),
                            vec![
                                Html::div(
                                    Attributes::new().class(Self::class("key-value")),
                                    Events::new(),
                                    vec![
                                        text::label("表示名", &self.element_id.input_display_name),
                                        Html::input(Attributes::new(), Events::new(), vec![]),
                                        text::label(
                                            "キャラクター名",
                                            &self.element_id.input_character_name,
                                        ),
                                        Html::input(
                                            Attributes::new()
                                                .value(character.name())
                                                .id(&self.element_id.input_character_name),
                                            Events::new(),
                                            vec![],
                                        ),
                                    ],
                                ),
                                Html::textarea(
                                    Attributes::new()
                                        .value(character.name())
                                        .class(Self::class("common-description")),
                                    Events::new(),
                                    vec![],
                                ),
                            ],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("common-imgs")),
                            Events::new(),
                            vec![
                                Dropdown::with_children(
                                    dropdown::Props {
                                        direction: dropdown::Direction::Bottom,
                                        text: String::from(character.current_tex_name()),
                                        toggle_type: dropdown::ToggleType::Click,
                                        variant: btn::Variant::DarkLikeMenu,
                                    },
                                    Subscription::none(),
                                    vec![
                                        character
                                            .tex_names()
                                            .into_iter()
                                            .enumerate()
                                            .map(|(tex_idx, tex_name)| {
                                                Self::render_tex_list_item(tex_idx, tex_name)
                                            })
                                            .collect(),
                                        vec![Html::div(
                                            Attributes::new()
                                                .class(Self::class("common-imgs-list-btn")),
                                            Events::new().on("click", |e| {
                                                e.stop_propagation();
                                                Msg::Sub(On::AddTexture)
                                            }),
                                            vec![Btn::with_child(
                                                btn::Props {
                                                    variant: btn::Variant::Dark,
                                                },
                                                Subscription::none(),
                                                Html::text("追加"),
                                            )],
                                        )],
                                    ]
                                    .into_iter()
                                    .flatten()
                                    .collect(),
                                ),
                                Html::input(
                                    Attributes::new().value(character.current_tex_name()),
                                    Events::new().on_input({
                                        let current_tex_idx = character.current_tex_idx();
                                        move |tex_name| {
                                            Msg::Sub(On::SetTextureName {
                                                tex_idx: current_tex_idx,
                                                tex_name: tex_name,
                                            })
                                        }
                                    }),
                                    vec![],
                                ),
                                Html::div(
                                    Attributes::new().class(Self::class("common-imgs-container")),
                                    Events::new(),
                                    vec![character
                                        .current_tex_id()
                                        .and_then(|r_id| {
                                            self.resource_arena.get_as::<resource::ImageData>(r_id)
                                        })
                                        .map(|img| {
                                            Html::img(
                                                Attributes::new()
                                                    .class(Self::class("common-imgs-img"))
                                                    .src(img.url().as_ref()),
                                                Events::new(),
                                                vec![],
                                            )
                                        })
                                        .unwrap_or(Html::none())],
                                ),
                                Btn::with_child(
                                    btn::Props {
                                        variant: btn::Variant::Primary,
                                    },
                                    Subscription::new(move |sub| match sub {
                                        btn::On::Click => Msg::SetModal(Modal::ImportedFiles),
                                    }),
                                    Html::text("画像を選択"),
                                ),
                            ],
                        ),
                    ],
                ),
                self.render_modal(character.current_tex_idx()),
            ],
        )
    }

    fn render_tex_list_item(tex_idx: usize, tex_name: &str) -> Html {
        Html::div(
            Attributes::new().class(Self::class("common-imgs-list-item")),
            Events::new(),
            vec![
                Btn::with_child(
                    btn::Props {
                        variant: btn::Variant::Menu,
                    },
                    Subscription::new(move |sub| match sub {
                        btn::On::Click => Msg::Sub(On::SetTextureIdx { tex_idx }),
                    }),
                    Html::text(tex_name),
                ),
                Html::div(
                    Attributes::new().class(Self::class("common-imgs-list-btn")),
                    Events::new().on("click", move |e| {
                        e.stop_propagation();
                        Msg::Sub(On::RemoveTexture { tex_idx: tex_idx })
                    }),
                    vec![Btn::with_child(
                        btn::Props {
                            variant: btn::Variant::Danger,
                        },
                        Subscription::none(),
                        Html::text("削除"),
                    )],
                ),
            ],
        )
    }

    fn render_tab(&self, prop_id: &BlockId) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class("pure-form"),
            Events::new(),
            vec![self
                .block_arena
                .map(prop_id, |prop: &block::property::Property| {
                    Html::div(
                        Attributes::new().class(Self::class("root-prop")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Self::class("key-value")),
                                Events::new(),
                                vec![
                                    text::label("タブ名", &self.element_id.input_tab_name),
                                    Html::input(
                                        Attributes::new().value(prop.name()),
                                        Events::new(),
                                        vec![],
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new().class("banner"),
                                Events::new(),
                                vec![Btn::with_child(
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
                                )],
                            ),
                            Html::div(
                                Attributes::new().class(Self::class("prop-list")),
                                Events::new(),
                                {
                                    let mut children: Vec<_> = prop
                                        .children()
                                        .map(|prop_id| self.render_prop(prop_id))
                                        .flatten()
                                        .collect();
                                    if self.is_editable {
                                        children.push(
                                            self.render_prop_add_prop(BlockId::clone(prop_id)),
                                        );
                                    }
                                    children
                                },
                            ),
                        ],
                    )
                })
                .unwrap_or(Html::none())],
        )
    }

    fn render_prop(&self, prop_id: &BlockId) -> Vec<Html> {
        self.block_arena
            .map(prop_id, |prop: &block::property::Property| {
                vec![
                    if !self.is_editable && prop.values().count() < 1 {
                        Html::input(
                            Attributes::new()
                                .value(prop.name())
                                .class(Self::class("banner")),
                            Events::new(),
                            vec![],
                        )
                    } else {
                        Html::input(Attributes::new().value(prop.name()), Events::new(), vec![])
                    },
                    if self.is_editable || prop.values().count() > 0 {
                        Html::div(
                            Attributes::new().class(Self::class("prop-value-list")),
                            Events::new(),
                            {
                                let mut values: Vec<_> = prop
                                    .values()
                                    .enumerate()
                                    .map(|(idx, value)| {
                                        if self.is_editable {
                                            match value {
                                                block::property::Value::None => vec![Html::div(
                                                    Attributes::new().class(Self::class("banner")),
                                                    Events::new(),
                                                    vec![self.render_prop_set_value_type(
                                                        prop_id, idx, value,
                                                    )],
                                                )],
                                                block::property::Value::Text(text) => vec![
                                                    Html::input(
                                                        Attributes::new().value(text.as_ref()),
                                                        Events::new(),
                                                        vec![],
                                                    ),
                                                    self.render_prop_set_value_type(
                                                        prop_id, idx, value,
                                                    ),
                                                ],
                                                block::property::Value::MultiLineText(text) => {
                                                    vec![
                                                        Html::textarea(
                                                            Attributes::new().value(text.as_ref()),
                                                            Events::new(),
                                                            vec![],
                                                        ),
                                                        self.render_prop_set_value_type(
                                                            prop_id, idx, value,
                                                        ),
                                                    ]
                                                }
                                            }
                                        } else {
                                            match value {
                                                block::property::Value::None => vec![],
                                                block::property::Value::Text(text) => {
                                                    vec![
                                                        Html::input(
                                                            Attributes::new().value(text.as_ref()),
                                                            Events::new(),
                                                            vec![],
                                                        ),
                                                        Html::div(
                                                            Attributes::new(),
                                                            Events::new(),
                                                            vec![],
                                                        ),
                                                    ]
                                                }
                                                block::property::Value::MultiLineText(text) => {
                                                    vec![
                                                        Html::textarea(
                                                            Attributes::new().value(text.as_ref()),
                                                            Events::new(),
                                                            vec![],
                                                        ),
                                                        Html::div(
                                                            Attributes::new(),
                                                            Events::new(),
                                                            vec![],
                                                        ),
                                                    ]
                                                }
                                            }
                                        }
                                    })
                                    .flatten()
                                    .collect();
                                if self.is_editable {
                                    values
                                        .push(self.render_prop_add_value(BlockId::clone(prop_id)));
                                }
                                values
                            },
                        )
                    } else {
                        Html::none()
                    },
                    if self.is_editable || prop.children().count() > 0 {
                        Html::div(
                            Attributes::new().class(Self::class("prop-list-container")),
                            Events::new(),
                            vec![Html::div(
                                Attributes::new().class(Self::class("prop-list")),
                                Events::new(),
                                {
                                    let mut children: Vec<_> = prop
                                        .children()
                                        .map(|prop_id| self.render_prop(prop_id))
                                        .flatten()
                                        .collect();
                                    if self.is_editable {
                                        children.push(
                                            self.render_prop_add_prop(BlockId::clone(prop_id)),
                                        );
                                    }
                                    children
                                },
                            )],
                        )
                    } else {
                        Html::none()
                    },
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_prop_add_prop(&self, prop_id: BlockId) -> Html {
        Html::div(
            Attributes::new().class(Self::class("banner")),
            Events::new(),
            vec![Btn::with_child(
                btn::Props {
                    variant: btn::Variant::Secondary,
                },
                Subscription::new(move |sub| match sub {
                    btn::On::Click => Msg::Sub(On::AddPropertyChild {
                        property_id: Some(prop_id),
                        name: String::from(""),
                    }),
                }),
                Html::text("追加"),
            )],
        )
    }

    fn render_prop_add_value(&self, prop_id: BlockId) -> Html {
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

    fn render_prop_set_value_type(
        &self,
        prop_id: &BlockId,
        idx: usize,
        value: &block::property::Value,
    ) -> Html {
        Dropdown::with_children(
            dropdown::Props {
                direction: dropdown::Direction::Bottom,
                text: String::from(match value {
                    block::property::Value::None => "未指定",
                    block::property::Value::Text(..) => "テキスト",
                    block::property::Value::MultiLineText(..) => "ノート",
                }),
                toggle_type: dropdown::ToggleType::Click,
                variant: btn::Variant::DarkLikeMenu,
            },
            Subscription::none(),
            vec![
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
                ),
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
                                    value: block::property::Value::MultiLineText(Rc::new(
                                        String::new(),
                                    )),
                                }),
                            },
                        }
                    }),
                    Html::text("ノート"),
                ),
            ],
        )
    }

    fn render_modal(&self, tex_idx: usize) -> Html {
        match &self.modal {
            Modal::None => Html::none(),
            Modal::ImportedFiles => ModalImportedFiles::empty(
                modal_imported_files::Props {
                    resource_arena: resource::ArenaRef::clone(&self.resource_arena),
                },
                Subscription::new(move |sub| match sub {
                    modal_imported_files::On::Close => Msg::SetModal(Modal::None),
                    modal_imported_files::On::SelectFile(resource_id) => {
                        Msg::Sub(On::SetTextureId {
                            tex_idx,
                            resource_id: Some(resource_id),
                        })
                    }
                }),
            ),
        }
    }
}

impl Styled for Character {
    fn style() -> Style {
        style! {
            "base" {
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

            "base textarea" {
                "resize": "none";
            }

            "common" {
                "display": "grid";
                "grid-template-columns": "1fr 15rem";
                "grid-template-rows": "20rem";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "common-props" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            "common-imgs" {
                "display": "grid";
                "grid-template-rows": "max-content max-content 1fr max-content";
                "row-gap": "0.35em";
            }

            "common-imgs-container" {
                "overflow": "hidden";
            }

            "common-imgs-img" {
                "height": "100%";
                "width": "100%";
                "object-fit": "contain";
            }

            "common-imgs-list-item" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".15em";
            }

            "common-imgs-list-btn" {
                "display": "grid";
            }

            "key-value" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "max-content 1fr";
            }

            "root-prop" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
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

            "prop-value-list" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "1fr max-content";
                "align-items": "start";
            }

            "banner" {
                "grid-column-start": "1";
                "grid-column-end": "-1";
            }

            "banner > button" {
                "width": "100%";
            }
        }
    }
}
