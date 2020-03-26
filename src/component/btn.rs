use kagura::prelude::*;

pub fn primary<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes
            .string("data-btn-variant", "primary")
            .class("btn"),
        events,
        children,
    )
}

pub fn info<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.string("data-btn-variant", "info").class("btn"),
        events,
        children,
    )
}
