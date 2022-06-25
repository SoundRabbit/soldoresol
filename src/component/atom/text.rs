use isaribi::{
    style,
    styled::{Style, Styled},
};
use nusa::prelude::*;

pub struct Text {}

impl Text {
    pub fn span(text: impl Into<String>) -> Html {
        Html::span(Attributes::new(), Events::new(), vec![Html::text(text)])
    }

    pub fn div(text: impl Into<String>) -> Html {
        Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
    }

    pub fn label(text: impl Into<String>, for_: impl Into<String>) -> Html {
        Html::label(
            Attributes::new().string("for", for_),
            Events::new(),
            vec![Html::text(text)],
        )
    }

    pub fn i(text: impl Into<String>) -> Html {
        Html::i(Attributes::new(), Events::new(), vec![Html::text(text)])
    }

    pub fn condense_75(text: impl Into<String>) -> Html {
        let text = text.into();
        let text = text.chars();
        Self::styled(Html::span(
            Attributes::new(),
            Events::new(),
            text.map(|a| {
                Html::span(
                    Attributes::new().class(Self::class("cond-75")),
                    Events::new(),
                    vec![Html::text(a.to_string())],
                )
            })
            .collect(),
        ))
    }
}

impl Styled for Text {
    fn style() -> Style {
        style! {
            ".cond-75" {
                "display": "inline-block";
                "transform": "scale(0.75, 1.0)";
                "width": "0.75em";
            }
        }
    }
}
