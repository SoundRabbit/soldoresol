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
    PrimaryLikeMenu,
    Secondary,
    SecondaryLikeMenu,
    Danger,
    Disable,
    Dark,
    DarkLikeMenu,
    TransparentDark,
    Menu,
    MenuAsSecondary,
    Success,
    Light,
    LightLikeMenu,
    TransparentLight,
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
            Variant::PrimaryLikeMenu => Self::class("primary") + " " + &Self::class("like-menu"),
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
            Variant::Success => Self::class("success"),
            Variant::Light => Self::class("light"),
            Variant::LightLikeMenu => Self::class("light") + " " + &Self::class("like-menu"),
            Variant::TransparentLight => Self::class("transparent-light"),
        }
    }

    pub fn with_variant<C: Component>(
        variant: Variant,
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::button(
            attrs
                .class("pure-button")
                .class(Self::class_name(&variant))
                .flag(if variant.is_disable() { "disabled" } else { "" }),
            events,
            children,
        ))
    }

    pub fn primary<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Primary, attrs, events, children)
    }

    pub fn secondary<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Secondary, attrs, events, children)
    }

    pub fn danger<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Danger, attrs, events, children)
    }

    pub fn dark<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Dark, attrs, events, children)
    }

    pub fn menu<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Menu, attrs, events, children)
    }

    pub fn menu_as_secondary<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::MenuAsSecondary, attrs, events, children)
    }

    pub fn success<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Success, attrs, events, children)
    }

    pub fn light<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::with_variant(Variant::Light, attrs, events, children)
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
                "border-radius": "2px";
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

            ".success" {
                "line-height": "1.5";
                "background-color": color_system::green(100, 5).to_string();
                "color": color_system::gray(100, 0).to_string();
            }

            ".light" {
                "line-height": "1.5";
                "background-color": color_system::gray(100, 3).to_string();
                "color": color_system::gray(100, 9).to_string();
            }

            ".transparent-light" {
                "line-height": "1.5";
                "background-color": "transparent";
                "color": color_system::gray(100, 9).to_string();
            }
        }
    }
}
