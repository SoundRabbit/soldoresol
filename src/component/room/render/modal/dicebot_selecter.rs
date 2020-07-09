use super::super::super::super::modal;
use super::super::super::state::dicebot;
use super::Msg;
use crate::{resource::Data, Resource};
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(dicebot_state: &dicebot::State) -> Html<Msg> {
    let dummy = vec![];

    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("ダイスボット"),
                modal::body(
                    Attributes::new().class("scroll-v").class("linear-v"),
                    Events::new(),
                    dicebot_state
                        .bcdice()
                        .names()
                        .map(|names| names.as_ref())
                        .unwrap_or(&dummy)
                        .iter()
                        .map(|name| {
                            let system = name.system().clone();
                            Html::div(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::GetBcdiceSystemInfo(system)),
                                vec![Html::text(name.name())],
                            )
                        })
                        .collect(),
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
