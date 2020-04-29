use super::btn;
// use super::chat;
use super::contextmenu;
// use super::handout;
// use super::measure_length::measure_length;
// use super::radio::radio;
use super::checkbox::checkbox;
use crate::model::Camera;
use crate::model::Character;
use crate::model::ColorSystem;
use crate::model::Table;
use crate::model::World;
use crate::random_id;
use crate::renderer::Renderer;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

enum FormKind {
    Chat,
    Handout,
}

// struct FormState {
//     chat: chat::State,
//     handout: handout::State,
// }

#[derive(Clone)]
pub enum TableTool {
    Selecter,
    Pen,
    Eracer,
    Measure(Option<[f64; 2]>),
}

struct TableState {
    selecting_tool: TableTool,
    last_mouse_coord: [f64; 2],
}

struct Contextmenu {
    state: contextmenu::State,
    position: [f64; 2],
}

pub struct State {
    room_name: String,
    world: World,
    camera: Camera,
    renderer: Option<Renderer>,
    canvas_size: [f64; 2],
    table_state: TableState,
    contextmenu: Contextmenu,
    // form_state: FormState,
}

// enum FormMsg {
//     ChatMsg(chat::Msg),
//     HandoutMsg(handout::Msg),
// }

pub enum Msg {
    NoOp,
    SetTableContext(web_sys::HtmlCanvasElement),
    WindowResized,

    // メッセージの伝搬
    TransportContextMenuMsg(contextmenu::Msg),

    //コンテキストメニューの制御
    OpenContextMenu([f64; 2]),
    AddChracaterToTable([f64; 2]),

    // テーブル操作の制御
    SetCameraRotationWithMouseCoord([f64; 2]),
    SetCameraMovementWithMouseCoord([f64; 2]),
    SetCameraMovementWithMouseWheel(f64),
    SetSelectingTableTool(TableTool),
    SetIsBindToGrid(bool),
    SetCursorWithMouseCoord([f64; 2]),
    DrawLineWithMouseCoord([f64; 2]),
    EraceLineWithMouseCoord([f64; 2]),
    SetMeasureStartPointAndEndPointWithMouseCoord([f64; 2], [f64; 2]),
}

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render).batch(|mut handler| {
        let a = Closure::wrap(Box::new(move || {
            handler(Msg::WindowResized);
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
        world: World::new([20.0, 20.0]),
        camera: Camera::new(),
        renderer: None,
        canvas_size: [0.0, 0.0],
        table_state: TableState {
            selecting_tool: TableTool::Pen,
            last_mouse_coord: [0.0, 0.0],
        },
        contextmenu: Contextmenu {
            state: contextmenu::init(),
            position: [0.0, 0.0],
        },
        // form_state: FormState {
        //     chat: chat::init(),
        //     handout: handout::init(),
        // },
    };
    let task = Cmd::task(|handler| {
        handler(Msg::SetTableContext(get_table_canvas_element()));
    });
    (state, task)
}

fn get_table_canvas_element() -> web_sys::HtmlCanvasElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("table")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetTableContext(canvas) => {
            let canvas_size = [canvas.client_width() as f64, canvas.client_height() as f64];
            canvas.set_height(canvas.client_height() as u32);
            canvas.set_width(canvas.client_width() as u32);
            state.canvas_size = canvas_size;
            let gl = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            let renderer = Renderer::new(gl);
            renderer.render(&mut state.world, &state.camera);
            state.renderer = Some(renderer);
            Cmd::none()
        }
        Msg::WindowResized => {
            let canvas = get_table_canvas_element();
            let canvas_size = [canvas.client_width() as f64, canvas.client_height() as f64];
            canvas.set_height(canvas.client_height() as u32);
            canvas.set_width(canvas.client_width() as u32);
            state.canvas_size = canvas_size;
            // chat::window_resized(&mut state.form_state.chat);
            // handout::window_resized(&mut state.form_state.handout);
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &state.camera);
            }
            Cmd::none()
        }

        //メッセージの伝搬
        Msg::TransportContextMenuMsg(msg) => {
            contextmenu::update(&mut state.contextmenu.state, msg);
            Cmd::none()
        }

        //コンテキストメニューの制御
        Msg::OpenContextMenu(mouse_coord) => {
            contextmenu::open(&mut state.contextmenu.state, mouse_coord);
            Cmd::none()
        }
        Msg::AddChracaterToTable(mouse_coord) => {
            let table_coord = state
                .camera
                .collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
            let table_coord = [table_coord[0], table_coord[1], 0.0];
            let mut character = Character::new();
            character.set_position(table_coord);
            state.world.add_character(character);
            Cmd::none()
        }

        //テーブル操作の制御
        Msg::SetCursorWithMouseCoord(mouse_coord) => {
            let camera = &state.camera;
            let table_coord = camera.collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
            let table_coord = [table_coord[0], table_coord[1]];
            let table = state.world.table_mut();
            match state.table_state.selecting_tool {
                TableTool::Pen => {
                    table.draw_cursor(
                        &table_coord,
                        0.25,
                        ColorSystem::gray_900(255),
                        ColorSystem::gray_900(255),
                    );
                }
                TableTool::Eracer => {
                    table.draw_cursor(
                        &table_coord,
                        0.5,
                        ColorSystem::gray_900(255),
                        ColorSystem::gray_100(255),
                    );
                }
                TableTool::Measure(_) => {
                    table.draw_cursor(
                        &table_coord,
                        0.125,
                        ColorSystem::red_500(255),
                        ColorSystem::red_500(255),
                    );
                }
                _ => {}
            }
            state.table_state.last_mouse_coord = mouse_coord;
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &camera);
            }
            Cmd::none()
        }
        Msg::SetCameraRotationWithMouseCoord(mouse_coord) => {
            let x_movement = mouse_coord[0] - state.table_state.last_mouse_coord[0];
            let y_movement = mouse_coord[1] - state.table_state.last_mouse_coord[1];
            let long_edge = state.canvas_size[0].max(state.canvas_size[1]);
            let rotation_factor = 1.0 / long_edge;
            let camera = &mut state.camera;
            camera.set_x_axis_rotation(camera.x_axis_rotation() + y_movement * rotation_factor);
            camera.set_z_axis_rotation(camera.z_axis_rotation() + x_movement * rotation_factor);
            state.table_state.last_mouse_coord = mouse_coord;
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &camera);
            }
            Cmd::none()
        }
        Msg::SetCameraMovementWithMouseCoord(mouse_coord) => {
            let x_movement = mouse_coord[0] - state.table_state.last_mouse_coord[0];
            let y_movement = mouse_coord[1] - state.table_state.last_mouse_coord[1];
            let long_edge = state.canvas_size[0].max(state.canvas_size[1]);
            let movement_factor = 20.0 / long_edge;
            let camera = &mut state.camera;
            let movement = camera.movement();
            let movement = [
                movement[0] + x_movement * movement_factor,
                movement[1] - y_movement * movement_factor,
                movement[2],
            ];
            camera.set_movement(movement);
            state.table_state.last_mouse_coord = mouse_coord;
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &camera);
            }
            Cmd::none()
        }
        Msg::SetCameraMovementWithMouseWheel(delta_y) => {
            let camera = &mut state.camera;
            let movement_factor = 0.1;
            let movement = camera.movement();
            let movement = [
                movement[0],
                movement[1],
                movement[2] + movement_factor * delta_y,
            ];
            camera.set_movement(movement);
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &camera);
            }
            Cmd::none()
        }
        Msg::SetSelectingTableTool(table_tool) => {
            state.table_state.selecting_tool = table_tool;
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &state.camera);
            }
            Cmd::none()
        }
        Msg::SetIsBindToGrid(is_bind_to_grid) => {
            state.world.table_mut().set_is_bind_to_grid(is_bind_to_grid);
            Cmd::none()
        }
        Msg::DrawLineWithMouseCoord(mouse_coord) => {
            let camera = &state.camera;
            let start_point = camera.collision_point_on_xy_plane(
                &state.canvas_size,
                &state.table_state.last_mouse_coord,
            );
            let start_point = [start_point[0], start_point[1]];
            let end_point = camera.collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
            let end_point = [end_point[0], end_point[1]];
            state.world.table_mut().draw_line(
                &start_point,
                &end_point,
                ColorSystem::gray_900(255),
                0.5,
            );
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::EraceLineWithMouseCoord(mouse_coord) => {
            let camera = &state.camera;
            let start_point = camera.collision_point_on_xy_plane(
                &state.canvas_size,
                &state.table_state.last_mouse_coord,
            );
            let start_point = [start_point[0], start_point[1]];
            let end_point = camera.collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
            let end_point = [end_point[0], end_point[1]];
            state
                .world
                .table_mut()
                .erace_line(&start_point, &end_point, 1.0);
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::SetMeasureStartPointAndEndPointWithMouseCoord(start_point, mouse_coord) => {
            let camera = &state.camera;
            let start_point = camera.collision_point_on_xy_plane(&state.canvas_size, &start_point);
            let start_point = [start_point[0], start_point[1]];
            let end_point = camera.collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
            let end_point = [end_point[0], end_point[1]];
            state.world.table_mut().draw_measure(
                &start_point,
                &end_point,
                ColorSystem::red_500(255),
                0.2,
            );
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app"),
        Events::new(),
        vec![
            render_canvas(&state.table_state),
            render_debug_modeless(&state),
            render_context_menu(&state.contextmenu),
        ],
    )
}

fn render_canvas(table_state: &TableState) -> Html<Msg> {
    Html::canvas(
        Attributes::new().class("app__table").id("table"),
        Events::new()
            .on_mousemove({
                let selecting_tool = table_state.selecting_tool.clone();
                move |e| {
                    let mouse_coord = [e.x() as f64, e.y() as f64];
                    if e.buttons() & 1 == 0 {
                        Msg::SetCursorWithMouseCoord(mouse_coord)
                    } else if e.ctrl_key() {
                        Msg::SetCameraRotationWithMouseCoord(mouse_coord)
                    } else {
                        match selecting_tool {
                            TableTool::Selecter => {
                                Msg::SetCameraMovementWithMouseCoord(mouse_coord)
                            }
                            TableTool::Pen => Msg::DrawLineWithMouseCoord(mouse_coord),
                            TableTool::Eracer => Msg::EraceLineWithMouseCoord(mouse_coord),
                            TableTool::Measure(Some(start_point)) => {
                                Msg::SetMeasureStartPointAndEndPointWithMouseCoord(
                                    start_point,
                                    mouse_coord,
                                )
                            }
                            _ => Msg::SetCursorWithMouseCoord(mouse_coord),
                        }
                    }
                }
            })
            .on("wheel", |e| {
                if let Ok(e) = e.dyn_into::<web_sys::WheelEvent>() {
                    Msg::SetCameraMovementWithMouseWheel(e.delta_y())
                } else {
                    Msg::NoOp
                }
            })
            .on_mousedown({
                let selecting_tool = table_state.selecting_tool.clone();
                move |e| match selecting_tool {
                    TableTool::Measure(_) => {
                        let mouse_coord = [e.x() as f64, e.y() as f64];
                        Msg::SetSelectingTableTool(TableTool::Measure(Some(mouse_coord)))
                    }
                    _ => Msg::NoOp,
                }
            })
            .on_mouseup({
                let selecting_tool = table_state.selecting_tool.clone();
                move |_| match selecting_tool {
                    TableTool::Measure(_) => Msg::SetSelectingTableTool(TableTool::Measure(None)),
                    _ => Msg::NoOp,
                }
            })
            .on_contextmenu(|e| {
                let mouse_coord = [e.x() as f64, e.y() as f64];
                e.prevent_default();
                e.stop_propagation();
                Msg::OpenContextMenu(mouse_coord)
            }),
        vec![],
    )
}

fn render_context_menu(contextmenu: &Contextmenu) -> Html<Msg> {
    contextmenu::render(
        false,
        &contextmenu.state,
        || Box::new(|| Box::new(|msg| Msg::TransportContextMenuMsg(msg))),
        Attributes::new(),
        Events::new(),
        vec![btn::contextmenu_text(
            Attributes::new(),
            Events::new().on_click({
                let position = contextmenu.position.clone();
                move |_| Msg::AddChracaterToTable(position)
            }),
            "キャラクターを作成",
        )],
    )
}

fn render_debug_modeless(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app__debug-modeless"),
        Events::new(),
        vec![
            checkbox(
                Attributes::new(),
                Events::new().on_click({
                    let is_bind_to_grid = state.world.table().is_bind_to_grid();
                    move |_| Msg::SetIsBindToGrid(!is_bind_to_grid)
                }),
                "グリッドにスナップ",
                state.world.table().is_bind_to_grid(),
            ),
            btn::primary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Selecter)),
                vec![Html::text("選択")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Pen)),
                vec![Html::text("ペン")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Eracer)),
                vec![Html::text("消しゴム")],
            ),
            btn::primary(
                Attributes::new(),
                Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Measure(None))),
                vec![Html::text("計測")],
            ),
        ],
    )
}
