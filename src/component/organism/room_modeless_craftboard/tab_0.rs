use super::super::atom::{
    btn::Btn,
    common::Common,
    fa,
    heading::{self, Heading},
    slider::{self, Slider},
    text::Text,
};
use super::super::organism::{
    popup_color_pallet::{self, PopupColorPallet},
    room_modeless::RoomModeless,
};
use super::ShowingModal;
use crate::arena::{block, BlockMut};
use crate::libs::color::Pallet;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub craftboard: block::craftboard::Block,
}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    OpenModal(ShowingModal),
    SetName(String),
    SetDisplayName0(String),
    SetDisplayName1(String),
    SetXSize(f64),
    SetYSize(f64),
    SetZSize(f64),
    SetGridColor(Pallet),
}

pub struct Tab0 {
    craftboard: block::craftboard::Block,
    element_id: ElementId,
}

ElementId! {
    input_craftboard_name,
    input_craftboard_display_name
}

impl Component for Tab0 {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Tab0 {}

impl Constructor for Tab0 {
    fn constructor(props: Self::Props) -> Self {
        Self {
            craftboard: props.craftboard,
            element_id: ElementId::new(),
        }
    }
}

impl Update for Tab0 {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.craftboard = props.craftboard;
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
                self.craftboard
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.craftboard
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
            ],
        ))
    }
}

impl Tab0 {
    fn render_header(&self, craftboard: &block::Craftboard) -> Html {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_craftboard_name),
                    Events::new(),
                    vec![fa::fas_i("fa-user")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_craftboard_name)
                        .value(craftboard.name()),
                    Events::new().on_input(self, |name| Msg::Sub(On::SetName(name))),
                    vec![],
                ),
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_craftboard_display_name),
                    Events::new(),
                    vec![Html::text("表示名")],
                ),
                Html::input(
                    Attributes::new().value(&craftboard.display_name().1),
                    Events::new().on_input(self, |name| Msg::Sub(On::SetDisplayName1(name))),
                    vec![],
                ),
                Text::span(""),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_craftboard_display_name)
                        .value(&craftboard.display_name().0),
                    Events::new().on_input(self, |name| Msg::Sub(On::SetDisplayName0(name))),
                    vec![],
                ),
            ],
        )
    }

    fn render_main(&self, craftboard: &block::Craftboard) -> Html {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            vec![
                self.render_props(craftboard),
                Heading::h3(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("テクスチャ")],
                ),
                self.render_textures(craftboard),
            ],
        )
    }

    fn render_props(&self, craftboard: &block::Craftboard) -> Html {
        Html::div(
            Attributes::new().class(Self::class("content")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Common::keyvalue()),
                    Events::new(),
                    vec![
                        Text::span("X幅（横幅）"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 1.0,
                                max: 100.0,
                                val: craftboard.size()[0],
                                step: 1.0,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::Sub(On::SetXSize(x)),
                                _ => Msg::NoOp,
                            }),
                            slider::Props {
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                        ),
                        Text::span("Y幅（奥行き）"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 1.0,
                                max: 100.0,
                                val: craftboard.size()[1],
                                step: 1.0,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(y) => Msg::Sub(On::SetYSize(y)),
                                _ => Msg::NoOp,
                            }),
                            slider::Props {
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                        ),
                        Text::span("Z幅（高さ）"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 0.0,
                                max: 100.0,
                                val: craftboard.size()[2],
                                step: 1.0,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(z) => Msg::Sub(On::SetZSize(z)),
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
                        Text::span("色"),
                        PopupColorPallet::empty(
                            self,
                            None,
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: craftboard.grid_color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    Msg::Sub(On::SetGridColor(color))
                                }
                            }),
                        ),
                    ],
                ),
            ],
        )
    }

    fn render_textures(&self, craftboard: &block::Craftboard) -> Html {
        Html::div(
            Attributes::new().class(Self::class("content")),
            Events::new(),
            vec![
                self.render_texture_block(craftboard, "PZ（上）", 2),
                self.render_texture_block(craftboard, "NZ（下）", 5),
                self.render_texture_block(craftboard, "PY（奥）", 1),
                self.render_texture_block(craftboard, "NY（前）", 4),
                self.render_texture_block(craftboard, "PX（右）", 0),
                self.render_texture_block(craftboard, "NX（左）", 3),
            ],
        )
    }

    fn render_texture_block(
        &self,
        craftboard: &block::Craftboard,
        name: impl Into<String>,
        tex_idx: usize,
    ) -> Html {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Heading::h5(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(name)],
                ),
                self.render_texture(craftboard, tex_idx),
            ],
        )
    }

    fn render_texture(&self, craftboard: &block::Craftboard, tex_idx: usize) -> Html {
        craftboard.textures()[tex_idx]
            .as_ref()
            .map(|texture| {
                texture.map(|texture| {
                    Html::img(
                        Attributes::new()
                            .draggable("false")
                            .src(texture.url().to_string())
                            .class(Common::bg_transparent()),
                        Events::new().on_click(self, move |_| {
                            Msg::Sub(On::OpenModal(ShowingModal::SelectTexture(tex_idx)))
                        }),
                        vec![],
                    )
                })
            })
            .unwrap_or(None)
            .unwrap_or_else(|| {
                Btn::secondary(
                    Attributes::new(),
                    Events::new().on_click(self, move |_| {
                        Msg::Sub(On::OpenModal(ShowingModal::SelectTexture(tex_idx)))
                    }),
                    vec![Html::text("画像を選択")],
                )
            })
    }
}

impl Styled for Tab0 {
    fn style() -> Style {
        style! {
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
        }
    }
}
