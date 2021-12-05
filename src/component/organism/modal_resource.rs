use super::atom::{
    attr,
    btn::{self, Btn},
    heading::{self, Heading},
    text,
};
use super::molecule::modal::{self, Modal};
use super::organism::modal_create_block_texture::{self, ModalCreateBlockTexture};
use super::template::common::Common;
use crate::arena::{block, resource, ArenaMut, BlockKind, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;

pub mod title {
    pub static VIEW_ALL_RESOURCE: &str = "リソース一覧";
    pub static SELECT_BLOCK_TEXTURE: &str = "ブロック用のテクスチャを選択";
    pub static SELECT_TEXTURE: &str = "画像を選択";
}

pub struct Props {
    pub arena: ArenaMut,
    pub filter: HashSet<BlockKind>,
    pub world: BlockMut<block::World>,
    pub is_selecter: bool,
    pub title: String,
}

pub enum Msg {
    Sub(On),
    AddResource(BlockKind),
    CloseModal,
    LoadBlockTexture(resource::BlockTexture),
    SelectResource(Resource),
    SetSelectedKind(BlockKind),
    SetSelectedResource(Resource),
}

pub enum On {
    Close,
    SelectNone,
    SelectImageData(BlockMut<resource::ImageData>),
    SelectBlockTexture(BlockMut<resource::BlockTexture>),
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub enum Resource {
    None,
    ImageData(BlockMut<resource::ImageData>),
    BlockTexture(BlockMut<resource::BlockTexture>),
}

impl Resource {
    fn id(&self) -> U128Id {
        match self {
            Self::None => U128Id::none(),
            Self::ImageData(data) => data.id(),
            Self::BlockTexture(data) => data.id(),
        }
    }
}

pub struct ModalResource {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    selected_kind: BlockKind,
    selected_resource: Resource,
    showing_modal: ShowingModal,
}

enum ShowingModal {
    None,
    CreateBlockTexture,
}

impl Component for ModalResource {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ModalResource {
    fn constructor(props: &Props) -> Self {
        let selected_kind = if props.filter.is_empty() {
            BlockKind::ImageData
        } else if props.filter.contains(&BlockKind::ImageData) {
            BlockKind::ImageData
        } else if props.filter.contains(&BlockKind::BlockTexture) {
            BlockKind::BlockTexture
        } else {
            BlockKind::None
        };

        Self {
            arena: ArenaMut::clone(&props.arena),
            world: BlockMut::clone(&props.world),
            selected_kind,
            selected_resource: Resource::None,
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for ModalResource {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        self.on_load(props)
    }

    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.arena = ArenaMut::clone(&props.arena);
        self.world = BlockMut::clone(&props.world);

        Cmd::none()
    }

    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::CloseModal => {
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
            Msg::AddResource(kind) => match kind {
                BlockKind::BlockTexture => {
                    self.showing_modal = ShowingModal::CreateBlockTexture;
                    Cmd::none()
                }
                _ => Cmd::none(),
            },
            Msg::LoadBlockTexture(texture) => {
                let texture = self.arena.insert(texture);
                let texture_id = texture.id();

                self.world.update(|world| {
                    world.push_block_texture_resource(texture);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::sub(On::UpdateBlocks {
                    insert: set! { texture_id },
                    update: set! { self.world.id() },
                })
            }
            Msg::SetSelectedKind(kind) => {
                self.selected_kind = kind;
                Cmd::none()
            }
            Msg::SetSelectedResource(resource) => {
                self.selected_resource = resource;
                Cmd::none()
            }
            Msg::SelectResource(resource) => {
                crate::debug::log_1("Msg::SelectResource");
                match resource {
                    Resource::None => Cmd::sub(On::SelectNone),
                    Resource::ImageData(data) => Cmd::sub(On::SelectImageData(data)),
                    Resource::BlockTexture(data) => Cmd::sub(On::SelectBlockTexture(data)),
                }
            }
        }
    }
}

impl Render for ModalResource {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Modal::with_children(
            modal::Props {
                header_title: props.title.clone(),
                footer_message: String::from(""),
            },
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Sub(On::Close),
            }),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("base")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("kind-list")),
                            Events::new(),
                            vec![
                                self.render_btn_to_select_kind(props, BlockKind::ImageData, "画像"),
                                self.render_btn_to_select_kind(
                                    props,
                                    BlockKind::BlockTexture,
                                    "ブロック用テクスチャ",
                                ),
                            ],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("resource-list")),
                            Events::new(),
                            match &self.selected_kind {
                                BlockKind::ImageData => self.render_list_image_data(props),
                                BlockKind::BlockTexture => self.render_list_block_texture(props),
                                _ => vec![],
                            },
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("detail")),
                            Events::new(),
                            vec![],
                        ),
                    ],
                ),
                self.render_modal(props),
            ],
        ))
    }
}

impl ModalResource {
    fn render_modal(&self, props: &Props) -> Html<Self> {
        match &self.showing_modal {
            ShowingModal::None => Html::none(),
            ShowingModal::CreateBlockTexture => ModalCreateBlockTexture::empty(
                modal_create_block_texture::Props {
                    arena: ArenaMut::clone(&props.arena),
                    world: BlockMut::clone(&props.world),
                },
                Sub::map(|sub| match sub {
                    modal_create_block_texture::On::Close => Msg::CloseModal,
                    modal_create_block_texture::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                    modal_create_block_texture::On::CreateTexure(texture) => {
                        Msg::LoadBlockTexture(texture)
                    }
                }),
            ),
        }
    }

    fn render_btn_to_select_kind(
        &self,
        props: &Props,
        kind: BlockKind,
        text: impl Into<String>,
    ) -> Html<Self> {
        if props.filter.is_empty() || props.filter.contains(&kind) {
            Btn::with_variant(
                if self.selected_kind == kind {
                    btn::Variant::PrimaryLikeMenu
                } else {
                    btn::Variant::LightLikeMenu
                },
                Attributes::new(),
                Events::new().on_click(move |_| Msg::SetSelectedKind(kind)),
                vec![Html::text(text)],
            )
        } else {
            Html::none()
        }
    }

    fn render_list_image_data(&self, props: &Props) -> Vec<Html<Self>> {
        self.world
            .map(|world| {
                vec![
                    if props.is_selecter {
                        self.render_cell_none(props, "画像無し")
                    } else {
                        Html::none()
                    },
                    Html::fragment(
                        world
                            .image_data_resources()
                            .iter()
                            .map(|data| self.render_cell_image_data(props, BlockMut::clone(&data)))
                            .collect(),
                    ),
                    Self::render_btn_to_add_cell(BlockKind::ImageData),
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_list_block_texture(&self, props: &Props) -> Vec<Html<Self>> {
        self.world
            .map(|world| {
                vec![
                    if props.is_selecter {
                        self.render_cell_none(props, "テクスチャ無し")
                    } else {
                        Html::none()
                    },
                    Html::fragment(
                        world
                            .block_texture_resources()
                            .iter()
                            .map(|data| {
                                self.render_cell_block_texture(props, BlockMut::clone(&data))
                            })
                            .collect(),
                    ),
                    Self::render_btn_to_add_cell(BlockKind::BlockTexture),
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_cell_none(&self, props: &Props, name: impl Into<String>) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("cell")),
            Events::new().on_click(move |_| Msg::SetSelectedResource(Resource::None)),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("cell-img")),
                    Events::new(),
                    vec![],
                ),
                attr::span(Attributes::new().class(Self::class("text")), name),
                if props.is_selecter {
                    Self::render_btn_to_select_cell(Resource::None)
                } else {
                    Html::none()
                },
            ],
        )
    }

    fn render_cell_image_data(
        &self,
        props: &Props,
        data: BlockMut<resource::ImageData>,
    ) -> Html<Self> {
        BlockMut::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new().on_click({
                        let data = BlockMut::clone(&data);
                        move |_| Msg::SetSelectedResource(Resource::ImageData(data))
                    }),
                    vec![
                        Html::img(
                            Attributes::new()
                                .class(Self::class("cell-img"))
                                .class(Common::bg_transparent())
                                .src(this.url().to_string()),
                            Events::new(),
                            vec![],
                        ),
                        attr::span(Attributes::new().class(Self::class("text")), this.name()),
                        if props.is_selecter {
                            Self::render_btn_to_select_cell(Resource::ImageData(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_cell_block_texture(
        &self,
        props: &Props,
        data: BlockMut<resource::BlockTexture>,
    ) -> Html<Self> {
        BlockMut::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new().on_click({
                        let data = BlockMut::clone(&data);
                        move |_| Msg::SetSelectedResource(Resource::BlockTexture(data))
                    }),
                    vec![
                        Html::img(
                            Attributes::new()
                                .class(Self::class("cell-img"))
                                .class(Common::bg_transparent())
                                .src(this.data().url().to_string()),
                            Events::new(),
                            vec![],
                        ),
                        attr::span(
                            Attributes::new().class(Self::class("text")),
                            this.data().name(),
                        ),
                        if props.is_selecter {
                            Self::render_btn_to_select_cell(Resource::BlockTexture(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_btn_to_select_cell(resource: Resource) -> Html<Self> {
        Btn::secondary(
            Attributes::new(),
            Events::new().on_click(move |_| Msg::SelectResource(resource)),
            vec![Html::text("選択")],
        )
    }

    fn render_btn_to_add_cell(kind: BlockKind) -> Html<Self> {
        Btn::secondary(
            Attributes::new().class(Self::class("cell")),
            Events::new().on_click(move |_| Msg::AddResource(kind)),
            vec![Html::text("追加")],
        )
    }
}

impl Styled for ModalResource {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "1fr max-content";
                "height": "100%";
                "overflow": "hidden";
            }
            ".kind-list, .resource-list, .detail" {
                "overflow-y": "scroll";
            }
            ".kind-list" {
                "grid-column": "1 / 2";
                "grid-row": "1 / 3";
                "display": "flex";
                "flex-direction": "column";
            }

            ".resource-list" {
                "grid-column": "2 / 3";
                "grid-row": "1 / 2";
                "display": "grid";
                "grid-template-columns": "repeat(auto-fill, minmax(10rem, 1fr))";
                "grid-auto-rows": "max-content";
            }

            ".detail" {
                "grid-column": "2 / 3";
                "grid-row": "2 / 3";
            }

            ".cell" {
                "min-height": "10rem";
                "width": "100%";
                "overflow": "hidden";
                "display": "flex";
                "flex-direction": "column";
                "justify-content": "center";
                "align-items": "center";
            }

            ".cell-img" {
                "height": "10rem";
                "object-fit": "contain";
            }

            ".cell .text" {
                "text-overflow": "ellipsis";
                "overflow": "hidden";
                "max-width": "100%";
            }

            ".text" {
                "color": crate::libs::color::Pallet::gray(9);
            }
        }
    }
}
