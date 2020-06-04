use kagura::prelude::*;

pub fn container<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(
        attributes.class("fullscreen bg-color-dark-t centering-v grid"),
        events,
        children,
    )
}

pub fn container_t<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(
        attributes.class("fullscreen centering-v grid"),
        events,
        children,
    )
}

pub fn frame<Msg>(
    size: u32,
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(
        attributes.class(format!("frame frame-modal grid-cc-2x{}", size / 2)),
        events,
        children,
    )
}

pub fn header<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("frame-header"), events, children)
}

pub fn body<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("frame-body"), events, children)
}

pub fn footer<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::div(attributes.class("frame-footer"), events, children)
}
