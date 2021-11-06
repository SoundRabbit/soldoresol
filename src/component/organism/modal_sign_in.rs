use super::atom::{
    btn::Btn,
    heading::{self, Heading},
};
use super::molecule::modal::{self, Modal};
use crate::libs::gapi::gapi;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub is_showing: Option<bool>,
}

pub enum Msg {
    NoOp,
    CloseSelf,
}

pub enum On {
    Close,
}

pub struct ModalSignIn {
    is_showing: bool,
}

impl Component for ModalSignIn {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ModalSignIn {
    fn constructor(_: &Props) -> Self {
        ModalSignIn {
            is_showing: !gapi.auth2().get_auth_instance().is_signed_in().get(),
        }
    }
}

impl Update for ModalSignIn {
    fn on_assemble(&mut self, _: &Props) -> Cmd<Self> {
        Cmd::batch(|mut handle| {
            let a = Closure::wrap(Box::new(move |is_signed_in| {
                if is_signed_in {
                    handle(Msg::CloseSelf)
                }
            }) as Box<dyn FnMut(bool)>);
            gapi.auth2()
                .get_auth_instance()
                .is_signed_in()
                .listen(a.as_ref().unchecked_ref());
            a.forget();
        })
    }

    fn on_load(&mut self, _: &Props) -> Cmd<Self> {
        Cmd::none()
    }

    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::CloseSelf => {
                self.is_showing = false;
                Cmd::sub(On::Close)
            }
        }
    }
}

impl Render for ModalSignIn {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        if props.is_showing.unwrap_or(self.is_showing) {
            Self::styled(Modal::with_children(
                modal::Props {
                    header_title: String::from("Googleアカウントにサインイン"),
                    footer_message: String::from(""),
                },
                Sub::map(|sub| match sub {
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
                                        vec![Html::text("Google Driveを使用")],
                                    ),
                                    Html::ul(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![
                                            Html::li(
                                                Attributes::new(),
                                                Events::new(),
                                                vec![Html::text(
                                                "ルームデータが自動でオンライン上に保存されます。",
                                            )],
                                            ),
                                            Html::li(
                                                Attributes::new(),
                                                Events::new(),
                                                vec![Html::text("大きなファイルを使用できます。")],
                                            ),
                                            Html::li(
                                                Attributes::new(),
                                                Events::new(),
                                                vec![Html::text("自身がルームを開いていないときでも、プレイヤーはルームにアクセスできます。")],
                                            ),
                                            Html::li(
                                                Attributes::new(),
                                                Events::new(),
                                                vec![Html::text("別のデバイスやブラウザからもルームを使用できるようになります。")],
                                            )
                                        ],
                                    ),
                                ],
                            )],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("container")).class(Self::class("btns")),
                            Events::new(),
                            vec![
                                Btn::secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|e| {
                                        e.stop_propagation();
                                        Msg::CloseSelf
                                    }),
                                    vec![Html::text("キャンセル")],
                                ),
                                Btn::primary(
                                    Attributes::new(),
                                    Events::new().on_click(|e| {
                                        e.stop_propagation();
                                        gapi.auth2().get_auth_instance().sign_in();
                                        Msg::NoOp
                                    }),
                                    vec![Html::text("サインイン")],
                                )
                            ],
                        ),
                    ],
                )],
            ))
        } else {
            Html::none()
        }
    }
}

impl Styled for ModalSignIn {
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
            ".btns" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "column-gap": "1em";
            }
        }
    }
}
