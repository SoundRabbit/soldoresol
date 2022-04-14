use super::atom::tab_btn::TabBtn;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub selected: usize,
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
    is_controlled: bool,
}

impl Component for TabMenu {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for TabMenu {}

impl Constructor for TabMenu {
    fn constructor(props: Props) -> Self {
        Self {
            selected_idx: props.selected,
            is_controlled: props.controlled,
        }
    }
}

impl Update for TabMenu {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        if self.is_controlled {
            self.selected_idx = props.selected;
        }

        Cmd::none()
    }

    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::SetSelectedIdx(idx) => {
                if !self.is_controlled {
                    self.selected_idx = idx;
                }
                Cmd::submit(On::ChangeSelectedTab(idx))
            }
        }
    }
}

impl Render<Html> for TabMenu {
    type Children = Vec<(Html, Html)>;
    fn render(&self, children: Self::Children) -> Html {
        let mut tabs = vec![];
        let mut contents = vec![];
        for (tab, content) in children {
            tabs.push(tab);
            contents.push(content);
        }

        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class(Self::class("tabs")),
                    Events::new(),
                    tabs.into_iter()
                        .enumerate()
                        .map(|(tab_idx, tab)| {
                            TabBtn::new(
                                false,
                                tab_idx == self.selected_idx,
                                Attributes::new(),
                                Events::new().on_click(self, move |_| Msg::SetSelectedIdx(tab_idx)),
                                vec![tab],
                            )
                        })
                        .collect(),
                ),
                if contents.len() > self.selected_idx {
                    contents.remove(self.selected_idx)
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
