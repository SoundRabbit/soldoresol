use kagura::prelude::*;

pub fn from_str<Msg>(attrs: Attributes, v: &str) -> Html<Msg> {
    Html::div(
        attrs
            .class("icon")
            .class("icon-medium")
            .class("icon-rounded")
            .class("bg-color-light")
            .class("text-color-dark"),
        Events::new(),
        vec![Html::text(
            v.chars().next().map(|c| c.to_string()).unwrap_or("".into()),
        )],
    )
}
