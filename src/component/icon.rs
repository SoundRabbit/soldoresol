use kagura::prelude::*;

pub fn none<Msg>(attrs: Attributes) -> Html<Msg> {
    Html::div(
        attrs.class("icon").class("icon-rounded"),
        Events::new(),
        vec![],
    )
}

pub fn from_str<Msg>(attrs: Attributes, v: &str) -> Html<Msg> {
    Html::div(
        attrs
            .class("icon")
            .class("icon-rounded")
            .class("bg-color-light")
            .class("text-color-dark"),
        Events::new(),
        vec![Html::text(
            v.chars().next().map(|c| c.to_string()).unwrap_or("".into()),
        )],
    )
}

pub fn from_char<Msg>(attrs: Attributes, c: char) -> Html<Msg> {
    Html::div(
        attrs
            .class("icon")
            .class("icon-rounded")
            .class("bg-color-light")
            .class("text-color-dark"),
        Events::new(),
        vec![Html::text(c.to_string())],
    )
}

pub fn from_img<Msg>(attrs: Attributes, url: impl Into<String>) -> Html<Msg> {
    Html::img(
        attrs
            .class("pure-img")
            .class("icon")
            .class("icon-rounded")
            .class("bg-color-light")
            .string("src", url),
        Events::new(),
        vec![],
    )
}
