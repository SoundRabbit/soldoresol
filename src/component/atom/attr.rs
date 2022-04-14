use nusa::prelude::*;

pub fn span(attrs: Attributes, text: impl Into<String>) -> Html {
    Html::span(attrs, Events::new(), vec![Html::text(text)])
}

pub fn div(attrs: Attributes, text: impl Into<String>) -> Html {
    Html::div(attrs, Events::new(), vec![Html::text(text)])
}
