use kagura::prelude::*;

pub fn i<Msg>(name: impl Into<String>) -> Html<Msg> {
    Html::i(
        Attributes::new()
            .class("fas")
            .class(name)
            .string("aria-hidden", "true"),
        Events::new(),
        vec![],
    )
}
