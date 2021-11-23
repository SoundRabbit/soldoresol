use super::atom::slider::{self, Slider};
use super::molecule::color_pallet::{self, ColorPallet};
use crate::libs::color::Pallet;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

pub struct Props {
    pub default_selected: Pallet,
    pub direction: Direction,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Bottom,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Bottom => write!(f, "bottom"),
        }
    }
}

pub enum Msg {
    SetIsToggled(bool),
    SetSelected(Pallet),
}

pub enum On {
    SelectColor(Pallet),
}

pub struct PopupColorPallet {
    selected: Pallet,
    is_toggled: bool,
}

impl Component for PopupColorPallet {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for PopupColorPallet {
    fn constructor(props: &Props) -> Self {
        Self {
            selected: props.default_selected,
            is_toggled: false,
        }
    }
}

impl Update for PopupColorPallet {
    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::SetIsToggled(is_toggled) => {
                self.is_toggled = is_toggled;

                if self.is_toggled {
                    self.selected = props.default_selected;
                    Cmd::none()
                } else {
                    Cmd::sub(On::SelectColor(self.selected))
                }
            }
            Msg::SetSelected(pallet) => {
                self.selected = pallet;
                Cmd::none()
            }
        }
    }
}

impl Render for PopupColorPallet {
    fn render(&self, props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        ColorPallet::styled(());
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Self::render_selected_color(props),
                self.render_mask(),
                self.render_color_pallet(props),
            ],
        ))
    }
}

impl PopupColorPallet {
    fn render_selected_color(props: &Props) -> Html<Self> {
        Html::div(
            Attributes::new(),
            Events::new().on_click(|_| Msg::SetIsToggled(true)),
            vec![ColorPallet::render_color_base(
                &slider::Theme::Light,
                &props.default_selected,
            )],
        )
    }

    fn render_mask(&self) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("mask"))
                .string("data-toggled", self.is_toggled.to_string()),
            Events::new().on_click(|_| Msg::SetIsToggled(false)),
            vec![],
        )
    }

    fn render_color_pallet(&self, props: &Props) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("pallet"))
                .class(Self::class(&format!("pallet--{}", &props.direction)))
                .string("data-toggled", self.is_toggled.to_string()),
            Events::new(),
            vec![if self.is_toggled {
                ColorPallet::empty(
                    color_pallet::Props {
                        title: None,
                        default_selected: props.default_selected,
                        theme: slider::Theme::Light,
                    },
                    Sub::map(|sub| match sub {
                        color_pallet::On::SelectColor(pallet) => Msg::SetSelected(pallet),
                    }),
                )
            } else {
                Html::none()
            }],
        )
    }
}

impl Styled for PopupColorPallet {
    fn style() -> Style {
        style! {
            ".base" {
                "position": "relative";
                "overflow": "visible";
            }

            ".mask" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "width": "100vw";
                "height": "100vh";
                "z-index": super::constant::z_index::MASK;
            }

            ".mask[data-toggled='false']" {
                "display": "none";
            }

            ".mask[data-toggled='true']" {
                "display": "block";
            }

            ".pallet" {
                "position": "absolute";
                "left": "calc(100% + .35rem)";
                "z-index": super::constant::z_index::MASK + 1;
            }

            ".pallet--left" {
                "top": "0";
                "right": "-0.35rem";
            }

            ".pallet--right" {
                "top": "0";
                "left": "calc(100% + .35rem)";
            }

            ".pallet--bottom" {
                "top": "calc(100% + .35rem)";
                "left": "0";
            }

            ".pallet[data-toggled='false']" {
                "display": "none";
            }

            ".pallet[data-toggled='true']" {
                "display": "block";
            }
        }
    }
}
