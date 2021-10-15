use kagura::prelude::*;

pub fn span<C: Component>(text: impl Into<String>) -> Html<C> {
    Html::span(Attributes::new(), Events::new(), vec![Html::text(text)])
}

pub fn div<C: Component>(text: impl Into<String>) -> Html<C> {
    Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
}

pub fn label<C: Component>(text: impl Into<String>, for_: impl Into<String>) -> Html<C> {
    Html::label(
        Attributes::new().string("for", for_),
        Events::new(),
        vec![Html::text(text)],
    )
}

pub fn i<C: Component>(text: impl Into<String>) -> Html<C> {
    Html::i(Attributes::new(), Events::new(), vec![Html::text(text)])
}
