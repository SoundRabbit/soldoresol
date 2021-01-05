use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Card {}

impl Constructor for Card {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {}
    }
}

impl Component for Card {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class("base--medium")),
            Events::new(),
            children,
        ))
    }
}

impl Styled for Card {
    fn style() -> Style {
        style! {
            "base"{
                "background-color": color_system::gray(100, 0).to_string();
                "color": color_system::gray(100, 9).to_string();
                "padding": ".4em";
                "box-shadow": format!("0px 0px 1px 1px {}", color_system::gray(25, 9));
                "margin": "2px";
                "border-radius": "2px";
            }

            "base--medium" {
                "width": "18em";
                "height": "24em";
            }
        }
    }
}
