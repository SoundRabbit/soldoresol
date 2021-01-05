use super::super::super::super::super::{awesome, btn, color_picker, modeless, text};
use super::super::state::{Modal, Modeless};
use super::Msg;
use crate::{
    block::{self, BlockId},
    model::{self},
    resource::Data,
    Color, Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    is_grubbed: bool,
    tablemask: &block::table_object::Tablemask,
    tablemask_id: &BlockId,
) -> Html {
    let [xw, yw, _] = tablemask.size().clone();
    let color = tablemask.color();
    let is_inved = tablemask.is_inved();

    modeless::body(
        Attributes::new().class("scroll-v"),
        Events::new().on_mousemove(move |e| {
            if !is_grubbed {
                e.stop_propagation();
            }
            Msg::NoOp
        }),
        vec![Html::div(
            Attributes::new()
                .class("editormodeless")
                .class("pure-form")
                .class("linear-v"),
            Events::new(),
            vec![Html::div(
                Attributes::new().class("container-a").class("keyvalue"),
                Events::new(),
                vec![
                    text::span("形状"),
                    Html::div(
                        Attributes::new().class("linear-h"),
                        Events::new(),
                        vec![
                            set_type_btn(tablemask_id, "矩形", false, !tablemask.is_rounded()),
                            set_type_btn(tablemask_id, "円形", true, tablemask.is_rounded()),
                        ],
                    ),
                    text::span("反転"),
                    btn::toggle(
                        is_inved,
                        Attributes::new(),
                        Events::new().on_click({
                            let tablemask_id = tablemask_id.clone();
                            move |_| Msg::SetTablemaskIsInved(tablemask_id, !is_inved)
                        }),
                    ),
                    text::span("X幅"),
                    set_size_input(tablemask_id, xw, move |xw| [xw, yw]),
                    text::span("Y幅"),
                    set_size_input(tablemask_id, yw, move |yw| [xw, yw]),
                    text::span("選択色"),
                    Html::div(
                        Attributes::new()
                            .class("cell")
                            .class("cell-medium")
                            .style("background-color", color.to_string()),
                        Events::new(),
                        vec![],
                    ),
                    Html::div(
                        Attributes::new().class("keyvalue-banner").class("linear-v"),
                        Events::new(),
                        vec![
                            table_color(tablemask_id, color.alpha, 3),
                            table_color(tablemask_id, color.alpha, 5),
                            table_color(tablemask_id, color.alpha, 7),
                        ],
                    ),
                    text::span("不透明度"),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .string("step", "1")
                            .value((color.alpha as f32 * 100.0 / 255.0).round().to_string()),
                        Events::new().on_input({
                            let tablemask_id = tablemask_id.clone();
                            let mut color = color.clone();
                            move |a| {
                                a.parse()
                                    .map(|a: f32| {
                                        let a = (a * 255.0 / 100.0).min(255.0).max(0.0) as u8;
                                        color.alpha = a;
                                        Msg::SetTablemaskColor(tablemask_id, color)
                                    })
                                    .unwrap_or(Msg::NoOp)
                            }
                        }),
                        vec![],
                    ),
                ],
            )],
        )],
    )
}

fn set_type_btn(
    tablemask_id: &BlockId,
    text: impl Into<String>,
    is_rounded: bool,
    selected: bool,
) -> Html {
    btn::selectable(
        selected,
        Attributes::new(),
        Events::new().on_click({
            let tablemask_id = tablemask_id.clone();
            move |_| {
                if selected {
                    Msg::NoOp
                } else {
                    Msg::SetTablemaskIsRounded(tablemask_id, is_rounded)
                }
            }
        }),
        vec![Html::text(text)],
    )
}

fn set_size_input(
    tablemask_id: &BlockId,
    s: f32,
    on_input: impl FnOnce(f32) -> [f32; 2] + 'static,
) -> Html {
    Html::input(
        Attributes::new()
            .type_("number")
            .value(s.to_string())
            .string("step", "0.1"),
        Events::new().on_input({
            let tablemask_id = tablemask_id.clone();
            move |s| {
                s.parse()
                    .map(|s| Msg::SetTablemaskSize(tablemask_id, on_input(s)))
                    .unwrap_or(Msg::NoOp)
            }
        }),
        vec![],
    )
}

fn table_color(tablemask_id: &BlockId, alpha: u8, idx: usize) -> Html {
    color_picker::idx(idx, Msg::NoOp, {
        let tablemask_id = tablemask_id.clone();
        move |mut color| {
            color.alpha = alpha;
            Msg::SetTablemaskColor(tablemask_id, color)
        }
    })
}
