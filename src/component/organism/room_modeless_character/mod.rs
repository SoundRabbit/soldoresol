use super::molecule::tab_menu::{self, TabMenu};
use super::organism::modal_resource::{self, ModalResource};
use super::organism::room_modeless::RoomModeless;
use super::template::common::Common;
use crate::arena::{block, resource, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::collections::HashSet;

mod tab0;

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
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetSize(f64),
    SetTexSize(f64),
    SetSelectedTextureIdx(usize),
    SetDescriptionViewAsEdit(Option<String>),
    SetDescriptionViewAsView,
    SetTextureImage(usize, Option<BlockRef<resource::ImageData>>),
    SetTextureName(usize, String),
    PushTexture,
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessCharacter {
    character: BlockMut<block::Character>,

    timestamp: f64,
    is_updated: bool,

    description_view: DescriptionView,

    selected_tab_idx: usize,
    showing_modal: ShowingModal,
    element_id: ElementId,
}

pub enum DescriptionView {
    Edit(String),
    View(block::chat_message::Message),
}

impl DescriptionView {
    fn is_edit(&self) -> bool {
        match self {
            Self::Edit(..) => true,
            _ => false,
        }
    }

    fn is_view(&self) -> bool {
        match self {
            Self::View(..) => true,
            _ => false,
        }
    }
}

pub enum ShowingModal {
    None,
    SelectCharacterTexture(usize),
}

ElementId! {
    input_character_name,
    input_character_display_name
}

impl Component for RoomModelessCharacter {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModelessCharacter {
    fn constructor(props: &Props) -> Self {
        Self {
            character: BlockMut::clone(&props.data),

            timestamp: props.data.timestamp(),
            is_updated: true,

            description_view: DescriptionView::View(Self::get_description(&props.data)),

            selected_tab_idx: 0,
            showing_modal: ShowingModal::None,
            element_id: ElementId::new(),
        }
    }
}

impl Update for RoomModelessCharacter {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        if self.character.id() != props.data.id() {
            self.character = BlockMut::clone(&props.data);

            self.timestamp = props.data.timestamp();
            self.is_updated = true;

            self.description_view = DescriptionView::View(
                props
                    .data
                    .map(|character| character.description().data().clone())
                    .unwrap_or_else(|| block::chat_message::Message::from(vec![])),
            );
        } else if self.timestamp < props.data.timestamp() {
            self.timestamp = props.data.timestamp();
            self.is_updated = true;

            if self.description_view.is_view() {
                self.description_view = DescriptionView::View(Self::get_description(&props.data));
            }
        } else {
            self.is_updated = false;
        }

        Cmd::none()
    }

    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
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

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetDisplayName0(display_name) => {
                self.character.update(|character| {
                    character.set_display_name((Some(display_name), None));
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetDisplayName1(display_name) => {
                self.character.update(|character| {
                    character.set_display_name((None, Some(display_name)));
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetSize(size) => {
                self.character.update(|character| {
                    character.set_size(size);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetTexSize(size) => {
                self.character.update(|character| {
                    character.set_tex_size(size);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetSelectedTextureIdx(tex_idx) => {
                self.character.update(|character| {
                    character.set_selected_texture_idx(tex_idx);
                });

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
            Msg::SetDescriptionViewAsEdit(description) => {
                if let Some(description) = description.or_else(|| {
                    self.character
                        .map(|character| character.description().raw().clone())
                }) {
                    self.description_view = DescriptionView::Edit(description);
                }

                Cmd::none()
            }
            Msg::SetDescriptionViewAsView => {
                if self.description_view.is_view() && !self.is_updated {
                    return Cmd::none();
                }

                let mut description_view =
                    DescriptionView::View(Self::get_description(&self.character));
                std::mem::swap(&mut self.description_view, &mut description_view);

                if let DescriptionView::Edit(description) = description_view {
                    self.character.update(|character| {
                        character.set_description(description);
                    });

                    Cmd::sub(On::UpdateBlocks {
                        insert: set! {},
                        update: set! { self.character.id() },
                    })
                } else {
                    Cmd::none()
                }
            }
            Msg::SetTextureImage(tex_idx, image) => {
                self.character.update(|character| {
                    character.set_texture_image(tex_idx, image);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }

            Msg::SetTextureName(tex_idx, name) => {
                self.character.update(|character| {
                    character.set_texture_name(tex_idx, name);
                });

                Cmd::sub(On::UpdateBlocks {
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

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
        }
    }
}

impl RoomModelessCharacter {
    fn get_description(character: &BlockMut<block::Character>) -> block::chat_message::Message {
        let description = character
            .map(|character| character.description().data().clone())
            .unwrap_or_else(|| block::chat_message::Message::from(vec![]));
        let (description, _) = block::chat_message::map(character.chat_ref(), description);
        description
    }
}

impl Render for RoomModelessCharacter {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::fragment(vec![
            self.render_tabs(),
            match &self.showing_modal {
                ShowingModal::None => Html::none(),
                ShowingModal::SelectCharacterTexture(tex_idx) => ModalResource::empty(
                    modal_resource::Props {
                        arena: ArenaMut::clone(&props.arena),
                        world: BlockMut::clone(&props.world),
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
    fn render_tabs(&self) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![TabMenu::with_children(
                tab_menu::Props {
                    selected: self.selected_tab_idx,
                    tabs: vec![String::from("Common")],
                    controlled: true,
                },
                Sub::map(|sub| match sub {
                    tab_menu::On::ChangeSelectedTab(tab_idx) => Msg::SetSelectedTabIdx(tab_idx),
                }),
                vec![if self.selected_tab_idx == 0 {
                    self.render_tab0()
                } else {
                    Common::none()
                }],
            )],
        )
    }
}

impl Styled for RoomModelessCharacter {
    fn style() -> Style {
        style! {
            ".dropdown" {
                "overflow": "visible !important";
            }

            ".base" {
                "width": "100%";
                "height": "100%";
                "padding-top": ".65rem";
            }

            ".tab0-main" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "overflow-y": "scroll";
            }

            ".tab0-content" {
                "display": "grid";
                "column-gap": ".65rem";
                "row-gap": ".65rem";
                "align-items": "start";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "grid-auto-rows": "max-content";
            }

            ".tab0-content img" {
                "width": "100%";
                "max-height": "20rem";
                "object-fit": "contain";
            }

            ".tab0-texture" {
                "display": "grid";
                "align-items": "start";
                "justify-items": "stretch";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content max-content";
            }

            ".tab0-textarea" {
                "resize": "none";
                "height": "15em";
            }
        }
    }
}
