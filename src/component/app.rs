// use super::btn;
// use super::chat;
// use super::context_menu;
// use super::handout;
// use super::measure_length::measure_length;
// use super::radio::radio;
use crate::model::Camera;
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
enum TableTool {
    Selecter,
    Pen,
    Eracer,
    Measure,
}

struct TableState {
    selecting_tool: TableTool,
    last_mouse_coord: [f64; 2],
}

pub struct State {
    room_name: String,
    world: World,
    camera: Camera,
    renderer: Option<Renderer>,
    canvas_size: [f64; 2],
    table_state: TableState,
    // context_menu_state: context_menu::State,
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

    // テーブル操作の制御
    SetMouseCoord([f64; 2]),
    SetCameraRotationWithMouseCoord([f64; 2]),
    SetCameraMovementWithMouseCoord([f64; 2]),
    SetCameraMovementWithMouseWheel(f64),
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
            selecting_tool: TableTool::Selecter,
            last_mouse_coord: [0.0, 0.0],
        },
        // context_menu_state: context_menu::init(),
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

        //テーブル操作の制御
        Msg::SetMouseCoord(mouse_coord) => {
            state.table_state.last_mouse_coord = mouse_coord;
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
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app"),
        Events::new(),
        vec![render_canvas(&state.table_state)],
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
                        Msg::SetMouseCoord(mouse_coord)
                    } else if e.ctrl_key() {
                        Msg::SetCameraRotationWithMouseCoord(mouse_coord)
                    } else {
                        match selecting_tool {
                            TableTool::Selecter => {
                                Msg::SetCameraMovementWithMouseCoord(mouse_coord)
                            }
                            _ => Msg::NoOp,
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
            }),
        vec![],
    )
}
