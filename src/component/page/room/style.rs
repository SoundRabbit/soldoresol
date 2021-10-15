use super::super::super::constant;
use super::Room;
use isaribi::{
    style,
    styled::{Style, Styled},
};

impl Styled for Room {
    fn style() -> Style {
        style! {
            ".overlay" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "height": "100vh";
                "width": "100vw";
                "z-index": format!("{}", constant::z_index::OVERLAY);
            }

            ".overlay-file-import" {
                "background-color": format!("{}", crate::libs::color::color_system::gray(25, 9));
            }

            ".overlay-file-import-text" {
                "position": "absolute";
                "color": format!("{}", crate::libs::color::color_system::gray(100, 0));
                "font-size": "4rem";
                "bottom": "1em";
                "right": "1em";
            }

            ".contextmenu" {
                "position": "absolute";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "rows";
                "row-gap": "0.05rem";
                "justify-items": "stretch";
                "background-color": crate::libs::color::color_system::gray(100, 0).to_string();
                "border-radius": "2px";
                "display": "grid";
            }

            ".header-row" {
                "display": "grid";
                "grid-template-columns": "1fr 1fr";
            }

            ".header-room-id" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "column-gap": "0.65em";
            }

            ".header-controller-menu" {
                "display": "grid";
                "grid-auto-columns": "max-content";
                "grid-auto-flow": "column";
                "column-gap": "0.65em";
            }

            ".body" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
            }

            ".side-menu" {
                "z-index": "1";
                "min-height": "max-content";
                "min-width": "max-content";
            }

            ".main" {
                "position": "relative";
            }

            ".canvas" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
            }

            ".modeless-container" {
                "position": "absolute";
                "top": "0";
                "left": "0";
                "width": "100%";
                "height": "100%";
                "z-index": "0";
                "overflow": "hidden";
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
                "grid-auto-flow": "max-content";
                "justify-content": "start";
                "align-content": "end";
            }
        }
    }
}
