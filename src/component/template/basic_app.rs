use super::util::styled::{Style, Styled};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    DragLeave(web_sys::DragEvent),
    DragOver(web_sys::DragEvent),
    Drop(web_sys::DragEvent),
}

pub struct BasicApp;

impl Constructor for BasicApp {
    fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
        Self {}
    }
}

impl Component for BasicApp {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;

    fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::sub(sub),
        }
    }

    fn render(&self, children: Vec<Html>) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new()
                .on("dragleave", |e| {
                    let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    Msg::Sub(On::DragLeave(e))
                })
                .on("dragover", |e| {
                    let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    Msg::Sub(On::DragOver(e))
                })
                .on("drop", |e| {
                    let e = unwrap_or!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    Msg::Sub(On::Drop(e))
                }),
            children,
        ))
    }
}

impl Styled for BasicApp {
    fn style() -> Style {
        style! {
            "base" {
                "display": "grid";
                "grid-template-rows": "max-content 1fr";
                "width": "100vw";
                "height": "100vh";
                "position": "absolute";
                "top": "0px";
                "left": "0px";
                "z-index": "0";
            }
        }
    }
}
