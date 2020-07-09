use kagura::prelude::*;

pub fn right_bottom<Msg>(is_showing: bool, btn: Html<Msg>, content: Html<Msg>) -> Html<Msg> {
    Html::span(
        Attributes::new()
            .class("dropdown")
            .class("dropdown-rb")
            .class(format!("dropdown-{}", is_showing)),
        Events::new(),
        vec![btn, content],
    )
}

pub fn bottom_right<Msg>(is_showing: bool, btn: Html<Msg>, content: Html<Msg>) -> Html<Msg> {
    Html::span(
        Attributes::new()
            .class("dropdown")
            .class("dropdown-br")
            .class(format!("dropdown-{}", is_showing)),
        Events::new(),
        vec![btn, content],
    )
}
