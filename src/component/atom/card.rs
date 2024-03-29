use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Card {}

impl Component for Card {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl Constructor for Card {
    fn constructor(props: Self::Props) -> Self {
        Self {}
    }
}

impl HtmlComponent for Card {}

impl Update for Card {}

impl Render<Html> for Card {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
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
            ".base"{
                "background-color": color_system::gray(100, 0).to_string();
                "color": color_system::gray(100, 9).to_string();
                "padding": ".4em";
                "box-shadow": format!("0px 0px 1px 1px {}", color_system::gray(25, 9));
                "margin": "2px";
                "border-radius": "2px";
            }

            ".base--medium" {
                "width": "18em";
                "height": "24em";
            }
        }
    }
}
