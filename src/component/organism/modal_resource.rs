use super::atom::common::Common;
use super::atom::{
    attr,
    btn::{self, Btn},
    text::Text,
};
use super::molecule::modal::{self, Modal};
use super::organism::modal_create_block_texture::{self, ModalCreateBlockTexture};
use super::organism::modal_create_terran_texture::{self, ModalCreateTerranTexture};
use crate::arena::{block, component, resource, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

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
    EditResource(Resource),
    CloseModal,
    LoadBlockTexture(resource::BlockTexture),
    SelectResource(Resource),
    SetSelectedKind(BlockKind),
    SetSelectedResource(Resource),
}

pub enum On {
    Close,
    SelectNone,
    SelectImageData(BlockRef<resource::ImageData>),
    SelectBlockTexture(BlockRef<resource::BlockTexture>),
    SelectTerranTexture(BlockMut<block::TerranTexture>),
    SelectComponent(U128Id),
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub enum Resource {
    None,
    ImageData(BlockRef<resource::ImageData>),
    BlockTexture(BlockRef<resource::BlockTexture>),
    TerranTexture(BlockMut<block::TerranTexture>),
    BoxblockComponent(BlockMut<component::BoxblockComponent>),
    CraftboardComponent(BlockMut<component::CraftboardComponent>),
    TextboardComponent(BlockMut<component::TextboardComponent>),
}

impl Resource {
    fn id(&self) -> U128Id {
        match self {
            Self::None => U128Id::none(),
            Self::ImageData(data) => data.id(),
            Self::BlockTexture(data) => data.id(),
            Self::TerranTexture(data) => data.id(),
            Self::BoxblockComponent(data) => data.id(),
            Self::CraftboardComponent(data) => data.id(),
            Self::TextboardComponent(data) => data.id(),
        }
    }
}

pub struct ModalResource {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    selected_kind: BlockKind,
    filter: HashSet<BlockKind>,
    selected_resource: Resource,
    is_selecter: bool,
    title: String,
    showing_modal: ShowingModal,
}

enum ShowingModal {
    None,
    CreateBlockTexture,
    CreateTerranTexture(BlockMut<block::TerranTexture>),
}

impl Component for ModalResource {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalResource {}

impl Constructor for ModalResource {
    fn constructor(props: Props) -> Self {
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
            arena: props.arena,
            world: props.world,
            filter: props.filter,
            selected_kind,
            selected_resource: Resource::None,
            is_selecter: props.is_selecter,
            title: props.title,
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for ModalResource {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.world = props.world;

        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::CloseModal => {
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
            Msg::AddResource(kind) => match kind {
                BlockKind::BlockTexture => {
                    self.showing_modal = ShowingModal::CreateBlockTexture;
                    Cmd::none()
                }
                BlockKind::TerranTexture => {
                    self.showing_modal = ShowingModal::CreateTerranTexture(BlockMut::none());
                    Cmd::none()
                }
                _ => Cmd::none(),
            },
            Msg::EditResource(resource) => match resource {
                Resource::TerranTexture(block) => {
                    self.showing_modal = ShowingModal::CreateTerranTexture(block);
                    Cmd::none()
                }
                _ => Cmd::none(),
            },
            Msg::LoadBlockTexture(texture) => {
                let texture = self.arena.insert(texture).as_ref();
                let texture_id = texture.id();

                self.world.update(|world| {
                    world.push_block_texture_resource(texture);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::submit(On::UpdateBlocks {
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
                    Resource::None => Cmd::submit(On::SelectNone),
                    Resource::ImageData(data) => Cmd::submit(On::SelectImageData(data)),
                    Resource::BlockTexture(data) => Cmd::submit(On::SelectBlockTexture(data)),
                    Resource::TerranTexture(data) => Cmd::submit(On::SelectTerranTexture(data)),
                    Resource::BoxblockComponent(data) => {
                        Cmd::submit(On::SelectComponent(data.id()))
                    }
                    Resource::CraftboardComponent(data) => {
                        Cmd::submit(On::SelectComponent(data.id()))
                    }
                    Resource::TextboardComponent(data) => {
                        Cmd::submit(On::SelectComponent(data.id()))
                    }
                    _ => Cmd::none(),
                }
            }
        }
    }
}

impl Render<Html> for ModalResource {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Modal::new(
            self,
            None,
            modal::Props {},
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Sub(On::Close),
            }),
            (
                self.title.clone(),
                String::from(""),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("base")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Self::class("kind-list")),
                                Events::new(),
                                vec![
                                    self.render_btn_to_select_kind(
                                        BlockKind::ImageData,
                                        Html::text("画像"),
                                    ),
                                    self.render_btn_to_select_kind(
                                        BlockKind::BlockTexture,
                                        Text::condense_75("ブロック用テクスチャ"),
                                    ),
                                    self.render_btn_to_select_kind(
                                        BlockKind::TerranTexture,
                                        Text::condense_75("地形用テクスチャ"),
                                    ),
                                    if self.filter.is_empty()
                                        || self.filter.contains(&BlockKind::BoxblockComponent)
                                        || self.filter.contains(&BlockKind::CraftboardComponent)
                                        || self.filter.contains(&BlockKind::TextboardComponent)
                                    {
                                        self.render_group_to_select_kind(Text::condense_75(
                                            "コンポーネント",
                                        ))
                                    } else {
                                        Html::none()
                                    },
                                    self.render_btn_to_select_kind(
                                        BlockKind::CraftboardComponent,
                                        Html::text("盤面"),
                                    ),
                                    self.render_btn_to_select_kind(
                                        BlockKind::BoxblockComponent,
                                        Html::text("ブロック"),
                                    ),
                                    self.render_btn_to_select_kind(
                                        BlockKind::TextboardComponent,
                                        Html::text("メモ"),
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new().class(Self::class("resource-list")),
                                Events::new(),
                                match &self.selected_kind {
                                    BlockKind::ImageData => self.render_list_image_data(),
                                    BlockKind::BlockTexture => self.render_list_block_texture(),
                                    BlockKind::TerranTexture => self.render_list_terran_texture(),
                                    BlockKind::BoxblockComponent => {
                                        self.render_list_boxblock_component()
                                    }
                                    BlockKind::CraftboardComponent => {
                                        self.render_list_craftboard_component()
                                    }
                                    BlockKind::TextboardComponent => {
                                        self.render_list_textboard_component()
                                    }
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
                    self.render_modal(),
                ],
            ),
        ))
    }
}

impl ModalResource {
    fn render_modal(&self) -> Html {
        match &self.showing_modal {
            ShowingModal::None => Html::none(),
            ShowingModal::CreateBlockTexture => ModalCreateBlockTexture::empty(
                self,
                None,
                modal_create_block_texture::Props {
                    arena: ArenaMut::clone(&self.arena),
                    world: BlockMut::clone(&self.world),
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
            ShowingModal::CreateTerranTexture(terran_texture) => ModalCreateTerranTexture::empty(
                self,
                None,
                modal_create_terran_texture::Props {
                    arena: ArenaMut::clone(&self.arena),
                    world: BlockMut::clone(&self.world),
                    terran_texture: BlockMut::clone(&terran_texture),
                },
                Sub::map(|sub| match sub {
                    modal_create_terran_texture::On::Close => Msg::CloseModal,
                    modal_create_terran_texture::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                }),
            ),
        }
    }

    fn render_btn_to_select_kind(&self, kind: BlockKind, text: Html) -> Html {
        if self.filter.is_empty() || self.filter.contains(&kind) {
            Btn::with_variant(
                if self.selected_kind == kind {
                    btn::Variant::PrimaryLikeMenu
                } else {
                    btn::Variant::LightLikeMenu
                },
                Attributes::new(),
                Events::new().on_click(self, move |_| Msg::SetSelectedKind(kind)),
                vec![text],
            )
        } else {
            Html::none()
        }
    }

    fn render_group_to_select_kind(&self, text: Html) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("group-to-select-kind"))
                .class(Self::class("text")),
            Events::new(),
            vec![text],
        )
    }

    fn render_list_image_data(&self) -> Vec<Html> {
        self.world
            .map(|world| {
                vec![
                    if self.is_selecter {
                        self.render_cell_none("画像無し")
                    } else {
                        Html::none()
                    },
                    Html::fragment(
                        world
                            .image_data_resources()
                            .iter()
                            .map(|data| self.render_cell_image_data(BlockRef::clone(&data)))
                            .collect(),
                    ),
                    self.render_btn_to_add_cell(BlockKind::ImageData),
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_list_block_texture(&self) -> Vec<Html> {
        self.world
            .map(|world| {
                vec![
                    if self.is_selecter {
                        self.render_cell_none("テクスチャ無し")
                    } else {
                        Html::none()
                    },
                    Html::fragment(
                        world
                            .block_texture_resources()
                            .iter()
                            .map(|data| self.render_cell_block_texture(BlockRef::clone(&data)))
                            .collect(),
                    ),
                    self.render_btn_to_add_cell(BlockKind::BlockTexture),
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_list_terran_texture(&self) -> Vec<Html> {
        self.world
            .map(|world| {
                vec![
                    if self.is_selecter {
                        self.render_cell_none("テクスチャ無し")
                    } else {
                        Html::none()
                    },
                    Html::fragment(
                        world
                            .terran_texture_blocks()
                            .iter()
                            .map(|data| self.render_cell_terran_texture(BlockMut::clone(&data)))
                            .collect(),
                    ),
                    self.render_btn_to_add_cell(BlockKind::TerranTexture),
                ]
            })
            .unwrap_or(vec![])
    }

    fn render_list_boxblock_component(&self) -> Vec<Html> {
        self.world
            .map(|world| {
                world
                    .components()
                    .boxblocks()
                    .iter()
                    .map(|data| self.render_cell_boxblock_component(BlockMut::clone(data)))
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn render_list_craftboard_component(&self) -> Vec<Html> {
        self.world
            .map(|world| {
                world
                    .components()
                    .craftboards()
                    .iter()
                    .map(|data| self.render_cell_craftboard_component(BlockMut::clone(data)))
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn render_list_textboard_component(&self) -> Vec<Html> {
        self.world
            .map(|world| {
                world
                    .components()
                    .textboards()
                    .iter()
                    .map(|data| self.render_cell_textboard_component(BlockMut::clone(data)))
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn render_cell_none(&self, name: impl Into<String>) -> Html {
        Html::div(
            Attributes::new().class(Self::class("cell")),
            Events::new().on_click(self, move |_| Msg::SetSelectedResource(Resource::None)),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("cell-img")),
                    Events::new(),
                    vec![],
                ),
                attr::span(Attributes::new().class(Self::class("text")), name),
                if self.is_selecter {
                    self.render_btn_to_select_cell(Resource::None)
                } else {
                    Html::none()
                },
            ],
        )
    }

    fn render_cell_image_data(&self, data: BlockRef<resource::ImageData>) -> Html {
        BlockRef::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new(),
                    vec![
                        Html::img(
                            Attributes::new()
                                .draggable("false")
                                .class(Self::class("cell-img"))
                                .class(Common::bg_transparent())
                                .src(this.url().to_string()),
                            Events::new(),
                            vec![],
                        ),
                        attr::span(Attributes::new().class(Self::class("text")), this.name()),
                        if self.is_selecter {
                            self.render_btn_to_select_cell(Resource::ImageData(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_cell_block_texture(&self, data: BlockRef<resource::BlockTexture>) -> Html {
        BlockRef::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new(),
                    vec![
                        Html::img(
                            Attributes::new()
                                .draggable("false")
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
                        if self.is_selecter {
                            self.render_btn_to_select_cell(Resource::BlockTexture(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_cell_terran_texture(&self, data: BlockMut<block::TerranTexture>) -> Html {
        BlockMut::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new(),
                    vec![
                        Html::node(this.data().clone().into()),
                        {
                            let data = BlockMut::clone(&data);
                            self.render_btn_to_edit_cell(Resource::TerranTexture(data))
                        },
                        if self.is_selecter {
                            self.render_btn_to_select_cell(Resource::TerranTexture(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_cell_boxblock_component(&self, data: BlockMut<component::BoxblockComponent>) -> Html {
        BlockMut::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new(),
                    vec![
                        self.render_cell_boxblock_component_img(this),
                        attr::span(Attributes::new().class(Self::class("text")), this.name()),
                        if self.is_selecter {
                            self.render_btn_to_select_cell(Resource::BoxblockComponent(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_cell_boxblock_component_img(&self, data: &component::BoxblockComponent) -> Html {
        data.texture()
            .and_then(|texture| {
                texture.map(|texture| {
                    Html::img(
                        Attributes::new()
                            .draggable("false")
                            .class(Self::class("cell-img"))
                            .class(Common::bg_transparent())
                            .src(texture.data().url().to_string()),
                        Events::new(),
                        vec![],
                    )
                })
            })
            .unwrap_or_else(|| self.render_cell_boxblock_component_bgcolor(data))
    }

    fn render_cell_boxblock_component_bgcolor(&self, data: &component::BoxblockComponent) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("cell-tile"))
                .style("background-color", data.color().to_string()),
            Events::new(),
            vec![],
        )
    }

    fn render_cell_craftboard_component(
        &self,
        data: BlockMut<component::CraftboardComponent>,
    ) -> Html {
        BlockMut::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new()
                                .class(Common::layered())
                                .style(
                                    "width",
                                    format!(
                                        "{}rem",
                                        (10.0 * this.size()[0] / this.size()[1]).min(10.0)
                                    ),
                                )
                                .style(
                                    "height",
                                    format!(
                                        "{}rem",
                                        (10.0 * this.size()[1] / this.size()[0]).min(10.0)
                                    ),
                                ),
                            Events::new(),
                            vec![
                                this.textures()
                                    .nz
                                    .as_ref()
                                    .and_then(|texture| {
                                        texture.map(|texture| {
                                            Html::img(
                                                Attributes::new()
                                                    .draggable("false")
                                                    .class(Self::class("fill-img"))
                                                    .class(Common::bg_transparent())
                                                    .class(Common::layered_item())
                                                    .src(texture.url().to_string()),
                                                Events::new(),
                                                vec![],
                                            )
                                        })
                                    })
                                    .unwrap_or_else(|| Html::none()),
                                Html::div(
                                    Attributes::new().class(Common::layered_item()).style(
                                        "background",
                                        Self::style_grid_line(
                                            this.size()[1] as u32,
                                            this.size()[0] as u32,
                                        ),
                                    ),
                                    Events::new(),
                                    vec![],
                                ),
                            ],
                        ),
                        attr::span(Attributes::new().class(Self::class("text")), this.name()),
                        if self.is_selecter {
                            self.render_btn_to_select_cell(Resource::CraftboardComponent(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn render_cell_textboard_component(
        &self,
        data: BlockMut<component::TextboardComponent>,
    ) -> Html {
        BlockMut::clone(&data)
            .map(|this| {
                Html::div(
                    Attributes::new().class(Self::class("cell")),
                    Events::new(),
                    vec![
                        Html::pre(
                            Attributes::new()
                                .class(Self::class("cell-tile"))
                                .class(Self::class("cell-textboard")),
                            Events::new(),
                            vec![Html::text(this.text())],
                        ),
                        attr::span(Attributes::new().class(Self::class("text")), this.title()),
                        if self.is_selecter {
                            self.render_btn_to_select_cell(Resource::TextboardComponent(data))
                        } else {
                            Html::none()
                        },
                    ],
                )
            })
            .unwrap_or(Html::none())
    }

    fn style_grid_line(rows: u32, cols: u32) -> String {
        format!(
            "repeating-linear-gradient(0deg, #000, #000 1px, transparent 1px, transparent calc((100% - 1px) / {})),\
            repeating-linear-gradient(90deg, #000, #000 1px, transparent 1px, transparent calc((100% - 1px) / {}))",
            rows, cols
        )
    }

    fn render_btn_to_select_cell(&self, resource: Resource) -> Html {
        Btn::secondary(
            Attributes::new(),
            Events::new().on_click(self, move |_| Msg::SelectResource(resource)),
            vec![Html::text("選択")],
        )
    }

    fn render_btn_to_edit_cell(&self, resource: Resource) -> Html {
        Btn::secondary(
            Attributes::new(),
            Events::new().on_click(self, move |_| Msg::EditResource(resource)),
            vec![Html::text("編集")],
        )
    }

    fn render_btn_to_add_cell(&self, kind: BlockKind) -> Html {
        Btn::secondary(
            Attributes::new().class(Self::class("cell")),
            Events::new().on_click(self, move |_| Msg::AddResource(kind)),
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

            ".kind-list > *" {
                "margin-top": ".35rem";
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

            ".cell > canvas" {
                "width": "10rem";
                "height": "10rem";
            }

            ".cell-img" {
                "height": "10rem";
                "object-fit": "contain";
            }

            ".cell-tile" {
                "height": "10rem";
                "width": "10rem";
            }

            ".cell .text" {
                "text-overflow": "ellipsis";
                "overflow": "hidden";
                "max-width": "100%";
            }

            ".text" {
                "color": crate::libs::color::Pallet::gray(9);
            }

            ".group-to-select-kind" {
                "margin-top": "1rem";
                "padding-left": "1em";
                "padding-right": "1em";
            }

            ".fill-img" {
                "width": "100%";
                "height": "100%";
                "object-fit": "fill";
            }

            ".cell-textboard" {
                "text-overflow": "ellipsis";
                "overflow": "hidden";
            }
        }
    }
}
