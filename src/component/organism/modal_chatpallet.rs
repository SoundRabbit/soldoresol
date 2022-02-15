use super::atom::{
    btn::Btn,
    heading::{self, Heading},
};
use super::molecule::modal::{self, Modal};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::{Cmd, Sub};
use kagura::prelude::*;

pub struct Props {
    pub data: String,
}

pub enum Msg {
    Close,
    Ok,
    Input(String),
}

pub enum On {
    Close,
    Ok(String),
}

pub struct ModalChatpallet {
    data: String,
}

impl Component for ModalChatpallet {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ModalChatpallet {
    fn constructor(props: &Props) -> Self {
        Self {
            data: props.data.clone(),
        }
    }
}

impl Update for ModalChatpallet {
    fn update(&mut self, _: &Props, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Close => Cmd::sub(On::Close),
            Msg::Ok => Cmd::sub(On::Ok(self.data.clone())),
            Msg::Input(data) => {
                self.data = data;
                Cmd::none()
            }
        }
    }
}

impl Render for ModalChatpallet {
    fn render(&self, props: &Props, _: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Modal::with_children(
            modal::Props {
                header_title: String::from("チャットパレットを編集"),
                footer_message: String::from(""),
            },
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Close,
            }),
            vec![Html::div(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![
                    Html::textarea(
                        Attributes::new()
                            .class(Self::class("content"))
                            .class(Self::class("container"))
                            .value(&self.data),
                        Events::new().on_input(Msg::Input),
                        vec![],
                    ),
                    Html::div(
                        Attributes::new().class(Self::class("container")),
                        Events::new(),
                        vec![Btn::primary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::Ok),
                            vec![Html::text("決定")],
                        )],
                    ),
                ],
            )],
        ))
    }
}

impl Styled for ModalChatpallet {
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
                "resize": "none";
            }
            ".container" {
                "padding": ".5em 1em";
            }
        }
    }
}
