use super::btn;
use super::chat;
use super::context_menu;
use super::handout;
use super::measure_length::measure_length;
use super::measure_tool;
use super::radio::radio;
use crate::random_id;
use crate::table::Table;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub enum FormKind {
    Chat,
    Handout,
    MeasureTool,
}

pub enum TableTool {
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
    table_rotating: bool,
    table_tool: TableTool,
    table_measure_start: Option<[f32; 2]>,
    table_measure: Option<([f32; 2], [f32; 2], f32)>,
    measure_tool_state: measure_tool::State,
    handout_state: handout::State,
    chat_state: chat::State,
    table_context_menu_state: context_menu::State,
    form_priority: [FormKind; 3],
}

pub enum Msg {
    NoOp,
    SetTableContext(web_sys::HtmlCanvasElement),
    ResizeTable,
    MoveTableCamera((i32, i32), (i32, i32)),
    ZoomTableCamera(f64),
    SetTableGrabbed((bool, bool), (i32, i32)),
    SetTableTool(TableTool),
    OpenHandoutForm,
    MeasureToolMsg(measure_tool::Msg),
    HandoutMsg(handout::Msg),
    ChatMsg(chat::Msg),
    OpenChatForm,
    SetTopForm(usize),
    ShowTableContextMenu([f32; 2]),
    TableContextMenuMsg(context_menu::Msg),
}

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render).batch(|mut handler| {
        let a = Closure::wrap(Box::new(move || {
            handler(Msg::ResizeTable);
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .set_onresize(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

fn init() -> (State, Cmd<Msg, Sub>) {
    let state = State {
        room_name: String::from("無名の部屋@") + &random_id::hex(16),
        table: Table::new(),
        table_height: 100,
        table_width: 100,
        table_rotation: (-0.25 * std::f32::consts::PI, 0.0625 * std::f32::consts::PI),
        table_movement: (0.0, 0.0),
        table_distance: 20.0,
        table_grabbed: (false, false),
        table_rotating: false,
        table_tool: TableTool::Selecter,
        table_measure_start: None,
        table_measure: None,
        table_context_menu_state: context_menu::init(),
        measure_tool_state: measure_tool::init(),
        handout_state: handout::init(),
        chat_state: chat::init(),
        form_priority: [FormKind::Chat, FormKind::Handout, FormKind::MeasureTool],
    };
    let task = Cmd::task(|handler| {
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
    });
    (state, task)
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
        Msg::ChatMsg(m) => {
            chat::update(&mut state.chat_state, m);
            Cmd::none()
        }
        Msg::OpenHandoutForm => {
            handout::toggle_open_close(&mut state.handout_state);
            Cmd::none()
        }
        Msg::OpenChatForm => {
            chat::toggle_open_close(&mut state.chat_state);
            Cmd::none()
        }
        Msg::SetTopForm(idx) => {
            let mut d = 0;
            let len = state.form_priority.len();
            let mut fp = [FormKind::Chat, FormKind::Chat, FormKind::Chat];
            for i in 0..len {
                if i == idx {
                    fp[len - 1] = state.form_priority[i].clone();
                    d = 1;
                } else {
                    fp[i - d] = state.form_priority[i].clone();
                }
            }
            state.form_priority = fp;
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
            chat::window_resized(&mut state.chat_state);
            handout::window_resized(&mut state.handout_state);
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
                state.table_rotating = true;
            } else {
                state.table_rotating = false;
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
            state.table_distance = (state.table_distance + (wheel / 20.0) as f32)
                .max(1.0)
                .min(100.0);
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
            match &state.table_tool {
                TableTool::Measure => {
                    measure_tool::open(&mut state.measure_tool_state);
                }
                _ => {
                    measure_tool::close(&mut state.measure_tool_state);
                }
            }
            Cmd::none()
        }
        Msg::ShowTableContextMenu(p) => {
            if !state.table_rotating {
                context_menu::open(&mut state.table_context_menu_state, p);
            }
            Cmd::none()
        }
        Msg::TableContextMenuMsg(m) => {
            context_menu::update(&mut state.table_context_menu_state, m);
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
    let some_form_is_moving = chat::is_moving(&state.chat_state)
        || handout::is_moving(&state.handout_state)
        || measure_tool::is_moving(&state.measure_tool_state);
    let mut children = vec![
        render_table_canvas(table_grabbed_r, table_grabbed_l),
        render_side_menu(),
        render_header(&state.room_name),
        render_table_context_menu(&state.table_context_menu_state),
        match state.table_measure {
            Some((b, e, l)) => measure_length(&b, &e, l),
            _ => Html::none(),
        },
    ];
    for i in 0..state.form_priority.len() {
        let ii = i;
        children.push(match &state.form_priority[i] {
            FormKind::Chat => chat::render(
                &state.chat_state,
                || Box::new(|| Box::new(|msg| Msg::ChatMsg(msg))),
                Attributes::new(),
                Events::new().on_click(move |_| Msg::SetTopForm(ii)),
            ),
            FormKind::Handout => handout::render(
                &state.handout_state,
                || Box::new(|| Box::new(|msg| Msg::HandoutMsg(msg))),
                Attributes::new(),
                Events::new().on_click(move |_| Msg::SetTopForm(ii)),
            ),
            FormKind::MeasureTool => measure_tool::render(
                &state.measure_tool_state,
                || Box::new(|| Box::new(|msg| Msg::MeasureToolMsg(msg))),
                Attributes::new(),
                Events::new().on_click(move |_| Msg::SetTopForm(ii)),
            ),
        })
    }
    Html::div(
        Attributes::new().id("app").string(
            "data-app-some_form_is_moving",
            some_form_is_moving.to_string(),
        ),
        Events::new(),
        children,
    )
}

fn render_table_canvas(table_grabbed_l: bool, table_grabbed_r: bool) -> Html<Msg> {
    Html::canvas(
        Attributes::new().id("table"),
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
                Msg::ShowTableContextMenu([e.client_x() as f32, e.client_y() as f32])
            }),
        vec![],
    )
}

fn render_table_context_menu(state: &context_menu::State) -> Html<Msg> {
    context_menu::render(
        false,
        state,
        || Box::new(|| Box::new(|msg| Msg::TableContextMenuMsg(msg))),
        Attributes::new(),
        Events::new(),
        vec![btn::context_menu_text(
            Attributes::new(),
            Events::new(),
            "キャラクターを作成",
        )],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().id("app-sidemenu"),
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
                Events::new().on_click(|_| Msg::OpenHandoutForm),
                vec![Html::text("資料")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::OpenChatForm),
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
