use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Heading {}

pub enum Variant {
    Dark,
    Light,
}

impl Heading {
    pub fn h1<C: Component>(
        variant: Variant,
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::h1(
            attrs.class(Self::class_name(1, &variant)),
            events,
            children,
        ))
    }

    pub fn h2<C: Component>(
        variant: Variant,
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::h2(
            attrs.class(Self::class_name(2, &variant)),
            events,
            children,
        ))
    }

    pub fn h3<C: Component>(
        variant: Variant,
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::h3(
            attrs.class(Self::class_name(3, &variant)),
            events,
            children,
        ))
    }

    pub fn h4<C: Component>(
        variant: Variant,
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::h4(
            attrs.class(Self::class_name(4, &variant)),
            events,
            children,
        ))
    }

    pub fn h5<C: Component>(
        variant: Variant,
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::h5(
            attrs.class(Self::class_name(5, &variant)),
            events,
            children,
        ))
    }

    pub fn h6<C: Component>(
        variant: Variant,
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::h6(
            attrs.class(Self::class_name(6, &variant)),
            events,
            children,
        ))
    }

    pub fn class_name(level: u32, variant: &Variant) -> String {
        let sufix = match variant {
            Variant::Dark => "dark",
            Variant::Light => "light",
        };

        format!(
            "{} {} {} {}",
            Self::class("base"),
            Self::class(&format!("base--{}", sufix)),
            Self::class(&format!("base--{}", level)),
            Self::class(&format!("base--{}-{}", level, sufix)),
        )
    }
}

impl Styled for Heading {
    fn style() -> Style {
        style! {
            ".base" {
                "margin" : "1.7em 0 0.7em";
                "padding" : "0.3em 1em 0.3em";
                "font-weight": "700";
            }

            ".base--dark" {
                "color" : format!("{}", crate::libs::color::Pallet::gray(2).a(100));
                "border-bottom" : format!("0.1em solid {}", crate::libs::color::Pallet::gray(2).a(100));
            }

            ".base--light" {
                "color" : format!("{}", crate::libs::color::Pallet::gray(6).a(100));
                "border-bottom" : format!("0.1em solid {}", crate::libs::color::Pallet::gray(6).a(100));
            }

            ".base--2" {
                "font-size": "1.5em";
            }

            ".base--3" {
                "font-size": "1.25em";
            }

            ".base--4" {
                "font-size": "1.13em";
            }

            ".base--5" {
                "font-size": "1.06em";
            }

            ".base--6" {
                "font-size": "1.03em";
            }
        }
    }
}
