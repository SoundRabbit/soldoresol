use super::molecule::{
    block_prop::{self, BlockProp},
    tab_menu::{self, TabMenu},
};
use super::organism::modal_resource::{self, ModalResource};
use crate::arena::{block, resource, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::HashSet;

mod tab_0;
use tab_0::Tab0;

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub data: BlockMut<block::Character>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetSelectedTabIdx(usize),
    SetShowingModal(ShowingModal),
    SetColor(crate::libs::color::Pallet),
    SetName(String),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetSize(f64),
    SetTexSize(f64),
    SetSelectedTextureIdx(usize),
    SetDescription(String),
    SetTextureImage(usize, Option<BlockRef<resource::ImageData>>),
    SetTextureName(usize, String),
    AddProperty,
    PushTexture,
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessCharacter {
    arena: ArenaMut,
    world: BlockMut<block::World>,
    character: BlockMut<block::Character>,
    selected_tab_idx: usize,
    showing_modal: ShowingModal,
}

pub enum ShowingModal {
    None,
    SelectCharacterTexture(usize),
}

impl Component for RoomModelessCharacter {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for RoomModelessCharacter {}

impl Constructor for RoomModelessCharacter {
    fn constructor(props: Self::Props) -> Self {
        Self {
            arena: props.arena,
            world: props.world,
            character: props.data,
            selected_tab_idx: 0,
            showing_modal: ShowingModal::None,
        }
    }
}

impl Update for RoomModelessCharacter {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.arena = props.arena;
        self.world = props.world;
        if self.character.id() != props.data.id() {
            self.character = BlockMut::clone(&props.data);
        }

        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetSelectedTabIdx(tab_idx) => {
                self.selected_tab_idx = tab_idx;
                Cmd::none()
            }
            Msg::SetShowingModal(showing_modal) => {
                self.showing_modal = showing_modal;
                Cmd::none()
            }
            Msg::SetColor(color) => {
                self.character.update(|character| {
                    character.set_color(color);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetName(name) => {
                self.character.update(|character| {
                    character.set_name(name);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetDisplayName0(display_name) => {
                self.character.update(|character| {
                    character.set_display_name((Some(display_name), None));
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetDisplayName1(display_name) => {
                self.character.update(|character| {
                    character.set_display_name((None, Some(display_name)));
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetSize(size) => {
                self.character.update(|character| {
                    character.set_size(size);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetTexSize(size) => {
                self.character.update(|character| {
                    character.set_tex_size(size);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetSelectedTextureIdx(tex_idx) => {
                self.character.update(|character| {
                    character.set_selected_texture_idx(tex_idx);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetDescription(description) => {
                self.character.update(|character| {
                    character.set_description(description);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetTextureImage(tex_idx, image) => {
                self.character.update(|character| {
                    character.set_texture_image(tex_idx, image);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }

            Msg::SetTextureName(tex_idx, name) => {
                self.character.update(|character| {
                    character.set_texture_name(tex_idx, name);
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }

            Msg::PushTexture => {
                self.character.update(|character| {
                    let n = character.textures().len();
                    character.push_texture(block::character::StandingTexture::new(format!(
                        "立ち絵[{}]",
                        n
                    )));
                });

                Cmd::submit(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }

            Msg::AddProperty => {
                let mut property = block::Property::new();
                property.set_name(String::from("新規プロパティ"));

                let property = self.arena.insert(property);
                let property_id = property.id();

                self.character.update(move |character| {
                    character.push_property(property);
                });
                Cmd::submit(On::UpdateBlocks {
                    insert: set! { property_id },
                    update: set! { self.character.id() },
                })
            }
        }
    }
}

impl Render<Html> for RoomModelessCharacter {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::fragment(vec![
            self.render_tabs(),
            match &self.showing_modal {
                ShowingModal::None => Html::none(),
                ShowingModal::SelectCharacterTexture(tex_idx) => ModalResource::empty(
                    self,
                    None,
                    modal_resource::Props {
                        arena: ArenaMut::clone(&self.arena),
                        world: BlockMut::clone(&self.world),
                        title: String::from(modal_resource::title::SELECT_TEXTURE),
                        filter: set! { BlockKind::ImageData },
                        is_selecter: true,
                    },
                    Sub::map({
                        let tex_idx = *tex_idx;
                        move |sub| match sub {
                            modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                            modal_resource::On::UpdateBlocks { insert, update } => {
                                Msg::Sub(On::UpdateBlocks { insert, update })
                            }
                            modal_resource::On::SelectImageData(image) => {
                                Msg::SetTextureImage(tex_idx, Some(image))
                            }
                            modal_resource::On::SelectNone => Msg::SetTextureImage(tex_idx, None),
                            _ => Msg::NoOp,
                        }
                    }),
                ),
            },
        ]))
    }
}

impl RoomModelessCharacter {
    fn render_tabs(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![TabMenu::new(
                self,
                None,
                tab_menu::Props {
                    selected: self.selected_tab_idx,
                    controlled: true,
                },
                Sub::map({
                    let prop_num = self
                        .character
                        .map(|character| character.properties().len())
                        .unwrap_or(0);
                    move |sub| match sub {
                        tab_menu::On::ChangeSelectedTab(tab_idx) => {
                            if tab_idx < prop_num + 1 {
                                Msg::SetSelectedTabIdx(tab_idx)
                            } else {
                                Msg::NoOp
                            }
                        }
                    }
                }),
                (
                    Attributes::new(),
                    Events::new(),
                    vec![
                        vec![(
                            Html::text(String::from("Common")),
                            Tab0::empty(
                                self,
                                None,
                                tab_0::Props {
                                    character: BlockMut::clone(&self.character),
                                },
                                Sub::map(|sub| match sub {
                                    tab_0::On::OpenModal(modal) => Msg::SetShowingModal(modal),
                                    tab_0::On::PushTexture => Msg::PushTexture,
                                    tab_0::On::SetColor(pallet) => Msg::SetColor(pallet),
                                    tab_0::On::SetDescription(description) => {
                                        Msg::SetDescription(description)
                                    }
                                    tab_0::On::SetDisplayName0(dn0) => Msg::SetDisplayName0(dn0),
                                    tab_0::On::SetDisplayName1(dn1) => Msg::SetDisplayName1(dn1),
                                    tab_0::On::SetName(name) => Msg::SetName(name),
                                    tab_0::On::SetSelectedTextureIdx(tex_idx) => {
                                        Msg::SetSelectedTextureIdx(tex_idx)
                                    }
                                    tab_0::On::SetSize(size) => Msg::SetSize(size),
                                    tab_0::On::SetTexSize(tex_size) => Msg::SetTexSize(tex_size),
                                    tab_0::On::SetTextureName(tex_idx, tex_name) => {
                                        Msg::SetTextureName(tex_idx, tex_name)
                                    }
                                }),
                            ),
                        )],
                        self.character
                            .map(|character| {
                                character
                                    .properties()
                                    .iter()
                                    .map(|prop| {
                                        (
                                            Html::text(
                                                prop.map(|prop| prop.name().clone())
                                                    .unwrap_or_else(|| String::from("")),
                                            ),
                                            BlockProp::new(
                                                self,
                                                None,
                                                block_prop::Props {
                                                    arena: ArenaMut::clone(&self.arena),
                                                    data: BlockMut::clone(&prop),
                                                },
                                                Sub::map(|sub| match sub {
                                                    block_prop::On::UpdateBlocks {
                                                        update,
                                                        insert,
                                                    } => Msg::Sub(On::UpdateBlocks {
                                                        update,
                                                        insert,
                                                    }),
                                                }),
                                                (),
                                            ),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                        vec![(
                            Html::span(
                                Attributes::new(),
                                Events::new().on_click(self, |_| Msg::AddProperty),
                                vec![Html::text(String::from("追加"))],
                            ),
                            Html::none(),
                        )],
                    ]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>(),
                ),
            )],
        )
    }
}

impl Styled for RoomModelessCharacter {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
                "padding-top": ".65rem";
            }
        }
    }
}
