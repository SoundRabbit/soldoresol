use super::super::atom::btn::{self, Btn};
use super::super::atom::dropdown::{self, Dropdown};
use super::super::modal_imported_files::{self, ModalImportedFiles};
use super::super::util::styled::{Style, Styled};
use crate::arena::block;
use crate::arena::resource;
use async_std::sync::{Arc, Mutex};
use kagura::prelude::*;

pub struct Props {
    pub block_arena: block::ArenaRef,
    pub resource_arena: resource::ArenaRef,
    pub character_id: block::BlockId,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetModal(Modal),
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
}

pub struct Character {
    block_arena: block::ArenaRef,
    resource_arena: resource::ArenaRef,
    character_id: block::BlockId,
    element_id: ElementId,
    modal: Modal,
}

struct ElementId {
    input_character_name: String,
}

impl Constructor for Character {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            block_arena: props.block_arena,
            resource_arena: props.resource_arena,
            character_id: props.character_id,
            element_id: ElementId {
                input_character_name: format!("{:X}", crate::libs::random_id::u128val()),
            },
            modal: Modal::None,
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
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(
            self.block_arena
                .map(
                    &self.character_id,
                    |character: &block::character::Character| self.render_character(character),
                )
                .unwrap_or(Html::none()),
        )
    }
}

impl Character {
    fn render_character(&self, character: &block::character::Character) -> Html {
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
                                Html::input(
                                    Attributes::new()
                                        .value(character.name())
                                        .id(&self.element_id.input_character_name),
                                    Events::new(),
                                    vec![],
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
                                        vec![Btn::with_child(
                                            btn::Props {
                                                variant: btn::Variant::Dark,
                                            },
                                            Subscription::new(|sub| match sub {
                                                btn::On::Click => Msg::Sub(On::AddTexture),
                                            }),
                                            Html::text("追加"),
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
                Btn::with_child(
                    btn::Props {
                        variant: btn::Variant::Danger,
                    },
                    Subscription::new(move |sub| match sub {
                        btn::On::Click => Msg::Sub(On::RemoveTexture { tex_idx: tex_idx }),
                    }),
                    Html::text("削除"),
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
                "column-gap": ".35em";
                "row-gap": ".65em";
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

            "common-imgs-list" {
                "display": "grid";
                "grid-template-columns": "1fr max-content max-content";
            }

            "common-imgs-list-item" {
                "display": "grid";
                "grid-template-columns": "1fr max-content max-content";
                "column-gap": ".15em";
            }
        }
    }
}
