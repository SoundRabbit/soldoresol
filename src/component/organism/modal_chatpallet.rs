use super::atom::btn::Btn;
use super::molecule::modal::{self, Modal};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

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
    type Event = On;
}

impl HtmlComponent for ModalChatpallet {}

impl Constructor for ModalChatpallet {
    fn constructor(props: Props) -> Self {
        Self { data: props.data }
    }
}

impl Update for ModalChatpallet {
    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Close => Cmd::submit(On::Close),
            Msg::Ok => Cmd::submit(On::Ok(self.data.clone())),
            Msg::Input(data) => {
                self.data = data;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for ModalChatpallet {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Modal::new(
            self,
            None,
            modal::Props {},
            Sub::map(|sub| match sub {
                modal::On::Close => Msg::Close,
            }),
            (
                String::from("チャットパレットを編集"),
                String::from(""),
                vec![Html::div(
                    Attributes::new().class(Self::class("base")),
                    Events::new(),
                    vec![
                        Html::textarea(
                            Attributes::new()
                                .class(Self::class("content"))
                                .class(Self::class("container"))
                                .value(&self.data),
                            Events::new().on_input(self, Msg::Input),
                            vec![],
                        ),
                        Html::div(
                            Attributes::new().class(Self::class("container")),
                            Events::new(),
                            vec![Btn::primary(
                                Attributes::new(),
                                Events::new().on_click(self, |_| Msg::Ok),
                                vec![Html::text("決定")],
                            )],
                        ),
                    ],
                )],
            ),
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
