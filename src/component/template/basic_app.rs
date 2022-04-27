use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;
use nusa::v_node::v_element::VEvent;

pub struct Props {}

pub enum Msg {
    NoOp,
    Sub(On),
}

pub enum On {
    DragLeave(VEvent<web_sys::DragEvent>),
    DragOver(VEvent<web_sys::DragEvent>),
    Drop(VEvent<web_sys::DragEvent>),
}

pub struct BasicApp;

impl Component for BasicApp {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for BasicApp {}

impl Constructor for BasicApp {
    fn constructor(_: Self::Props) -> Self {
        Self {}
    }
}

impl Update for BasicApp {
    fn update(self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Sub(sub) => Cmd::submit(sub),
        }
    }
}

impl Render<Html> for BasicApp {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("base")),
            Events::new()
                .on("dragleave", self, |e| {
                    let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    Msg::Sub(On::DragLeave(e))
                })
                .on("dragover", self, |e| {
                    let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
                    Msg::Sub(On::DragOver(e))
                })
                .on("drop", self, |e| {
                    let e = unwrap!(e.dyn_into::<web_sys::DragEvent>().ok(); Msg::NoOp);
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
                "overflow": "hidden";
            }
        }
    }
}
