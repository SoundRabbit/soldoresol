use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Cmd;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Common;

impl Component for Common {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for Common {
    fn constructor(_: &Props) -> Self {
        Self {}
    }
}

impl Update for Common {}

impl Render for Common {
    fn render(&self, _: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::fragment(children))
    }
}

impl Common {
    pub fn layered() -> String {
        Self::class("layered")
    }

    pub fn layered_item() -> String {
        Self::class("layered-item")
    }

    pub fn keyvalue() -> String {
        Self::class("keyvalue")
    }

    pub fn valuekey() -> String {
        Self::class("valuekey")
    }

    pub fn selectable() -> String {
        Self::class("selectable")
    }
}

impl Styled for Common {
    fn style() -> Style {
        style! {
            ".layered" {
                "position": "relative";
            }

            ".layered-item" {
                "position": "absolute";
                "width": "100%";
                "height": "100%";
            }

            ".text-selectable" {
                "-moz-user-select": "text";
                "-webkit-user-select": "text";
                "-ms-user-select": "text";
                "user-select": "text";
            }

            ".keyvalue" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "align-items": "center";
            }

            ".valuekey" {
                "display": "grid";
                "grid-template-columns": "1fr max-content";
                "column-gap": ".35rem";
                "row-gap": ".65rem";
                "align-items": "center";
            }

            ".keyvalue > *, .valuekey > *" {
                "overflow": "hidden";
            }

            ".selectable" {
                "-moz-user-select": "all";
                "-webkit-user-select": "all";
                "-ms-user-select": "all";
                "user-select": "all";
            }
        }
    }
}
