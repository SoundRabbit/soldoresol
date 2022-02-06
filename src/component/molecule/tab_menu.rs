use super::atom::tab_btn::TabBtn;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Cmd;
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

impl Component for TabMenu {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for TabMenu {
    fn constructor(props: &Props) -> Self {
        Self {
            selected_idx: props.selected,
            tabs: props.tabs.clone(),
            is_controlled: props.controlled,
        }
    }
}

impl Update for TabMenu {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        if self.is_controlled {
            self.selected_idx = props.selected;
            self.tabs = props.tabs.clone();
        }

        Cmd::none()
    }

    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
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
}

impl Render for TabMenu {
    fn render(&self, props: &Props, mut children: Vec<Html<Self>>) -> Html<Self> {
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
                if children.len() > self.selected_idx {
                    children.remove(self.selected_idx)
                } else {
                    Html::div(Attributes::new(), Events::new(), vec![])
                },
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
                "height": "100%";
                "overflow": "hidden";
            }

            ".tabs" {
                "display": "flex";
                "flex-wrap": "wrap";
            }
        }
    }
}
