use kagura::prelude::*;

pub fn right_bottom<Msg>(is_showing: bool, btn: Html<Msg>, content: Html<Msg>) -> Html<Msg> {
    Html::span(
        Attributes::new()
            .class("dropdown")
            .class(format!("dropdown-{}", is_showing)),
        Events::new(),
        vec![btn, content],
    )
}
