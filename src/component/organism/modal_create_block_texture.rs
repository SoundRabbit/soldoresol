use super::atom::{
    btn::{self, Btn},
    text,
};
use super::molecule::modal::{self, Modal};
use super::organism::modal_resource::{self, ModalResource};
use super::template::common::Common;
use crate::arena::{
    block,
    resource::{self, LoadFrom},
    ArenaMut, BlockKind, BlockMut, BlockRef,
};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    CloseModal,
    CreateTexure,
    SelectCustomTextureImage(TextureDirection),
    SelectPrefabTextureImage,
    SetCustomTextureImage(TextureDirection, Option<BlockRef<resource::ImageData>>),
    SetPrefabTextureImage(Option<BlockRef<resource::ImageData>>),
    SetSelectingKind(TextureKind),
}

pub enum On {
    Close,
    CreateTexure(resource::BlockTexture),
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct ModalCreateBlockTexture {
    custom_texture: CustomTexture,
    prefab_texture: Option<BlockRef<resource::ImageData>>,
    selecting_kind: TextureKind,
    showing_modal: ShowingModal,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureKind {
    CustomTexture,
    PrefabTexture,
}

struct CustomTexture {
    texture_px: Option<BlockRef<resource::ImageData>>,
    texture_py: Option<BlockRef<resource::ImageData>>,
    texture_pz: Option<BlockRef<resource::ImageData>>,
    texture_nx: Option<BlockRef<resource::ImageData>>,
    texture_ny: Option<BlockRef<resource::ImageData>>,
    texture_nz: Option<BlockRef<resource::ImageData>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureDirection {
    PX,
    PY,
    PZ,
    NX,
    NY,
    NZ,
}

enum ShowingModal {
    None,
    SelectCustomTextureImage(TextureDirection),
    SelectPrefabTextureImage,
}

impl std::fmt::Display for TextureDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PX => write!(f, "px"),
            Self::PY => write!(f, "py"),
            Self::PZ => write!(f, "pz"),
            Self::NX => write!(f, "nx"),
            Self::NY => write!(f, "ny"),
            Self::NZ => write!(f, "nz"),
        }
    }
}

impl Component for ModalCreateBlockTexture {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ModalCreateBlockTexture {
    fn constructor(props: &Props) -> Self {
        Self {
            custom_texture: CustomTexture {
                texture_px: None,
                texture_py: None,
                texture_pz: None,
                texture_nx: None,
                texture_ny: None,
                texture_nz: None,
            },
            prefab_texture: None,
            selecting_kind: TextureKind::CustomTexture,
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for ModalCreateBlockTexture {
    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
            Msg::CloseModal => {
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
            Msg::CreateTexure => match &self.selecting_kind {
                TextureKind::CustomTexture => {
                    let px = self
                        .custom_texture
                        .texture_px
                        .clone()
                        .unwrap_or(BlockRef::none());
                    let py = self
                        .custom_texture
                        .texture_py
                        .clone()
                        .unwrap_or(BlockRef::none());
                    let pz = self
                        .custom_texture
                        .texture_pz
                        .clone()
                        .unwrap_or(BlockRef::none());
                    let nx = self
                        .custom_texture
                        .texture_nx
                        .clone()
                        .unwrap_or(BlockRef::none());
                    let ny = self
                        .custom_texture
                        .texture_ny
                        .clone()
                        .unwrap_or(BlockRef::none());
                    let nz = self
                        .custom_texture
                        .texture_nz
                        .clone()
                        .unwrap_or(BlockRef::none());
                    Cmd::task(|resolve| {
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Some(texture) = resource::BlockTexture::load_from((
                                [1024, 1024],
                                [px, py, pz, nx, ny, nz],
                            ))
                            .await
                            {
                                resolve(Msg::Sub(On::CreateTexure(texture)));
                            } else {
                                resolve(Msg::NoOp);
                            }
                        })
                    })
                }
                TextureKind::PrefabTexture => {
                    let texture = self.prefab_texture.clone().unwrap_or(BlockRef::none());
                    Cmd::task(|resolve| {
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Some(texture) = resource::BlockTexture::load_from(texture).await
                            {
                                resolve(Msg::Sub(On::CreateTexure(texture)));
                            } else {
                                resolve(Msg::NoOp);
                            }
                        })
                    })
                }
            },
            Msg::SelectCustomTextureImage(direction) => {
                self.showing_modal = ShowingModal::SelectCustomTextureImage(direction);
                Cmd::none()
            }
            Msg::SelectPrefabTextureImage => {
                self.showing_modal = ShowingModal::SelectPrefabTextureImage;
                Cmd::none()
            }
            Msg::SetCustomTextureImage(direction, img) => {
                match direction {
                    TextureDirection::PX => {
                        self.custom_texture.texture_px = img;
                    }
                    TextureDirection::PY => {
                        self.custom_texture.texture_py = img;
                    }
                    TextureDirection::PZ => {
                        self.custom_texture.texture_pz = img;
                    }
                    TextureDirection::NX => {
                        self.custom_texture.texture_nx = img;
                    }
                    TextureDirection::NY => {
                        self.custom_texture.texture_ny = img;
                    }
                    TextureDirection::NZ => {
                        self.custom_texture.texture_nz = img;
                    }
                }
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
            Msg::SetPrefabTextureImage(img) => {
                self.prefab_texture = img;
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
            Msg::SetSelectingKind(kind) => {
                self.selecting_kind = kind;
                Cmd::none()
            }
        }
    }
}

impl Render for ModalCreateBlockTexture {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Modal::with_children(
            modal::Props {
                header_title: String::from("新規ブロック用テクスチャを作成"),
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
                                self.render_btn_to_select_kind(
                                    TextureKind::CustomTexture,
                                    "画像から作成",
                                ),
                                self.render_btn_to_select_kind(
                                    TextureKind::PrefabTexture,
                                    "作成済みの画像を選択",
                                ),
                            ],
                        ),
                        match &self.selecting_kind {
                            TextureKind::CustomTexture => self.render_custom_texture_editer(),
                            TextureKind::PrefabTexture => self.render_prefab_texture_editer(),
                        },
                        Html::div(
                            Attributes::new().class(Self::class("controller")),
                            Events::new(),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::CreateTexure),
                                vec![Html::text("作成")],
                            )],
                        ),
                    ],
                ),
                self.render_modal(props),
            ],
        ))
    }
}

impl ModalCreateBlockTexture {
    fn render_modal(&self, props: &Props) -> Html<Self> {
        match &self.showing_modal {
            ShowingModal::None => Common::none(),
            ShowingModal::SelectCustomTextureImage(direction) => ModalResource::empty(
                modal_resource::Props {
                    arena: ArenaMut::clone(&props.arena),
                    world: BlockMut::clone(&props.world),
                    filter: set! {BlockKind::ImageData},
                    is_selecter: true,
                    title: String::from(modal_resource::title::SELECT_TEXTURE),
                },
                Sub::map({
                    let direction = *direction;
                    move |sub| match sub {
                        modal_resource::On::SelectImageData(img) => {
                            Msg::SetCustomTextureImage(direction, Some(img))
                        }
                        modal_resource::On::SelectNone => {
                            Msg::SetCustomTextureImage(direction, None)
                        }
                        modal_resource::On::Close => Msg::CloseModal,
                        _ => Msg::NoOp,
                    }
                }),
            ),
            ShowingModal::SelectPrefabTextureImage => ModalResource::empty(
                modal_resource::Props {
                    arena: ArenaMut::clone(&props.arena),
                    world: BlockMut::clone(&props.world),
                    filter: set! {BlockKind::ImageData},
                    is_selecter: true,
                    title: String::from(modal_resource::title::SELECT_BLOCK_TEXTURE),
                },
                Sub::map(move |sub| match sub {
                    modal_resource::On::SelectImageData(img) => {
                        Msg::SetPrefabTextureImage(Some(img))
                    }
                    modal_resource::On::SelectNone => Msg::SetPrefabTextureImage(None),
                    modal_resource::On::Close => Msg::CloseModal,
                    modal_resource::On::UpdateBlocks { insert, update } => {
                        Msg::Sub(On::UpdateBlocks { insert, update })
                    }
                    _ => Msg::NoOp,
                }),
            ),
        }
    }

    fn render_btn_to_select_kind(&self, kind: TextureKind, text: impl Into<String>) -> Html<Self> {
        Btn::with_variant(
            if self.selecting_kind == kind {
                btn::Variant::PrimaryLikeMenu
            } else {
                btn::Variant::LightLikeMenu
            },
            Attributes::new(),
            Events::new().on_click(move |_| Msg::SetSelectingKind(kind)),
            vec![Html::text(text)],
        )
    }

    fn render_custom_texture_editer(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("editer"))
                .class(Self::class("custom-editer")),
            Events::new(),
            vec![
                self.render_custom_texture_cell(TextureDirection::PX),
                self.render_custom_texture_cell(TextureDirection::PY),
                self.render_custom_texture_cell(TextureDirection::PZ),
                self.render_custom_texture_cell(TextureDirection::NX),
                self.render_custom_texture_cell(TextureDirection::NY),
                self.render_custom_texture_cell(TextureDirection::NZ),
            ],
        )
    }

    fn render_custom_texture_cell(&self, direction: TextureDirection) -> Html<Self> {
        let img = match direction {
            TextureDirection::PX => self.custom_texture.texture_px.as_ref(),
            TextureDirection::PY => self.custom_texture.texture_py.as_ref(),
            TextureDirection::PZ => self.custom_texture.texture_pz.as_ref(),
            TextureDirection::NX => self.custom_texture.texture_nx.as_ref(),
            TextureDirection::NY => self.custom_texture.texture_ny.as_ref(),
            TextureDirection::NZ => self.custom_texture.texture_nz.as_ref(),
        };

        Html::div(
            Attributes::new()
                .class(Self::class("texture-cell"))
                .class(Self::class(&format!("texture-cell--{}", direction))),
            Events::new().on_click(move |_| Msg::SelectCustomTextureImage(direction)),
            vec![
                if let Some(src) = img.and_then(|img| img.map(|img| img.url().to_string())) {
                    Html::img(
                        Attributes::new()
                            .draggable(false)
                            .class(Common::bg_transparent())
                            .src(src),
                        Events::new(),
                        vec![],
                    )
                } else {
                    Html::none()
                },
                match direction {
                    TextureDirection::PX => text::span("PX（右）"),
                    TextureDirection::PY => text::span("PY（後）"),
                    TextureDirection::PZ => text::span("PZ（上）"),
                    TextureDirection::NX => text::span("NX（左）"),
                    TextureDirection::NY => text::span("NY（前）"),
                    TextureDirection::NZ => text::span("NZ（下）"),
                },
            ],
        )
    }

    fn render_prefab_texture_editer(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("editer"))
                .class(Self::class("prefab-editer")),
            Events::new(),
            vec![if let Some(src) = self
                .prefab_texture
                .as_ref()
                .and_then(|img| img.map(|img| img.url().to_string()))
            {
                Html::img(
                    Attributes::new()
                        .draggable(false)
                        .src(src)
                        .class(Self::class("prefab-img"))
                        .class(Common::bg_transparent()),
                    Events::new(),
                    vec![],
                )
            } else {
                Btn::secondary(
                    Attributes::new(),
                    Events::new().on_click(|_| Msg::SelectPrefabTextureImage),
                    vec![Html::text("画像を選択")],
                )
            }],
        )
    }
}

impl Styled for ModalCreateBlockTexture {
    fn style() -> Style {
        let cell_size = 7.5;
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-template-rows": "1fr max-content";
                "overflow": "hidden";
            }

            ".kind-list" {
                "grid-column": "1 / 2";
                "grid-row": "1 / 3";
                "display": "flex";
                "flex-direction": "column";
                "overflow-y": "scroll";
            }

            ".editer" {
                "grid-column": "2 / 3";
                "grid-row": "1 / 2";
                "overflow-y": "scroll";
            }

            ".custom-editer" {
                "display": "grid";
                "grid-template-columns": "repeat(4, max-content)";
                "grid-template-rows": "repeat(3, max-content)";
                "column-gap": ".35rem";
                "row-gap": ".35rem";
                "justify-content": "center";
                "align-content": "center";
            }

            ".prefab-editer" {
                "display": "flex";
                "justify-content": "center";
                "align-items": "center";
            }

            ".controller" {
                "grid-column": "2 / 3";
                "grid-row": "2 / 3";
                "display": "grid";
                "grid-auto-columns": "max-content";
                "justify-content": "end";
            }

            ".texture-cell" {
                "width": format!("{}rem", cell_size);
                "height": format!("{}rem", cell_size);
                "border-style": "solid";
                "border-width": ".1rem";
                "position": "relative";
            }

            ".texture-cell:hover" {
                "cursor": "pointer";
            }

            ".texture-cell > img" {
                "width": "100%";
                "height": "100%";
                "object-fit": "fill";
            }

            ".texture-cell > span" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "color": crate::libs::color::Pallet::gray(0);
                "-webkit-text-stroke": format!("1px {}", crate::libs::color::Pallet::gray(9));
            }

            ".prefab-img" {
                "height": "100%";
                "object-fit": "contain";
            }

            ".texture-cell--px" {
                "grid-column": "2 / 3";
                "grid-row": "2 / 3";
                "border-color": "#FF0000";
            }

            ".texture-cell--py" {
                "grid-column": "3 / 4";
                "grid-row": "2 / 3";
                "border-color": "#00FF00";
            }

            ".texture-cell--nx" {
                "grid-column": "4 / 5";
                "grid-row": "2 / 3";
                "border-color": "#00FFFF";
            }

            ".texture-cell--ny" {
                "grid-column": "1 / 2";
                "grid-row": "2 / 3";
                "border-color": "#FF00FF";
            }

            ".texture-cell--pz" {
                "grid-column": "1 / 2";
                "grid-row": "1 / 2";
                "border-color": "#0000FF";
            }

            ".texture-cell--nz" {
                "grid-column": "1 / 2";
                "grid-row": "3 / 4";
                "border-color": "#FFFF00";
            }
        }
    }
}
