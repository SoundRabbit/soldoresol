use crate::random_id;
use kagura::prelude::*;

pub fn checkbox<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    text: impl Into<String>,
    checked: bool,
) -> Html<Msg> {
    let checkbox_id = random_id::hex(6);
    let checkbox_id = String::from("checkbox") + &checkbox_id;
    Html::div(
        attributes.class("checkbox"),
        Events::new(),
        vec![
            Html::input(
                if checked {
                    Attributes::new()
                        .type_("checkbox")
                        .id(&checkbox_id)
                        .checked()
                } else {
                    Attributes::new().type_("checkbox").id(&checkbox_id)
                },
                events,
                vec![],
            ),
            Html::label(
                Attributes::new().string("for", &checkbox_id),
                Events::new(),
                vec![Html::text(text)],
            ),
        ],
    )
}
