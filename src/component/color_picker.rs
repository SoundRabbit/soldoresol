use crate::{color_system, Color};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! color_cell {
    ($cn: ident, $idx: expr) => {
        Html::div(
            Attributes::new()
                .class("cell")
                .class("cell-medium")
                .style("background-color", color_system::$cn(255, $idx).to_string())
                .nut("data-color", color_system::$cn(255, $idx).to_u32() as u64),
            Events::new(),
            vec![],
        )
    };
}

macro_rules! color_column {
    ($cn:ident) => {
        Html::div(
            Attributes::new().class("linear-h"),
            Events::new(),
            (0..10).map(|idx| color_cell!($cn, idx)).collect(),
        )
    };
}

pub fn major<Msg: 'static>(no_op: Msg, on_pick: impl FnOnce(Color) -> Msg + 'static) -> Html {
    Html::div(
        Attributes::new().class("linear-h"),
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
            color_cell!(gray, 0),
            color_cell!(gray, 5),
            color_cell!(gray, 9),
            color_cell!(blue, 5),
            color_cell!(green, 5),
            color_cell!(purple, 5),
            color_cell!(yellow, 5),
            color_cell!(orange, 5),
            color_cell!(red, 5),
        ],
    )
}

pub fn idx<Msg: 'static>(
    idx: usize,
    no_op: Msg,
    on_pick: impl FnOnce(Color) -> Msg + 'static,
) -> Html {
    Html::div(
        Attributes::new().class("linear-h"),
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
            color_cell!(gray, idx),
            color_cell!(blue, idx),
            color_cell!(green, idx),
            color_cell!(purple, idx),
            color_cell!(yellow, idx),
            color_cell!(orange, idx),
            color_cell!(red, idx),
        ],
    )
}

pub fn all<Msg: 'static>(no_op: Msg, on_pick: impl FnOnce(Color) -> Msg + 'static) -> Html {
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
