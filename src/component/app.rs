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
use std::rc::Rc;
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
    character_id: Option<u32>,
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
    debug__character_id: Option<u32>,
}

// enum FormMsg {
//     ChatMsg(chat::Msg),
//     HandoutMsg(handout::Msg),
// }

pub enum Msg {
    NoOp,
    SetTableContext,
    WindowResized,
    Render,

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

    // Worldに対する操作
    LoadCharacterImageFromFile(u32, web_sys::File),
    SetCharacterImage(u32, web_sys::HtmlImageElement),

    //デバッグ用
    Debug_SetSelectingCharacterId(u32),
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
            character_id: None,
        },
        // form_state: FormState {
        //     chat: chat::init(),
        //     handout: handout::init(),
        // },
        debug__character_id: None,
    };
    let task = Cmd::task(|handler| {
        handler(Msg::SetTableContext);
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

fn get_device_pixel_ratio() -> f64 {
    web_sys::window().unwrap().device_pixel_ratio()
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetTableContext => {
            let canvas = get_table_canvas_element();
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
            web_sys::console::log_1(&JsValue::from("Renderer::new"));
            let mut renderer = Renderer::new(gl);
            web_sys::console::log_1(&JsValue::from("renderer.render"));

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
            update(state, Msg::Render)
        }
        Msg::Render => {
            if let Some(renderer) = &mut state.renderer {
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
            state.contextmenu.position = mouse_coord.clone();
            contextmenu::open(&mut state.contextmenu.state, mouse_coord);
            Cmd::none()
        }
        Msg::AddChracaterToTable(mouse_coord) => {
            let camera = &state.camera;
            let table_coord = camera.collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
            let table_coord = [table_coord[0], table_coord[1], 0.0];
            let mut character = Character::new();
            character.set_position(table_coord);
            state.world.add_character(character);
            update(state, Msg::Render)
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
            if let Some(renderer) = &mut state.renderer {
                renderer.render(&mut state.world, &camera);
                let focused_id = renderer
                    .table_object_id(&[mouse_coord[0], state.canvas_size[1] - mouse_coord[1]]);
                if let Some(character) = state.world.character_mut(focused_id) {
                    character.set_is_focused(true);
                    state.contextmenu.character_id = Some(focused_id);
                } else {
                    state.contextmenu.character_id = None;
                }
            }
            Cmd::none()
        }
        Msg::SetCameraRotationWithMouseCoord(mouse_coord) => {
            let x_movement = mouse_coord[0] - state.table_state.last_mouse_coord[0];
            let y_movement = mouse_coord[1] - state.table_state.last_mouse_coord[1];
            let long_edge = state.canvas_size[0].max(state.canvas_size[1]);
            let rotation_factor = 3.0 / long_edge;
            let camera = &mut state.camera;
            camera.set_x_axis_rotation(camera.x_axis_rotation() + y_movement * rotation_factor);
            camera.set_z_axis_rotation(camera.z_axis_rotation() + x_movement * rotation_factor);
            state.table_state.last_mouse_coord = mouse_coord;
            update(state, Msg::Render)
        }
        Msg::SetCameraMovementWithMouseCoord(mouse_coord) => {
            let x_movement = mouse_coord[0] - state.table_state.last_mouse_coord[0];
            let y_movement = mouse_coord[1] - state.table_state.last_mouse_coord[1];
            let long_edge = state.canvas_size[0].max(state.canvas_size[1]);
            let movement_factor = 50.0 / long_edge;
            let camera = &mut state.camera;
            let movement = camera.movement();
            let movement = [
                movement[0] + x_movement * movement_factor,
                movement[1] - y_movement * movement_factor,
                movement[2],
            ];
            camera.set_movement(movement);
            state.table_state.last_mouse_coord = mouse_coord;
            update(state, Msg::Render)
        }
        Msg::SetCameraMovementWithMouseWheel(delta_y) => {
            let camera = &mut state.camera;
            let movement_factor = 0.1;
            let movement = camera.movement();
            let movement = [
                movement[0],
                movement[1],
                movement[2] - movement_factor * delta_y,
            ];
            camera.set_movement(movement);
            update(state, Msg::Render)
        }
        Msg::SetSelectingTableTool(table_tool) => {
            state.table_state.selecting_tool = table_tool;
            update(state, Msg::Render)
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

        // Worldに対する操作
        Msg::LoadCharacterImageFromFile(character_id, file) => Cmd::task(move |resolver| {
            let file_reader = Rc::new(web_sys::FileReader::new().unwrap());
            let file_reader_ = Rc::clone(&file_reader);
            let on_load = Closure::once(Box::new(move || {
                if let Ok(result) = file_reader_.result() {
                    if let Some(data_url) = result.as_string() {
                        let image = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .create_element("img")
                            .unwrap()
                            .dyn_into::<web_sys::HtmlImageElement>()
                            .unwrap();
                        let image_ = image.clone();
                        let on_load = Closure::once(Box::new(move || {
                            resolver(Msg::SetCharacterImage(character_id, image));
                        }));
                        image_.set_onload(Some(&on_load.as_ref().unchecked_ref()));
                        image_.set_src(&data_url);
                        on_load.forget();
                        return;
                    }
                }
                resolver(Msg::NoOp);
            }) as Box<dyn FnOnce()>);
            file_reader.set_onload(Some(&on_load.as_ref().unchecked_ref()));
            file_reader.read_as_data_url(&file).unwrap();
            on_load.forget();
        }),
        Msg::SetCharacterImage(character_id, image) => {
            if let Some(character) = state.world.character_mut(character_id) {
                character.set_image(image);
                character.stretch_height();
                update(state, Msg::Render)
            } else {
                Cmd::none()
            }
        }

        //デバッグ用
        Msg::Debug_SetSelectingCharacterId(character_id) => {
            state.debug__character_id = Some(character_id);
            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app").id("app"),
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
    if let Some(character_id) = contextmenu.character_id {
        render_context_menu_character(contextmenu, character_id)
    } else {
        render_context_menu_default(contextmenu)
    }
}

fn render_context_menu_default(contextmenu: &Contextmenu) -> Html<Msg> {
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

fn render_context_menu_character(contextmenu: &Contextmenu, character_id: u32) -> Html<Msg> {
    contextmenu::render(
        false,
        &contextmenu.state,
        || Box::new(|| Box::new(|msg| Msg::TransportContextMenuMsg(msg))),
        Attributes::new(),
        Events::new(),
        vec![btn::contextmenu_text(
            Attributes::new(),
            Events::new().on_click(move |_| Msg::Debug_SetSelectingCharacterId(character_id)),
            "キャラクターを選択",
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
            render_debug_modeless_character(&state.debug__character_id),
        ],
    )
}

fn render_debug_modeless_character(character_id: &Option<u32>) -> Html<Msg> {
    if let Some(character_id) = character_id {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new(),
                    Events::new(),
                    vec![
                        Html::text("キャラクターID："),
                        Html::text(character_id.to_string()),
                    ],
                ),
                Html::input(
                    Attributes::new().type_("file"),
                    Events::new().on("change", {
                        let character_id = *character_id;
                        move |e| {
                            let files = e
                                .target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .files()
                                .unwrap();
                            if let Some(file) = files.item(0) {
                                Msg::LoadCharacterImageFromFile(character_id, file)
                            } else {
                                Msg::NoOp
                            }
                        }
                    }),
                    vec![],
                ),
            ],
        )
    } else {
        Html::div(Attributes::new(), Events::new(), vec![])
    }
}
