use super::super::super::super::{btn, icon, modal};
use super::super::super::state::dicebot;
use super::Msg;
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render(dicebot_state: &dicebot::State) -> Html<Msg> {
    let dummy = vec![];
    let mut last_initial = '\0';
    let mut systems = vec![('\0', vec![])];
    for name in dicebot_state
        .bcdice()
        .names()
        .map(|names| names.as_ref())
        .unwrap_or(&dummy)
        .iter()
    {
        let initial = name.sort_key().chars().next().unwrap_or('\0');
        if initial == last_initial {
            let tail = systems.len() - 1;
            systems[tail].1.push((name.name(), name.system()));
        } else {
            systems.push((initial, vec![(name.name(), name.system())]));
            last_initial = initial;
        }
    }

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
                    systems
                        .into_iter()
                        .filter_map(|(initial, systems)| {
                            if systems.len() > 0 {
                                Some(vec![
                                    icon::from_char(
                                        Attributes::new().class("icon-medium"),
                                        initial,
                                    ),
                                    Html::div(
                                        Attributes::new()
                                            .class("container-indent")
                                            .class("linear-v")
                                            .class("linear-v-stretch"),
                                        Events::new(),
                                        systems
                                            .into_iter()
                                            .map(|(name, system)| {
                                                let system = system.clone();
                                                btn::selectable(
                                                    system == dicebot_state.bcdice().system_name(),
                                                    Attributes::new().class("pure-button-list"),
                                                    Events::new().on_click(move |_| {
                                                        Msg::GetBcdiceSystemInfo(system)
                                                    }),
                                                    vec![Html::text(name)],
                                                )
                                            })
                                            .collect(),
                                    ),
                                ])
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect(),
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
