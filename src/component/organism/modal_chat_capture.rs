use super::atom::{btn::Btn, text};
use super::molecule::modal::{self, Modal};
use super::NoProps;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::rc::Rc;

pub struct Props {
    pub vars: Rc<Vec<(String, String)>>,
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
    vars: Rc<Vec<(String, String)>>,
    input: Vec<String>,
}

impl Component for ModalChatCapture {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for ModalChatCapture {}

impl Constructor for ModalChatCapture {
    fn constructor(props: Props) -> Self {
        let mut input = vec![];
        for _ in 0..props.vars.len() {
            input.push(String::from(""));
        }

        Self {
            input,
            vars: props.vars,
        }
    }
}

impl Update for ModalChatCapture {
    fn on_load(self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        while self.input.len() > props.vars.len() {
            self.input.pop();
        }
        while self.input.len() < props.vars.len() {
            self.input.push(String::from(""));
        }
        self.vars = props.vars;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Cancel => Cmd::submit(On::Cancel),
            Msg::Input(idx, data) => {
                self.input[idx] = data;
                Cmd::none()
            }
            Msg::Send => Cmd::submit(On::Send(self.input.drain(..).collect())),
        }
    }
}

impl Render<Html> for ModalChatCapture {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Modal::new(
            self,
            None,
            NoProps(),
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Cancel,
            }),
            (
                String::from("チャットの送信"),
                String::from(""),
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
                                            text::span(&self.vars[idx].1),
                                            Html::input(
                                                Attributes::new().value(v),
                                                Events::new()
                                                    .on_input(self, move |v| Msg::Input(idx, v)),
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
                                Events::new().on_click(self, |_| Msg::Send),
                                vec![Html::text("送信")],
                            )],
                        ),
                    ],
                )],
            ),
        ))
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
