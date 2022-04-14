use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Marker {}

impl Marker {
    fn render(color: &str, attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::styled(Html::span(
            attrs.class(Self::class("base")).class(Self::class(color)),
            events,
            children,
        ))
    }

    pub fn purple(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::render("purple", attrs, events, children)
    }

    pub fn light(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::render("light", attrs, events, children)
    }
}

impl Styled for Marker {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "inline-block";
                "border-radius": "2px";
                "background-color": color_system::gray(255, 0);
                "padding": "0.5em 1em";
            }

            ".purple" {
                "color": color_system::purple(255, 5);
                "border": format!("0.1em solid {}", color_system::purple(255, 5));
            }

            ".light" {
                "color": color_system::gray(255, 9);
                "border": format!("0.1em solid {}", color_system::gray(255, 9));
            }
        }
    }
}
