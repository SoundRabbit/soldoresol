use super::super::{awesome, btn, text};
use super::{
    state::{self, table, Modal, Modeless},
    Msg, State,
};
use crate::block::{self, BlockId};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

mod canvas_container;
mod common;
mod contextmenu;
mod header_menu;
mod modal;
mod modeless;

pub fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .id("app")
            .class("fullscreen")
            .class("unselectable")
            .class("app")
            .style("grid-template-columns", "max-content 1fr"),
        Events::new()
            .on("dragover", |e| {
                e.prevent_default();
                Msg::NoOp
            })
            .on("drop", |e| {
                e.prevent_default();
                e.stop_propagation();
                let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                e.data_transfer()
                    .unwrap()
                    .files()
                    .map(|files| Msg::LoadFromFileList(files))
                    .unwrap_or(Msg::NoOp)
            }),
        vec![
            header_menu::render(
                state.room().id.as_ref(),
                state.table().selecting_tool(),
                state.table().is_2d_mode(),
            ),
            side_menu(state.table().selecting_tool()),
            if let Some(world) = state.block_field().get::<block::World>(state.world()) {
                canvas_container::render(&state, world)
            } else {
                Html::none()
            },
            if let Some(contextmenu_state) = state.contextmenu() {
                contextmenu::render(state.block_field(), contextmenu_state)
            } else {
                Html::none()
            },
            modal::render(state.block_field(), state.resource(), state.modal(), state),
        ],
    )
}

fn side_menu(selecting_tool: &table::Tool) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("panel")
            .class("keyvalue")
            .class("keyvalue-rev"),
        Events::new(),
        vec![
            row(
                selecting_tool.is_selector(),
                "fa-mouse-pointer",
                "選択",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Selector)),
                Events::new(),
            ),
            delm("描画"),
            row(
                selecting_tool.is_pen(),
                "fa-pen",
                "ペン",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Pen)),
                Events::new(),
            ),
            row(
                selecting_tool.is_eracer(),
                "fa-eraser",
                "消しゴム",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Eracer)),
                Events::new(),
            ),
            delm("作成"),
            row(
                selecting_tool.is_area(),
                "fa-ruler-combined",
                "範囲",
                Events::new().on_click(|_| {
                    Msg::SetSelectingTableTool(table::Tool::Area {
                        line_width: 2.0,
                        is_rounded: false,
                    })
                }),
                Events::new(),
            ),
            row(
                selecting_tool.is_route(),
                "fa-route",
                "経路",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Route(None))),
                Events::new(),
            ),
            delm("表示"),
            row(
                selecting_tool.is_measure(),
                "fa-ruler",
                "距離",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Measure(None))),
                Events::new(),
            ),
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn row(
    selected: bool,
    icon: impl Into<String>,
    text: impl Into<String>,
    btn_event: Events<Msg>,
    option_event: Events<Msg>,
) -> Vec<Html<Msg>> {
    vec![
        btn::selectable(
            selected,
            Attributes::new(),
            btn_event,
            vec![awesome::i(icon), Html::text(" "), text::span(text)],
        ),
        option(selected, Attributes::new(), option_event),
    ]
}

fn option(show: bool, attrs: Attributes, events: Events<Msg>) -> Html<Msg> {
    if show {
        btn::transparent(attrs, events, vec![awesome::i("fa-angle-right")])
    } else {
        text::span("")
    }
}

fn delm(text: impl Into<String>) -> Vec<Html<Msg>> {
    vec![
        Html::hr(
            Attributes::new().class("keyvalue-banner"),
            Events::new(),
            vec![],
        ),
        Html::span(
            Attributes::new().class("keyvalue-banner"),
            Events::new(),
            vec![Html::text(text)],
        ),
    ]
}
