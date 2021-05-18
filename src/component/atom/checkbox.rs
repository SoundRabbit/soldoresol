use super::fa;
use super::text;
use super::util::styled::{Style, Styled};
use kagura::prelude::*;

pub struct Checkbox {}

impl Checkbox {
    pub fn light(is_checked: bool, events: Events) -> Html {
        Self::styled(Html::span(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class("base--light")),
            events,
            vec![if is_checked {
                fa::i("fa-check")
            } else {
                text::i("")
            }],
        ))
    }
}

impl Styled for Checkbox {
    fn style() -> Style {
        style! {
            "base" {
                "display": "inline-block";
                "width": "1.4em";
                "height": "1.4em";
                "color": format!("{}", crate::libs::color::Pallet::blue(5).a(100));
                "border-radius": "0.2em";
                "text-align": "center";
                "margin-left": "0.1em";
                "margin-right": "0.1em";
                "vertical-align": "baseline";
            }

            "base--light" {
                "background-color": format!("{}", crate::libs::color::Pallet::gray(0).a(100));
                "box-shadow": format!("inset 0 1px 3px {}", crate::libs::color::Pallet::gray(9).a(100));
            }
        }
    }
}
