use super::atom::{
    btn::{self, Btn},
    fa,
};
use super::constant;
use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props {
    pub state: Option<State>,
    pub header_title: String,
    pub footer_message: String,
}

pub struct State {
    payload: Rc<RefCell<ImplState>>,
}

struct ImplState {
    is_showing: bool,
}

pub enum Msg {
    CloseSelf,
}

pub enum On {
    Close,
}

pub struct Modal {
    state: State,
    header_title: String,
    footer_message: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            payload: Rc::new(RefCell::new(ImplState { is_showing: true })),
        }
    }

    pub fn is_showing(&self) -> bool {
        self.payload.borrow().is_showing
    }

    pub fn set_is_showing(&mut self, value: bool) {
        self.payload.borrow_mut().is_showing = value;
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            payload: Rc::clone(&this.payload),
        }
    }
}

impl Constructor for Modal {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            state: props.state.unwrap_or(State::new()),
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
        if let Some(state) = props.state {
            self.state = state;
        }
        self.header_title = props.header_title;
        self.footer_message = props.footer_message;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::CloseSelf => Cmd::sub(On::Close),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        if self.state.is_showing() {
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
                                Btn::with_child(
                                    btn::Props {
                                        variant: btn::Variant::Secondary,
                                    },
                                    Subscription::new(|sub| match sub {
                                        btn::On::Click => Msg::CloseSelf,
                                    }),
                                    fa::i("fa-times"),
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
        } else {
            Html::none()
        }
    }
}

impl Styled for Modal {
    fn style() -> Style {
        style! {
            "background" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": constant::z_index::modal.to_string();
                "background-color": color_system::gray(13, 9).to_string();
                "display": "grid";
                "align-items": "center";
                "justify-items": "center";
            }
            "base" {
                "width": "50%";
                "height": "50%";
                "display": "grid";
                "grid-template-rows": "max-content 1fr max-content";
                "border-radius": "2px";
                "overflow": "hidden";
            }
            "header" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "align-items": "center";
                "color": color_system::gray(100, 0).to_string();
                "background-color": color_system::gray(100, 8).to_string();
                "padding-left": "1em";
            }
            "body" {
                "background-color": color_system::gray(100, 0).to_string();
            }
            "footer" {
                "color": color_system::gray(100, 0).to_string();
                "background-color": color_system::gray(100, 8).to_string();
                "padding" : ".5em 1em";
            }

            @media "(orientation: portrait), (max-width: 60rem)" {
                "base" {
                    "width": "95%";
                }
            }

            @media "(max-height: 60rem)" {
                "base" {
                    "height": "80%";
                }
            }
        }
    }
}
