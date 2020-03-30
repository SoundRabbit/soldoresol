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

pub fn secondary<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes
            .string("data-btn-variant", "secondary")
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

pub fn success<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(
        attributes
            .string("data-btn-variant", "success")
            .class("btn"),
        events,
        children,
    )
}

pub fn close<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(attributes.class("btn_close"), events, vec![Html::text("×")])
}

pub fn tab<Msg>(
    selected: bool,
    deletable: bool,
    attributes: Attributes,
    events: Events<Msg>,
    name: impl Into<String>,
) -> Html<Msg> {
    Html::button(
        attributes
            .class("btn_tab")
            .string("data-btn_tab-selected", selected.to_string()),
        events,
        vec![
            Html::text(name),
            if deletable {
                Html::button(
                    Attributes::new().class("btn_tab-close"),
                    Events::new(),
                    vec![Html::text("×")],
                )
            } else {
                Html::none()
            },
        ],
    )
}

pub fn context_menu<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    text: impl Into<String>,
) -> Html<Msg> {
    Html::button(
        attributes.class("btn_contextmenu"),
        events,
        vec![Html::text(text)],
    )
}
