use super::atom::{btn::Btn, fa};
use super::constant;
use crate::libs::color::color_system;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Props {
    pub header_title: String,
    pub footer_message: String,
}

pub enum Msg {
    CloseSelf,
}

pub enum On {
    Close,
}

pub struct Modal {
    header_title: String,
    footer_message: String,
}

impl Constructor for Modal {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            header_title: props.header_title,
            footer_message: props.footer_message,
        }
    }
}

impl Component for Modal {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.header_title = props.header_title;
        self.footer_message = props.footer_message;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::CloseSelf => Cmd::sub(On::Close),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("background")),
            Events::new(),
            vec![Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("header")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text(&self.header_title)],
                            ),
                            Btn::secondary(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::CloseSelf),
                                vec![fa::i("fa-times")],
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("body")),
                        Events::new(),
                        children,
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("footer")),
                        Events::new(),
                        vec![Html::text(&self.footer_message)],
                    ),
                ],
            )],
        ))
    }
}

impl Styled for Modal {
    fn style() -> Style {
        style! {
            ".background" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": constant::z_index::MODAL.to_string();
                "background-color": color_system::gray(13, 9).to_string();
                "display": "grid";
                "align-items": "center";
                "justify-items": "center";
            }
            ".base" {
                "width": "50%";
                "height": "50%";
                "display": "grid";
                "grid-template-rows": "max-content 1fr max-content";
                "border-radius": "2px";
                "overflow": "hidden";
            }
            ".header" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "align-items": "center";
                "color": color_system::gray(100, 0).to_string();
                "background-color": color_system::gray(100, 8).to_string();
                "padding-left": "1em";
            }
            ".body" {
                "background-color": color_system::gray(100, 0).to_string();
            }
            ".footer" {
                "color": color_system::gray(100, 0).to_string();
                "background-color": color_system::gray(100, 8).to_string();
                "padding" : ".5em 1em";
            }

            @media "(orientation: portrait), (max-width: 60rem)" {
                ".base" {
                    "width": "95%";
                }
            }

            @media "(max-height: 60rem)" {
                ".base" {
                    "height": "80%";
                }
            }
        }
    }
}
