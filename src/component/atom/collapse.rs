use super::atom::fa;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

pub struct Props {
    pub is_collapsed: bool,
}

pub enum Msg {
    SetIsCollapsed(bool),
}

pub enum On {}

pub struct Collapse {
    is_collapsed: bool,
}

impl Component for Collapse {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Collapse {
    fn constructor(props: &Props) -> Self {
        Self {
            is_collapsed: props.is_collapsed,
        }
    }
}

impl Update for Collapse {
    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::SetIsCollapsed(is_collapsed) => {
                self.is_collapsed = is_collapsed;
                Cmd::none()
            }
        }
    }
}

impl Render for Collapse {
    fn render(&self, _props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        let mut children = std::collections::VecDeque::from(children);
        let head = children.pop_front().unwrap_or(Html::none());
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("toggle")),
                    Events::new().on_click({
                        let is_collapsed = self.is_collapsed;
                        move |_| Msg::SetIsCollapsed(!is_collapsed)
                    }),
                    vec![fa::far_i(if children.is_empty() {
                        "fa-square"
                    } else if self.is_collapsed {
                        "fa-caret-square-down"
                    } else {
                        "fa-caret-square-right"
                    })],
                ),
                Html::div(
                    Attributes::new().class(Self::class("head")),
                    Events::new(),
                    vec![head],
                ),
                Html::div(
                    Attributes::new().class(Self::class("content")).string(
                        "data-collapsed",
                        (!children.is_empty() && self.is_collapsed).to_string(),
                    ),
                    Events::new(),
                    children.into(),
                ),
            ],
        ))
    }
}

impl Collapse {}

impl Styled for Collapse {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "display": "grid";
                "grid-template-columns": "1rem max-content 1fr";
                "grid-template-rows": "max-content max-content";
                "overflow": "hidden";
            }

            ".toggle" {
                "grid-column": "1 / 3";
                "grid-row": "1 / 2";
                "padding": ".65em";
                "line-height": "1.5";
                "align-self": "start";
            }

            ".head" {
                "grid-column": "3 / 4";
                "grid-row": "1 / 2";
                "display": "flex";
                "align-items": "center";
                "overflow": "hidden";
            }

            ".content" {
                "grid-column": "2 / 4";
                "grid-row": "2 / 3";
                "overflow": "hidden";
            }

            ".content[data-collapsed='false']" {
                "min-height": "0";
                "max-height": "0";
            }

            ".content[data-collapsed='true']" {
                "min-height": "max-content";
                "max-height": "max-content";
            }
        }
    }
}
