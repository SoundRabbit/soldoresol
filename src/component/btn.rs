use super::awesome;
use kagura::prelude::*;

pub fn primary<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-primary"),
        events,
        children,
    )
}

pub fn secondary<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-secondary"),
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
        attributes.class("pure-button pure-button-info"),
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
        attributes.class("pure-button pure-button-danger"),
        events,
        children,
    )
}

pub fn success<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-success"),
        events,
        children,
    )
}

pub fn light<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button").class("pure-button-light"),
        events,
        children,
    )
}

pub fn dark<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button").class("pure-button-dark"),
        events,
        children,
    )
}

pub fn selectable<Msg>(
    is_selected: bool,
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    if is_selected {
        Html::button(
            attributes.class("pure-button").class("pure-button-primary"),
            events,
            children,
        )
    } else {
        Html::button(
            attributes.class("pure-button").class("pure-button-dark"),
            events,
            children,
        )
    }
}

pub fn toggle<Msg>(is_toggled: bool, attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    if is_toggled {
        Html::span(attributes.class("toggle toggle-on"), events, vec![])
    } else {
        Html::span(attributes.class("toggle toggle-off"), events, vec![])
    }
}

pub fn close<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-close"),
        events,
        vec![awesome::i("fa-times")],
    )
}

pub fn transparent<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-transparent"),
        events,
        children,
    )
}

pub fn spacer<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-spacer"),
        events,
        children,
    )
}

pub fn check<Msg>(is_checked: bool, attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    if is_checked {
        Html::div(
            attributes.class("checkbox").class("checkbox-checked"),
            events,
            vec![Html::text("✔")],
        )
    } else {
        Html::div(attributes.class("checkbox"), events, vec![])
    }
}

#[allow(dead_code)]
pub fn add<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(
        attributes.class("app__add-btn").class("material-icons"),
        events,
        vec![Html::text("add")],
    )
}

pub fn tab<Msg>(is_selected: bool, events: Events<Msg>, name: impl Into<String>) -> Html<Msg> {
    if is_selected {
        Html::span(
            Attributes::new().draggable(false),
            events,
            vec![Html::span(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-frametab")
                    .class("pure-button-primary"),
                Events::new(),
                vec![Html::text(name)],
            )],
        )
    } else {
        Html::span(
            Attributes::new().draggable(false),
            events,
            vec![Html::span(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-frametab")
                    .class("pure-button-dark"),
                Events::new(),
                vec![Html::text(name)],
            )],
        )
    }
}

pub fn frame_tab<Msg>(
    is_selected: bool,
    events: Events<Msg>,
    name: impl Into<String>,
) -> Html<Msg> {
    if is_selected {
        Html::span(
            Attributes::new().draggable(true),
            events,
            vec![Html::span(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-frametab")
                    .class("pure-button-primary"),
                Events::new(),
                vec![Html::text(name)],
            )],
        )
    } else {
        Html::span(
            Attributes::new().draggable(true),
            events,
            vec![Html::span(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-frametab")
                    .class("pure-button-darkgray"),
                Events::new(),
                vec![Html::text(name)],
            )],
        )
    }
}

pub fn contextmenu_text<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    text: impl Into<String>,
) -> Html<Msg> {
    Html::li(
        Attributes::new().class("pure-menu-item"),
        Events::new(),
        vec![Html::a(
            attributes.class("pure-menu-link"),
            events,
            vec![Html::text(text)],
        )],
    )
}

pub fn contextmenu_text_window<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    text: impl Into<String>,
) -> Html<Msg> {
    Html::li(
        Attributes::new().class("pure-menu-item"),
        Events::new(),
        vec![Html::a(
            attributes
                .class("pure-menu-link")
                .class("keyvalue")
                .class("keyvalue-rev"),
            events,
            vec![Html::text(text), awesome::i("fa-external-link-alt")],
        )],
    )
}

pub fn headermenu<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes
            .class("pure-button")
            .class("pure-button-headermenu"),
        events,
        children,
    )
}
