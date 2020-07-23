use kagura::prelude::*;

pub mod load_table;

pub use load_table::LoadTable;

pub fn container(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(
        attributes.class("fullscreen bg-color-dark-t centering-v grid"),
        events,
        children,
    )
}

pub fn frame(size: u32, attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(
        attributes.class(format!("frame frame-modal grid-cc-2x{}", size / 2)),
        events,
        children,
    )
}

pub fn header(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(attributes.class("frame-header"), events, children)
}

pub fn body(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(attributes.class("frame-body"), events, children)
}

pub fn footer(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::div(attributes.class("frame-footer"), events, children)
}
