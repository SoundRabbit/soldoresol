use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Props {
    pub level: u32,
}

pub enum Msg {}

pub enum On {}

pub struct Heading {
    level: u32,
}

impl Constructor for Heading {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self { level: props.level }
    }
}

impl Component for Heading {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.level = props.level;
    }

    fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        Cmd::none()
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(match self.level {
            1 => Html::h1(
                Attributes::new()
                    .class(Self::class("base"))
                    .class("base--1"),
                Events::new(),
                children,
            ),
            2 => Html::h2(
                Attributes::new()
                    .class(Self::class("base"))
                    .class("base--2"),
                Events::new(),
                children,
            ),
            3 => Html::h3(
                Attributes::new()
                    .class(Self::class("base"))
                    .class("base--3"),
                Events::new(),
                children,
            ),
            4 => Html::h4(
                Attributes::new()
                    .class(Self::class("base"))
                    .class("base--4"),
                Events::new(),
                children,
            ),
            5 => Html::h5(
                Attributes::new()
                    .class(Self::class("base"))
                    .class("base--5"),
                Events::new(),
                children,
            ),
            6 => Html::h6(
                Attributes::new()
                    .class(Self::class("base"))
                    .class("base--6"),
                Events::new(),
                children,
            ),
            _ => unreachable!(),
        })
    }
}

impl Styled for Heading {
    fn style() -> Style {
        style! {}
    }
}
