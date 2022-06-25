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
    pub textboard: BlockMut<block::Textboard>,
}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    OpenModal(ShowingModal),
    SetTitle(String),
    SetText(String),
    SetXSize(f64),
    SetZSize(f64),
    SetFontSize(f64),
    SetColor(Pallet),
}

pub struct Tab0 {
    textboard: BlockMut<block::Textboard>,
    element_id: ElementId,
}

ElementId! {
    input_textboard_title
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
            textboard: props.textboard,
            element_id: ElementId::new(),
        }
    }
}

impl Update for Tab0 {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.textboard = props.textboard;
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
                self.textboard
                    .map(|data| self.render_header(data))
                    .unwrap_or(Common::none()),
                self.textboard
                    .map(|data| self.render_main(data))
                    .unwrap_or(Common::none()),
            ],
        ))
    }
}

impl Tab0 {
    fn render_header(&self, textboard: &block::Textboard) -> Html {
        Html::div(
            Attributes::new().class(RoomModeless::class("common-header")),
            Events::new(),
            vec![
                Html::label(
                    Attributes::new()
                        .class(RoomModeless::class("common-label"))
                        .string("for", &self.element_id.input_textboard_title),
                    Events::new(),
                    vec![fa::fas_i("fa-file-lines")],
                ),
                Html::input(
                    Attributes::new()
                        .id(&self.element_id.input_textboard_title)
                        .value(textboard.title()),
                    Events::new().on_input(self, |title| Msg::Sub(On::SetTitle(title))),
                    vec![],
                ),
            ],
        )
    }

    fn render_main(&self, textboard: &block::Textboard) -> Html {
        Html::div(
            Attributes::new().class(Self::class("main")),
            Events::new(),
            vec![
                self.render_props(textboard),
                Heading::h3(
                    heading::Variant::Light,
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("テキスト")],
                ),
                self.render_text(textboard),
            ],
        )
    }

    fn render_props(&self, textboard: &block::Textboard) -> Html {
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
                                min: 2.0,
                                max: 10.0,
                                val: textboard.size()[0],
                                step: 0.5,
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
                        Text::span("Z幅（高さ）"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 2.0,
                                max: 10.0,
                                val: textboard.size()[1],
                                step: 0.5,
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
                        Text::span("文字サイズ"),
                        Slider::new(
                            self,
                            None,
                            slider::Position::Linear {
                                min: 0.1,
                                max: 1.0,
                                val: textboard.font_size(),
                                step: 0.025,
                            },
                            Sub::map(move |sub| match sub {
                                slider::On::Input(x) => Msg::Sub(On::SetFontSize(x)),
                                _ => Msg::NoOp,
                            }),
                            slider::Props {
                                range_is_editable: false,
                                theme: slider::Theme::Light,
                            },
                        ),
                        Text::span("色"),
                        PopupColorPallet::empty(
                            self,
                            None,
                            popup_color_pallet::Props {
                                direction: popup_color_pallet::Direction::Bottom,
                                default_selected: textboard.color().clone(),
                            },
                            Sub::map(|sub| match sub {
                                popup_color_pallet::On::SelectColor(color) => {
                                    Msg::Sub(On::SetColor(color))
                                }
                            }),
                        ),
                    ],
                ),
            ],
        )
    }

    fn render_text(&self, textboard: &block::Textboard) -> Html {
        Html::div(
            Attributes::new().class(Self::class("content")),
            Events::new(),
            vec![Html::textarea(
                Attributes::new().value(textboard.text()),
                Events::new().on_input(self, |text| Msg::Sub(On::SetText(text))),
                vec![],
            )],
        )
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
