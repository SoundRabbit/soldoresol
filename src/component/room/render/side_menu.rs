use super::super::super::{awesome, btn, color_picker, dropdown, text};
use super::{state::table, Msg};
use crate::{
    block::{self, BlockId},
    color_system, Color,
};
use kagura::prelude::*;

pub fn render(z_index: u64, selecting_tool: &table::Tool) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("panel")
            .class("keyvalue")
            .class("keyvalue-rev")
            .style("overflow", "visible")
            .style("z-index", z_index.to_string()),
        Events::new(),
        vec![
            row(
                selecting_tool.is_selector(),
                "fa-mouse-pointer",
                "選択",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Selector)),
                match selecting_tool {
                    table::Tool::Selector => no_option(),
                    _ => text::span(""),
                },
            ),
            delm("描画"),
            row_pen(selecting_tool),
            row_eraser(selecting_tool),
            delm("作成"),
            row(
                selecting_tool.is_character(),
                "fa-user",
                "キャラクター",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Character)),
                match selecting_tool {
                    table::Tool::Character => no_option(),
                    _ => text::span(""),
                },
            ),
            row(
                selecting_tool.is_tablemask(),
                "fa-circle",
                "マップマスク",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Tablemask)),
                match selecting_tool {
                    table::Tool::Tablemask => no_option(),
                    _ => text::span(""),
                },
            ),
            row(
                selecting_tool.is_boxblock(),
                "fa-cube",
                "ブロック",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Boxblock)),
                match selecting_tool {
                    table::Tool::Boxblock => no_option(),
                    _ => text::span(""),
                },
            ),
            row(
                selecting_tool.is_area(),
                "fa-ruler-combined",
                "範囲",
                Events::new().on_click(|_| {
                    Msg::SetSelectingTableTool(table::Tool::Area {
                        type_: block::table_object::area::Type::Line(2.0),
                        color: color_system::red(192, 4),
                        block_id: None,
                        show_option_menu: false,
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
                Events::new().on_click(|_| {
                    Msg::SetSelectingTableTool(table::Tool::Route {
                        block_id: None,
                        show_option_menu: false,
                    })
                }),
                match selecting_tool {
                    table::Tool::Route { .. } => option(false, Events::new(), vec![]),
                    _ => text::span(""),
                },
            ),
            delm("表示"),
            row_measure(selecting_tool),
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn row_pen(selecting_tool: &table::Tool) -> Vec<Html<Msg>> {
    row(
        selecting_tool.is_pen(),
        "fa-pen",
        "ペン",
        Events::new().on_click(|_| {
            Msg::SetSelectingTableTool(table::Tool::Pen {
                line_width: 0.5,
                color: color_system::gray(255, 9),
                show_option_menu: true,
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
                    row_pen_menu(line_width, color),
                )
            }
            _ => text::span(""),
        },
    )
}

fn row_pen_menu(line_width: f64, color: Color) -> Vec<Html<Msg>> {
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
                                Msg::SetSelectingTableTool(table::Tool::Pen {
                                    line_width: w,
                                    color: color,
                                    show_option_menu: true,
                                })
                            })
                            .unwrap_or(Msg::NoOp)
                    }),
                    vec![],
                ),
                text::span("現在の描画色"),
                Html::div(
                    Attributes::new()
                        .class("cell")
                        .class("cell-medium")
                        .style("background-color", color.to_string()),
                    Events::new(),
                    vec![],
                ),
            ],
        ),
        Html::hr(Attributes::new(), Events::new(), vec![]),
        text::div("描画色"),
        color_picker::major(Msg::NoOp, move |color| {
            Msg::SetSelectingTableTool(table::Tool::Pen {
                line_width: line_width,
                color: color,
                show_option_menu: true,
            })
        }),
    ]
}

fn row_eraser(selecting_tool: &table::Tool) -> Vec<Html<Msg>> {
    row(
        selecting_tool.is_eracer(),
        "fa-eraser",
        "消しゴム",
        Events::new().on_click(|_| {
            Msg::SetSelectingTableTool(table::Tool::Eracer {
                line_width: 1.0,
                show_option_menu: true,
            })
        }),
        match selecting_tool {
            table::Tool::Eracer {
                line_width,
                show_option_menu,
            } => {
                let line_width = *line_width;
                let show_option_menu = *show_option_menu;
                option(
                    show_option_menu,
                    Events::new().on_click(move |_| {
                        Msg::SetSelectingTableTool(table::Tool::Eracer {
                            line_width: line_width,
                            show_option_menu: !show_option_menu,
                        })
                    }),
                    row_eraser_menu(line_width),
                )
            }
            _ => text::span(""),
        },
    )
}

fn row_eraser_menu(line_width: f64) -> Vec<Html<Msg>> {
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
                                Msg::SetSelectingTableTool(table::Tool::Eracer {
                                    line_width: w,
                                    show_option_menu: true,
                                })
                            })
                            .unwrap_or(Msg::NoOp)
                    }),
                    vec![],
                ),
            ],
        ),
        btn::secondary(
            Attributes::new(),
            Events::new().on_click(|_| Msg::ClearTable),
            vec![Html::text("クリア")],
        ),
    ]
}

fn row_measure(selecting_tool: &table::Tool) -> Vec<Html<Msg>> {
    row(
        selecting_tool.is_measure(),
        "fa-ruler",
        "距離",
        Events::new().on_click(|_| {
            Msg::SetSelectingTableTool(table::Tool::Measure {
                color: color_system::red(255, 7),
                block_id: None,
                show_option_menu: true,
            })
        }),
        match selecting_tool {
            table::Tool::Measure {
                color,
                block_id,
                show_option_menu,
            } => {
                let color = *color;
                let show_option_menu = *show_option_menu;
                option(
                    show_option_menu,
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| {
                            Msg::SetSelectingTableTool(table::Tool::Measure {
                                color,
                                block_id,
                                show_option_menu: !show_option_menu,
                            })
                        }
                    }),
                    row_measure_menu(color, block_id),
                )
            }
            _ => text::span(""),
        },
    )
}

fn row_measure_menu(color: Color, block_id: &Option<BlockId>) -> Vec<Html<Msg>> {
    vec![
        Html::div(
            Attributes::new().class("keyvalue"),
            Events::new(),
            vec![
                text::span("現在の描画色"),
                Html::div(
                    Attributes::new()
                        .class("cell")
                        .class("cell-medium")
                        .style("background-color", color.to_string()),
                    Events::new(),
                    vec![],
                ),
            ],
        ),
        Html::hr(Attributes::new(), Events::new(), vec![]),
        text::div("描画色"),
        color_picker::idx(7, Msg::NoOp, {
            let block_id = block_id.clone();
            move |color| {
                Msg::SetSelectingTableTool(table::Tool::Measure {
                    color,
                    block_id,
                    show_option_menu: true,
                })
            }
        }),
    ]
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
            Attributes::new().title(text),
            btn_event,
            vec![awesome::i(icon)],
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
            vec![if show_option {
                awesome::i("fa-angle-left")
            } else {
                awesome::i("fa-angle-right")
            }],
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

fn no_option() -> Html<Msg> {
    btn::spacer(
        Attributes::new().flag("disabled"),
        Events::new(),
        vec![awesome::i("fa-angle-right")],
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
