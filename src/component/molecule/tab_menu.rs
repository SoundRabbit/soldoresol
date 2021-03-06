use super::atom::tab_btn::{self, TabBtn};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub selected: usize,
    pub tabs: Vec<String>,
    pub controlled: bool,
}

pub enum Msg {
    NoOp,
    SetSelectedIdx(usize),
}

pub enum On {
    ChangeSelectedTab(usize),
}

pub struct TabMenu {
    selected_idx: usize,
    tabs: Vec<String>,
    is_controlled: bool,
}

impl Constructor for TabMenu {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            selected_idx: props.selected,
            tabs: props.tabs,
            is_controlled: props.controlled,
        }
    }
}

impl Component for TabMenu {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        if self.is_controlled {
            self.selected_idx = props.selected;
            self.tabs = props.tabs;
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetSelectedIdx(idx) => {
                if self.is_controlled {
                    Cmd::sub(On::ChangeSelectedTab(idx))
                } else {
                    self.selected_idx = idx;
                    Cmd::none()
                }
            }
        }
    }

    fn render(&self, mut children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("tabs")),
                    Events::new(),
                    self.tabs
                        .iter()
                        .enumerate()
                        .map(|(tab_idx, tab_name)| {
                            TabBtn::new(
                                false,
                                tab_idx == self.selected_idx,
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::SetSelectedIdx(tab_idx)),
                                vec![Html::text(tab_name)],
                            )
                        })
                        .collect(),
                ),
                children.remove(self.selected_idx),
            ],
        ))
    }
}

impl Styled for TabMenu {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "max-height": "100%";
                "width": "100%";
                "overflow": "hidden";
            }

            ".tabs" {
                "display": "flex";
                "flex-wrap": "wrap";
            }
        }
    }
}
