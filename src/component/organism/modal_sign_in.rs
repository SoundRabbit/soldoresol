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
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {}

pub enum Msg {
    NoOp,
    CloseSelf,
}

pub enum On {
    Close,
}

pub struct ModalSignIn {}

impl Component for ModalSignIn {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalSignIn {}

impl Constructor for ModalSignIn {
    fn constructor(_: Self::Props) -> Self {
        ModalSignIn {}
    }
}

impl Update for ModalSignIn {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::task(kagura::util::Task::new(|resolve| {
            let resolve = RefCell::new(Some(Box::new(resolve)));
            let a = Closure::wrap(Box::new(move |is_signed_in| {
                if is_signed_in {
                    if let Some(resolve) = resolve.borrow_mut().take() {
                        resolve(Cmd::chain(Msg::CloseSelf))
                    }
                }
            }) as Box<dyn FnMut(bool)>);
            gapi.auth2()
                .get_auth_instance()
                .is_signed_in()
                .listen(a.as_ref().unchecked_ref());
            a.forget();
        }))
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::CloseSelf => Cmd::submit(On::Close),
        }
    }
}

impl Render<Html> for ModalSignIn {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        crate::debug::log_1("render ModalSignIn");
        Self::styled(Modal::new(
                self,None,
                modal::Props {},
                Sub::map(|sub| match sub {
                    modal::On::Close => Msg::CloseSelf,
                }),
                (String::from("Googleアカウントにサインイン"),String::from(""),
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
                                    Events::new().on_click(self, |e| {
                                        crate::debug::log_1("ModalSignIn::Msg::CloseSelf");
                                        Msg::CloseSelf
                                    }),
                                    vec![Html::text("キャンセル")],
                                ),
                                Btn::primary(
                                    Attributes::new(),
                                    Events::new().on_click(self, |e| {
                                        gapi.auth2().get_auth_instance().sign_in();
                                        Msg::NoOp
                                    }),
                                    vec![Html::text("サインイン")],
                                )
                            ],
                        ),
                    ],
                )],
            )
            ))
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
