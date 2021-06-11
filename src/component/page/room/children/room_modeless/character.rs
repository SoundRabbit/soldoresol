use super::super::atom::btn::{self, Btn};
use super::super::atom::dropdown::{self, Dropdown};
use super::super::atom::text;
use super::super::modal_imported_files::{self, ModalImportedFiles};
use super::super::molecule::block_prop::{self, BlockProp};
use super::super::molecule::color_pallet::{self, ColorPallet};
use super::super::molecule::tab_menu::{self, TabMenu};
use super::super::organism::block_option::{self, BlockOption};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

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
    PackToDownload,
    Download(toml::Value),
}

pub enum Modal {
    None,
    ImportedFiles,
}

pub enum On {
    SetCommonProps {
        name: Option<String>,
        display_name: Option<String>,
        description: Option<String>,
        name_color: Option<crate::libs::color::Pallet>,
    },
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

pub struct Character {
    block_arena: block::ArenaRef,
    resource_arena: resource::ArenaRef,
    character_id: block::BlockId,
    element_id: ElementId,
    modal: Modal,
    selected_tab_idx: usize,
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
            Msg::PackToDownload => {
                let mut block_ids = vec![BlockId::clone(&self.character_id)];

                self.block_arena.map(
                    &self.character_id,
                    |character: &block::character::Character| {
                        for prop_id in character.properties() {
                            let prop_ids =
                                block::property::Property::flat_tree(&self.block_arena, prop_id);

                            for prop_id in prop_ids {
                                block_ids.push(prop_id);
                            }
                        }
                    },
                );

                let task = self.block_arena.pack_to_toml(block_ids.into_iter());
                Cmd::task(move |resolve| {
                    wasm_bindgen_futures::spawn_local(async move {
                        let packed = task().await;
                        resolve(Msg::Download(packed));
                    })
                })
            }
            Msg::Download(packed) => {
                if let Ok(serialized) = toml::to_string(&packed) {
                    let blob = web_sys::Blob::new_with_str_sequence_and_options(
                        &array![serialized].into(),
                        web_sys::BlobPropertyBag::new().type_("application/toml"),
                    )
                    .unwrap();
                    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                    let document = web_sys::window().unwrap().document().unwrap();
                    let a = document.create_element("a").unwrap();
                    let _ = a.set_attribute("href", &url);
                    let character_name = self
                        .block_arena
                        .map(
                            &self.character_id,
                            |character: &block::character::Character| character.name().clone(),
                        )
                        .unwrap_or(String::from("キャラクター"));
                    let _ = a.set_attribute("download", &(character_name + ".toml"));
                    let _ = a.set_attribute("style", "display: none");
                    let _ = document.body().unwrap().append_child(&a);
                    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
                    let _ = document.body().unwrap().remove_child(&a);
                }
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
                        Html::div(
                            Attributes::new().class(Self::class("base")),
                            Events::new(),
                            vec![
                                BlockOption::with_children(
                                    block_option::Props {
                                        tabs: vec![
                                            String::from("[common]"),
                                            String::from("[立ち絵]"),
                                        ],
                                        props: character
                                            .properties()
                                            .map(|x| BlockId::clone(x))
                                            .collect(),
                                        block_arena: block::ArenaRef::clone(&self.block_arena),
                                    },
                                    Subscription::new(|sub| match sub {
                                        block_option::On::AddPropertyChild {
                                            property_id,
                                            name,
                                        } => Msg::Sub(On::AddPropertyChild { property_id, name }),
                                        block_option::On::AddPropertyValue { property_id } => {
                                            Msg::Sub(On::AddPropertyValue { property_id })
                                        }
                                        block_option::On::SetPropertyName { property_id, name } => {
                                            Msg::Sub(On::SetPropertyName { property_id, name })
                                        }
                                        block_option::On::SetPropertyValue {
                                            property_id,
                                            idx,
                                            value,
                                        } => Msg::Sub(On::SetPropertyValue {
                                            property_id,
                                            idx,
                                            value,
                                        }),
                                        block_option::On::RemovePropertyValue {
                                            property_id,
                                            idx,
                                        } => Msg::Sub(On::RemovePropertyValue { property_id, idx }),
                                        block_option::On::SetPropertyValueMode {
                                            property_id,
                                            value_mode,
                                        } => Msg::Sub(On::SetPropertyValueMode {
                                            property_id,
                                            value_mode,
                                        }),
                                        block_option::On::RemoveProperty { property_id, idx } => {
                                            Msg::Sub(On::RemoveProperty { property_id, idx })
                                        }
                                    }),
                                    vec![self.render_common(character), self.render_tex(character)],
                                ),
                                self.render_bottom_menu(),
                                self.render_modal(character.current_tex_idx()),
                            ],
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
                .class(Self::class("content-base"))
                .class("pure-form"),
            Events::new(),
            vec![Html::div(
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
                                    Html::input(
                                        Attributes::new()
                                            .value(character.display_name())
                                            .id(&self.element_id.input_display_name),
                                        Events::new().on_input(|display_name| {
                                            Msg::Sub(On::SetCommonProps {
                                                name: None,
                                                display_name: Some(display_name),
                                                description: None,
                                                name_color: None,
                                            })
                                        }),
                                        vec![],
                                    ),
                                    text::label(
                                        "キャラクター名",
                                        &self.element_id.input_character_name,
                                    ),
                                    Html::input(
                                        Attributes::new()
                                            .value(character.name())
                                            .id(&self.element_id.input_character_name),
                                        Events::new().on_input(|name| {
                                            Msg::Sub(On::SetCommonProps {
                                                name: Some(name),
                                                display_name: None,
                                                description: None,
                                                name_color: None,
                                            })
                                        }),
                                        vec![],
                                    ),
                                ],
                            ),
                            Html::textarea(
                                Attributes::new()
                                    .value(character.description())
                                    .class(Self::class("common-description")),
                                Events::new().on_input(|description| {
                                    Msg::Sub(On::SetCommonProps {
                                        name: None,
                                        display_name: None,
                                        description: Some(description),
                                        name_color: None,
                                    })
                                }),
                                vec![],
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new(),
                        Events::new(),
                        vec![ColorPallet::empty(
                            color_pallet::Props {
                                default_selected: character.name_color().clone(),
                                title: Some(String::from("キャラクター色")),
                            },
                            Subscription::new(move |sub| match sub {
                                color_pallet::On::SelectColor(name_color) => {
                                    Msg::Sub(On::SetCommonProps {
                                        name: None,
                                        display_name: None,
                                        description: None,
                                        name_color: Some(name_color),
                                    })
                                }
                            }),
                        )],
                    ),
                ],
            )],
        )
    }

    fn render_tex(&self, character: &block::character::Character) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("content-base"))
                .class("pure-form"),
            Events::new(),
            vec![Html::div(
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
                            vec![Btn::dark(
                                Attributes::new(),
                                Events::new().on("click", |e| {
                                    e.stop_propagation();
                                    Msg::Sub(On::AddTexture)
                                }),
                                vec![Html::text("追加")],
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
                                        .draggable(false)
                                        .class(Self::class("common-imgs-img"))
                                        .src(img.url().as_ref()),
                                    Events::new(),
                                    vec![],
                                )
                            })
                            .unwrap_or(Html::none())],
                    ),
                    Btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetModal(Modal::ImportedFiles)),
                        vec![Html::text("画像を選択")],
                    ),
                ],
            )],
        )
    }

    fn render_tex_list_item(tex_idx: usize, tex_name: &str) -> Html {
        Html::div(
            Attributes::new().class(Self::class("common-imgs-list-item")),
            Events::new(),
            vec![
                Btn::menu(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::Sub(On::SetTextureIdx { tex_idx })),
                    vec![Html::text(tex_name)],
                ),
                Btn::danger(
                    Attributes::new(),
                    Events::new().on("click", move |e| {
                        e.stop_propagation();
                        Msg::Sub(On::RemoveTexture { tex_idx: tex_idx })
                    }),
                    vec![Html::text("削除")],
                ),
            ],
        )
    }

    fn render_tab(&self, prop_id: &BlockId) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("content-base"))
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
                    )
                })
                .unwrap_or(Html::none())],
        )
    }

    fn render_bottom_menu(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("bottom-menu")),
            Events::new(),
            vec![
                Html::div(Attributes::new(), Events::new(), vec![]),
                Html::div(
                    Attributes::new(),
                    Events::new(),
                    vec![Btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::PackToDownload),
                        vec![Html::text("ダウンロード")],
                    )],
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
            ".base" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "1fr max-content";
                "height": "100%";
            }

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

            ".base textarea" {
                "resize": "none";
            }

            ".common" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "grid-template-rows": "20rem";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            ".common-props" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content 1fr";
                "grid-auto-flow": "row";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }

            ".common-imgs" {
                "display": "grid";
                "grid-template-rows": "max-content max-content 1fr max-content";
                "row-gap": "0.35em";
                "max-height": "100%";
                "min-height": "100%";
            }

            ".common-imgs-container" {
                "overflow": "hidden";
            }

            ".common-imgs-img" {
                "height": "100%";
                "width": "100%";
                "object-fit": "contain";
            }

            ".common-imgs-list-item" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".15em";
            }

            ".key-value" {
                "display": "grid";
                "column-gap": ".35em";
                "row-gap": ".65em";
                "grid-template-columns": "max-content 1fr";
            }

            ".root-prop" {
                "display": "grid";
                "row-gap": ".65em";
                "grid-template-columns": "1fr";
            }

            ".bottom-menu" {
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "padding": "0.6ch 1.2ch";
            }
        }
    }
}
