use super::awesome;
use kagura::prelude::*;

pub fn primary(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-primary"),
        events,
        children,
    )
}

pub fn secondary(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-secondary"),
        events,
        children,
    )
}

pub fn info(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-info"),
        events,
        children,
    )
}

pub fn danger(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-danger"),
        events,
        children,
    )
}

pub fn success(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-success"),
        events,
        children,
    )
}

pub fn light(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button").class("pure-button-light"),
        events,
        children,
    )
}

pub fn dark(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button").class("pure-button-dark"),
        events,
        children,
    )
}

pub fn selectable(
    is_selected: bool,
    attributes: Attributes,
    events: Events,
    children: Vec<Html>,
) -> Html {
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

pub fn toggle(is_toggled: bool, attributes: Attributes, events: Events) -> Html {
    if is_toggled {
        Html::span(attributes.class("toggle toggle-on"), events, vec![])
    } else {
        Html::span(attributes.class("toggle toggle-off"), events, vec![])
    }
}

pub fn close(attributes: Attributes, events: Events) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-close"),
        events,
        vec![awesome::i("fa-times")],
    )
}

pub fn transparent(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-transparent"),
        events,
        children,
    )
}

pub fn spacer(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes.class("pure-button pure-button-spacer"),
        events,
        children,
    )
}

pub fn check(is_checked: bool, attributes: Attributes, events: Events) -> Html {
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
pub fn add(attributes: Attributes, events: Events) -> Html {
    Html::button(
        attributes.class("app__add-btn").class("material-icons"),
        events,
        vec![Html::text("add")],
    )
}

pub fn frame_tab(
    is_selected: bool,
    is_draggable: bool,
    events: Events,
    name: impl Into<String>,
) -> Html {
    if is_selected {
        Html::span(
            Attributes::new().draggable(is_draggable),
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
            Attributes::new().draggable(is_draggable),
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

pub fn contextmenu_text(attributes: Attributes, events: Events, text: impl Into<String>) -> Html {
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

pub fn contextmenu_text_window(
    attributes: Attributes,
    events: Events,
    text: impl Into<String>,
) -> Html {
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

pub fn headermenu(attributes: Attributes, events: Events, children: Vec<Html>) -> Html {
    Html::button(
        attributes
            .class("pure-button")
            .class("pure-button-headermenu"),
        events,
        children,
    )
}
