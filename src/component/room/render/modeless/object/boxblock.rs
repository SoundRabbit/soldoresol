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
    boxblock: &block::table_object::Boxblock,
    boxblock_id: &BlockId,
) -> Html {
    let [xw, yw, zw] = boxblock.size().clone();
    let color = boxblock.color();
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
                    text::span("X幅"),
                    set_size_input(boxblock_id, xw, move |xw| [xw, yw, zw]),
                    text::span("Y幅"),
                    set_size_input(boxblock_id, yw, move |yw| [xw, yw, zw]),
                    text::span("Z幅"),
                    set_size_input(boxblock_id, zw, move |zw| [xw, yw, zw]),
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
                            table_color(boxblock_id, color.alpha, 3),
                            table_color(boxblock_id, color.alpha, 5),
                            table_color(boxblock_id, color.alpha, 7),
                        ],
                    ),
                ],
            )],
        )],
    )
}

fn set_size_input(
    boxblock_id: &BlockId,
    s: f32,
    on_input: impl FnOnce(f32) -> [f32; 3] + 'static,
) -> Html {
    Html::input(
        Attributes::new()
            .type_("number")
            .value(s.to_string())
            .string("step", "0.1"),
        Events::new().on_input({
            let boxblock_id = boxblock_id.clone();
            move |s| {
                s.parse()
                    .map(|s| Msg::SetBoxblockSize(boxblock_id, on_input(s)))
                    .unwrap_or(Msg::NoOp)
            }
        }),
        vec![],
    )
}

fn table_color(boxblock_id: &BlockId, alpha: u8, idx: usize) -> Html {
    color_picker::idx(idx, Msg::NoOp, {
        let boxblock_id = boxblock_id.clone();
        move |mut color| {
            color.alpha = alpha;
            Msg::SetBoxblockColor(boxblock_id, color)
        }
    })
}
