use kagura::prelude::*;

#[allow(dead_code)]
pub fn primary<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(attributes.class("app__btn--primary"), events, children)
}

#[allow(dead_code)]
pub fn secondary<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(attributes.class("app__btn--secondary"), events, children)
}

#[allow(dead_code)]
pub fn info<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(attributes.class("app__btn--info"), events, children)
}

#[allow(dead_code)]
pub fn danger<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(attributes.class("app__btn--danger"), events, children)
}

#[allow(dead_code)]
pub fn success<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(attributes.class("app__btn--success"), events, children)
}

#[allow(dead_code)]
pub fn dark<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::button(attributes.class("app__btn--dark"), events, children)
}

#[allow(dead_code)]
pub fn close<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(
        attributes.class("app__close-btn").class("material-icons"),
        events,
        vec![Html::text("clear")],
    )
}

#[allow(dead_code)]
pub fn add<Msg>(attributes: Attributes, events: Events<Msg>) -> Html<Msg> {
    Html::button(
        attributes.class("app__add-btn").class("material-icons"),
        events,
        vec![Html::text("add")],
    )
}

#[allow(dead_code)]
pub fn tab<Msg>(
    selected: bool,
    deletable: bool,
    attributes: Attributes,
    events: Events<Msg>,
    name: impl Into<String>,
) -> Html<Msg> {
    Html::a(
        attributes
            .class("app__tab-btn")
            .string("data-selected", selected.to_string()),
        events,
        vec![
            Html::span(
                Attributes::new().class("app__tab-btn-text"),
                Events::new(),
                vec![Html::text(name)],
            ),
            if deletable {
                Html::button(
                    Attributes::new()
                        .class("app__close-tab-btn")
                        .class("material-icons"),
                    Events::new(),
                    vec![Html::text("clear")],
                )
            } else {
                Html::none()
            },
        ],
    )
}

#[allow(dead_code)]
pub fn contextmenu_text<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    text: impl Into<String>,
) -> Html<Msg> {
    Html::button(
        attributes.class("app__contextmenu-btn"),
        events,
        vec![Html::text(text)],
    )
}
