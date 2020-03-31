use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::random_id;

pub fn radio<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    name: impl Into<String>,
    text: impl Into<String>,
    checked: bool,
) -> Html<Msg> {
    let radio_id = random_id::hex(5);
    let radio_id = String::from("radio") + &radio_id;
    if checked {
        Html::div(
            attributes.class("radio"),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new()
                        .type_("radio")
                        .id(&radio_id)
                        .string("name", name)
                        .checked(),
                    events,
                    vec![],
                ),
                Html::label(
                    Attributes::new().string("for", &radio_id),
                    Events::new(),
                    vec![Html::text(text)],
                ),
            ],
        )
    } else {
        Html::div(
            attributes.class("radio"),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new()
                        .type_("radio")
                        .id(&radio_id)
                        .string("name", name),
                    events,
                    vec![],
                ),
                Html::label(
                    Attributes::new().string("for", &radio_id),
                    Events::new(),
                    vec![Html::text(text)],
                ),
            ],
        )
    }
}
