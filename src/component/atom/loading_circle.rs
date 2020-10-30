use super::util::styled::{Style, Styled};
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

pub struct LoadingCircle {
    variant: Variant,
}

impl Constructor for LoadingCircle {
    fn constructor(variant: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self { variant: variant }
    }
}

impl Component for LoadingCircle {
    type Props = Variant;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::span(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(&format!("{}", self.variant))),
            Events::new(),
            vec![],
        ))
    }
}

impl Styled for LoadingCircle {
    fn style() -> Style {
        style! {
            "base" {
                "display": "inline-block";
                "padding": "0";
                "margin": "0";
                "width": "1.1em";
                "height": "1.1em";
                "border-radius": "50%";
                "animation": "0.5s linear infinite rotation";
            }

            "dark" {
                "border-top": format!("0.2em solid {}", crate::color_system::blue(255, 6).to_string());
                "border-left": format!("0.2em solid {}", crate::color_system::blue(191, 6).to_string());
                "border-bottom": format!("0.2em solid {}", crate::color_system::blue(127, 6).to_string());
                "border-right": format!("0.2em solid {}", crate::color_system::blue(63, 6).to_string());
            }

            "light" {
                "border-top": format!("0.2em solid {}", crate::color_system::blue(255, 3).to_string());
                "border-left": format!("0.2em solid {}", crate::color_system::blue(191, 3).to_string());
                "border-bottom": format!("0.2em solid {}", crate::color_system::blue(127, 3).to_string());
                "border-right": format!("0.2em solid {}", crate::color_system::blue(63, 3).to_string());
            }
        }
    }
}
