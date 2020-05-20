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
use crate::model::Tablemask;
use crate::model::World;
use crate::random_id;
use crate::renderer::Renderer;
use crate::skyway;
use crate::skyway::ReceiveData;
use crate::skyway::Room;
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
    Selector,
    Pen,
    Eracer,
    Measure(Option<[f64; 2]>),
}

impl TableTool {
    fn is_selector(&self) -> bool {
        match self {
            Self::Selector => true,
            _ => false,
        }
    }
    fn is_pen(&self) -> bool {
        match self {
            Self::Pen => true,
            _ => false,
        }
    }
    fn is_eracer(&self) -> bool {
        match self {
            Self::Eracer => true,
            _ => false,
        }
    }
    fn is_measure(&self) -> bool {
        match self {
            Self::Measure(_) => true,
            _ => false,
        }
    }
}

struct TableState {
    selecting_tool: TableTool,
    measure_length: Option<f64>,
    last_mouse_coord: [f64; 2],
}

struct Contextmenu {
    state: contextmenu::State,
    grobal_position: [f64; 2],
    canvas_position: [f64; 2],
}

pub struct State {
    room: Rc<Room>,
    world: World,
    camera: Camera,
    renderer: Option<Renderer>,
    canvas_size: [f64; 2],
    table_state: TableState,
    contextmenu: Contextmenu,
    focused_object_id: Option<u128>,
    is_2d_mode: bool,
    // form_state: FormState,
    debug__character_id: Option<u128>,
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
    OpenContextMenu([f64; 2], [f64; 2]),
    AddChracaterWithMouseCoord([f64; 2]),
    AddTablemaskWithMouseCoord([f64; 2]),
    CloneObjectWithObjectId(u128),

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
    SetObjectPositionWithMouseCoord(u128, [f64; 2]),
    BindObjectToTableGrid(u128),
    SetIs2dMode(bool),

    // Worldに対する操作
    LoadCharacterImageFromFile(u128, web_sys::File),
    LoadCharacterImageFromDataUrl(u128, String, bool),
    SetCharacterImage(u128, web_sys::HtmlImageElement),
    AddChracater(Character),
    AddTablemask(Tablemask),

    // 接続に関する操作
    ReceiveMsg(skyway::Msg),
    DisconnectFromRoom,

    //デバッグ用
    Debug_SetSelectingCharacterId(u128),
}

pub enum Sub {
    DisconnectFromRoom,
}

pub fn new(room: Rc<Room>) -> Component<Msg, State, Sub> {
    Component::new(init(Rc::clone(&room)), update, render)
        .batch(|mut handler| {
            let a = Closure::wrap(Box::new(move || {
                handler(Msg::WindowResized);
            }) as Box<dyn FnMut()>);
            web_sys::window()
                .unwrap()
                .set_onresize(Some(a.as_ref().unchecked_ref()));
            a.forget();
        })
        .batch({
            let room = Rc::clone(&room);
            move |mut handler| {
                let a = Closure::wrap(Box::new({
                    move |receive_data: ReceiveData| {
                        if let Ok(msg) = serde_json::from_str::<skyway::Msg>(&receive_data.data()) {
                            handler(Msg::ReceiveMsg(msg));
                        } else {
                            web_sys::console::log_1(&JsValue::from("faild to deserialize message"));
                        }
                    }
                }) as Box<dyn FnMut(ReceiveData)>);
                room.payload.on("data", Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        })
}

fn init(room: Rc<Room>) -> impl FnOnce() -> (State, Cmd<Msg, Sub>) {
    || {
        let state = State {
            room: room,
            world: World::new([20.0, 20.0]),
            camera: Camera::new(),
            renderer: None,
            canvas_size: [0.0, 0.0],
            table_state: TableState {
                selecting_tool: TableTool::Selector,
                measure_length: None,
                last_mouse_coord: [0.0, 0.0],
            },
            contextmenu: Contextmenu {
                state: contextmenu::init(),
                canvas_position: [0.0, 0.0],
                grobal_position: [0.0, 0.0],
            },
            // form_state: FormState {
            //     chat: chat::init(),
            //     handout: handout::init(),
            // },
            is_2d_mode: false,
            focused_object_id: None,
            debug__character_id: None,
        };
        let task = Cmd::task(|handler| {
            handler(Msg::SetTableContext);
        });
        (state, task)
    }
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

fn get_table_position(state: &State, mouse_coord: &[f64; 2]) -> [f64; 2] {
    let dpr = get_device_pixel_ratio();
    let mouse_coord = [mouse_coord[0] * dpr, mouse_coord[1] * dpr];
    let p = state
        .camera
        .collision_point_on_xy_plane(&state.canvas_size, &mouse_coord);
    [p[0], p[1]]
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetTableContext => {
            let canvas = get_table_canvas_element();
            let dpr = get_device_pixel_ratio();
            let canvas_size = [
                canvas.client_width() as f64 * dpr,
                canvas.client_height() as f64 * dpr,
            ];
            canvas.set_width(canvas_size[0] as u32);
            canvas.set_height(canvas_size[1] as u32);
            state.canvas_size = canvas_size;
            let gl = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            let mut renderer = Renderer::new(gl);

            renderer.render(&mut state.world, &state.camera);
            state.renderer = Some(renderer);
            Cmd::none()
        }
        Msg::WindowResized => {
            let canvas = get_table_canvas_element();
            let dpr = get_device_pixel_ratio();
            let canvas_size = [
                canvas.client_width() as f64 * dpr,
                canvas.client_height() as f64 * dpr,
            ];
            canvas.set_width(canvas_size[0] as u32);
            canvas.set_height(canvas_size[1] as u32);
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
        Msg::OpenContextMenu(page_mouse_coord, offset_mouse_coord) => {
            state.contextmenu.grobal_position = page_mouse_coord.clone();
            state.contextmenu.canvas_position = offset_mouse_coord;
            contextmenu::open(&mut state.contextmenu.state, page_mouse_coord);
            Cmd::none()
        }
        Msg::AddChracaterWithMouseCoord(mouse_coord) => {
            let position = get_table_position(&state, &mouse_coord);
            let position = [position[0], position[1], 0.0];
            let mut character = Character::new();
            character.set_position(position);
            if state.world.table().is_bind_to_grid() {
                character.bind_to_grid();
            }
            update(state, Msg::AddChracater(character))
        }
        Msg::AddTablemaskWithMouseCoord(mouse_coord) => {
            let position = get_table_position(&state, &mouse_coord);
            let position = [position[0], position[1], 0.0];
            let mut tablemask = Tablemask::new();
            tablemask.set_position(position);
            if state.world.table().is_bind_to_grid() {
                tablemask.bind_to_grid();
            }
            update(state, Msg::AddTablemask(tablemask))
        }
        Msg::CloneObjectWithObjectId(object_id) => {
            if let Some(character) = state.world.character(&object_id) {
                let mut character = character.clone();
                let p = character.position().clone();
                character.set_position([p[0] + 1.0, p[1] + 1.0, p[2]]);
                update(state, Msg::AddChracater(character))
            } else if let Some(tablemask) = state.world.tablemask(&object_id) {
                let mut tablemask = tablemask.clone();
                let p = tablemask.position().clone();
                tablemask.set_position([p[0] + 1.0, p[1] + 1.0, p[2]]);
                update(state, Msg::AddTablemask(tablemask))
            } else {
                Cmd::none()
            }
        }

        //テーブル操作の制御
        Msg::SetCursorWithMouseCoord(mouse_coord) => {
            let camera = &state.camera;
            let table_coord = get_table_position(&state, &mouse_coord);
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
                let dpr = get_device_pixel_ratio();
                let focused_id = renderer.table_object_id(&[
                    mouse_coord[0] * dpr,
                    state.canvas_size[1] - mouse_coord[1] * dpr,
                ]);
                if let Some(character) = state.world.character_mut(&focused_id) {
                    character.set_is_focused(true);
                    state.focused_object_id = Some(focused_id);
                } else if let Some(_) = state.world.tablemask(&focused_id) {
                    state.focused_object_id = Some(focused_id);
                } else {
                    state.focused_object_id = None;
                }
            }
            Cmd::none()
        }
        Msg::SetCameraRotationWithMouseCoord(mouse_coord) => {
            let x_movement = mouse_coord[0] - state.table_state.last_mouse_coord[0];
            let y_movement = mouse_coord[1] - state.table_state.last_mouse_coord[1];
            let long_edge = state.canvas_size[0].max(state.canvas_size[1]);
            let rotation_factor = 3.0 / long_edge * get_device_pixel_ratio();
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
            let movement_factor = 50.0 / long_edge * get_device_pixel_ratio();
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
            let movement_factor = 0.02;
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
            match &table_tool {
                TableTool::Measure(Option::None) => {
                    state.table_state.measure_length = None;
                    state.world.table_mut().clear_measure();
                }
                _ => {}
            }
            state.table_state.selecting_tool = table_tool;
            update(state, Msg::Render)
        }
        Msg::SetIsBindToGrid(is_bind_to_grid) => {
            state.world.table_mut().set_is_bind_to_grid(is_bind_to_grid);
            state
                .room
                .send(&skyway::Msg::SetIsBindToGrid(is_bind_to_grid));
            Cmd::none()
        }
        Msg::DrawLineWithMouseCoord(mouse_coord) => {
            let start_point = get_table_position(&state, &state.table_state.last_mouse_coord);
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord);
            let end_point = [end_point[0], end_point[1]];
            state.world.table_mut().draw_line(
                &start_point,
                &end_point,
                ColorSystem::gray_900(255),
                0.5,
            );
            state
                .room
                .send(&skyway::Msg::DrawLineToTable(start_point, end_point));
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::EraceLineWithMouseCoord(mouse_coord) => {
            let start_point = get_table_position(&state, &state.table_state.last_mouse_coord);
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord);
            let end_point = [end_point[0], end_point[1]];
            state
                .world
                .table_mut()
                .erace_line(&start_point, &end_point, 1.0);
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::SetMeasureStartPointAndEndPointWithMouseCoord(start_point, mouse_coord) => {
            let start_point = get_table_position(&state, &start_point);
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord);
            let end_point = [end_point[0], end_point[1]];
            let measure_length = state.world.table_mut().draw_measure(
                &start_point,
                &end_point,
                ColorSystem::red_500(255),
                0.2,
            );
            state.table_state.measure_length = Some(measure_length);
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::SetObjectPositionWithMouseCoord(object_id, mouse_coord) => {
            let movement = {
                let a = get_table_position(&state, &state.table_state.last_mouse_coord);
                let b = get_table_position(&state, &mouse_coord);
                [b[0] - a[0], b[1] - a[1]]
            };
            if let Some(character) = state.world.character_mut(&object_id) {
                let p = character.position();
                let p = [p[0] + movement[0], p[1] + movement[1], p[2]];
                state
                    .room
                    .send(&skyway::Msg::SetObjectPosition(object_id, p.clone()));
                character.set_position(p);
            }
            if let Some(tablemask) = state.world.tablemask_mut(&object_id) {
                let p = tablemask.position();
                let p = [p[0] + movement[0], p[1] + movement[1], p[2]];
                state
                    .room
                    .send(&skyway::Msg::SetObjectPosition(object_id, p.clone()));
                tablemask.set_position(p);
            }
            state.table_state.last_mouse_coord = mouse_coord;
            update(state, Msg::Render)
        }
        Msg::BindObjectToTableGrid(object_id) => {
            if state.world.table().is_bind_to_grid() {
                if let Some(character) = state.world.character_mut(&object_id) {
                    character.bind_to_grid();
                    state.room.send(&skyway::Msg::SetObjectPosition(
                        object_id,
                        character.position().clone(),
                    ));
                }
                if let Some(tablemask) = state.world.tablemask_mut(&object_id) {
                    tablemask.bind_to_grid();
                    state.room.send(&skyway::Msg::SetObjectPosition(
                        object_id,
                        tablemask.position().clone(),
                    ));
                }
            }
            update(state, Msg::Render)
        }
        Msg::SetIs2dMode(is_2d_mode) => {
            if is_2d_mode {
                state.camera.set_x_axis_rotation(0.0);
                state.camera.set_z_axis_rotation(0.0);
            }
            state.is_2d_mode = is_2d_mode;
            update(state, Msg::Render)
        }

        // Worldに対する操作
        Msg::LoadCharacterImageFromFile(character_id, file) => Cmd::task(move |resolver| {
            let file_reader = Rc::new(web_sys::FileReader::new().unwrap());
            let file_reader_ = Rc::clone(&file_reader);
            let on_load = Closure::once(Box::new(move || {
                if let Ok(result) = file_reader_.result() {
                    if let Some(data_url) = result.as_string() {
                        resolver(Msg::LoadCharacterImageFromDataUrl(
                            character_id,
                            data_url,
                            true,
                        ));
                        return;
                    }
                }
                resolver(Msg::NoOp);
            }) as Box<dyn FnOnce()>);
            file_reader.set_onload(Some(&on_load.as_ref().unchecked_ref()));
            file_reader.read_as_data_url(&file).unwrap();
            on_load.forget();
        }),
        Msg::LoadCharacterImageFromDataUrl(character_id, data_url, t) => Cmd::task({
            let room = state.room.clone();
            move |resolver| {
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
                if t {
                    room.send(&skyway::Msg::SetCharacterImage(character_id, data_url));
                }
            }
        }),
        Msg::SetCharacterImage(character_id, image) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.set_image(image);
                character.stretch_height();
                update(state, Msg::Render)
            } else {
                Cmd::none()
            }
        }
        Msg::AddChracater(character) => {
            let position = character.position().clone();
            let character_id = state.world.add_character(character);

            state
                .room
                .send(&skyway::Msg::CreateCharacterToTable(character_id, position));
            update(state, Msg::Render)
        }
        Msg::AddTablemask(tablemask) => {
            state.world.add_tablemask(tablemask);
            update(state, Msg::Render)
        }

        // 接続に関する操作
        Msg::ReceiveMsg(msg) => match msg {
            skyway::Msg::CreateCharacterToTable(character_id, position) => {
                let mut character = Character::new();
                character.set_position(position);
                state.world.add_character_with_id(character_id, character);
                update(state, Msg::Render)
            }
            skyway::Msg::DrawLineToTable(start_point, end_point) => {
                state.world.table_mut().draw_line(
                    &start_point,
                    &end_point,
                    ColorSystem::gray_900(255),
                    0.5,
                );
                update(state, Msg::Render)
            }
            skyway::Msg::SetCharacterImage(character_id, data_url) => update(
                state,
                Msg::LoadCharacterImageFromDataUrl(character_id, data_url, false),
            ),
            skyway::Msg::SetObjectPosition(object_id, position) => {
                if let Some(character) = state.world.character_mut(&object_id) {
                    character.set_position(position);
                    update(state, Msg::Render)
                } else if let Some(tablemask) = state.world.tablemask_mut(&object_id) {
                    tablemask.set_position(position);
                    update(state, Msg::Render)
                } else {
                    Cmd::none()
                }
            }
            skyway::Msg::SetIsBindToGrid(is_bind_to_grid) => {
                state.world.table_mut().set_is_bind_to_grid(is_bind_to_grid);
                update(state, Msg::Render)
            }
        },
        Msg::DisconnectFromRoom => Cmd::Sub(Sub::DisconnectFromRoom),

        //デバッグ用
        Msg::Debug_SetSelectingCharacterId(character_id) => {
            state.debug__character_id = Some(character_id);
            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .id("app")
            .class("fullscreen")
            .style("display", "grid")
            .style("grid-template-rows", "max-content 1fr"),
        Events::new(),
        vec![
            render_header_menu(
                &state.room.id,
                &state.table_state.selecting_tool,
                state.world.table().is_bind_to_grid(),
                state.is_2d_mode,
            ),
            render_canvas_container(&state),
            render_context_menu(&state.contextmenu, &state.focused_object_id),
        ],
    )
}

fn render_canvas_container(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .style("position", "relative"),
        Events::new(),
        vec![
            render_canvas(
                &state.table_state,
                &state.focused_object_id,
                state.is_2d_mode,
            ),
            render_measure_length(&state.table_state.measure_length),
            render_hint(),
        ],
    )
}

fn render_canvas(
    table_state: &TableState,
    focused_object_id: &Option<u128>,
    is_2d_mode: bool,
) -> Html<Msg> {
    Html::canvas(
        Attributes::new().id("table").class("cover"),
        Events::new()
            .on_mousemove({
                let selecting_tool = table_state.selecting_tool.clone();
                let focused_object_id = focused_object_id.clone();
                move |e| {
                    let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                    if e.buttons() & 1 == 0 {
                        Msg::SetCursorWithMouseCoord(mouse_coord)
                    } else if (e.alt_key() || e.ctrl_key()) && !is_2d_mode {
                        Msg::SetCameraRotationWithMouseCoord(mouse_coord)
                    } else {
                        match selecting_tool {
                            TableTool::Selector => match focused_object_id {
                                Some(character_id) => {
                                    Msg::SetObjectPositionWithMouseCoord(character_id, mouse_coord)
                                }
                                None => Msg::SetCameraMovementWithMouseCoord(mouse_coord),
                            },
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
                        let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                        Msg::SetSelectingTableTool(TableTool::Measure(Some(mouse_coord)))
                    }
                    _ => Msg::NoOp,
                }
            })
            .on_mouseup({
                let selecting_tool = table_state.selecting_tool.clone();
                let focused_object_id = focused_object_id.clone();
                move |_| match selecting_tool {
                    TableTool::Selector => match focused_object_id {
                        Some(object_id) => Msg::BindObjectToTableGrid(object_id),
                        None => Msg::NoOp,
                    },
                    TableTool::Measure(_) => Msg::SetSelectingTableTool(TableTool::Measure(None)),
                    _ => Msg::NoOp,
                }
            })
            .on_contextmenu(|e| {
                let page_mouse_coord = [e.page_x() as f64, e.page_y() as f64];
                let offset_mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                e.prevent_default();
                e.stop_propagation();
                Msg::OpenContextMenu(page_mouse_coord, offset_mouse_coord)
            }),
        vec![render_hint()],
    )
}

fn render_context_menu(contextmenu: &Contextmenu, focused_object_id: &Option<u128>) -> Html<Msg> {
    if let Some(character_id) = focused_object_id {
        render_context_menu_object(contextmenu, *character_id)
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
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let position = contextmenu.canvas_position.clone();
                        move |_| Msg::AddChracaterWithMouseCoord(position)
                    }),
                    "キャラクターを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let position = contextmenu.canvas_position.clone();
                        move |_| Msg::AddTablemaskWithMouseCoord(position)
                    }),
                    "マップマスクを作成",
                ),
            ],
        )],
    )
}

fn render_context_menu_object(contextmenu: &Contextmenu, object_id: u128) -> Html<Msg> {
    contextmenu::render(
        false,
        &contextmenu.state,
        || Box::new(|| Box::new(|msg| Msg::TransportContextMenuMsg(msg))),
        Attributes::new(),
        Events::new(),
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![
                btn::contextmenu_text(Attributes::new(), Events::new(), "編集"),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::CloneObjectWithObjectId(object_id)),
                    "コピーを作成",
                ),
            ],
        )],
    )
}

fn render_header_menu(
    room_id: &String,
    selecting_tool: &TableTool,
    is_bind_to_grid: bool,
    is_2d_mode: bool,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("panel grid"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("grid-w-6 keyvalue pure-form"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new().string("for", "roomid"),
                        Events::new(),
                        vec![Html::text("ルームID")],
                    ),
                    Html::input(
                        Attributes::new()
                            .value(room_id)
                            .id("roomid")
                            .flag("readonly"),
                        Events::new(),
                        vec![],
                    ),
                ],
            ),
            Html::div(Attributes::new().class("grid-w-15"), Events::new(), vec![]),
            Html::div(
                Attributes::new().class("grid-w-3 justify-r"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![btn::danger(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::DisconnectFromRoom),
                        vec![Html::text("ルームから出る")],
                    )],
                )],
            ),
            Html::div(
                Attributes::new().class("grid-w-12 linear-h container-a"),
                Events::new(),
                vec![
                    btn::selectable(
                        selecting_tool.is_selector(),
                        Attributes::new()
                            .class("fas fa-mouse-pointer")
                            .title("選択"),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Selector)),
                        vec![],
                    ),
                    btn::selectable(
                        selecting_tool.is_pen(),
                        Attributes::new().class("fas fa-pen").title("ペン"),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Pen)),
                        vec![],
                    ),
                    btn::selectable(
                        selecting_tool.is_eracer(),
                        Attributes::new().class("fas fa-eraser").title("消しゴム"),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Eracer)),
                        vec![],
                    ),
                    btn::selectable(
                        selecting_tool.is_measure(),
                        Attributes::new().class("fas fa-ruler").title("計測"),
                        Events::new()
                            .on_click(|_| Msg::SetSelectingTableTool(TableTool::Measure(None))),
                        vec![],
                    ),
                    Html::div(
                        Attributes::new().class("keyvalue").title(""),
                        Events::new(),
                        vec![
                            Html::span(
                                Attributes::new().class("text-label"),
                                Events::new(),
                                vec![Html::text("グリッドにスナップ")],
                            ),
                            btn::toggle(
                                is_bind_to_grid,
                                Attributes::new(),
                                Events::new()
                                    .on_click(move |_| Msg::SetIsBindToGrid(!is_bind_to_grid)),
                            ),
                        ],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("grid-w-12 justify-r"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![
                        Html::span(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("2Dモード")],
                        ),
                        btn::toggle(
                            is_2d_mode,
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::SetIs2dMode(!is_2d_mode)),
                        ),
                    ],
                )],
            ),
        ],
    )
}

fn render_hint() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("text-color-secondary-d")
            .style("position", "absolute")
            .style("bottom", "5em")
            .style("right", "5em"),
        Events::new(),
        vec![Html::text("Ctrl + ドラッグで視界を回転")],
    )
}

fn render_measure_length(measure_length: &Option<f64>) -> Html<Msg> {
    if let Some(measure_length) = measure_length {
        Html::div(
            Attributes::new()
                .style("position", "absolute")
                .style("top", "5em")
                .style("right", "5em"),
            Events::new(),
            vec![Html::text(format!("計測結果：{:.1}", measure_length))],
        )
    } else {
        Html::none()
    }
}
