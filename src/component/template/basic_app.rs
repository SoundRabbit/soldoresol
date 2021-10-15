use component::Cmd;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

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

impl Component for BasicApp {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for BasicApp {
    fn constructor(_: &Props) -> Self {
        Self {}
    }
}

impl Update for BasicApp {
    fn update(&mut self, _: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::Sub(sub),
        }
    }
}

impl Render for BasicApp {
    fn render(&self, props: &Props, children: Vec<Html<Self>>) -> Html<Self> {
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
            ".base" {
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
