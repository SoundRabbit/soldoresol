use super::super::super::{awesome, btn, color_picker, dropdown, text};
use super::{state::table, Msg};
use crate::{color_system, Color};
use kagura::prelude::*;

pub fn render(selecting_tool: &table::Tool) -> Html<Msg> {
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
            row_pen(selecting_tool),
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
                text::span("選択色"),
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
        color_picker::major(Msg::NoOp, move |color| {
            Msg::SetSelectingTableTool(table::Tool::Pen {
                line_width: line_width,
                color: color,
                show_option_menu: true,
            })
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
