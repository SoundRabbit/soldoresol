use kagura::prelude::*;

pub fn i(name: impl Into<String>) -> Html {
    Html::i(
        Attributes::new()
            .class("fas")
            .class(name)
            .string("aria-hidden", "true"),
        Events::new(),
        vec![],
    )
}

pub fn far_i(name: impl Into<String>) -> Html {
    Html::i(
        Attributes::new()
            .class("far")
            .class(name)
            .string("aria-hidden", "true"),
        Events::new(),
        vec![],
    )
}
