use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::btn;
use super::handout;
use super::measure_length::measure_length;
use super::measure_tool;
use super::radio::radio;
use crate::table::Table;

enum TableTool {
    Selecter,
    Pen,
    Eracer,
    Measure,
}

pub struct State {
    room_name: String,
    table: Table,
    table_height: i32,
    table_width: i32,
    table_rotation: (f32, f32),
    table_movement: (f32, f32),
    table_distance: f32,
    table_grabbed: (bool, bool),
    table_tool: TableTool,
    table_measure_start: Option<[f32; 2]>,
    table_measure: Option<([f32; 2], [f32; 2], f32)>,
    measure_tool_state: measure_tool::State,
    handout_state: handout::State,
    show_handout: bool,
}

pub enum Msg {
    NoOp,
    SetTableContext(web_sys::HtmlCanvasElement),
    ResizeTable,
    MoveTableCamera((i32, i32), (i32, i32)),
    ZoomTableCamera(f64),
    SetTableGrabbed((bool, bool), (i32, i32)),
    SetTableTool(TableTool),
    MeasureToolMsg(measure_tool::Msg),
    HandoutMsg(handout::Msg),
    SetShowHandoutFlag(bool),
}

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render).batch(|mut handler| {
        let a = Closure::wrap(Box::new(move || {
            handler(Msg::SetTableContext(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("table")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap(),
            ));
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .set_onload(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

fn init() -> (State, Cmd<Msg, Sub>) {
    let state = State {
        room_name: String::from("無名の部屋@") + &crate::ramdom_id::hex(16),
        table: Table::new(),
        table_height: 100,
        table_width: 100,
        table_rotation: (-0.25 * std::f32::consts::PI, 0.0625 * std::f32::consts::PI),
        table_movement: (0.0, 0.0),
        table_distance: 20.0,
        table_grabbed: (false, false),
        table_tool: TableTool::Selecter,
        table_measure_start: None,
        table_measure: None,
        measure_tool_state: measure_tool::init(),
        handout_state: handout::init(),
        show_handout: false,
    };
    (state, Cmd::none())
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::MeasureToolMsg(m) => {
            measure_tool::update(&mut state.measure_tool_state, m);
            Cmd::none()
        }
        Msg::HandoutMsg(m) => {
            handout::update(&mut state.handout_state, m);
            Cmd::none()
        }
        Msg::SetShowHandoutFlag(s) => {
            state.show_handout = s;
            Cmd::none()
        }
        Msg::SetTableContext(canvas) => {
            state.table_height = canvas.client_height();
            state.table_width = canvas.client_width();
            canvas.set_height(canvas.client_height() as u32);
            canvas.set_width(canvas.client_width() as u32);
            let context = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            state.table.set_context(context);
            update(state, Msg::ResizeTable)
        }
        Msg::ResizeTable => {
            state.table.resize();
            render_table(
                &mut state.table,
                state.table_movement,
                state.table_rotation,
                state.table_distance,
            );
            Cmd::none()
        }
        Msg::MoveTableCamera((dx, dy), (x, y)) => {
            let dx = dx as f32;
            let dy = dy as f32;
            let x = x as f32;
            let y = y as f32;
            let inner_height = web_sys::window()
                .unwrap()
                .inner_height()
                .unwrap()
                .as_f64()
                .unwrap();
            let inner_width = web_sys::window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap();
            match state.table_tool {
                TableTool::Selecter => {
                    if state.table_grabbed.0 {
                        let movement_factor = inner_height.min(inner_width) as f32 / 30.0;

                        state.table_movement.0 += dx as f32 / movement_factor;
                        state.table_movement.1 -= dy as f32 / movement_factor;

                        state.table_movement.1 = state.table_movement.1.min(0.0);
                    }
                }
                TableTool::Pen => {
                    if state.table_grabbed.0 {
                        state.table.draw_line(&[x - dx, y - dy], &[x, y]);
                    }
                    state
                        .table
                        .draw_pointer(&[x, y], 16.0, "#0366d6", "#0366d6", false);
                }
                TableTool::Eracer => {
                    if state.table_grabbed.0 {
                        state.table.erace_line(&[x - dx, y - dy], &[x, y]);
                    }
                    state
                        .table
                        .draw_pointer(&[x, y], 64.0, "#6f42c1", "#fff", false);
                }
                TableTool::Measure => {
                    if let Some(p) = &state.table_measure_start {
                        let r = state.table.draw_measure_and_get_length(
                            p,
                            &[x, y],
                            state.measure_tool_state.show_circle_on_table,
                            state.measure_tool_state.bind_to_grid,
                        );
                        state.table_measure = Some(([p[0], p[1]], [x, y], r as f32));
                        state.table.draw_pointer(
                            &[p[0], p[1]],
                            16.0,
                            "#d73a49",
                            "#fff",
                            state.measure_tool_state.bind_to_grid,
                        );
                    }
                    state.table.draw_pointer(
                        &[x, y],
                        16.0,
                        "#d73a49",
                        "#fff",
                        state.measure_tool_state.bind_to_grid,
                    );
                }
            }
            if state.table_grabbed.1 {
                let rotate_factor = inner_height.min(inner_width) as f32 / 3.0;
                state.table_rotation.0 = (state.table_rotation.0 + (dy as f32) / rotate_factor)
                    .max(-0.49 * std::f32::consts::PI)
                    .min(0.0);
                state.table_rotation.1 = state.table_rotation.1 - (dx as f32) / rotate_factor;
            }

            render_table(
                &mut state.table,
                state.table_movement,
                state.table_rotation,
                state.table_distance,
            );
            Cmd::none()
        }
        Msg::ZoomTableCamera(wheel) => {
            state.table_distance = (state.table_distance + (wheel / 20.0) as f32).max(0.0);
            render_table(
                &mut state.table,
                state.table_movement,
                state.table_rotation,
                state.table_distance,
            );
            Cmd::none()
        }
        Msg::SetTableGrabbed((grabbed_l, grabbed_r), (x, y)) => {
            state.table_grabbed = (grabbed_l, grabbed_r);
            match &state.table_tool {
                TableTool::Measure => {
                    if grabbed_l {
                        state.table_measure_start = Some([x as f32, y as f32]);
                    } else {
                        state.table_measure_start = None;
                        state.table_measure = None;
                    }
                }
                _ => {}
            }
            Cmd::none()
        }
        Msg::SetTableTool(table_tool) => {
            state.table_tool = table_tool;
            Cmd::none()
        }
    }
}

fn render_table(table: &mut Table, m: (f32, f32), r: (f32, f32), d: f32) {
    table.reset_translate();
    table.set_movement(&[-m.0, -m.1, -d]);
    table.set_x_axis_rotation(-r.0);
    table.set_z_axis_rotation(-r.1);
    table.render();
}

fn render(state: &State) -> Html<Msg> {
    let (table_grabbed_r, table_grabbed_l) = state.table_grabbed;
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![
            Html::canvas(
                Attributes::new()
                    .id("table")
                    .int("height", state.table_height as i64)
                    .int("width", state.table_width as i64),
                Events::new()
                    .on_mousedown(move |e| {
                        let l = e.buttons() & 1 != 0 || table_grabbed_r;
                        let r = e.buttons() & 2 != 0 || table_grabbed_l;
                        Msg::SetTableGrabbed((l, r), (e.client_x(), e.client_y()))
                    })
                    .on_mouseup(move |e| {
                        let l = e.buttons() & 1 != 0 && table_grabbed_r;
                        let r = e.buttons() & 2 != 0 && table_grabbed_l;
                        Msg::SetTableGrabbed((l, r), (e.client_x(), e.client_y()))
                    })
                    .on_mouseleave(|_| Msg::SetTableGrabbed((false, false), (0, 0)))
                    .on_mousemove(|e| {
                        Msg::MoveTableCamera(
                            (e.movement_x(), e.movement_y()),
                            (e.client_x(), e.client_y()),
                        )
                    })
                    .on("wheel", |e| {
                        Msg::ZoomTableCamera(e.dyn_into::<web_sys::WheelEvent>().unwrap().delta_y())
                    })
                    .on_contextmenu(|e| {
                        e.prevent_default();
                        Msg::NoOp
                    }),
                vec![],
            ),
            render_side_menu(),
            render_header(&state.room_name),
            match state.table_tool {
                TableTool::Measure => measure_tool::render(&state.measure_tool_state, || {
                    Box::new(|msg| Msg::MeasureToolMsg(msg))
                }),
                _ => Html::none(),
            },
            match &state.table_measure {
                Some((s, p, len)) => measure_length(&s, &p, len.clone()),
                _ => Html::none(),
            },
            if state.show_handout {
                handout::render(&state.handout_state, || {
                    Box::new(|msg| Msg::HandoutMsg(msg))
                })
            } else {
                Html::none()
            },
        ],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().id("app-side-menu"),
        Events::new(),
        vec![
            btn::primary(
                Attributes::new(),
                Events::new(),
                vec![Html::text("テーブル")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new(),
                vec![Html::text("リソース")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new(),
                vec![Html::text("キャラクター")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new(),
                vec![Html::text("オブジェクト")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::SetShowHandoutFlag(true)),
                vec![Html::text("資料")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new(),
                vec![Html::text("チャット")],
            ),
        ],
    )
}

fn render_header(room_name: impl Into<String>) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app-header"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![
                    Html::input(Attributes::new().value(room_name), Events::new(), vec![]),
                    btn::primary(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("ユーザー設定")],
                    ),
                    btn::info(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("ルーム情報")],
                    ),
                ],
            ),
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![
                    radio(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetTableTool(TableTool::Selecter)),
                        "toolbox",
                        "選択",
                        true,
                    ),
                    radio(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetTableTool(TableTool::Pen)),
                        "toolbox",
                        "ペン",
                        false,
                    ),
                    radio(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetTableTool(TableTool::Eracer)),
                        "toolbox",
                        "消しゴム",
                        false,
                    ),
                    radio(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetTableTool(TableTool::Measure)),
                        "toolbox",
                        "計測",
                        false,
                    ),
                ],
            ),
        ],
    )
}
