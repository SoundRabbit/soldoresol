use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use kagura::prelude::*;

pub struct Props {
    class: Option<String>,
}

pub enum Msg {}

pub enum On {}

pub struct Header {
    class: Option<String>,
}

impl Props {
    pub fn new() -> Self {
        Self { class: None }
    }

    fn class(mut self, class_name: impl Into<String>) -> Self {
        self.class = Some(class_name.into());
        self
    }
}

impl Constructor for Header {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self { class: props.class }
    }
}

impl Component for Header {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        let attrs = Attributes::new().class(Self::class("base"));
        let attrs = if let Some(class_name) = &self.class {
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
            "base"{
                "background-color": color_system::gray(255, 8).to_string();
                "color": color_system::gray(255, 0).to_string();
                "padding": ".65em";
                "row-gap": ".65em";
                "display": "grid";
            }
        }
    }
}
