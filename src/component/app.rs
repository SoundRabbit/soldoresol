use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::table::Table;

pub struct State {
    table: Table,
    table_height: i32,
    table_width: i32,
    table_rotation: (f32, f32),
    table_movement: (f32, f32),
    table_distance: f32,
    table_grabbed: (bool, bool),
}

pub enum Msg {
    NoOp,
    SetTableSize(web_sys::HtmlCanvasElement),
    SetTableContext(web_sys::WebGlRenderingContext),
    MoveTableCamera(i32, i32),
    SetTableGrabbed(bool, bool),
}

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render).batch(|mut handler| {
        let a = Closure::wrap(Box::new(move || {
            handler(Msg::SetTableSize(
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
        table: Table::new(),
        table_height: 100,
        table_width: 100,
        table_rotation: (0.0, 0.0),
        table_movement: (0.0, 0.0),
        table_distance: 40.0,
        table_grabbed: (false, false),
    };
    (state, Cmd::none())
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match (msg) {
        Msg::NoOp => Cmd::none(),
        Msg::SetTableSize(canvas) => {
            state.table_height = canvas.client_height();
            state.table_width = canvas.client_width();
            Cmd::task(move |resolve| {
                let context = canvas
                    .get_context("webgl")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::WebGlRenderingContext>()
                    .unwrap();
                resolve(Msg::SetTableContext(context));
            })
        }
        Msg::SetTableContext(context) => {
            state.table.set_context(context);
            state.table.resize();
            render_table(
                &mut state.table,
                state.table_movement,
                state.table_rotation,
                state.table_distance,
            );
            Cmd::none()
        }
        Msg::MoveTableCamera(dx, dy) => {
            let dx = dx as f32;
            let dy = dy as f32;
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
        Msg::SetTableGrabbed(grabbed_l, grabbed_r) => {
            state.table_grabbed = (grabbed_l, grabbed_r);
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
                    .on_mousemove(|e| Msg::MoveTableCamera(e.movement_x(), e.movement_y()))
                    .on_contextmenu(|e| {
                        e.prevent_default();
                        Msg::NoOp
                    }),
                vec![],
            ),
            render_side_menu(),
        ],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().id("app-side-menu"),
        Events::new(),
        vec![
            render_side_menu_item("テーブル"),
            render_side_menu_item("画像"),
            render_side_menu_item("音楽"),
            render_side_menu_item("キャラクター"),
            render_side_menu_item("オブジェクト"),
            render_side_menu_item("資料"),
            render_side_menu_item("チャット"),
        ],
    )
}

fn render_side_menu_item(text: impl Into<String>) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app-side-menu-item"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("app-side-menu-item-content-wrapper"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("app-side-menu-item-text"),
                    Events::new(),
                    vec![Html::text(text)],
                )],
            ),
            Html::div(
                Attributes::new().class("app-side-menu-item-content-wrapper"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("app-side-menu-item-frame"),
                    Events::new(),
                    vec![],
                )],
            ),
        ],
    )
}
