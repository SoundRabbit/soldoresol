use kagura::prelude::*;

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
