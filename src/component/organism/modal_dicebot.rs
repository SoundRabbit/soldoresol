use super::atom::{
    btn::Btn,
    heading::{self, Heading},
};
use super::molecule::modal::{self, Modal};
use crate::libs::bcdice::js::{DynamicLoader, GameSystemClass, GameSystemInfo};
use crate::libs::js_object::Object;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub bcdice_loader: Rc<DynamicLoader>,
    pub selected_game_system: Option<String>,
}

pub enum Msg {
    Sub(On),
    SetSelectedInitial(String),
    SetSelectedGameSystem(String),
    LoadSelectedGameSystem,
}

pub enum On {
    Close,
    SelectGameSystem { game_system_class: GameSystemClass },
}

pub struct ModalDicebot {
    bcdice_loader: Rc<DynamicLoader>,
    system_infos: Vec<(String, Vec<GameSystemInfo>)>,
    selected_initial: String,
    selected_game_system: String,
    selected_game_system_name: String,
}

impl Component for ModalDicebot {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalDicebot {}

impl Constructor for ModalDicebot {
    fn constructor(props: Self::Props) -> Self {
        let system_infos = Self::get_system_infos(props.bcdice_loader.available_game_systems());
        let selected_game_system = props.selected_game_system.unwrap_or_else(|| {
            system_infos
                .first()
                .map(|(_, system_info)| system_info[0].id().clone())
                .unwrap_or_else(|| String::new())
        });
        let selected_game_system_name =
            Self::get_selected_game_system_name(&system_infos, &selected_game_system);
        Self {
            bcdice_loader: props.bcdice_loader,
            system_infos,
            selected_initial: String::new(),
            selected_game_system,
            selected_game_system_name,
        }
    }
}

impl Update for ModalDicebot {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.bcdice_loader = props.bcdice_loader;
        self.system_infos = Self::get_system_infos(self.bcdice_loader.available_game_systems());
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetSelectedInitial(initial) => {
                self.selected_initial = initial;
                Cmd::none()
            }
            Msg::SetSelectedGameSystem(game_system) => {
                self.selected_game_system = game_system;
                self.selected_game_system_name = Self::get_selected_game_system_name(
                    &self.system_infos,
                    &self.selected_game_system,
                );
                Cmd::none()
            }
            Msg::LoadSelectedGameSystem => {
                let game_system = self.selected_game_system.clone();
                let bcdice_loader = Rc::clone(&self.bcdice_loader);
                Cmd::task(async move {
                    bcdice_loader
                        .dynamic_load(&game_system)
                        .await
                        .map(|game_system_class| {
                            Cmd::list(vec![
                                Cmd::submit(On::SelectGameSystem { game_system_class }),
                                Cmd::submit(On::Close),
                            ])
                        })
                        .unwrap_or_else(|| Cmd::submit(On::Close))
                })
            }
        }
    }
}

impl ModalDicebot {
    fn get_system_infos(system_infos: &Vec<Object>) -> Vec<(String, Vec<GameSystemInfo>)> {
        let mut system_infos: Vec<_> = system_infos
            .iter()
            .filter_map(|game_system_info| game_system_info.try_as::<GameSystemInfo>())
            .collect();

        system_infos.sort_by(|a, b| {
            a.sort_key()
                .partial_cmp(b.sort_key())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut initial = '\0';
        let mut categorised_system_infos = vec![];

        for system_info in system_infos {
            let item_initial = system_info.sort_key().chars().nth(0).unwrap_or('\0');
            if item_initial != initial {
                initial = item_initial;
                categorised_system_infos.push((String::from(initial), vec![]));
            }

            if let Some(system_infos) = categorised_system_infos.last_mut() {
                system_infos.1.push(system_info);
            }
        }

        categorised_system_infos
    }

    fn get_selected_game_system_name(
        system_infos: &Vec<(String, Vec<GameSystemInfo>)>,
        selected: &String,
    ) -> String {
        for (_, system_infos) in system_infos {
            for system_info in system_infos {
                if *system_info.id() == *selected {
                    return system_info.name().clone();
                }
            }
        }

        String::from("")
    }
}

impl Render<Html> for ModalDicebot {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Modal::new(
            self,
            None,
            modal::Props {},
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Sub(On::Close),
            }),
            (
                String::from("システムを選択"),
                String::from(""),
                vec![Html::div(
                    Attributes::new().class(Self::class("base")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("container")),
                            Events::new(),
                            vec![Heading::h3(
                                heading::Variant::Light,
                                Attributes::new(),
                                Events::new(),
                                vec![
                                    Html::text("選択中："),
                                    Html::text(&self.selected_game_system_name)
                                ]
                            )],
                        ),
                        Html::div(
                            Attributes::new()
                                .class(Self::class("content"))
                                .class(Self::class("container")),
                            Events::new(),
                            vec![
                                Html::div(
                                    Attributes::new().class(Self::class("list")),
                                    Events::new(),
                                    self.system_infos
                                        .iter()
                                        .map(|(initial, ..)| {
                                            if *initial != *self.selected_initial {
                                                Btn::menu(
                                                    Attributes::new(),
                                                    Events::new().on_click(self, {
                                                        let initial = initial.clone();
                                                        move |_| Msg::SetSelectedInitial(initial)
                                                    }),
                                                    vec![Html::text(initial)],
                                                )
                                            } else {
                                                Btn::menu_as_primary(
                                                    Attributes::new(),
                                                    Events::new(),
                                                    vec![Html::text(initial)],
                                                )
                                            }
                                        })
                                        .collect(),
                                ),
                                Html::div(
                                    Attributes::new().class(Self::class("list")),
                                    Events::new(),
                                    self.system_infos
                                        .iter()
                                        .filter_map(|(initial, game_system_infos)| {
                                            if *initial == self.selected_initial {
                                                Some(
                                                    game_system_infos
                                                        .iter()
                                                        .map(|game_system_info| {
                                                            if *game_system_info.id()
                                                                != self.selected_game_system
                                                            {
                                                                Btn::menu(
                                                                    Attributes::new(),
                                                                    Events::new().on_click(self, {
                                                                        let game_system = game_system_info.id().clone();
                                                                        move |_| Msg::SetSelectedGameSystem(game_system)
                                                                    }),
                                                                    vec![Html::text(
                                                                        game_system_info.name(),
                                                                    )],
                                                                )
                                                            } else {
                                                                Btn::menu_as_primary(
                                                                    Attributes::new(),
                                                                    Events::new(),
                                                                    vec![Html::text(
                                                                        game_system_info.name(),
                                                                    )],
                                                                )
                                                            }
                                                        })
                                                        .collect::<Vec<_>>(),
                                                )
                                            } else {
                                                None
                                            }
                                        })
                                        .flatten()
                                        .collect(),
                                ),
                            ],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("container")),
                            Events::new(),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(self, |_| Msg::LoadSelectedGameSystem),
                                vec![Html::text("決定")],
                            )],
                        ),
                    ],
                )],
            ),
        ))
    }
}

impl Styled for ModalDicebot {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "height": "100%";
                "display": "grid";
                "grid-template-rows": "max-content 1fr max-content";
                "overflow-y": "hidden";
            }
            ".content" {
                "display": "grid";
                "grid-template-columns": "repeat(auto-fit, minmax(20rem, 1fr))";
                "overflow": "hidden";
            }
            ".container" {
                "padding": ".5em 1em";
            }
            ".list" {
                "overflow-y": "scroll";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-auto-rows": "max-content";
                "row-gap": "0.25rem";
            }
        }
    }
}
