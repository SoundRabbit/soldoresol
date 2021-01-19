use super::atom::btn::{self, Btn};
use super::atom::fa;
use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Props {
    pub direction: Direction,
    pub text: String,
    pub variant: btn::Variant,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            text: String::new(),
            variant: btn::Variant::Primary,
            direction: Direction::BottomLeft,
        }
    }
}

pub enum Direction {
    BottomLeft,
    BottomRight,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::BottomRight => write!(f, "bottom-right"),
        }
    }
}

pub enum Msg {
    Toggle,
}

pub enum On {}

pub struct Dropdown {
    direction: Direction,
    is_dropdowned: bool,
    text: String,
    variant: btn::Variant,
}

impl Constructor for Dropdown {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            is_dropdowned: false,
            direction: props.direction,
            text: props.text,
            variant: props.variant,
        }
    }
}

impl Component for Dropdown {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Toggle => {
                self.is_dropdowned = !self.is_dropdowned;
                Cmd::none()
            }
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new().on_click(|_| Msg::Toggle),
            vec![
                Btn::with_children(
                    btn::Props {
                        variant: self.variant.clone(),
                    },
                    Subscription::none(),
                    vec![
                        Html::text(format!("{} ", &self.text)),
                        fa::i("fa-caret-down"),
                    ],
                ),
                Html::div(
                    Attributes::new()
                        .class(Self::class("mask"))
                        .string("data-toggled", self.is_dropdowned.to_string()),
                    Events::new(),
                    vec![],
                ),
                Html::div(
                    Attributes::new()
                        .class(Self::class("content"))
                        .class(Self::class(&format!("content-{}", &self.direction)))
                        .string("data-toggled", self.is_dropdowned.to_string()),
                    Events::new(),
                    children,
                ),
            ],
        ))
    }
}

impl Styled for Dropdown {
    fn style() -> Style {
        style! {
            "base" {
                "position": "relative";
                "max-width": "max-content";
            }

            "mask" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "width": "100vw";
                "height": "100vh";
                "z-index": format!("{}", super::constant::z_index::mask);
            }

            r#"mask[data-toggled="false"]"# {
                "display": "none";
            }

            r#"mask[data-toggled="true"]"# {
                "display": "block";
            }

            "content" {
                "position": "absolute";
                "z-index": format!("{}", super::constant::z_index::mask + 1);
            }

            r#"content[data-toggled="false"]"# {
                "display": "none";
            }
            r#"content[data-toggled="true"]"# {
                "display": "block";
            }

            "content-bottom-left" {
                "top": "100%";
                "right": "0";
            }

            "content-bottom-right" {
                "top": "100%";
                "left": "0";
            }
        }
    }
}
