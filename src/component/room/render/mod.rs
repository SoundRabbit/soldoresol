use super::super::{awesome, btn, color_picker, dropdown, text};
use super::{
    state::{self, table, Modal, Modeless},
    Msg, State,
};
use crate::{
    block::{self, BlockId},
    color_system,
};
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
            .class("keyvalue-rev")
            .style("overflow", "visible")
            .style("z-index", "1"),
        Events::new(),
        vec![
            row(
                selecting_tool.is_selector(),
                "fa-mouse-pointer",
                "選択",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Selector)),
                match selecting_tool {
                    table::Tool::Selector => option(false, Events::new(), vec![]),
                    _ => text::span(""),
                },
            ),
            delm("描画"),
            row(
                selecting_tool.is_pen(),
                "fa-pen",
                "ペン",
                Events::new().on_click(|_| {
                    Msg::SetSelectingTableTool(table::Tool::Pen {
                        line_width: 0.5,
                        color: color_system::gray(255, 9),
                        show_option_menu: false,
                    })
                }),
                match selecting_tool {
                    table::Tool::Pen {
                        line_width,
                        color,
                        show_option_menu,
                    } => {
                        let line_width = *line_width;
                        let color = *color;
                        let show_option_menu = *show_option_menu;
                        option(
                            show_option_menu,
                            Events::new().on_click(move |_| {
                                Msg::SetSelectingTableTool(table::Tool::Pen {
                                    line_width: line_width,
                                    color: color,
                                    show_option_menu: !show_option_menu,
                                })
                            }),
                            vec![
                                Html::div(
                                    Attributes::new().class("keyvalue"),
                                    Events::new(),
                                    vec![
                                        text::span("太さ"),
                                        Html::input(
                                            Attributes::new()
                                                .type_("number")
                                                .value(line_width.to_string())
                                                .string("step", "0.1"),
                                            Events::new().on_input(move |w| {
                                                w.parse()
                                                    .map(|w| {
                                                        Msg::SetSelectingTableTool(
                                                            table::Tool::Pen {
                                                                line_width: w,
                                                                color: color,
                                                                show_option_menu: show_option_menu,
                                                            },
                                                        )
                                                    })
                                                    .unwrap_or(Msg::NoOp)
                                            }),
                                            vec![],
                                        ),
                                    ],
                                ),
                                color_picker::major(Msg::NoOp, move |color| {
                                    Msg::SetSelectingTableTool(table::Tool::Pen {
                                        line_width: line_width,
                                        color: color,
                                        show_option_menu: show_option_menu,
                                    })
                                }),
                            ],
                        )
                    }
                    _ => text::span(""),
                },
            ),
            row(
                selecting_tool.is_eracer(),
                "fa-eraser",
                "消しゴム",
                Events::new().on_click(|_| {
                    Msg::SetSelectingTableTool(table::Tool::Eracer {
                        show_option_menu: false,
                    })
                }),
                match selecting_tool {
                    table::Tool::Eracer { show_option_menu } => {
                        let show_option_menu = *show_option_menu;
                        option(
                            show_option_menu,
                            Events::new().on_click(move |_| {
                                Msg::SetSelectingTableTool(table::Tool::Eracer {
                                    show_option_menu: !show_option_menu,
                                })
                            }),
                            vec![],
                        )
                    }
                    _ => text::span(""),
                },
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
                match selecting_tool {
                    table::Tool::Area { .. } => option(false, Events::new(), vec![]),
                    _ => text::span(""),
                },
            ),
            row(
                selecting_tool.is_route(),
                "fa-route",
                "経路",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Route(None))),
                match selecting_tool {
                    table::Tool::Route(..) => option(false, Events::new(), vec![]),
                    _ => text::span(""),
                },
            ),
            delm("表示"),
            row(
                selecting_tool.is_measure(),
                "fa-ruler",
                "距離",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Measure(None))),
                match selecting_tool {
                    table::Tool::Measure(..) => option(false, Events::new(), vec![]),
                    _ => text::span(""),
                },
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
    option: Html<Msg>,
) -> Vec<Html<Msg>> {
    vec![
        btn::selectable(
            selected,
            Attributes::new(),
            btn_event,
            vec![awesome::i(icon), Html::text(" "), text::span(text)],
        ),
        option,
    ]
}

fn option(show_option: bool, events: Events<Msg>, menu: Vec<Html<Msg>>) -> Html<Msg> {
    dropdown::right_bottom(
        show_option,
        btn::transparent(
            Attributes::new(),
            events,
            vec![awesome::i("fa-angle-right")],
        ),
        Html::div(
            Attributes::new()
                .class("panel")
                .class("pure-form")
                .class("linear-v")
                .style("width", "max-content"),
            Events::new(),
            menu,
        ),
    )
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
