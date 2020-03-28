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

pub fn danger<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.string("data-btn-variant", "danger").class("btn"),
        events,
        children,
    )
}

pub fn close<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(attributes.class("btn-close"), events, vec![Html::text("×")])
}
