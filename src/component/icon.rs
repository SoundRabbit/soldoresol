use kagura::prelude::*;

pub fn small<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::span(
        attributes
            .class("icon")
            .string("data-icon-variant", "small"),
        events,
        vec![],
    )
}

pub fn medium<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::span(
        attributes
            .class("icon")
            .string("data-icon-variant", "medium"),
        events,
        vec![],
    )
}

pub fn large<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::span(
        attributes
            .class("icon")
            .string("data-icon-variant", "large"),
        events,
        vec![],
    )
}
