use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Select {}

impl Select {
    fn render<C: Component>(
        color: &str,
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::styled(Html::select(
            attrs.class(Self::class("base")).class(Self::class(color)),
            events,
            children,
        ))
    }

    pub fn light<C: Component>(
        attrs: Attributes<C>,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Self::render("light", attrs, events, children)
    }
}

impl Styled for Select {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "inline-block";
                "border-radius": "2px";
                "background-color": color_system::gray(255, 0);
                "padding": "0.5em 1em";
            }

            ".light" {
                "color": color_system::gray(255, 9);
                "border": format!("0.1em solid {}", color_system::gray(255, 9));
            }
        }
    }
}
