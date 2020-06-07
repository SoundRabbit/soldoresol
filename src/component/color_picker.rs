use crate::model::{Color, ColorSystem};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! color_column {
    ($cn:ident) => {
        Html::div(
            Attributes::new().class("linear-h"),
            Events::new(),
            (0..10)
                .map(|idx| {
                    Html::div(
                        Attributes::new()
                            .class("cell")
                            .class("cell-medium")
                            .style("background-color", ColorSystem::$cn(255, idx).to_string())
                            .nut("data-color", ColorSystem::$cn(255, idx).to_u32() as u64),
                        Events::new(),
                        vec![],
                    )
                })
                .collect(),
        )
    };
}

pub fn picker<Msg: 'static>(no_op: Msg, on_pick: impl FnOnce(Color) -> Msg + 'static) -> Html<Msg> {
    Html::div(
        Attributes::new().class("linear-v"),
        Events::new().on_click(move |e| {
            e.target()
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap()
                .get_attribute("data-color")
                .map(move |c| on_pick(Color::from(c.parse().unwrap_or(0))))
                .unwrap_or(no_op)
        }),
        vec![
            color_column!(gray),
            color_column!(blue),
            color_column!(green),
            color_column!(purple),
            color_column!(yellow),
            color_column!(orange),
            color_column!(red),
        ],
    )
}
