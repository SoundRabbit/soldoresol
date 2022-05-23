use super::atom::{
    btn::Btn,
    heading::{self, Heading},
};
use super::molecule::modal::{self, Modal};
use crate::libs::bcdice::js::{DynamicLoader, GameSystemClass, GameSystemInfo};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Props {
    bcdice_loader: Rc<DynamicLoader>,
}

pub enum Msg {
    Sub(On),
}

pub enum On {
    Close,
}

pub struct ModalDicebot {
    bcdice_loader: Rc<DynamicLoader>,
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
            bcdice_loader: props.bcdice_loader,
        }
    }
}

impl Update for ModalDicebot {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.bcdice_loader = props.bcdice_loader;
        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Sub(sub) => Cmd::submit(sub),
        }
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