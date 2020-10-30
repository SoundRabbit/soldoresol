use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Props {
    pub variant: Variant,
}

pub enum Variant {
    Primary,
    Danger,
}

pub enum Msg {
    Clicked,
}

pub enum On {
    Click,
}

pub struct Btn {
    variant: Variant,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
            Self::Danger => write!(f, "danger"),
        }
    }
}

impl Constructor for Btn {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            variant: props.variant,
        }
    }
}

impl Component for Btn {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Clicked => Cmd::sub(On::Click),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::button(
            Attributes::new()
                .class("pure-button")
                .class(Self::class(&format!("{}", &self.variant))),
            Events::new().on_click(|_| Msg::Clicked),
            children,
        ))
    }
}

impl Styled for Btn {
    fn style() -> Style {
        style! {
            "primary" {
                "background-color": crate::color_system::blue(255, 5).to_string();
                "color": crate::color_system::gray(255, 0).to_string();
            }
        }
    }
}
