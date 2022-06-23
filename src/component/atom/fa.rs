use nusa::prelude::*;

pub fn fas_i(name: impl Into<String>) -> Html {
    Html::i(
        Attributes::new().class("fa-solid").class(name),
        Events::new(),
        vec![],
    )
}

pub fn far_i(name: impl Into<String>) -> Html {
    Html::i(
        Attributes::new().class("fa-regular").class(name),
        Events::new(),
        vec![],
    )
}
