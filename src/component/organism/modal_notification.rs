use super::atom::btn::Btn;
use super::atom::heading::{self, Heading};
use super::molecule::modal::{self, Modal};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {}

pub enum Msg {
    CloseSelf,
}

pub enum On {}

pub struct ModalNotification {
    is_showing: bool,
}

impl Constructor for ModalNotification {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self { is_showing: true }
    }
}

impl Component for ModalNotification {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::CloseSelf => {
                self.is_showing = false;
                Cmd::none()
            }
        }
    }

    fn render(&self, _: Vec<Html>) -> Html {
        if self.is_showing {
            Self::styled(Modal::with_children(
                modal::Props {
                    header_title: String::from("更新情報"),
                    footer_message: String::from("開発者 twitter：@SoundRabbit_"),
                },
                Subscription::new(|sub| match sub {
                    modal::On::Close => Msg::CloseSelf,
                }),
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
                            Events::new().on_click(|_| Msg::CloseSelf),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::CloseSelf),
                                vec![Html::text("閉じる")],
                            )],
                        ),
                    ],
                )],
            ))
        } else {
            Html::none()
        }
    }
}

impl Styled for ModalNotification {
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
