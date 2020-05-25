use super::{btn, contextmenu, modal, modeless, modeless_modal};
use crate::{
    model::{
        resource::{Data, DataString},
        Camera, Character, ColorSystem, Resource, Tablemask, World,
    },
    random_id,
    renderer::Renderer,
    skyway::{self, Peer, ReceiveData, Room},
};
use kagura::prelude::*;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

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

struct ModelessState {
    is_showing: bool,
    loc_a: [i32; 2],
    loc_b: [i32; 2],
}

impl ModelessState {
    pub fn new(is_showing: bool) -> Self {
        Self {
            is_showing,
            loc_a: [20, 1],
            loc_b: [25, 15],
        }
    }
}

enum Modeless {
    Object { tabs: Vec<u128>, focused: usize },
}

type ModelessCollection = Vec<(ModelessState, Modeless)>;

pub enum Modal {
    Resource,
    SelectCharacterImage(u128),
}

struct CmdQueue<M, S> {
    payload: VecDeque<Cmd<M, S>>,
}

impl<M, S> CmdQueue<M, S> {
    fn new() -> Self {
        Self {
            payload: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, cmd: Cmd<M, S>) {
        self.payload.push_back(cmd);
    }

    fn dequeue(&mut self) -> Cmd<M, S> {
        self.payload.pop_front().unwrap_or(Cmd::none())
    }
}

pub struct State {
    _peer: Rc<Peer>,
    peers: HashSet<String>,
    room: Rc<Room>,
    world: World,
    resource: Resource,
    camera: Camera,
    renderer: Option<Renderer>,
    canvas_size: [f64; 2],
    table_state: TableState,
    contextmenu: Contextmenu,
    focused_object_id: Option<u128>,
    is_2d_mode: bool,
    modelesses: ModelessCollection,
    modals: Vec<Modal>,
    editing_modeless: Option<(usize, Rc<RefCell<modeless_modal::State>>)>,
    object_modeless_address: HashMap<u128, [usize; 2]>,
    cmd_queue: CmdQueue<Msg, Sub>,
}

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
    RemoveObjectWithObjectId(u128, bool),

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

    // モードレス
    OpenObjectModeless(u128),
    CloseModeless(usize),
    OpenModelessModal(usize),
    CloseModelessModal,
    ReflectModelessModal(modeless_modal::Props),
    CloseModelessModalWithProps(modeless_modal::Props),

    // モーダル
    OpenModal(Modal),
    CloseModal,

    // Worldに対する操作
    SetCharacterImage(u128, u128, bool),
    SetCharacterHp(u128, i64),
    SetCharacterMp(u128, i64),
    AddChracater(Character),
    AddTablemask(Tablemask),
    SetTablemaskSize(u128, [f64; 2]),
    SetTablemaskSizeIsBinded(u128, bool),

    // リソース管理
    LoadFromFileList(web_sys::FileList),
    LoadFromDataUrls(HashMap<u128, DataString>, bool),
    LoadReasources(HashMap<u128, Rc<web_sys::HtmlImageElement>>),

    // 接続に関する操作
    ReceiveMsg(skyway::Msg),
    ReceiveLog(skyway::LogList),
    DeelMsgListWithNonCmd(Vec<skyway::Msg>),
    PeerJoin(String),
    DisconnectFromRoom,
}

pub enum Sub {
    DisconnectFromRoom,
}

pub fn new(peer: Rc<Peer>, room: Rc<Room>) -> Component<Msg, State, Sub> {
    let init = {
        let peer = Rc::clone(&peer);
        let room = Rc::clone(&room);
        move || {
            let state = State {
                _peer: peer,
                peers: HashSet::new(),
                room: room,
                world: World::new([20.0, 20.0]),
                resource: Resource::new(),
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
                is_2d_mode: false,
                modelesses: vec![],
                modals: vec![],
                editing_modeless: None,
                object_modeless_address: HashMap::new(),
                focused_object_id: None,
                cmd_queue: CmdQueue::new(),
            };
            let task = Cmd::task(|handler| {
                handler(Msg::SetTableContext);
            });
            (state, task)
        }
    };
    Component::new(init, update, render)
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
        .batch({
            let room = Rc::clone(&room);
            move |mut handler| {
                let a =
                    Closure::wrap(
                        Box::new(move |peer_id: String| handler(Msg::PeerJoin(peer_id)))
                            as Box<dyn FnMut(String)>,
                    );
                room.payload
                    .on("peerJoin", Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        })
        .batch({
            let room = Rc::clone(&room);
            move |mut handler| {
                let a = Closure::wrap(Box::new(move |logs: skyway::LogList| {
                    handler(Msg::ReceiveLog(logs))
                }) as Box<dyn FnMut(skyway::LogList)>);
                room.payload.on("log", Some(a.as_ref().unchecked_ref()));
                room.payload.get_log();
                a.forget();
            }
        })
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
        Msg::NoOp => state.cmd_queue.dequeue(),
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

            renderer.render(&mut state.world, &state.camera, &state.resource);
            state.renderer = Some(renderer);
            state.cmd_queue.dequeue()
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
                renderer.render(&mut state.world, &state.camera, &state.resource);
            }
            state.cmd_queue.dequeue()
        }

        //メッセージの伝搬
        Msg::TransportContextMenuMsg(msg) => {
            contextmenu::update(&mut state.contextmenu.state, msg);
            state.cmd_queue.dequeue()
        }

        //コンテキストメニューの制御
        Msg::OpenContextMenu(page_mouse_coord, offset_mouse_coord) => {
            update(
                state,
                Msg::SetCursorWithMouseCoord(offset_mouse_coord.clone()),
            );
            state.contextmenu.grobal_position = page_mouse_coord.clone();
            state.contextmenu.canvas_position = offset_mouse_coord;
            contextmenu::open(&mut state.contextmenu.state, page_mouse_coord);
            state.cmd_queue.dequeue()
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
                state.cmd_queue.dequeue()
            }
        }
        Msg::RemoveObjectWithObjectId(object_id, transport) => {
            state.world.remove_object(&object_id);
            if transport {
                state.room.send(&skyway::Msg::RemoveObject(object_id));
            }
            update(state, Msg::Render)
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
                renderer.render(&mut state.world, &camera, &state.resource);
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
            state.cmd_queue.dequeue()
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
            state.cmd_queue.dequeue()
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
            state
                .room
                .send(&skyway::Msg::EraceLineToTable(start_point, end_point));
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

        // モードレス
        Msg::OpenObjectModeless(object_id) => {
            let mut modeless_is_registered = false;
            if let Some(address) = state.object_modeless_address.get(&object_id) {
                if let Some((state, modeless)) = state.modelesses.get_mut(address[0]) {
                    match modeless {
                        Modeless::Object { focused, .. } => {
                            *focused = address[1];
                            state.is_showing = true;
                            modeless_is_registered = true;
                        }
                    }
                }
            }
            if !modeless_is_registered {
                state.modelesses.push((
                    ModelessState::new(true),
                    Modeless::Object {
                        tabs: vec![object_id],
                        focused: 0,
                    },
                ));
                state
                    .object_modeless_address
                    .insert(object_id, [state.modelesses.len() - 1, 0]);
            }
            state.cmd_queue.dequeue()
        }
        Msg::CloseModeless(modeless_idx) => {
            state.modelesses[modeless_idx].0.is_showing = false;
            state.cmd_queue.dequeue()
        }
        Msg::OpenModelessModal(modeless_idx) => {
            if let Some((modeless, ..)) = state.modelesses.get(modeless_idx) {
                let props = modeless_modal::Props {
                    origin: modeless.loc_a.clone(),
                    corner: modeless.loc_b.clone(),
                };
                state.editing_modeless = Some((
                    modeless_idx,
                    Rc::new(RefCell::new(modeless_modal::State::new(&props))),
                ));
            }
            state.cmd_queue.dequeue()
        }
        Msg::CloseModelessModal => {
            state.editing_modeless = None;
            state.cmd_queue.dequeue()
        }
        Msg::ReflectModelessModal(props) => {
            let modeless = state
                .editing_modeless
                .as_ref()
                .map(|(idx, ..)| *idx)
                .and_then(|idx| state.modelesses.get_mut(idx));
            if let Some((modeless, ..)) = modeless {
                modeless.loc_a = props.origin;
                modeless.loc_b = props.corner;
            }
            state.cmd_queue.dequeue()
        }
        Msg::CloseModelessModalWithProps(props) => {
            update(state, Msg::ReflectModelessModal(props));
            update(state, Msg::CloseModelessModal)
        }

        // モーダル
        Msg::OpenModal(modal) => {
            state.modals.push(modal);
            state.cmd_queue.dequeue()
        }
        Msg::CloseModal => {
            state.modals.pop();
            state.cmd_queue.dequeue()
        }

        // Worldに対する操作
        Msg::SetCharacterImage(character_id, data_id, transport) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.set_image_id(data_id);
                if transport {
                    state
                        .room
                        .send(&skyway::Msg::SetCharacterImage(character_id, data_id));
                }
                update(state, Msg::Render)
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::SetCharacterHp(character_id, hp) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.set_hp(hp);
                state.cmd_queue.dequeue()
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::SetCharacterMp(character_id, mp) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.set_mp(mp);
                state.cmd_queue.dequeue()
            } else {
                state.cmd_queue.dequeue()
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
        Msg::SetTablemaskSize(tablemask_id, size) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_size(size);
            }
            update(state, Msg::Render)
        }
        Msg::SetTablemaskSizeIsBinded(tablemask_id, is_binded) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_size_is_binded(is_binded);
            }
            update(state, Msg::Render)
        }

        // リソース
        Msg::LoadFromFileList(file_list) => Cmd::task(move |resolve| {
            let len = file_list.length();
            let common = Rc::new((RefCell::new(HashMap::new()), Some(resolve)));
            let mut file_readers = vec![];
            for i in 0..len {
                if let Some(file) = file_list.item(i) {
                    if file.type_() == String::from("image/png") {
                        let file_reader = Rc::new(web_sys::FileReader::new().unwrap());
                        let a = {
                            let file_reader = Rc::clone(&file_reader);
                            let data_id = random_id::u128val();
                            let mut common = Rc::clone(&common);
                            Closure::once(Box::new(move || {
                                file_reader
                                    .result()
                                    .ok()
                                    .and_then(|result| result.as_string())
                                    .map(|data_url| {
                                        common
                                            .0
                                            .borrow_mut()
                                            .insert(data_id, DataString::Image(data_url));
                                        if let Some((data_urls, resolve)) = Rc::get_mut(&mut common)
                                        {
                                            let mut r = None;
                                            std::mem::swap(&mut r, resolve);
                                            r.map(|r| {
                                                r(Msg::LoadFromDataUrls(
                                                    data_urls.borrow_mut().drain().collect(),
                                                    true,
                                                ))
                                            });
                                        };
                                    });
                            }) as Box<dyn FnOnce()>)
                        };
                        file_reader.set_onload(Some(&a.as_ref().unchecked_ref()));
                        file_readers.push((file, file_reader));
                        a.forget();
                    }
                }
            }
            for (file, file_reader) in file_readers {
                file_reader.read_as_data_url(&file).unwrap();
            }
        }),
        Msg::LoadFromDataUrls(data_urls, transport) => {
            let room = Rc::clone(&state.room);
            Cmd::task(move |resolve| {
                let common = Rc::new((RefCell::new(HashMap::new()), Some(resolve)));
                for (data_id, data_url) in data_urls {
                    match &data_url {
                        DataString::Image(data_url) => {
                            let image = Rc::new(
                                web_sys::window()
                                    .unwrap()
                                    .document()
                                    .unwrap()
                                    .create_element("img")
                                    .unwrap()
                                    .dyn_into::<web_sys::HtmlImageElement>()
                                    .unwrap(),
                            );
                            let a = {
                                let image = Rc::clone(&image);
                                let data_id = data_id;
                                let mut common = Rc::clone(&common);
                                Closure::once(Box::new(move || {
                                    common.0.borrow_mut().insert(data_id, image);
                                    if let Some((data, resolve)) = Rc::get_mut(&mut common) {
                                        let mut r = None;
                                        std::mem::swap(&mut r, resolve);
                                        r.map(|r| {
                                            r(Msg::LoadReasources(
                                                data.borrow_mut().drain().collect(),
                                            ))
                                        });
                                    };
                                }))
                            };
                            image.set_onload(Some(&a.as_ref().unchecked_ref()));
                            image.set_src(&data_url);
                            a.forget();
                        }
                    }
                    if transport {
                        room.send(&skyway::Msg::AddResource(data_id, data_url));
                    }
                }
            })
        }
        Msg::LoadReasources(images) => {
            for image in images {
                state.resource.insert(image.0, Data::Image(image.1));
            }
            state.cmd_queue.dequeue()
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
            skyway::Msg::EraceLineToTable(start_point, end_point) => {
                state
                    .world
                    .table_mut()
                    .erace_line(&start_point, &end_point, 1.0);
                update(state, Msg::Render)
            }
            skyway::Msg::SetCharacterImage(character_id, data_id) => {
                update(state, Msg::SetCharacterImage(character_id, data_id, false))
            }
            skyway::Msg::SetObjectPosition(object_id, position) => {
                if let Some(character) = state.world.character_mut(&object_id) {
                    character.set_position(position);
                    update(state, Msg::Render)
                } else if let Some(tablemask) = state.world.tablemask_mut(&object_id) {
                    tablemask.set_position(position);
                    update(state, Msg::Render)
                } else {
                    state.cmd_queue.dequeue()
                }
            }
            skyway::Msg::SetIsBindToGrid(is_bind_to_grid) => {
                state.world.table_mut().set_is_bind_to_grid(is_bind_to_grid);
                update(state, Msg::Render)
            }
            skyway::Msg::SetWorld(world_data) => {
                state.world = World::from(world_data);
                update(state, Msg::Render)
            }
            skyway::Msg::SetResource(resource_data) => {
                update(state, Msg::LoadFromDataUrls(resource_data, false))
            }
            skyway::Msg::AddResource(data_id, data) => {
                let mut data_urls = HashMap::new();
                data_urls.insert(data_id, data);
                update(state, Msg::LoadFromDataUrls(data_urls, false))
            }
            skyway::Msg::RemoveObject(object_id) => {
                update(state, Msg::RemoveObjectWithObjectId(object_id, false))
            }
        },
        Msg::ReceiveLog(logs) => {
            let mut msgs = vec![];
            let mut resource_data = HashMap::new();
            let mut i = 0;

            while let Some(log) = logs.get(i) {
                if let Ok(log) = serde_json::from_str::<skyway::Log>(&log) {
                    let message_type = log.message_type;
                    let message = log.message;

                    if message_type == "ROOM_USER_JOIN" {
                        state.peers.insert(message.src);
                    } else if message_type == "ROOM_DATA" {
                        message
                            .data
                            .and_then(|data| serde_json::from_str::<skyway::Msg>(&data).ok())
                            .map(|msg| {
                                if let skyway::Msg::SetResource(data) = msg {
                                    for a_data in data {
                                        resource_data.insert(a_data.0, a_data.1);
                                    }
                                } else if let skyway::Msg::AddResource(data_id, data) = msg {
                                    resource_data.insert(data_id, data);
                                } else {
                                    msgs.push(msg);
                                }
                            });
                    }
                }
                i += 1;
            }

            state.cmd_queue.enqueue(Cmd::task(move |resolve| {
                resolve(Msg::DeelMsgListWithNonCmd(msgs))
            }));
            update(state, Msg::LoadFromDataUrls(resource_data, false))
        }
        Msg::DeelMsgListWithNonCmd(msgs) => {
            for msg in msgs {
                update(state, Msg::ReceiveMsg(msg));
            }
            state.cmd_queue.dequeue()
        }
        Msg::PeerJoin(peer_id) => {
            state.peers.insert(peer_id);
            state.cmd_queue.dequeue()
        }
        Msg::DisconnectFromRoom => Cmd::Sub(Sub::DisconnectFromRoom),
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .id("app")
            .class("fullscreen unselectable")
            .style("display", "grid")
            .style("grid-template-rows", "max-content 1fr"),
        Events::new().on("drop", |e| {
            e.prevent_default();
            e.stop_propagation();
            Msg::NoOp
        }),
        vec![
            render_header_menu(
                &state.room.id,
                &state.table_state.selecting_tool,
                state.world.table().is_bind_to_grid(),
                state.is_2d_mode,
            ),
            render_canvas_container(&state),
            render_context_menu(&state.contextmenu, &state.focused_object_id, &state.world),
            render_modals(&state.modals, &state.resource),
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
            render_canvas(),
            render_measure_length(&state.table_state.measure_length),
            render_hint(),
            render_canvas_overlaper(
                &state.table_state,
                &state.focused_object_id,
                state.is_2d_mode,
                &state.world,
                &state.resource,
                &state.modelesses,
            ),
            state
                .editing_modeless
                .as_ref()
                .map(|(_, props)| {
                    Html::component(modeless_modal::new(Rc::clone(props)).subscribe(
                        |sub| match sub {
                            modeless_modal::Sub::Close => Msg::CloseModelessModal,
                            modeless_modal::Sub::Reflect(props) => Msg::ReflectModelessModal(props),
                            modeless_modal::Sub::ReflectToClose(props) => {
                                Msg::CloseModelessModalWithProps(props)
                            }
                        },
                    ))
                })
                .unwrap_or(Html::none()),
        ],
    )
}

fn render_canvas() -> Html<Msg> {
    Html::canvas(
        Attributes::new().id("table").class("cover"),
        Events::new(),
        vec![],
    )
}

fn render_canvas_overlaper(
    table_state: &TableState,
    focused_object_id: &Option<u128>,
    is_2d_mode: bool,
    world: &World,
    resource: &Resource,
    modelesses: &ModelessCollection,
) -> Html<Msg> {
    modeless::container(
        Attributes::new().class("cover"),
        Events::new()
            .on_mousemove({
                let selecting_tool = table_state.selecting_tool.clone();
                let focused_object_id = focused_object_id.clone();
                move |e| {
                    e.stop_propagation();
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
                e.stop_propagation();
                if let Ok(e) = e.dyn_into::<web_sys::WheelEvent>() {
                    Msg::SetCameraMovementWithMouseWheel(e.delta_y())
                } else {
                    Msg::NoOp
                }
            })
            .on_mousedown({
                let selecting_tool = table_state.selecting_tool.clone();
                move |e| {
                    e.stop_propagation();
                    match selecting_tool {
                        TableTool::Measure(_) => {
                            let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                            Msg::SetSelectingTableTool(TableTool::Measure(Some(mouse_coord)))
                        }
                        _ => Msg::NoOp,
                    }
                }
            })
            .on_mouseup({
                let selecting_tool = table_state.selecting_tool.clone();
                let focused_object_id = focused_object_id.clone();
                move |e| {
                    e.stop_propagation();
                    match selecting_tool {
                        TableTool::Selector => match focused_object_id {
                            Some(object_id) => Msg::BindObjectToTableGrid(object_id),
                            None => Msg::NoOp,
                        },
                        TableTool::Measure(_) => {
                            Msg::SetSelectingTableTool(TableTool::Measure(None))
                        }
                        _ => Msg::NoOp,
                    }
                }
            })
            .on_contextmenu(|e| {
                let page_mouse_coord = [e.page_x() as f64, e.page_y() as f64];
                let offset_mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                e.prevent_default();
                e.stop_propagation();
                Msg::OpenContextMenu(page_mouse_coord, offset_mouse_coord)
            }),
        modelesses
            .iter()
            .enumerate()
            .map(|(idx, (state, modeless))| {
                if !state.is_showing {
                    Html::none()
                } else {
                    match modeless {
                        Modeless::Object { focused, tabs } => {
                            render_object_modeless(idx, state, tabs, *focused, world, resource)
                        }
                    }
                }
            })
            .collect(),
    )
}

fn render_context_menu(
    contextmenu: &Contextmenu,
    focused_object_id: &Option<u128>,
    world: &World,
) -> Html<Msg> {
    if let Some(focused_object_id) = focused_object_id {
        if world.tablemask(focused_object_id).is_some() {
            render_context_menu_tablemask(contextmenu, *focused_object_id)
        } else {
            render_context_menu_character(contextmenu, *focused_object_id)
        }
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

fn render_context_menu_character(contextmenu: &Contextmenu, object_id: u128) -> Html<Msg> {
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
                    Events::new().on_click(move |_| Msg::OpenObjectModeless(object_id)),
                    "編集",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::CloneObjectWithObjectId(object_id)),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::RemoveObjectWithObjectId(object_id, true)),
                    "削除",
                ),
            ],
        )],
    )
}

fn render_context_menu_tablemask(contextmenu: &Contextmenu, object_id: u128) -> Html<Msg> {
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
                Html::li(
                    Attributes::new().class("pure-menu-item pure-menu-has-children"),
                    Events::new(),
                    vec![
                        btn::contextmenu_text(Attributes::new(), Events::new(), "サイズ"),
                        Html::ul(
                            Attributes::new().class("pure-menu-children"),
                            Events::new(),
                            vec![
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [2., 2.])
                                    }),
                                    "半径1",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [4., 4.])
                                    }),
                                    "半径2",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [6., 6.])
                                    }),
                                    "半径3",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [8., 8.])
                                    }),
                                    "半径4",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [10., 10.])
                                    }),
                                    "半径5",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [12., 12.])
                                    }),
                                    "半径6",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskSize(object_id, [14., 14.])
                                    }),
                                    "半径7",
                                ),
                            ],
                        ),
                    ],
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::CloneObjectWithObjectId(object_id)),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::RemoveObjectWithObjectId(object_id, true)),
                    "削除",
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
            Html::div(Attributes::new().class("grid-w-12"), Events::new(), vec![]),
            Html::div(
                Attributes::new().class("grid-w-6 justify-r"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::OpenModal(Modal::Resource)),
                            vec![Html::text("リソース")],
                        ),
                        btn::danger(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::DisconnectFromRoom),
                            vec![Html::text("ルームから出る")],
                        ),
                    ],
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

fn render_object_modeless(
    modeless_idx: usize,
    state: &ModelessState,
    tabs: &Vec<u128>,
    focused: usize,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    let height = state.loc_b[1] - state.loc_a[1];
    let focused_id = tabs[focused];
    modeless::frame(
        &state.loc_a,
        &state.loc_b,
        Attributes::new(),
        Events::new()
            .on_mousedown(move |e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on_mousemove(|e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on_contextmenu(|e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on("wheel", |e| {
                e.stop_propagation();
                Msg::NoOp
            }),
        vec![
            modeless::header(
                Attributes::new()
                    .style("display", "grid")
                    .style("grid-template-columns", "1fr max-content"),
                Events::new(),
                vec![
                    Html::div(Attributes::new(), Events::new(), vec![]),
                    Html::div(
                        Attributes::new().class("linear-h"),
                        Events::new(),
                        vec![
                            btn::allocate(
                                Attributes::new(),
                                Events::new()
                                    .on_click(move |_| Msg::OpenModelessModal(modeless_idx)),
                            ),
                            btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::CloseModeless(modeless_idx)),
                            ),
                        ],
                    ),
                ],
            ),
            if let Some(character) = world.character(&focused_id) {
                render_object_modeless_character(height, character, focused_id, resource)
            } else if let Some(tablemask) = world.tablemask(&focused_id) {
                render_object_modeless_tablemask(tablemask, focused_id)
            } else {
                Html::none()
            },
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn render_object_modeless_character(
    modeless_height: i32,
    character: &Character,
    character_id: u128,
    resource: &Resource,
) -> Html<Msg> {
    modeless::body(
        Attributes::new().class("scroll-v flex-h"),
        Events::new(),
        vec![Html::div(
            Attributes::new().class("container-a"),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class("centering-a"),
                    Events::new(),
                    vec![
                        character
                            .texture_id()
                            .and_then(|data_id| resource.get_as_image(&data_id))
                            .map(|image| {
                                Html::img(
                                    Attributes::new()
                                        .string("src", image.src())
                                        .class("pure-img")
                                        .style(
                                            "max-height",
                                            format!(
                                                "{}vh",
                                                (modeless_height - 1) as f64 * 100.0 / 14.0
                                            ),
                                        ),
                                    Events::new(),
                                    vec![],
                                )
                            })
                            .unwrap_or(Html::none()),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click(move |_| {
                                Msg::OpenModal(Modal::SelectCharacterImage(character_id))
                            }),
                            vec![Html::text("画像を選択")],
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class("keyvalue pure-form"),
                    Events::new(),
                    vec![
                        Html::span(Attributes::new(), Events::new(), vec![Html::text("HP")]),
                        Html::input(
                            Attributes::new()
                                .value(character.hp().to_string())
                                .type_("number"),
                            Events::new().on_input(move |s| {
                                if let Ok(s) = s.parse() {
                                    Msg::SetCharacterHp(character_id, s)
                                } else {
                                    Msg::NoOp
                                }
                            }),
                            vec![],
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class("keyvalue pure-form"),
                    Events::new(),
                    vec![
                        Html::span(Attributes::new(), Events::new(), vec![Html::text("MP")]),
                        Html::input(
                            Attributes::new()
                                .value(character.mp().to_string())
                                .type_("number"),
                            Events::new().on_input(move |s| {
                                if let Ok(s) = s.parse() {
                                    Msg::SetCharacterMp(character_id, s)
                                } else {
                                    Msg::NoOp
                                }
                            }),
                            vec![],
                        ),
                    ],
                ),
            ],
        )],
    )
}

fn render_object_modeless_tablemask(tablemask: &Tablemask, tablemask_id: u128) -> Html<Msg> {
    let input_width_id = random_id::hex(4);
    let input_height_id = random_id::hex(4);
    let width = tablemask.size()[0];
    let height = tablemask.size()[1];

    modeless::body(
        Attributes::new().class("container-a grid pure-form"),
        Events::new(),
        vec![
            Html::fieldset(
                Attributes::new().class("grid-w-11 keyvalue"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new()
                            .class("text-label")
                            .string("for", &input_width_id),
                        Events::new(),
                        vec![Html::text("width")],
                    ),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .value(width.to_string())
                            .class("pure-input-1")
                            .id(input_width_id),
                        Events::new().on_input({
                            let size_is_binded = tablemask.size_is_binded();
                            move |w| {
                                if let Ok(w) = w.parse() {
                                    Msg::SetTablemaskSize(
                                        tablemask_id,
                                        [w, if size_is_binded { w } else { height }],
                                    )
                                } else {
                                    Msg::NoOp
                                }
                            }
                        }),
                        vec![],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("grid-w-2 centering-a"),
                Events::new(),
                vec![if tablemask.size_is_binded() {
                    btn::transparent(
                        Attributes::new().class("fas fa-link text-color-light"),
                        Events::new()
                            .on_click(move |_| Msg::SetTablemaskSizeIsBinded(tablemask_id, false)),
                        vec![],
                    )
                } else {
                    btn::transparent(
                        Attributes::new().class("fas fa-link text-color-gray"),
                        Events::new()
                            .on_click(move |_| Msg::SetTablemaskSizeIsBinded(tablemask_id, true)),
                        vec![],
                    )
                }],
            ),
            Html::fieldset(
                Attributes::new().class("grid-w-11 keyvalue"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new()
                            .class("text-label")
                            .string("for", &input_height_id),
                        Events::new(),
                        vec![Html::text("height")],
                    ),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .value(height.to_string())
                            .class("pure-input-1")
                            .id(input_height_id),
                        Events::new().on_input({
                            let size_is_binded = tablemask.size_is_binded();
                            move |h| {
                                if let Ok(h) = h.parse() {
                                    Msg::SetTablemaskSize(
                                        tablemask_id,
                                        [if size_is_binded { h } else { width }, h],
                                    )
                                } else {
                                    Msg::NoOp
                                }
                            }
                        }),
                        vec![],
                    ),
                ],
            ),
        ],
    )
}

fn render_modals(modals: &Vec<Modal>, resource: &Resource) -> Html<Msg> {
    if modals.is_empty() {
        Html::none()
    } else {
        let mut children = vec![];
        for modal in modals {
            let child = match modal {
                Modal::Resource => render_modal_resource(resource),
                Modal::SelectCharacterImage(character_id) => {
                    render_modal_select_character_image(*character_id, resource)
                }
            };
            children.push(child);
        }
        modal::container(Attributes::new(), Events::new(), children)
    }
}

fn render_modal_resource(resource: &Resource) -> Html<Msg> {
    modal::frame(
        12,
        Attributes::new(),
        Events::new(),
        vec![
            modal::header(
                Attributes::new()
                    .style("display", "grid")
                    .style("grid-template-columns", "1fr max-content"),
                Events::new(),
                vec![
                    Html::div(Attributes::new(), Events::new(), vec![]),
                    Html::div(
                        Attributes::new().class("linear-h"),
                        Events::new(),
                        vec![btn::close(
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::CloseModal),
                        )],
                    ),
                ],
            ),
            modal::body(
                Attributes::new()
                    .class("scroll-v grid container")
                    .style("min-height", "50vh"),
                Events::new()
                    .on("dragover", |e| {
                        e.prevent_default();
                        Msg::NoOp
                    })
                    .on("drop", |e| {
                        e.prevent_default();
                        let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                        e.data_transfer()
                            .unwrap()
                            .files()
                            .map(|files| Msg::LoadFromFileList(files))
                            .unwrap_or(Msg::NoOp)
                    }),
                resource
                    .get_images()
                    .into_iter()
                    .map(|(_, image)| {
                        Html::img(
                            Attributes::new()
                                .class("grid-w-2 pure-img")
                                .string("src", image.src()),
                            Events::new(),
                            vec![],
                        )
                    })
                    .collect(),
            ),
            modal::footer(
                Attributes::new(),
                Events::new(),
                vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
            ),
        ],
    )
}

fn render_modal_select_character_image(character_id: u128, resource: &Resource) -> Html<Msg> {
    modal::frame(
        12,
        Attributes::new(),
        Events::new(),
        vec![
            modal::header(
                Attributes::new()
                    .style("display", "grid")
                    .style("grid-template-columns", "1fr max-content"),
                Events::new(),
                vec![
                    Html::div(Attributes::new(), Events::new(), vec![]),
                    Html::div(
                        Attributes::new().class("linear-h"),
                        Events::new(),
                        vec![btn::close(
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::CloseModal),
                        )],
                    ),
                ],
            ),
            modal::body(
                Attributes::new()
                    .class("scroll-v grid container")
                    .style("min-height", "50vh"),
                Events::new()
                    .on("dragover", |e| {
                        e.prevent_default();
                        Msg::NoOp
                    })
                    .on("drop", |e| {
                        e.prevent_default();
                        let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                        e.data_transfer()
                            .unwrap()
                            .files()
                            .map(|files| Msg::LoadFromFileList(files))
                            .unwrap_or(Msg::NoOp)
                    }),
                resource
                    .get_images()
                    .into_iter()
                    .map(|(data_id, image)| {
                        Html::img(
                            Attributes::new()
                                .class("grid-w-2 pure-img clickable")
                                .string("src", image.src()),
                            Events::new().on_click(move |_| {
                                Msg::SetCharacterImage(character_id, data_id, true)
                            }),
                            vec![],
                        )
                    })
                    .collect(),
            ),
            modal::footer(
                Attributes::new(),
                Events::new(),
                vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
            ),
        ],
    )
}
