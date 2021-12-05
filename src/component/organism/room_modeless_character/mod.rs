use super::atom::btn::{self, Btn};
use super::atom::dropdown::{self, Dropdown};
use super::atom::fa;
use super::atom::slider::{self, Slider};
use super::atom::text;
use super::organism::modal_resource::{self, ModalResource};
use super::organism::popup_color_pallet::{self, PopupColorPallet};
use super::organism::room_modeless::RoomModeless;
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

pub struct Props {
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub data: BlockMut<block::Character>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetShowingModal(ShowingModal),
    SetColor(crate::libs::color::Pallet),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetSize(f64),
    SetTexSize(f64),
    SetTexture(Option<BlockMut<resource::ImageData>>),
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct RoomModelessCharacter {
    character: BlockMut<block::Character>,

    showing_modal: ShowingModal,
    element_id: ElementId,
}

pub enum ShowingModal {
    None,
    SelectCharacterTexture,
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
            showing_modal: ShowingModal::None,
            element_id: ElementId::new(),
        }
    }
}

impl Update for RoomModelessCharacter {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.character = BlockMut::clone(&props.data);
        Cmd::none()
    }

    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
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
            Msg::SetTexture(texture) => {
                self.character.update(|character| {
                    character.set_texture(texture);
                });

                self.showing_modal = ShowingModal::None;

                Cmd::sub(On::UpdateBlocks {
                    insert: set! {},
                    update: set! { self.character.id() },
                })
            }
        }
    }
}

impl Render for RoomModelessCharacter {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.character
                    .map(|data| self.render_boxblock_header(data))
                    .unwrap_or(Common::none()),
                self.character
                    .map(|data| self.render_boxblock_main(data))
                    .unwrap_or(Common::none()),
                match &self.showing_modal {
                    ShowingModal::None => Html::none(),
                    ShowingModal::SelectCharacterTexture => ModalResource::empty(
                        modal_resource::Props {
                            arena: ArenaMut::clone(&props.arena),
                            world: BlockMut::clone(&props.world),
                            title: String::from(modal_resource::title::SELECT_TEXTURE),
                            filter: set! { BlockKind::ImageData },
                            is_selecter: true,
                        },
                        Sub::map(|sub| match sub {
                            modal_resource::On::Close => Msg::SetShowingModal(ShowingModal::None),
                            modal_resource::On::UpdateBlocks { insert, update } => {
                                Msg::Sub(On::UpdateBlocks { insert, update })
                            }
                            modal_resource::On::SelectImageData(texture) => {
                                Msg::SetTexture(Some(texture))
                            }
                            modal_resource::On::SelectNone => Msg::SetTexture(None),
                            _ => Msg::NoOp,
                        }),
                    ),
                },
            ],
        ))
    }
}

impl RoomModelessCharacter {
    fn render_boxblock_header(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_character_name),
                    Events::new(),
                    vec![fa::i("fa-user")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_character_name)
                        .value(character.name()),
                    Events::new(),
                    vec![],
                ),
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_character_display_name),
                    Events::new(),
                    vec![Html::text("表示名")],
                ),
                Html::input(
                    Attributes::new().value(&character.display_name().1),
                    Events::new().on_input(Msg::SetDisplayName1),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_character_display_name)
                        .value(&character.display_name().0),
                    Events::new().on_input(Msg::SetDisplayName0),
                    vec![],
                ),
            ],
        )
    }

    fn render_boxblock_main(&self, character: &block::Character) -> Html<Self> {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("サイズ"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: character.size(),
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::SetSize(x),
                                _ => Msg::NoOp,
                            }),
                        ),
                        text::span("立ち絵サイズ"),
                        Slider::empty(
                            slider::Props {
                                position: slider::Position::Linear {
                                    min: 0.1,
                                    max: 10.0,
                                    val: character.tex_size(),
                                    step: 0.1,
                                },
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::SetTexSize(x),
                                _ => Msg::NoOp,
                            }),
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("色"),
                        PopupColorPallet::empty(
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: character.color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => Msg::SetColor(color),
                            }),
                        ),
                        text::span("立ち絵"),
                        character
                            .texture()
                            .as_ref()
                            .map(|texture| {
                                texture.map(|texture| {
                                    Html::img(
                                        Attributes::new()
                                            .src(texture.url().to_string())
                                            .class(Common::bg_transparent()),
                                        Events::new().on_click(|_| {
                                            Msg::SetShowingModal(
                                                ShowingModal::SelectCharacterTexture,
                                            )
                                        }),
                                        vec![],
                                    )
                                })
                            })
                            .unwrap_or(None)
                            .unwrap_or_else(|| {
                                Btn::secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| {
                                        Msg::SetShowingModal(ShowingModal::SelectCharacterTexture)
                                    }),
                                    vec![Html::text("立ち絵を選択")],
                                )
                            }),
                    ],
                ),
            ],
        )
    }
}

impl Styled for RoomModelessCharacter {
    fn style() -> Style {
        style! {
            ".dropdown" {
                "overflow": "visible !important";
            }

            ".main" {
                "display": "grid";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "align-items": "start";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "column-gap": ".65rem";
            }
        }
    }
}
