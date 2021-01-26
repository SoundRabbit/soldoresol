use super::util::styled::{Style, Styled};
use crate::libs::color::color_system;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct Props {
    pub is_selected: bool,
    pub data: String,
    pub draggable: bool,
}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    DragStart,
    Click,
    Drop(web_sys::DragEvent),
}

pub struct TabBtn {
    is_selected: bool,
    data: Rc<String>,
    draggable: bool,
}

impl Constructor for TabBtn {
    fn constructor(props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {
            is_selected: props.is_selected,
            data: Rc::new(props.data),
            draggable: props.draggable,
        }
    }
}

impl Component for TabBtn {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, props: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {
        self.is_selected = props.is_selected;
        self.data = Rc::new(props.data);
        self.draggable = props.draggable;
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .draggable(self.draggable),
            Events::new()
                .on_click(|_| Msg::Sub(On::Click))
                .on_mousedown(|e| {
                    e.stop_propagation();
                    Msg::NoOp
                })
                .on("dragstart", {
                    let data = Rc::clone(&self.data);
                    move |e| {
                        let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                        e.stop_propagation();
                        let data_transfer = unwrap_or!(e.data_transfer(); Msg::NoOp);
                        unwrap_or!(data_transfer.set_data("text/plain", &data).ok(); Msg::NoOp);
                        Msg::Sub(On::DragStart)
                    }
                })
                .on("drop", |e| {
                    let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    Msg::Sub(On::Drop(e))
                }),
            vec![Html::div(
                Attributes::new()
                    .class("pure-button")
                    .class(Self::class("btn"))
                    .string("data-tab-selected", self.is_selected.to_string()),
                Events::new(),
                children,
            )],
        ))
    }
}

impl Styled for TabBtn {
    fn style() -> Style {
        style! {
            "base" {
                "max-width": "max-content";
                "min-width": "max-content";
                "max-height": "max-content";
                "min-height": "max-content";
            }

            "btn" {
                "border-radius": "2px 2px 0 0";
                "color": color_system::gray(100, 0).to_string();
                "max-width": "12em";
                "overflow": "hidden";
                "text-overflow": "ellipsis";
            }

            r#"btn[data-tab-selected="true"]"# {
                "background-color": color_system::blue(100, 5).to_string();
            }

            r#"btn[data-tab-selected="false"]"# {
                "background-color": color_system::gray(100, 9).to_string();
            }
        }
    }
}
