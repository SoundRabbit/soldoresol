use super::atom::btn::{self, Btn};
use super::atom::fa;
use super::util::window;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Cmd;
use kagura::prelude::*;

pub struct Props {
    pub direction: Direction,
    pub text: Text,
    pub variant: btn::Variant,
    pub toggle_type: ToggleType,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            direction: Direction::BottomLeft,
            text: Text::Text(String::new()),
            toggle_type: ToggleType::Click,
            variant: btn::Variant::Primary,
        }
    }
}

pub enum Direction {
    Bottom,
    BottomLeft,
    BottomRight,
    RightBottom,
}

pub enum Text {
    Text(String),
    Menu,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bottom => write!(f, "bottom"),
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::BottomRight => write!(f, "bottom-right"),
            Self::RightBottom => write!(f, "right-bottom"),
        }
    }
}

impl Direction {
    fn render_caret<C: Component>(&self) -> Html<C> {
        match self {
            Self::Bottom | Self::BottomLeft | Self::BottomRight => fa::i("fa-caret-down"),
            Self::RightBottom => fa::i("fa-caret-right"),
        }
    }
}

#[derive(PartialEq)]
pub enum ToggleType {
    Click,
    Hover,
    Manual(bool),
}

pub enum Msg {
    NoOp,
    ToggleTo(bool),
}

pub enum On {}

pub struct Dropdown {
    is_dropdowned: bool,
}

impl Component for Dropdown {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Dropdown {
    fn constructor(props: &Props) -> Self {
        let is_dropdowned = if let ToggleType::Manual(is_dropdowned) = &props.toggle_type {
            *is_dropdowned
        } else {
            false
        };

        Self {
            is_dropdowned: is_dropdowned,
        }
    }
}

impl Update for Dropdown {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        if let ToggleType::Manual(is_dropdowned) = &props.toggle_type {
            self.is_dropdowned = *is_dropdowned;
        }

        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::ToggleTo(is_dropdowned) => {
                self.is_dropdowned = is_dropdowned;

                if self.is_dropdowned && props.toggle_type == ToggleType::Click {
                    Cmd::task(|resolve| {
                        window::add_event_listener("click", true, move |_| {
                            resolve(Msg::ToggleTo(false))
                        });
                    })
                } else {
                    Cmd::none()
                }
            }
        }
    }
}

impl Render for Dropdown {
    fn render(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(match &props.toggle_type {
            ToggleType::Click => self.render_toggle_by_click(props, children),
            ToggleType::Hover => self.render_toggle_by_hover(props, children),
            ToggleType::Manual(_) => self.render_toggle_by_manual(props, children),
        })
    }
}

impl Dropdown {
    fn toggle_to_up(_: web_sys::Event) -> Msg {
        Msg::ToggleTo(false)
    }

    fn toggle_to_drop(_: web_sys::Event) -> Msg {
        Msg::ToggleTo(true)
    }

    fn base_class_option(&self, props: &Props) -> &str {
        match &props.variant {
            btn::Variant::Menu => "base-menu",
            btn::Variant::MenuAsSecondary => "base-menu",
            btn::Variant::DarkLikeMenu => "base-menu",
            btn::Variant::SecondaryLikeMenu => "base-menu",
            _ => "base-default",
        }
    }

    fn render_toggle_by_click(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option(props))),
            Events::new().on("click", {
                let is_dropdowned = self.is_dropdowned;
                move |_| Msg::ToggleTo(!is_dropdowned)
            }),
            vec![
                self.render_toggle_btn(props),
                self.render_toggled(props, children),
            ],
        )
    }

    fn render_toggle_by_hover(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option(props))),
            Events::new()
                .on("mouseenter", Self::toggle_to_drop)
                .on("mouseleave", Self::toggle_to_up),
            vec![
                self.render_toggle_btn(props),
                self.render_toggled(props, children),
            ],
        )
    }

    fn render_toggle_by_manual(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class(self.base_class_option(props))),
            Events::new(),
            vec![
                self.render_toggle_btn(props),
                self.render_toggled(props, children),
            ],
        )
    }

    fn render_toggle_btn(&self, props: &Props) -> Html<Self> {
        Html::button(
            Attributes::new()
                .class("pure-button")
                .class(Btn::class_name(&props.variant))
                .class(Self::class("root-btn"))
                .string("data-toggled", self.is_dropdowned.to_string()),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("btn")),
                Events::new(),
                match &props.text {
                    Text::Text(text) => vec![Html::text(text), props.direction.render_caret()],
                    Text::Menu => vec![fa::i("fa-ellipsis-v")],
                },
            )],
        )
    }

    fn render_toggled(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Html::div(
            Attributes::new()
                .class(Self::class("content"))
                .class(Self::class(&format!("content-{}", &props.direction)))
                .string("data-toggled", self.is_dropdowned.to_string()),
            Events::new(),
            if self.is_dropdowned { children } else { vec![] },
        )
    }
}

impl Styled for Dropdown {
    fn style() -> Style {
        style! {
            ".base" {
                "position": "relative";
                "overflow": "visible !important";
            }

            ".base-menu" {
                "justify-self": "stretch";
                "display": "grid";
            }

            ".base-default" {
                "max-width": "max-content";
            }

            ".root-btn" {
                "height": "100%";
            }

            r#".root-btn[data-toggled="false"]"# {
                "z-index": "auto";
            }

            r#".root-btn[data-toggled="true"]"# {
                "z-index": format!("{}", super::constant::z_index::MASK + 1);
            }

            ".btn" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "align-items": "center";
                "column-gap": "1ch";
            }

            ".content" {
                "position": "absolute";
                "z-index": format!("{}", super::constant::z_index::MASK + 1);
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "rows";
                "row-gap": "0.05rem";
                "padding-top": "0.05rem";
                "padding-bottom": "0.05rem";
                "justify-items": "stretch";
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "border-radius": "2px";
                "display": "grid";
            }

            r#".content[data-toggled="false"]"# {
                "display": "none";
            }
            r#".content[data-toggled="true"]"# {
                "display": "grid";
            }

            ".content-bottom" {
                "top": "100%";
                "left": "0";
                "right": "0";
                "grid-template-columns": "1fr";
            }

            ".content-bottom-left" {
                "top": "100%";
                "right": "0";
                "grid-template-columns": "max-content";
            }

            ".content-bottom-right" {
                "top": "100%";
                "left": "0";
                "grid-template-columns": "max-content";

            }

            ".content-right-bottom" {
                "top": "0";
                "left": "100%";
                "grid-template-columns": "max-content";
            }

            ".menu-heading" {
                "padding": ".5em .5em";
                "line-height": "1.5";
                "align-items": "center";
                "display": "grid";
                "grid-template-columns": "1fr max-content 1fr";
                "column-gap": ".25em";
            }

            ".menu-heading:before, .menu-heading:after" {
                "content": "''";
                "height": ".15rem";
                "background-color": crate::libs::color::Pallet::gray(1);
            }
        }
    }
}
