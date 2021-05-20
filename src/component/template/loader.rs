use super::atom::loading_circle::{self, LoadingCircle};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum Sub {}

pub struct Loader {}

impl Constructor for Loader {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {}
    }
}

impl Component for Loader {
    type Props = Props;
    type Msg = Msg;
    type Sub = Sub;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, _: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                LoadingCircle::empty(loading_circle::Variant::Dark, Subscription::none()),
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("loading")],
                ),
            ],
        ))
    }
}

impl Styled for Loader {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "height": "100%";
                "grid-template-columns": "max-content max-content";
                "justify-content": "center";
                "align-content": "center";
                "align-items": "center";
                "column-gap": "0.1em";
            }
        }
    }
}
