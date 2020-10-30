use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct BasicApp;

impl Constructor for BasicApp {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        crate::debug::log_1(format!("construct {}", std::any::type_name::<Self>()));

        Self {}
    }
}

impl Component for BasicApp {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        crate::debug::log_1(format!("init {}", std::any::type_name::<Self>()));
    }

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            children,
        ))
    }
}

impl Styled for BasicApp {
    fn style() -> Style {
        style! {
            "base"{
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
            }
        }
    }
}
