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

pub struct Header {}

impl Component for Header {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Header {}

impl Constructor for Header {
    fn constructor(props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for Header {}

impl Render<Html> for Header {
    type Children = (Attributes, Events, Vec<Html>);
    fn render(&self, (attrs, events, children): Self::Children) -> Html {
        let attrs = attrs.class(Self::class("base"));
        Self::styled(Html::div(attrs, events, children))
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
