use isaribi::{
    style,
    styled::{Style, Styled},
};
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Common;

impl Common {
    pub fn layered() -> String {
        Self::styled(Self::class("layered"))
    }

    pub fn layered_item() -> String {
        Self::styled(Self::class("layered-item"))
    }

    pub fn keyvalue() -> String {
        Self::styled(Self::class("keyvalue"))
    }

    pub fn valuekey() -> String {
        Self::styled(Self::class("valuekey"))
    }

    pub fn selectable() -> String {
        Self::styled(Self::class("selectable"))
    }

    pub fn bg_transparent() -> String {
        Self::styled(Self::class("bg-transparent"))
    }

    pub fn none() -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("none")),
            Events::new(),
            vec![],
        ))
    }

    pub fn banner() -> String {
        Self::class("banner")
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

            ".keyvalue > img" {
                "width": "100%";
                "object-fit": "contain";
            }

            ".selectable" {
                "-moz-user-select": "all";
                "-webkit-user-select": "all";
                "-ms-user-select": "all";
                "user-select": "all";
            }

            ".none" {
                "display": "none";
            }

            ".bg-transparent" {
                "background-color": format!("{}", crate::libs::color::Pallet::gray(2).a(100));
                "background-image": "linear-gradient(45deg,  #fff 25%, #fff 25%, transparent 25%, transparent 75%, #fff 75%, #fff 75%),
                    linear-gradient(-135deg, #fff 25%, #fff 25%, transparent 25%, transparent 75%, #fff 75%, #fff 75%)";
                "background-size": "1rem 1rem";
                "background-position": "0 0, 0.5rem 0.5rem";
            }

            ".banner" {
                "grid-column": "1 / -1";
            }
        }
    }
}
