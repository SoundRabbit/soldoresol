use super::atom::loading_circle::{self, LoadingCircle};
use component::Sub;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Loader {}

impl Component for Loader {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Loader {
    fn constructor(_: &Props) -> Self {
        Self {}
    }
}

impl Update for Loader {}

impl Render for Loader {
    fn render(&self, _: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                LoadingCircle::empty(loading_circle::Variant::Dark, Sub::none()),
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
