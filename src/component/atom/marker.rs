use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Marker {}

impl Marker {
    fn outline(color: &str, attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::styled(Html::span(
            attrs
                .class(Self::class("outline-base"))
                .class(Self::class(color)),
            events,
            children,
        ))
    }

    fn fill(color: &str, attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::styled(Html::span(
            attrs
                .class(Self::class("fill-base"))
                .class(Self::class(color)),
            events,
            children,
        ))
    }

    pub fn blue(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::outline("outline-blue", attrs, events, children)
    }

    pub fn purple(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::outline("outline-purple", attrs, events, children)
    }

    pub fn light(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::outline("outline-light", attrs, events, children)
    }

    pub fn fill_blue(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::fill("fill-blue", attrs, events, children)
    }

    pub fn fill_purple(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Self::fill("fill-purple", attrs, events, children)
    }
}

impl Styled for Marker {
    fn style() -> Style {
        style! {
            ".outline-base" {
                "display": "inline-block";
                "border-radius": "2px";
                "background-color": color_system::gray(255, 0);
                "padding": "0.5em 1em";
            }

            ".fill-base" {
                "display": "inline-block";
                "border-radius": "2px";
                "border": format!("0.1em solid {}", color_system::gray(255, 0));
                "color": color_system::gray(255, 0);
                "padding": "0.5em 1em";
            }

            ".outline-blue" {
                "color": color_system::blue(255, 5);
                "border": format!("0.1em solid {}", color_system::blue(255, 5));
            }

            ".outline-purple" {
                "color": color_system::purple(255, 5);
                "border": format!("0.1em solid {}", color_system::purple(255, 5));
            }

            ".outline-light" {
                "color": color_system::gray(255, 9);
                "border": format!("0.1em solid {}", color_system::gray(255, 9));
            }

            ".fill-blue" {
                "background-color": color_system::blue(255, 5);
            }

            ".fill-purple" {
                "background-color": color_system::purple(255, 5);
            }
        }
    }
}
