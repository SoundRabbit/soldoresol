use super::atom::fa;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub is_default_collapsed: bool,
    pub is_indented: bool,
}

pub enum Msg {
    SetIsCollapsed(bool),
}

pub enum On {}

pub struct Collapse {
    is_collapsed: bool,
    is_indented: bool,
}

impl Component for Collapse {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Collapse {}

impl Constructor for Collapse {
    fn constructor(props: &Props) -> Self {
        Self {
            is_collapsed: props.is_default_collapsed,
            is_indented: props.is_indented,
        }
    }
}

impl Update for Collapse {
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.is_indented = props.is_indented;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, _props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::SetIsCollapsed(is_collapsed) => {
                self.is_collapsed = is_collapsed;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Collapse {
    type Children = (Html, Vec<Html>);

    fn render(&self, (head, children): Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("toggle")),
                    Events::new().on_click(self, {
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
                    Attributes::new()
                        .class(Self::class("content"))
                        .string(
                            "data-collapsed",
                            (!children.is_empty() && self.is_collapsed).to_string(),
                        )
                        .string("data-indented", self.is_indented.to_string()),
                    Events::new(),
                    children,
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
                "grid-row": "2 / 3";
                "overflow": "hidden";
            }

            ".content[data-indented='false']" {
                "grid-column": "1 / 4";
            }

            ".content[data-indented='true']" {
                "grid-column": "2 / 4";
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
