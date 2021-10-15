use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;

pub struct Align {}

impl Align {
    pub fn key_value<C: Component>(
        attrs: Attributes,
        events: Events<C::Msg>,
        children: Vec<Html<C>>,
    ) -> Html<C> {
        Html::div(attrs.class(Self::class("key-value")), events, children)
    }
}

impl Styled for Align {
    fn style() -> Style {
        style! {
            ".key-value" {
                "display": "grid";
                "grid-template-columns": "max-content 1fr";
                "grid-auto-rows": "max-content";
                "column-gap": ".35em";
                "row-gap": ".65em";
            }
        }
    }
}
