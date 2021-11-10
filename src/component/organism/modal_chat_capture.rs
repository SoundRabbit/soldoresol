use super::atom::{
    btn::Btn,
    heading::{self, Heading},
    text,
};
use super::molecule::modal::{self, Modal};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub vars: Rc<Vec<(String, String)>>,
    pub is_showing: bool,
}

pub enum Msg {
    Cancel,
    Send,
    Input(usize, String),
}

pub enum On {
    Cancel,
    Send(Vec<String>),
}

pub struct ModalChatCapture {
    input: Vec<String>,
}

impl Component for ModalChatCapture {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ModalChatCapture {
    fn constructor(props: &Props) -> Self {
        let mut input = vec![];
        for _ in 0..props.vars.len() {
            input.push(String::from(""));
        }

        Self { input }
    }
}

impl Update for ModalChatCapture {
    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        while self.input.len() > props.vars.len() {
            self.input.pop();
        }
        while self.input.len() < props.vars.len() {
            self.input.push(String::from(""));
        }
        Cmd::none()
    }

    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Cancel => Cmd::sub(On::Cancel),
            Msg::Input(idx, data) => {
                self.input[idx] = data;
                Cmd::none()
            }
            Msg::Send => Cmd::sub(On::Send(self.input.drain(..).collect())),
        }
    }
}

impl Render for ModalChatCapture {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        if props.is_showing {
            Self::styled(Modal::with_children(
                modal::Props {
                    header_title: String::from("チャットの送信"),
                    footer_message: String::from(""),
                },
                Sub::map(|sub| match sub {
                    modal::On::Close => Msg::Cancel,
                }),
                vec![Html::div(
                    Attributes::new().class(Self::class("base")),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class(Self::class("content")),
                            Events::new(),
                            vec![Html::div(
                                Attributes::new()
                                    .class(Self::class("container"))
                                    .class(Self::class("key-key")),
                                Events::new(),
                                self.input
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, v)| {
                                        Html::fragment(vec![
                                            text::span(&props.vars[idx].1),
                                            Html::input(
                                                Attributes::new().value(v),
                                                Events::new().on_input(move |v| Msg::Input(idx, v)),
                                                vec![],
                                            ),
                                        ])
                                    })
                                    .collect(),
                            )],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("container")),
                            Events::new(),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::Send),
                                vec![Html::text("送信")],
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

impl Styled for ModalChatCapture {
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
            ".key-key" {
                "display": "grid";
                "grid-template-columns": "max-content max-content";
                "justify-content": "center";
                "align-items": "center";
                "row-gap": ".65rem";
                "column-gap": ".35rem";
            }
        }
    }
}
