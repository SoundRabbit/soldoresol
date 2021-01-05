use super::atom::{
    btn::{self, Btn},
    fa,
};
use super::constant;
use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Props {
    pub state: Option<State>,
}

pub struct State {
    payload: Rc<RefCell<ImplState>>,
}

struct ImplState {
    size: [f64; 2],
    loc: [f64; 2],
}

pub enum Msg {
    CloseSelf,
}

pub enum On {
    Close,
}

pub struct Modeless {
    state: State,
}

impl State {
    pub fn new() -> Self {
        Self {
            payload: Rc::new(RefCell::new(ImplState {
                size: [30.0, 30.0],
                loc: [0.0, 0.0],
            })),
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            payload: Rc::clone(&self.payload),
        }
    }
}

impl Constructor for Modeless {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            state: props.state.unwrap_or(State::new()),
        }
    }
}

impl Component for Modeless {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        if let Some(state) = props.state {
            self.state = state;
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::CloseSelf => Cmd::sub(On::Close),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Html::none()
    }
}

impl Styled for Modeless {
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
        }
    }
}
