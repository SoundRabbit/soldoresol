use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    class: Option<String>,
}

pub enum Msg {}

pub enum On {}

pub struct Header {}

impl Props {
    pub fn new() -> Self {
        Self { class: None }
    }

    fn class(mut self, class_name: impl Into<String>) -> Self {
        self.class = Some(class_name.into());
        self
    }
}

impl Component for Header {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Header {
    fn constructor(props: &Props) -> Self {
        Self {}
    }
}

impl Update for Header {}

impl Render for Header {
    fn render(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        let attrs = Attributes::new().class(Self::class("base"));
        let attrs = if let Some(class_name) = &props.class {
            attrs.class(class_name)
        } else {
            attrs
        };

        Self::styled(Html::div(attrs, Events::new(), children))
    }
}

impl Styled for Header {
    fn style() -> Style {
        style! {
            ".base"{
                "background-color": color_system::gray(255, 8).to_string();
                "color": color_system::gray(255, 0).to_string();
                "padding": ".65em";
                "row-gap": ".65em";
                "display": "grid";
            }
        }
    }
}
