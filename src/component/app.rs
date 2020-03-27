use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::btn;
use super::radio::radio;
use crate::table::Table;

enum TableTool {
    Selecter,
    Pen,
    Eracer,
    Pointer,
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
}

pub enum Msg {
    NoOp,
    SetTableContext(web_sys::HtmlCanvasElement),
    ResizeTable,
    MoveTableCamera((i32, i32), (i32, i32)),
    ZoomTableCamera(f64),
    SetTableGrabbed(bool, bool),
    SetTableTool(TableTool),
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
    };
    (state, Cmd::none())
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetTableContext(canvas) => {
            state.table_height = canvas.client_height();
            state.table_width = canvas.client_width();
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
            match state.table_tool {
                TableTool::Selecter => {
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
                    let rotate_factor = inner_height.min(inner_width) as f32 / 3.0;
                    let movement_factor = inner_height.min(inner_width) as f32 / 30.0;

                    if state.table_grabbed.0 {
                        state.table_movement.0 += dx as f32 / movement_factor;
                        state.table_movement.1 -= dy as f32 / movement_factor;
                    }

                    if state.table_grabbed.1 {
                        state.table_rotation.0 = (state.table_rotation.0
                            + (dy as f32) / rotate_factor)
                            .max(-0.49 * std::f32::consts::PI)
                            .min(0.0);
                        state.table_rotation.1 =
                            state.table_rotation.1 - (dx as f32) / rotate_factor;
                    }
                }
                TableTool::Pen => {
                    if state.table_grabbed.0 {
                        state.table.draw_line(&[x - dx, y - dy], &[x, y]);
                    }
                }
                TableTool::Eracer => {
                    if state.table_grabbed.0 {
                        state.table.erace_line(&[x - dx, y - dy], &[x, y]);
                    }
                }
                _ => {}
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
        Msg::SetTableGrabbed(grabbed_l, grabbed_r) => {
            state.table_grabbed = (grabbed_l, grabbed_r);
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
                        Msg::SetTableGrabbed(l, r)
                    })
                    .on_mouseup(move |e| {
                        let l = e.buttons() & 1 != 0 && table_grabbed_r;
                        let r = e.buttons() & 2 != 0 && table_grabbed_l;
                        Msg::SetTableGrabbed(l, r)
                    })
                    .on_mouseleave(|_| Msg::SetTableGrabbed(false, false))
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
            btn::primary(Attributes::new(), Events::new(), vec![Html::text("資料")]),
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
                        Events::new().on_click(|_| Msg::SetTableTool(TableTool::Pointer)),
                        "toolbox",
                        "ポインター",
                        false,
                    ),
                ],
            ),
        ],
    )
}
