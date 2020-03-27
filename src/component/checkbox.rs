use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn checkbox<Msg>(
    attributes: Attributes,
    events: Events<Msg>,
    text: impl Into<String>,
) -> Html<Msg> {
    let mut checkbox_id = [0, 0, 0, 0, 0, 0];
    web_sys::window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut checkbox_id);
    let checkbox_id = hex::encode(checkbox_id);
    let checkbox_id = String::from("checkbox") + &checkbox_id;
    Html::div(
        attributes.class("checkbox"),
        Events::new(),
        vec![
            Html::input(
                Attributes::new().type_("checkbox").id(&checkbox_id),
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
