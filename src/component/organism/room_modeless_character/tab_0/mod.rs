use super::super::atom::{
    btn::{self, Btn},
    common::Common,
    dropdown::{self, Dropdown},
    fa,
    heading::{self, Heading},
    slider::{self, Slider},
    text,
};
use super::super::organism::{
    popup_color_pallet::{self, PopupColorPallet},
    room_modeless::RoomModeless,
};
use super::ShowingModal;
use crate::arena::{block, resource, BlockMut, BlockRef};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

mod description;

use description::Description;

pub struct Props {
    pub character: BlockMut<block::Character>,
}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    OpenModal(ShowingModal),
    SetDescription(String),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetName(String),
    SetColor(crate::libs::color::Pallet),
    SetSize(f64),
    SetTexSize(f64),
    SetSelectedTextureIdx(usize),
    SetTextureName(usize, String),
    PushTexture,
}

pub struct Tab0 {
    character: BlockMut<block::Character>,
    element_id: ElementId,
}

ElementId! {
    input_character_display_name,
    input_character_name
}

impl Component for Tab0 {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Tab0 {}

impl Constructor for Tab0 {
    fn constructor(props: Props) -> Self {
        Self {
            character: props.character,
            element_id: ElementId::new(),
        }
    }
}

impl Update for Tab0 {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.character = props.character;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(event) => Cmd::submit(event),
        }
    }
}

impl Render<Html> for Tab0 {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .class(RoomModeless::class("common-base"))
                .class("pure-form"),
            Events::new(),
            vec![
                self.character
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.character
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
            ],
        ))
    }
}

impl Tab0 {
    fn render_header(&self, character: &block::Character) -> Html {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_character_name),
                    Events::new(),
                    vec![fa::fas_i("fa-user")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_character_name)
                        .value(character.name()),
                    Events::new().on_input(self, |name| Msg::Sub(On::SetName(name))),
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
                    Events::new().on_input(self, |name| Msg::Sub(On::SetDisplayName1(name))),
                    vec![],
                ),
                text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_character_display_name)
                        .value(&character.display_name().0),
                    Events::new().on_input(self, |name| Msg::Sub(On::SetDisplayName0(name))),
                    vec![],
                ),
            ],
        )
    }

    fn render_main(&self, character: &block::Character) -> Html {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            vec![
                self.render_props(character),
                Heading::h3(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("概要")],
                ),
                Description::empty(
                    self,
                    None,
                    description::Props {
                        character: BlockMut::clone(&self.character),
                    },
                    Sub::map(|sub| match sub {
                        description::On::SetDescription(desc) => Msg::Sub(On::SetDescription(desc)),
                    }),
                ),
                Heading::h3(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("立ち絵")],
                ),
                self.render_textures(character),
            ],
        )
    }

    fn render_props(&self, character: &block::Character) -> Html {
        Html::div(
            Attributes::new().class(Self::class("content")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("サイズ"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 0.1,
                                max: 10.0,
                                val: character.size(),
                                step: 0.1,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::Sub(On::SetSize(x)),
                                _ => Msg::NoOp,
                            }),
                            slider::Props {
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                        ),
                        text::span("立ち絵サイズ"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 0.1,
                                max: 10.0,
                                val: character.tex_size(),
                                step: 0.1,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::Sub(On::SetTexSize(x)),
                                _ => Msg::NoOp,
                            }),
                            slider::Props {
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        text::span("色"),
                        PopupColorPallet::empty(
                            self,
                            None,
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: character.color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    Msg::Sub(On::SetColor(color))
                                }
                            }),
                        ),
                        text::label("立ち絵", ""),
                        Dropdown::new(
                            self,
                            None,
                            dropdown::Props {
                                direction: dropdown::Direction::Bottom,
                                variant: btn::Variant::DarkLikeMenu,
                                toggle_type: dropdown::ToggleType::Click,
                            },
                            Sub::none(),
                            (
                                vec![Html::text(
                                    character
                                        .selected_texture()
                                        .map(|texture| texture.name().clone())
                                        .unwrap_or(String::from("")),
                                )],
                                character
                                    .textures()
                                    .iter()
                                    .enumerate()
                                    .map(|(tex_idx, texture)| {
                                        Btn::menu(
                                            Attributes::new(),
                                            Events::new().on_click(self, move |_| {
                                                Msg::Sub(On::SetSelectedTextureIdx(tex_idx))
                                            }),
                                            vec![Html::text(texture.name())],
                                        )
                                    })
                                    .collect(),
                            ),
                        ),
                    ],
                ),
            ],
        )
    }

    fn render_textures(&self, character: &block::Character) -> Html {
        Html::div(
            Attributes::new().class(Self::class("content")),
            Events::new(),
            vec![
                Html::fragment(
                    character
                        .textures()
                        .iter()
                        .enumerate()
                        .map(|(tex_idx, texture)| self.render_texture(texture, tex_idx))
                        .collect(),
                ),
                self.render_new_texture(),
            ],
        )
    }
    fn render_texture(&self, texture: &block::character::StandingTexture, tex_idx: usize) -> Html {
        Html::div(
            Attributes::new().class(Self::class("texture")),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new().value(texture.name()),
                    Events::new().on_input(self, move |name| {
                        Msg::Sub(On::SetTextureName(tex_idx, name))
                    }),
                    vec![],
                ),
                self.render_texture_image(texture.image(), tex_idx),
            ],
        )
    }

    fn render_texture_image(
        &self,
        image: Option<&BlockRef<resource::ImageData>>,
        tex_idx: usize,
    ) -> Html {
        image
            .and_then(|image| {
                image.map(|image| {
                    Html::img(
                        Attributes::new()
                            .draggable("false")
                            .src(image.url().to_string())
                            .class(Common::bg_transparent()),
                        Events::new().on_click(self, move |_| {
                            Msg::Sub(On::OpenModal(ShowingModal::SelectCharacterTexture(tex_idx)))
                        }),
                        vec![],
                    )
                })
            })
            .unwrap_or_else(|| {
                Btn::secondary(
                    Attributes::new(),
                    Events::new().on_click(self, move |_| {
                        Msg::Sub(On::OpenModal(ShowingModal::SelectCharacterTexture(tex_idx)))
                    }),
                    vec![Html::text("立ち絵を選択")],
                )
            })
    }

    fn render_new_texture(&self) -> Html {
        Html::div(
            Attributes::new().class(Self::class("texture")),
            Events::new(),
            vec![Btn::secondary(
                Attributes::new(),
                Events::new().on_click(self, |_| Msg::Sub(On::PushTexture)),
                vec![Html::text("立ち絵を追加")],
            )],
        )
    }
}

impl Tab0 {}

impl Styled for Tab0 {
    fn style() -> Style {
        style! {
            ".dropdown" {
                "overflow": "visible !important";
            }

            ".main" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "overflow-y": "scroll";
            }

            ".content" {
                "display": "grid";
                "column-gap": ".65rem";
                "row-gap": ".65rem";
                "align-items": "start";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "grid-auto-rows": "max-content";
            }

            ".content img" {
                "width": "100%";
                "max-height": "20rem";
                "object-fit": "contain";
            }

            ".texture" {
                "display": "grid";
                "align-items": "start";
                "justify-items": "stretch";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content max-content";
            }

            ".textarea" {
                "resize": "none";
                "height": "15em";
            }
        }
    }
}
