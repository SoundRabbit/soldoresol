use super::super::super::{awesome, btn, color_picker, dropdown, text};
use super::{state::table, Msg};
use crate::{
    block::{self, BlockId},
    color_system, Color,
};
use kagura::prelude::*;

pub fn render(z_index: u64, selecting_tool: &table::Tool) -> Html {
    Html::div(
        Attributes::new()
            .class("panel")
            .class("keyvalue")
            .class("keyvalue-rev")
            .style("overflow", "visible")
            .style("z-index", z_index.to_string()),
        Events::new(),
        {
            let mut children = vec![];
            children.append(&mut row(
                selecting_tool.is_selector(),
                "fa-mouse-pointer",
                "選択",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Selector)),
                match selecting_tool {
                    table::Tool::Selector => no_option(),
                    _ => text::span(""),
                },
            ));
            children.append(&mut delm("描画"));
            children.append(&mut row_pen(selecting_tool));
            children.append(&mut row_eraser(selecting_tool));
            children.append(&mut delm("作成"));
            children.append(&mut row(
                selecting_tool.is_character(),
                "fa-user",
                "キャラクター",
                Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Character)),
                match selecting_tool {
                    table::Tool::Character => no_option(),
                    _ => text::span(""),
                },
            ));
            children.append(&mut row_tablemask(selecting_tool));
            children.append(&mut row_boxblock(selecting_tool));
            children.append(&mut row_area(selecting_tool));
            children.append(&mut delm("表示"));
            children.append(&mut row_measure(selecting_tool));
            children
        },
    )
}

fn row_pen(selecting_tool: &table::Tool) -> Vec<Html> {
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

fn row_pen_menu(line_width: f64, color: Color) -> Vec<Html> {
    vec![Html::div(
        Attributes::new().class("keyvalue"),
        Events::new(),
        vec![
            text::span("線幅"),
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
            Html::hr(
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
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
            Html::div(
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
                vec![color_picker::major(Msg::NoOp, move |color| {
                    Msg::SetSelectingTableTool(table::Tool::Pen {
                        line_width: line_width,
                        color: color,
                        show_option_menu: true,
                    })
                })],
            ),
        ],
    )]
}

fn row_eraser(selecting_tool: &table::Tool) -> Vec<Html> {
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

fn row_eraser_menu(line_width: f64) -> Vec<Html> {
    vec![
        Html::div(
            Attributes::new().class("keyvalue"),
            Events::new(),
            vec![
                text::span("線幅"),
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

fn row_tablemask(selecting_tool: &table::Tool) -> Vec<Html> {
    row(
        selecting_tool.is_tablemask(),
        "fa-clone",
        "マップマスク",
        Events::new().on_click(|_| {
            Msg::SetSelectingTableTool(table::Tool::Tablemask {
                size: [8.0, 8.0],
                color: color_system::gray((0.6 * 255.0) as u8, 5),
                is_rounded: true,
                is_inved: false,
                show_option_menu: true,
            })
        }),
        match selecting_tool {
            table::Tool::Tablemask {
                size,
                color,
                is_rounded,
                is_inved,
                show_option_menu,
            } => {
                let color = *color;
                let is_rounded = *is_rounded;
                let is_inved = *is_inved;
                let show_option_menu = *show_option_menu;
                option(
                    show_option_menu,
                    Events::new().on_click({
                        let size = size.clone();
                        move |_| {
                            Msg::SetSelectingTableTool(table::Tool::Tablemask {
                                size,
                                color,
                                is_rounded,
                                is_inved,
                                show_option_menu: !show_option_menu,
                            })
                        }
                    }),
                    row_tablemask_menu(size, color, is_rounded, is_inved),
                )
            }
            _ => text::span(""),
        },
    )
}

fn row_tablemask_menu(
    size: &[f32; 2],
    mut color: Color,
    is_rounded: bool,
    is_inved: bool,
) -> Vec<Html> {
    let [xw, yw] = size.clone();
    vec![Html::div(
        Attributes::new().class("keyvalue"),
        Events::new(),
        vec![
            text::span("形状"),
            Html::div(
                Attributes::new().class("linear-h"),
                Events::new(),
                vec![
                    row_tablemask_menu_type(color, xw, yw, is_inved, "矩形", false, !is_rounded),
                    row_tablemask_menu_type(color, xw, yw, is_inved, "円形", true, is_rounded),
                ],
            ),
            text::span("反転"),
            btn::toggle(
                is_inved,
                Attributes::new(),
                Events::new().on_click(move |_| {
                    Msg::SetSelectingTableTool(table::Tool::Tablemask {
                        size: [xw, yw],
                        color,
                        is_rounded,
                        is_inved: !is_inved,
                        show_option_menu: true,
                    })
                }),
            ),
            text::span("X幅"),
            row_tablemask_menu_size(color, is_rounded, is_inved, xw, move |xw| [xw, yw]),
            text::span("Y幅"),
            row_tablemask_menu_size(color, is_rounded, is_inved, yw, move |yw| [xw, yw]),
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
                    row_tablemask_menu_color(3, is_rounded, is_inved, xw, yw),
                    row_tablemask_menu_color(5, is_rounded, is_inved, xw, yw),
                    row_tablemask_menu_color(7, is_rounded, is_inved, xw, yw),
                ],
            ),
            text::span("不透明度"),
            Html::input(
                Attributes::new()
                    .type_("number")
                    .string("step", "1")
                    .value((color.alpha as f32 * 100.0 / 255.0).round().to_string()),
                Events::new().on_input(move |a| {
                    a.parse()
                        .map(|a: f32| {
                            let a = (a * 255.0 / 100.0).min(255.0).max(0.0) as u8;
                            color.alpha = a;
                            Msg::SetSelectingTableTool(table::Tool::Tablemask {
                                size: [xw, yw],
                                color,
                                is_rounded: is_rounded,
                                is_inved: is_inved,
                                show_option_menu: true,
                            })
                        })
                        .unwrap_or(Msg::NoOp)
                }),
                vec![],
            ),
        ],
    )]
}

fn row_tablemask_menu_type(
    color: Color,
    xw: f32,
    yw: f32,
    is_inved: bool,
    text: impl Into<String>,
    is_rounded: bool,
    selected: bool,
) -> Html {
    btn::selectable(
        selected,
        Attributes::new(),
        Events::new().on_click(move |_| {
            if selected {
                Msg::NoOp
            } else {
                Msg::SetSelectingTableTool(table::Tool::Tablemask {
                    size: [xw, yw],
                    color,
                    is_rounded: is_rounded,
                    is_inved: is_inved,
                    show_option_menu: true,
                })
            }
        }),
        vec![Html::text(text)],
    )
}

fn row_tablemask_menu_size(
    color: Color,
    is_rounded: bool,
    is_inved: bool,
    s: f32,
    on_input: impl FnOnce(f32) -> [f32; 2] + 'static,
) -> Html {
    Html::input(
        Attributes::new()
            .type_("number")
            .value(s.to_string())
            .string("step", "0.1"),
        Events::new().on_input(move |w| {
            w.parse()
                .map(|w| {
                    Msg::SetSelectingTableTool(table::Tool::Tablemask {
                        color,
                        size: on_input(w),
                        is_rounded,
                        is_inved,
                        show_option_menu: true,
                    })
                })
                .unwrap_or(Msg::NoOp)
        }),
        vec![],
    )
}

fn row_tablemask_menu_color(
    idx: usize,
    is_rounded: bool,
    is_inved: bool,
    xw: f32,
    yw: f32,
) -> Html {
    color_picker::idx(idx, Msg::NoOp, {
        move |mut color| {
            Msg::SetSelectingTableTool(table::Tool::Tablemask {
                color,
                size: [xw, yw],
                is_rounded,
                is_inved,
                show_option_menu: true,
            })
        }
    })
}

fn row_boxblock(selecting_tool: &table::Tool) -> Vec<Html> {
    row(
        selecting_tool.is_boxblock(),
        "fa-cube",
        "ブロック",
        Events::new().on_click(|_| {
            Msg::SetSelectingTableTool(table::Tool::Boxblock {
                color: color_system::blue(255, 5),
                size: [1.0, 1.0, 1.0],
                show_option_menu: true,
            })
        }),
        match selecting_tool {
            table::Tool::Boxblock {
                size,
                color,
                show_option_menu,
            } => {
                let color = *color;
                let show_option_menu = *show_option_menu;
                option(
                    show_option_menu,
                    Events::new().on_click({
                        let size = size.clone();
                        move |_| {
                            Msg::SetSelectingTableTool(table::Tool::Boxblock {
                                size,
                                color,
                                show_option_menu: !show_option_menu,
                            })
                        }
                    }),
                    row_boxblock_menu(size, color),
                )
            }
            _ => text::span(""),
        },
    )
}

fn row_boxblock_menu(size: &[f32; 3], color: Color) -> Vec<Html> {
    let [xw, yw, zw] = size.clone();
    vec![Html::div(
        Attributes::new().class("keyvalue"),
        Events::new(),
        vec![
            text::span("X幅"),
            row_boxblock_menu_size(color, xw, move |xw| [xw, yw, zw]),
            text::span("Y幅"),
            row_boxblock_menu_size(color, yw, move |yw| [xw, yw, zw]),
            text::span("Z幅"),
            row_boxblock_menu_size(color, zw, move |zw| [xw, yw, zw]),
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
                    row_boxblock_menu_color(3, xw, yw, zw),
                    row_boxblock_menu_color(5, xw, yw, zw),
                    row_boxblock_menu_color(7, xw, yw, zw),
                ],
            ),
        ],
    )]
}

fn row_boxblock_menu_size(
    color: Color,
    s: f32,
    on_input: impl FnOnce(f32) -> [f32; 3] + 'static,
) -> Html {
    Html::input(
        Attributes::new()
            .type_("number")
            .value(s.to_string())
            .string("step", "0.1"),
        Events::new().on_input(move |w| {
            w.parse()
                .map(|w| {
                    Msg::SetSelectingTableTool(table::Tool::Boxblock {
                        color,
                        size: on_input(w),
                        show_option_menu: true,
                    })
                })
                .unwrap_or(Msg::NoOp)
        }),
        vec![],
    )
}

fn row_boxblock_menu_color(idx: usize, xw: f32, yw: f32, zw: f32) -> Html {
    color_picker::idx(idx, Msg::NoOp, {
        move |color| {
            Msg::SetSelectingTableTool(table::Tool::Boxblock {
                color,
                size: [xw, yw, zw],
                show_option_menu: true,
            })
        }
    })
}

fn row_area(selecting_tool: &table::Tool) -> Vec<Html> {
    row(
        selecting_tool.is_area(),
        "fa-chess-board",
        "範囲マーカー",
        Events::new().on_click(|_| {
            Msg::SetSelectingTableTool(table::Tool::Area {
                type_: block::table_object::area::Type::Line(2.0),
                color_1: color_system::red(192, 3),
                color_2: color_system::red(192, 2),
                block_id: None,
                show_option_menu: true,
            })
        }),
        match selecting_tool {
            table::Tool::Area {
                type_,
                color_1,
                color_2,
                block_id,
                show_option_menu,
            } => {
                let color_1 = *color_1;
                let color_2 = *color_2;
                let show_option_menu = *show_option_menu;
                option(
                    show_option_menu,
                    Events::new().on_click({
                        let type_ = type_.clone();
                        let block_id = block_id.clone();
                        move |_| {
                            Msg::SetSelectingTableTool(table::Tool::Area {
                                type_,
                                color_1,
                                color_2,
                                block_id,
                                show_option_menu: !show_option_menu,
                            })
                        }
                    }),
                    row_area_menu(type_, color_1, color_2, block_id),
                )
            }
            _ => text::span(""),
        },
    )
}

fn row_area_menu(
    type_: &block::table_object::area::Type,
    color_1: Color,
    color_2: Color,
    block_id: &Option<BlockId>,
) -> Vec<Html> {
    vec![Html::div(
        Attributes::new().class("keyvalue"),
        Events::new(),
        vec![
            text::span("形状"),
            Html::div(
                Attributes::new().class("linear-h"),
                Events::new(),
                vec![
                    btn::selectable(
                        type_.is_line(),
                        Attributes::new(),
                        Events::new().on_click({
                            let block_id = block_id.clone();
                            let is_line = type_.is_line();
                            move |_| {
                                if is_line {
                                    Msg::NoOp
                                } else {
                                    Msg::SetSelectingTableTool(table::Tool::Area {
                                        type_: block::table_object::area::Type::Line(2.0),
                                        color_1,
                                        color_2,
                                        block_id,
                                        show_option_menu: true,
                                    })
                                }
                            }
                        }),
                        vec![Html::text("直線")],
                    ),
                    btn::selectable(
                        type_.is_rounded(),
                        Attributes::new(),
                        Events::new().on_click({
                            let block_id = block_id.clone();
                            let is_rounded = type_.is_rounded();
                            move |_| {
                                if is_rounded {
                                    Msg::NoOp
                                } else {
                                    Msg::SetSelectingTableTool(table::Tool::Area {
                                        type_: block::table_object::area::Type::Rounded,
                                        color_1,
                                        color_2,
                                        block_id,
                                        show_option_menu: true,
                                    })
                                }
                            }
                        }),
                        vec![Html::text("円内")],
                    ),
                ],
            ),
            text::span("線幅"),
            match type_ {
                block::table_object::area::Type::Line(line_width) => Html::input(
                    Attributes::new()
                        .type_("number")
                        .value(line_width.to_string())
                        .string("step", "0.1"),
                    Events::new().on_input({
                        let block_id = block_id.clone();
                        move |w| {
                            w.parse()
                                .map(|w| {
                                    Msg::SetSelectingTableTool(table::Tool::Area {
                                        type_: block::table_object::area::Type::Line(w),
                                        color_1,
                                        color_2,
                                        block_id,
                                        show_option_menu: true,
                                    })
                                })
                                .unwrap_or(Msg::NoOp)
                        }
                    }),
                    vec![],
                ),
                _ => Html::div(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::input(
                        Attributes::new().flag("disabled"),
                        Events::new(),
                        vec![],
                    )],
                ),
            },
            Html::hr(
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
                vec![],
            ),
            text::span("選択色1"),
            Html::div(
                Attributes::new()
                    .class("cell")
                    .class("cell-medium")
                    .style("background-color", color_1.to_string()),
                Events::new(),
                vec![],
            ),
            Html::div(
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
                vec![color_picker::idx(3, Msg::NoOp, {
                    let block_id = block_id.clone();
                    let type_ = type_.clone();
                    move |mut color_1| {
                        color_1.alpha = 192;
                        Msg::SetSelectingTableTool(table::Tool::Area {
                            type_,
                            color_1,
                            color_2,
                            block_id,
                            show_option_menu: true,
                        })
                    }
                })],
            ),
            Html::hr(
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
                vec![],
            ),
            text::span("選択色2"),
            Html::div(
                Attributes::new()
                    .class("cell")
                    .class("cell-medium")
                    .style("background-color", color_2.to_string()),
                Events::new(),
                vec![],
            ),
            Html::div(
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
                vec![color_picker::idx(2, Msg::NoOp, {
                    let block_id = block_id.clone();
                    let type_ = type_.clone();
                    move |mut color_2| {
                        color_2.alpha = 192;
                        Msg::SetSelectingTableTool(table::Tool::Area {
                            type_,
                            color_1,
                            color_2,
                            block_id,
                            show_option_menu: true,
                        })
                    }
                })],
            ),
        ],
    )]
}

fn row_measure(selecting_tool: &table::Tool) -> Vec<Html> {
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

fn row_measure_menu(color: Color, block_id: &Option<BlockId>) -> Vec<Html> {
    vec![Html::div(
        Attributes::new().class("keyvalue"),
        Events::new(),
        vec![
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
                Attributes::new().class("keyvalue-banner"),
                Events::new(),
                vec![color_picker::idx(7, Msg::NoOp, {
                    let block_id = block_id.clone();
                    move |color| {
                        Msg::SetSelectingTableTool(table::Tool::Measure {
                            color,
                            block_id,
                            show_option_menu: true,
                        })
                    }
                })],
            ),
        ],
    )]
}

fn row(
    selected: bool,
    icon: impl Into<String>,
    text: impl Into<String>,
    btn_event: Events,
    option: Html,
) -> Vec<Html> {
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

fn option(show_option: bool, events: Events, menu: Vec<Html>) -> Html {
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

fn no_option() -> Html {
    btn::spacer(
        Attributes::new().flag("disabled"),
        Events::new(),
        vec![awesome::i("fa-angle-right")],
    )
}

fn delm(text: impl Into<String>) -> Vec<Html> {
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
