use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn radio<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    name: impl Into<String>,
    text: impl Into<String>,
) -> Html<Msg> {
    let mut radio_id = [0, 0, 0, 0, 0, 0];
    web_sys::window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut radio_id);
    let radio_id = hex::encode(&radio_id);
    let radio_id = String::from("radio") + &radio_id;
    Html::div(
        attributes.class("radio"),
        Events::new(),
        vec![
            Html::input(
                Attributes::new()
                    .type_("radio")
                    .id(&radio_id)
                    .string("name", "app-toolbox"),
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
