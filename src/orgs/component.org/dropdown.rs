use kagura::prelude::*;

pub fn right_bottom(is_showing: bool, btn: Html, content: Html) -> Html {
    Html::span(
        Attributes::new()
            .class("dropdown")
            .class("dropdown-rb")
            .class(format!("dropdown-{}", is_showing)),
        Events::new(),
        vec![btn, content],
    )
}

pub fn bottom_right(is_showing: bool, btn: Html, content: Html) -> Html {
    Html::span(
        Attributes::new()
            .class("dropdown")
            .class("dropdown-br")
            .class(format!("dropdown-{}", is_showing)),
        Events::new(),
        vec![btn, content],
    )
}
