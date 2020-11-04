use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Props {
    pub variant: Variant,
}

pub enum Variant {
    Primary,
    Secondary,
    Danger,
    Disable,
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

impl Variant {
    fn is_disable(&self) -> bool {
        match self {
            Self::Disable => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
            Self::Secondary => write!(f, "secondary"),
            Self::Danger => write!(f, "danger"),
            Self::Disable => write!(f, "disable"),
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

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.variant = props.variant;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Clicked => Cmd::sub(On::Click),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::button(
            Attributes::new()
                .class("pure-button")
                .class(Self::class(&format!("{}", &self.variant)))
                .flag(if self.variant.is_disable() {
                    "disabled"
                } else {
                    ""
                }),
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

            "secondary" {
                "background-color": crate::color_system::gray(255, 5).to_string();
                "color": crate::color_system::gray(255, 0).to_string();
            }
        }
    }
}
