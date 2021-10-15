use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub enum Variant {
    Dark,
    Light,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => write!(f, "dark"),
            Self::Light => write!(f, "light"),
        }
    }
}

pub enum Msg {}

pub enum On {}

pub struct LoadingCircle {}

impl Component for LoadingCircle {
    type Props = Variant;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for LoadingCircle {
    fn constructor(variant: &Variant) -> Self {
        Self {}
    }
}

impl Update for LoadingCircle {}

impl Render for LoadingCircle {
    fn render(&self, variant: &Variant, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::span(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(&format!("{}", variant))),
            Events::new(),
            vec![],
        ))
    }
}

impl Styled for LoadingCircle {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "inline-block";
                "padding": "0";
                "margin": "0";
                "width": "1.1em";
                "height": "1.1em";
                "border-radius": "50%";
                "animation": "0.5s linear infinite rotation";
            }

            ".dark" {
                "border-top": format!("0.2em solid {}", color_system::blue(100, 6).to_string());
                "border-left": format!("0.2em solid {}", color_system::blue(75, 6).to_string());
                "border-bottom": format!("0.2em solid {}", color_system::blue(50, 6).to_string());
                "border-right": format!("0.2em solid {}", color_system::blue(25, 6).to_string());
            }

            ".light" {
                "border-top": format!("0.2em solid {}", color_system::blue(100, 3).to_string());
                "border-left": format!("0.2em solid {}", color_system::blue(75, 3).to_string());
                "border-bottom": format!("0.2em solid {}", color_system::blue(50, 3).to_string());
                "border-right": format!("0.2em solid {}", color_system::blue(25, 3).to_string());
            }
        }
    }
}
