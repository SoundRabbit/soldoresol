use kagura::prelude::*;

pub enum Icon<'a> {
    Character(char),
    MaterialIcon(&'a str),
}

pub fn small<Msg>(attributes: Attributes, events: Events<Msg>, icon: Icon) -> Html<Msg> {
    match icon {
        Icon::Character(c) => Html::span(
            attributes
                .class("icon")
                .class("icon-font")
                .string("data-icon-variant", "small"),
            events,
            vec![Html::text(c.to_string())],
        ),
        Icon::MaterialIcon(icon_name) => Html::span(
            attributes
                .class("icon")
                .class("material-icons")
                .string("data-icon-variant", "small"),
            events,
            vec![Html::text(icon_name)],
        ),
    }
}

pub fn medium<Msg>(attributes: Attributes, events: Events<Msg>, icon: Icon) -> Html<Msg> {
    match icon {
        Icon::Character(c) => Html::span(
            attributes
                .class("icon")
                .class("icon-font")
                .string("data-icon-variant", "medium"),
            events,
            vec![Html::text(c.to_string())],
        ),
        Icon::MaterialIcon(icon_name) => Html::span(
            attributes
                .class("icon")
                .class("material-icons")
                .string("data-icon-variant", "medium"),
            events,
            vec![Html::text(icon_name)],
        ),
    }
}

pub fn large<Msg>(attributes: Attributes, events: Events<Msg>, icon: Icon) -> Html<Msg> {
    match icon {
        Icon::Character(c) => Html::span(
            attributes
                .class("icon")
                .class("icon-font")
                .string("data-icon-variant", "large"),
            events,
            vec![Html::text(c.to_string())],
        ),
        Icon::MaterialIcon(icon_name) => Html::span(
            attributes
                .class("icon")
                .class("material-icons")
                .string("data-icon-variant", "large"),
            events,
            vec![Html::text(icon_name)],
        ),
    }
}
