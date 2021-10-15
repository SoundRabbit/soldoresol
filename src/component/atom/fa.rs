use kagura::prelude::*;

pub fn i<C: Component>(name: impl Into<String>) -> Html<C> {
    Html::i(
        Attributes::new()
            .class("fas")
            .class(name)
            .string("aria-hidden", "true"),
        Events::new(),
        vec![],
    )
}

pub fn far_i<C: Component>(name: impl Into<String>) -> Html<C> {
    Html::i(
        Attributes::new()
            .class("far")
            .class(name)
            .string("aria-hidden", "true"),
        Events::new(),
        vec![],
    )
}
