use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub variant: Variant,
}

#[derive(Clone)]
pub enum Variant {
    Primary,
    Secondary,
    SecondaryLikeMenu,
    Danger,
    Disable,
    Dark,
    DarkLikeMenu,
    TransparentDark,
    Menu,
    MenuAsSecondary,
}

pub struct Btn {}

impl Variant {
    fn is_disable(&self) -> bool {
        match self {
            Self::Disable => true,
            _ => false,
        }
    }
}

impl Btn {
    pub fn class_name(variant: &Variant) -> String {
        match variant {
            Variant::Primary => Self::class("primary"),
            Variant::Secondary => Self::class("secondary"),
            Variant::SecondaryLikeMenu => {
                Self::class("secondary") + " " + &Self::class("like-menu")
            }
            Variant::Danger => Self::class("danger"),
            Variant::Disable => Self::class("disable"),
            Variant::Dark => Self::class("dark"),
            Variant::DarkLikeMenu => Self::class("dark") + " " + &Self::class("like-menu"),
            Variant::TransparentDark => Self::class("transparent-dark"),
            Variant::Menu => Self::class("menu") + " " + &Self::class("like-menu"),
            Variant::MenuAsSecondary => {
                Self::class("menu-secondary") + " " + &Self::class("like-menu")
            }
        }
    }

    pub fn with_variant(
        variant: Variant,
        attrs: Attributes,
        events: Events,
        children: Vec<Html>,
    ) -> Html {
        Self::styled(Html::button(
            Attributes::new()
                .class("pure-button")
                .class(Self::class_name(&variant))
                .flag(if variant.is_disable() { "disabled" } else { "" }),
            events,
            children,
        ))
    }

    pub fn primary(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::with_variant(Variant::Primary, attrs, events, children)
    }

    pub fn secondary(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::with_variant(Variant::Secondary, attrs, events, children)
    }

    pub fn danger(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::with_variant(Variant::Danger, attrs, events, children)
    }

    pub fn dark(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::with_variant(Variant::Dark, attrs, events, children)
    }

    pub fn menu(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::with_variant(Variant::Menu, attrs, events, children)
    }

    pub fn menu_as_secondary(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::with_variant(Variant::MenuAsSecondary, attrs, events, children)
    }
}

impl Styled for Btn {
    fn style() -> Style {
        style! {
            ".primary" {
                "line-height": "1.5";
                "background-color": color_system::blue(100, 5).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".secondary" {
                "line-height": "1.5";
                "background-color": color_system::gray(100, 5).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".danger" {
                "line-height": "1.5";
                "background-color": color_system::red(100, 5).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".dark" {
                "line-height": "1.5";
                "background-color": color_system::gray(100, 9).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".transparent-dark" {
                "line-height": "1.5";
                "background-color": "transparent";
                "color": color_system::gray(100, 0).to_string();
            }

            ".like-menu" {
                "text-align": "left";
            }

            ".menu" {
                "line-height": "1.5";
                "background-color": color_system::gray(100, 9).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".menu:hover" {
                "background-color": color_system::blue(100, 5).to_string();
            }

            ".menu-secondary" {
                "line-height": "1.5";
                "background-color": color_system::gray(100, 5).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".menu-secondary:hover" {
                "background-color": color_system::blue(100, 5).to_string();
            }
        }
    }
}
