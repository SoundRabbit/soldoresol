use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::checkbox::checkbox;
use super::form::form;

pub fn measure_length<Msg>(s: &[f32; 2], p: &[f32; 2], len: f32) -> Html<Msg> {
    let quadrant = if s[0] <= p[0] && p[1] <= s[1] {
        ("px - 13ch);", "px + 1.4em);")
    } else if p[0] <= s[0] && p[1] <= s[1] {
        ("px + 3ch);", "px + 1.4em);")
    } else if p[0] <= s[0] && s[1] <= p[1] {
        ("px + 3ch);", "px - 0.2em);")
    } else {
        ("px - 13ch);", "px - 0.2em);")
    };
    Html::div(
        Attributes::new()
            .id("measure_length")
            .style(
                "left",
                String::from("calc(") + &s[0].to_string() + quadrant.0,
            )
            .style(
                "top",
                String::from("calc(") + &s[1].to_string() + quadrant.1,
            ),
        Events::new(),
        vec![Html::div(
            Attributes::new(),
            Events::new(),
            vec![Html::text(((len * 10.0).round() / 10.0).to_string() + " m")],
        )],
    )
}
