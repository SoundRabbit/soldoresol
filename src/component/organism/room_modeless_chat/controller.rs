use super::super::atom::{btn::Btn, text};
use super::{InputingMessage, SharedState};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct Props {
    pub shared_state: Rc<RefCell<SharedState>>,
}

pub enum Msg {
    NoOp,
    Sub(On),
    SetInputingChatMessage(String),
    SendInputingChatMessage,
}

pub enum On {
    SendInputingChatMessage,
}

pub struct Controller {
    shared_state: Rc<RefCell<SharedState>>,
    ignore_input: Rc<Cell<bool>>,
}

impl Component for Controller {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Controller {}

impl Constructor for Controller {
    fn constructor(props: Self::Props) -> Self {
        Self {
            shared_state: props.shared_state,
            ignore_input: Rc::new(Cell::new(false)),
        }
    }
}

impl Update for Controller {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.shared_state = props.shared_state;
        Cmd::none()
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(e) => Cmd::submit(e),
            Msg::SetInputingChatMessage(text) => {
                self.shared_state.borrow_mut().inputing_message = InputingMessage::Text(text);
                Cmd::none()
            }
            Msg::SendInputingChatMessage => {
                self.ignore_input.set(true);
                Cmd::submit(On::SendInputingChatMessage)
            }
        }
    }
}

impl Render<Html> for Controller {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![
                Html::textarea(
                    Attributes::new()
                        .value(self.shared_state.borrow().inputing_message.to_string()),
                    Events::new()
                        .on_input(self, {
                            let ignore_input = Rc::clone(&self.ignore_input);
                            move |input| {
                                if ignore_input.get() {
                                    ignore_input.set(false);
                                    Msg::NoOp
                                } else {
                                    Msg::SetInputingChatMessage(input)
                                }
                            }
                        })
                        .on_keydown(self, |e| {
                            let e = unwrap!(e.dyn_into::<web_sys::KeyboardEvent>().ok(); Msg::NoOp);
                            if e.key() == "Enter" && !e.shift_key() {
                                Msg::SendInputingChatMessage
                            } else {
                                Msg::NoOp
                            }
                        }),
                    vec![],
                ),
                Html::div(
                    Attributes::new().class(Self::class("guide")),
                    Events::new(),
                    vec![
                        text::span("Shift＋Enterで改行できます。"),
                        Btn::primary(
                            Attributes::new(),
                            Events::new().on_click(self, |_| Msg::Sub(On::SendInputingChatMessage)),
                            vec![Html::text("送信")],
                        ),
                    ],
                ),
            ],
        ))
    }
}

impl Styled for Controller {
    fn style() -> Style {
        style! {
            ".base" {
                "grid-column": "2 / 3";
                "grid-row": "2 / 3";
                "padding-left": ".65rem";
                "padding-right": ".65rem";
                "height": "10rem";
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
            }

            ".base textarea" {
                "grid-column": "1 / -1";
                "resize": "none";
            }

            ".guide" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "align-items": "center";
            }
        }
    }
}
