use super::atom::common::Common;
use super::atom::{
    btn::{self, Btn},
    text::Text,
};
use super::molecule::modal::{self, Modal};
use super::organism::modal_resource::{self, ModalResource};
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
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub terran_texture: BlockMut<block::TerranTexture>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    CloseModal,
    CreateTexure,
    SelectCustomTextureImage(u32),
    SetCustomTextureImage(u32, Option<BlockRef<resource::BlockTexture>>),
}

pub enum On {
    Close,
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct ModalCreateTerranTexture {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    terran_texture: BlockMut<block::TerranTexture>,
    custom_texture: [BlockRef<resource::BlockTexture>; block::terran_texture::TEX_NUM],
    showing_modal: ShowingModal,
}

enum ShowingModal {
    None,
    SelectCustomTextureImage(u32),
}

impl Component for ModalCreateTerranTexture {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalCreateTerranTexture {}

impl Constructor for ModalCreateTerranTexture {
    fn constructor(props: Props) -> Self {
        let custom_texture = props
            .terran_texture
            .map(|texture| {
                array_macro::array![i => BlockRef::clone(&texture.textures()[i]); block::terran_texture::TEX_NUM]
            })
            .unwrap_or_else(
                || array_macro::array![_ => BlockRef::none(); block::terran_texture::TEX_NUM],
            );
        Self {
            arena: props.arena,
            world: props.world,
            terran_texture: props.terran_texture,
            custom_texture,
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for ModalCreateTerranTexture {
    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::CloseModal => {
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
            Msg::CreateTexure => {
                let textures = self
                    .custom_texture
                    .iter()
                    .enumerate()
                    .map(|(tex_idx, tex)| (tex_idx as u32, BlockRef::clone(&tex)))
                    .collect::<Vec<_>>();
                let is_update_texture = self.terran_texture.update(|terran_texture| {
                    terran_texture.set_textures(textures.into_iter());
                });
                if is_update_texture {
                    Cmd::list(vec![
                        Cmd::submit(On::UpdateBlocks {
                            insert: set! {},
                            update: set! {self.terran_texture.id()},
                        }),
                        Cmd::submit(On::Close),
                    ])
                } else {
                    let mut terran_texture = block::TerranTexture::new();
                    terran_texture.set_textures(
                        self.custom_texture
                            .iter()
                            .enumerate()
                            .map(|(tex_idx, tex)| (tex_idx as u32, BlockRef::clone(&tex))),
                    );
                    let terran_texture = self.arena.insert(terran_texture);
                    let terran_texture_id = terran_texture.id();
                    self.world.update(|world| {
                        world.push_terran_texture_block(terran_texture);
                    });
                    Cmd::list(vec![
                        Cmd::submit(On::UpdateBlocks {
                            insert: set! {terran_texture_id},
                            update: set! {self.world.id()},
                        }),
                        Cmd::submit(On::Close),
                    ])
                }
            }
            Msg::SelectCustomTextureImage(direction) => {
                self.showing_modal = ShowingModal::SelectCustomTextureImage(direction);
                Cmd::none()
            }
            Msg::SetCustomTextureImage(direction, img) => {
                if direction < block::terran_texture::TEX_NUM as u32 {
                    self.custom_texture[direction as usize] =
                        img.unwrap_or_else(|| BlockRef::none());
                }
                self.showing_modal = ShowingModal::None;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for ModalCreateTerranTexture {
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
                String::from("新規ブロック用テクスチャを作成"),
                String::from(""),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("base")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Self::class("kind-list")),
                                Events::new(),
                                vec![Btn::with_variant(
                                    btn::Variant::PrimaryLikeMenu,
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text("画像から作成")],
                                )],
                            ),
                            self.render_custom_texture_editer(),
                            Html::div(
                                Attributes::new().class(Self::class("controller")),
                                Events::new(),
                                vec![Btn::primary(
                                    Attributes::new(),
                                    Events::new().on_click(self, |_| Msg::CreateTexure),
                                    vec![Html::text("作成")],
                                )],
                            ),
                        ],
                    ),
                    self.render_modal(),
                ],
            ),
        ))
    }
}

impl ModalCreateTerranTexture {
    fn render_modal(&self) -> Html {
        match &self.showing_modal {
            ShowingModal::None => Common::none(),
            ShowingModal::SelectCustomTextureImage(direction) => ModalResource::empty(
                self,
                None,
                modal_resource::Props {
                    arena: ArenaMut::clone(&self.arena),
                    world: BlockMut::clone(&self.world),
                    filter: set! {BlockKind::BlockTexture},
                    is_selecter: true,
                    title: String::from(modal_resource::title::SELECT_TEXTURE),
                },
                Sub::map({
                    let direction = *direction;
                    move |sub| match sub {
                        modal_resource::On::SelectBlockTexture(img) => {
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
        }
    }

    fn render_custom_texture_editer(&self) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("editer"))
                .class(Self::class("custom-editer")),
            Events::new(),
            self.custom_texture
                .iter()
                .enumerate()
                .map(|(tex_idx, tex)| self.render_custom_texture_cell(tex_idx as u32, tex))
                .collect(),
        )
    }

    fn render_custom_texture_cell(
        &self,
        direction: u32,
        texture: &BlockRef<resource::BlockTexture>,
    ) -> Html {
        Html::div(
            Attributes::new().class(Self::class("texture-cell")),
            Events::new().on_click(self, move |_| Msg::SelectCustomTextureImage(direction)),
            vec![
                if let Some(src) = texture.map(|texture| texture.data().url().to_string()) {
                    Html::img(
                        Attributes::new()
                            .draggable("false")
                            .class(Common::bg_transparent())
                            .src(src),
                        Events::new(),
                        vec![],
                    )
                } else {
                    Html::none()
                },
                Text::span(format!("ブロック-{}", direction)),
            ],
        )
    }
}

impl Styled for ModalCreateTerranTexture {
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
        }
    }
}
