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

pub fn dark<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-dark"),
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
            attributes.class("pure-button pure-button-primary"),
            events,
            children,
        )
    } else {
        Html::button(
            attributes.class("pure-button pure-button-secondary"),
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

pub fn allocate<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-light allocate fab fa-buromobelexperte"),
        events,
        vec![],
    )
}

pub fn close<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(
        attributes.class("pure-button pure-button-dark"),
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

pub fn check<Msg>(is_checked: bool, attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    if is_checked {
        Html::div(
            attributes.class("checkbox").class("checkbox-checked"),
            events,
            vec![awesome::i("fa-check-square")],
        )
    } else {
        Html::div(
            attributes.class("checkbox"),
            events,
            vec![awesome::i("fa-square")],
        )
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

pub fn tab<Msg>(
    is_selected: bool,
    attributes: Attributes,
    events: Events<Msg>,
    name: impl Into<String>,
) -> Html<Msg> {
    if is_selected {
        Html::button(
            attributes.class("pure-button pure-button-tab pure-button-primary"),
            events,
            vec![Html::text(name)],
        )
    } else {
        Html::button(
            attributes.class("pure-button pure-button-tab pure-button-secondary"),
            events,
            vec![Html::text(name)],
        )
    }
}

#[allow(dead_code)]
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
