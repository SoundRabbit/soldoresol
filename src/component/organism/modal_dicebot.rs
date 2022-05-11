use super::atom::{
    btn::Btn,
    heading::{self, Heading},
};
use super::molecule::modal::{self, Modal};
use crate::libs::bcdice;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Props {
    api_root: Rc<String>,
}

pub enum Msg {
    Sub(On),
    SetGameSystems(VecDeque<bcdice::api::GameSystem>),
}

pub enum On {
    Close,
}

pub struct ModalDicebot {
    api_root: Rc<String>,
    dicebot: Option<bcdice::api::GameSystem>,
    game_systems: Vec<Rc<bcdice::api::GameSystem>>,
}

impl Component for ModalDicebot {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalDicebot {}

impl Constructor for ModalDicebot {
    fn constructor(props: Self::Props) -> Self {
        Self {
            api_root: props.api_root,
            dicebot: None,
            game_systems: vec![],
        }
    }
}

impl Update for ModalDicebot {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        if *self.api_root != *props.api_root {
            self.api_root = props.api_root;
            self.get_game_system()
        } else {
            Cmd::none()
        }
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
            Msg::SetGameSystems(mut game_systems) => {
                self.dicebot = game_systems.pop_front();

                let mut game_systems: Vec<_> = game_systems.into();
                game_systems.sort_by(|a, b| a.sort_key.partial_cmp(&b.sort_key).unwrap());
                self.game_systems = game_systems.into_iter().map(|x| Rc::new(x)).collect();
                Cmd::none()
            }
        }
    }
}

impl ModalDicebot {
    fn get_game_system(&self) -> Cmd<Self> {
        let api_root = Rc::clone(&self.api_root);
        Cmd::task(async move {
            let game_systems = bcdice::api::game_system(&api_root).await;
            Cmd::chain(Msg::SetGameSystems(game_systems.into()))
        })
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
                String::from("更新情報"),
                String::from(""),
                vec![Html::div(
                    Attributes::new().class(Self::class("base")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("content")),
                            Events::new(),
                            vec![Html::div(
                                Attributes::new().class(Self::class("container")),
                                Events::new(),
                                vec![
                                    Heading::h2(
                                        heading::Variant::Light,
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("Soldoresol dev")],
                                    ),
                                    Html::p(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("更新：2020-01-24")],
                                    ),
                                    Heading::h3(
                                        heading::Variant::Light,
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("主な変更点")],
                                    ),
                                    Html::ul(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::li(
                                            Attributes::new(),
                                            Events::new(),
                                            vec![Html::text(
                                                "ペンを使用して、テーブル上に描画できるようにする",
                                            )],
                                        )],
                                    ),
                                ],
                            )],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("container")),
                            Events::new(),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(self, |_| {
                                    crate::debug::log_1("close notification");
                                    Msg::Sub(On::Close)
                                }),
                                vec![Html::text("閉じる")],
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
                "grid-template-rows": "1fr max-content";
                "justify-items": "center";
                "height": "100%";
            }
            ".content" {
                "overflow-y": "scroll";
                "width": "100%";
            }
            ".container" {
                "padding": ".5em 1em";
            }
        }
    }
}
