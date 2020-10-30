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
