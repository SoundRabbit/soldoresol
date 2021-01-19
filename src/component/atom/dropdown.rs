use super::atom::btn::{self, Btn};
use super::atom::fa;
use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Props {
    pub direction: Direction,
    pub text: String,
    pub variant: btn::Variant,
    pub toggle_type: ToggleType,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            direction: Direction::BottomLeft,
            text: String::new(),
            toggle_type: ToggleType::Click,
            variant: btn::Variant::Primary,
        }
    }
}

pub enum Direction {
    BottomLeft,
    BottomRight,
    RightBottom,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::BottomRight => write!(f, "bottom-right"),
            Self::RightBottom => write!(f, "right-bottom"),
        }
    }
}

impl Direction {
    fn render_caret(&self) -> Html {
        match self {
            Self::BottomLeft | Self::BottomRight => fa::i("fa-caret-down"),
            Self::RightBottom => fa::i("fa-caret-right"),
        }
    }
}

pub enum ToggleType {
    Click,
    Hover,
}

pub enum Msg {
    Toggle,
    ToggleTo(bool),
}

pub enum On {}

pub struct Dropdown {
    direction: Direction,
    is_dropdowned: bool,
    text: String,
    toggle_type: ToggleType,
    variant: btn::Variant,
}

impl Constructor for Dropdown {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            is_dropdowned: false,
            direction: props.direction,
            text: props.text,
            toggle_type: props.toggle_type,
            variant: props.variant,
        }
    }
}

impl Component for Dropdown {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.direction = props.direction;
        self.text = props.text;
        self.toggle_type = props.toggle_type;
        self.variant = props.variant;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::Toggle => {
                self.is_dropdowned = !self.is_dropdowned;
                Cmd::none()
            }
            Msg::ToggleTo(is_dropdowned) => {
                self.is_dropdowned = is_dropdowned;
                Cmd::none()
            }
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(match &self.toggle_type {
            ToggleType::Click => self.render_toggle_by_click(children),
            ToggleType::Hover => self.render_toggle_by_hover(children),
        })
    }
}

impl Dropdown {
    fn toggle(_: web_sys::Event) -> Msg {
        Msg::Toggle
    }

    fn toggle_to_up(_: web_sys::Event) -> Msg {
        Msg::ToggleTo(false)
    }

    fn toggle_to_drop(_: web_sys::Event) -> Msg {
        Msg::ToggleTo(true)
    }

    fn base_class_option(&self) -> &str {
        match &self.variant {
            btn::Variant::Menu => "base-menu",
            _ => "base-default",
        }
    }

    fn render_toggle_by_click(&self, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option())),
            Events::new().on("click", Self::toggle),
            vec![
                self.render_toggle_btn(),
                self.render_toggle_mask(),
                self.render_toggled(children),
            ],
        )
    }
    fn render_toggle_by_hover(&self, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option())),
            Events::new()
                .on("mouseenter", Self::toggle_to_drop)
                .on("mouseleave", Self::toggle_to_up),
            vec![self.render_toggle_btn(), self.render_toggled(children)],
        )
    }

    fn render_toggle_btn(&self) -> Html {
        Btn::with_children(
            btn::Props {
                variant: self.variant.clone(),
            },
            Subscription::none(),
            vec![Html::div(
                Attributes::new().class(Self::class("btn")),
                Events::new(),
                vec![Html::text(&self.text), self.direction.render_caret()],
            )],
        )
    }

    fn render_toggle_mask(&self) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("mask"))
                .string("data-toggled", self.is_dropdowned.to_string()),
            Events::new(),
            vec![],
        )
    }

    fn render_toggled(&self, children: Vec<Html>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("content"))
                .class(Self::class(&format!("content-{}", &self.direction)))
                .string("data-toggled", self.is_dropdowned.to_string()),
            Events::new(),
            children,
        )
    }
}

impl Styled for Dropdown {
    fn style() -> Style {
        style! {
            "base" {
                "position": "relative";
            }

            "base-menu" {
                "justify-self": "stretch";
                "display": "grid";
            }

            "base-default" {
                "max-width": "max-content";
            }

            "btn" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "align-items": "center";
                "column-gap": "1ch";
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
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "rows";
                "row-gap": "0.05rem";
                "justify-items": "stretch";
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "border-radius": "2px";
            }

            r#"content[data-toggled="false"]"# {
                "display": "none";
            }
            r#"content[data-toggled="true"]"# {
                "display": "grid";
            }

            "content-bottom-left" {
                "top": "100%";
                "right": "0";
            }

            "content-bottom-right" {
                "top": "100%";
                "left": "0";
            }

            "content-right-bottom" {
                "top": "0";
                "left": "100%";
            }
        }
    }
}
