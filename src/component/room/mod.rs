mod modal;
mod modeless;

use super::{awesome, btn, contextmenu, modeless::container as modeless_container, modeless_modal};
use crate::{
    model::{
        resource::{Data, ResourceData},
        Camera, Character, Color, ColorSystem, Resource, Tablemask, World,
    },
    random_id,
    renderer::Renderer,
    skyway::{self, DataConnection, Peer, ReceiveData, Room},
    JsObject,
};
use js_sys::JsString;
use kagura::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeSet, HashMap, VecDeque},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

pub struct PersonalData {
    name: String,
    icon: Option<u128>,
}

impl PersonalData {
    fn new() -> Self {
        Self {
            name: "Player".into(),
            icon: None,
        }
    }

    fn with_peer_id(mut self, peer_id: &str) -> Self {
        self.name = self.name + "_" + peer_id;
        self
    }
}

enum Icon {
    None,
    Resource(u128),
    DefaultUser,
}

struct ChatItem {
    display_name: String,
    peer_id: String,
    icon: Icon,
    payload: String,
}

struct ChatTab {
    name: String,
    items: Vec<ChatItem>,
}

impl ChatTab {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: vec![ChatItem {
                display_name: "System".into(),
                peer_id: "".into(),
                icon: Icon::None,
                payload:
                    "チャット機能は開発途中のため、他のクライアントとの通信には対応していません。また、タブの作成や消去にも対応していません。"
                        .into(),
            }],
        }
    }
}

enum Sender {
    Player,
    Character(u128),
}

impl Sender {
    fn as_character(&self) -> Option<u128> {
        match self {
            Self::Character(c_id) => Some(*c_id),
            _ => None,
        }
    }
}

pub struct ChatDataCollection {
    selecting_tab_idx: usize,
    selecting_sender_idx: usize,
    inputing_message: String,
    senders: Vec<Sender>,
    tabs: Vec<ChatTab>,
}

impl ChatDataCollection {
    fn new() -> Self {
        Self {
            selecting_tab_idx: 0,
            selecting_sender_idx: 0,
            inputing_message: "".into(),
            senders: vec![Sender::Player],
            tabs: vec![ChatTab::new("メイン"), ChatTab::new("サブ")],
        }
    }
}

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

pub struct ModelessState {
    is_showing: bool,
    grubbed: Option<[bool; 4]>,
    loc_a: [i32; 2],
    loc_b: [i32; 2],
}

impl ModelessState {
    pub fn new(is_showing: bool) -> Self {
        Self {
            is_showing,
            grubbed: None,
            loc_a: [18, 2],
            loc_b: [24, 6],
        }
    }
}

enum Modeless {
    Object { tabs: Vec<u128>, focused: usize },
    Chat,
}

type ModelessCollection = Vec<(ModelessState, Modeless)>;

#[derive(Clone)]
pub enum SelectImageModal {
    Player,
    Character(u128),
}

#[derive(Clone)]
pub enum ColorPickerType {
    TablemaskColor(u128),
}

#[derive(Clone)]
pub enum CharacterSelecterType {
    ChatSender,
}

pub enum Modal {
    Resource,
    SelectImage(SelectImageModal),
    PersonalSetting,
    ColorPicker(ColorPickerType),
    CharacterSelecter(CharacterSelecterType),
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
    peer: Rc<Peer>,
    peers: BTreeSet<String>,
    room: Rc<Room>,
    personal_data: PersonalData,
    world: World,
    resource: Resource,
    chat_data: ChatDataCollection,
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
    object_modeless_address: HashMap<u128, usize>,
    chat_modeless_address: Option<usize>,
    pixel_ratio: f64,
    is_low_loading_mode: bool,
    loading_state: i64,
    loading_resource_num: u64,
    loaded_resource_num: u64,
    cmd_queue: CmdQueue<Msg, Sub>,
}

pub enum Msg {
    NoOp,
    SetTableContext,
    WindowResized,
    Render,
    SetLowLoadingMode(bool),

    // メッセージの伝搬
    TransportContextMenuMsg(contextmenu::Msg),
    PickColor(Color, ColorPickerType),

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
    OpenChatModeless,
    CloseModeless(usize),
    GrubModeless(usize, Option<[bool; 4]>),
    FocusModeless(usize),
    OpenModelessModal(usize),
    CloseModelessModal,
    ReflectModelessModal(modeless_modal::Props),
    CloseModelessModalWithProps(modeless_modal::Props),

    // モーダル
    OpenModal(Modal),
    CloseModal,

    // PersonalData
    SetPersonalDataWithPlayerName(String),
    SetPersonalDataWithIconImage(u128),

    // Worldに対する操作
    SetCharacterImage(u128, u128, bool),
    SetCharacterSize(u128, Option<f64>, Option<f64>),
    SetCharacterHp(u128, i32),
    SetCharacterMp(u128, i32),
    AddChracater(Character),
    AddTablemask(Tablemask),
    SetTablemaskSizeWithStyle(u128, [f64; 2], bool),
    SetTablemaskSizeIsBinded(u128, bool),
    SetTablemaskColor(u128, Color),

    // チャット関係
    SetSelectingChatTabIdx(usize),
    InputChatMessage(String),
    SendChatItem,
    SetChatSender(usize),

    // リソース管理
    LoadFromFileList(web_sys::FileList),
    LoadFromBlobs(HashMap<u128, Rc<web_sys::Blob>>, bool),
    LoadReasource(u128, Data),

    // 接続に関する操作
    ReceiveMsg(skyway::Msg),
    PeerJoin(String),
    PeerLeave(String),
    SetLoadingState(bool),
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
            let peers = {
                let mut p = BTreeSet::new();
                p.insert(peer.id());
                p
            };
            let peer_id = peer.id();
            let state = State {
                peer: peer,
                peers: peers,
                room: room,
                personal_data: PersonalData::new().with_peer_id(&peer_id),
                world: World::new([20.0, 20.0]),
                resource: Resource::new(),
                chat_data: ChatDataCollection::new(),
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
                chat_modeless_address: None,
                focused_object_id: None,
                pixel_ratio: 1.0,
                is_low_loading_mode: false,
                loading_state: 0,
                loading_resource_num: 0,
                loaded_resource_num: 0,
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
                    move |receive_data: Option<ReceiveData>| {
                        let msg = receive_data
                            .and_then(|receive_data| receive_data.data())
                            .map(|receive_data| skyway::Msg::from(receive_data))
                            .and_then(|msg| {
                                if let skyway::Msg::None = msg {
                                    None
                                } else {
                                    Some(msg)
                                }
                            });
                        if let Some(msg) = msg {
                            handler(Msg::ReceiveMsg(msg));
                        } else {
                            web_sys::console::log_1(&JsValue::from("faild to deserialize message"));
                        }
                    }
                }) as Box<dyn FnMut(Option<ReceiveData>)>);
                room.payload.on("data", Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        })
        .batch({
            let room = Rc::clone(&room);
            move |mut handler| {
                let a = Closure::wrap(Box::new(move |peer_id: String| {
                    handler(Msg::PeerJoin(peer_id));
                }) as Box<dyn FnMut(String)>);
                room.payload
                    .on("peerJoin", Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        })
        .batch({
            let room = Rc::clone(&room);
            move |mut handler| {
                let a = Closure::wrap(Box::new(move |peer_id: String| {
                    handler(Msg::PeerLeave(peer_id));
                }) as Box<dyn FnMut(String)>);
                room.payload
                    .on("peerLeave", Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        })
        .batch({
            let peer = Rc::clone(&peer);
            move |handler| {
                let handler = Rc::new(RefCell::new(Some(handler)));
                let connection_num = Rc::new(Cell::new(0));

                // 接続済みユーザーからの接続が発生するごとに発火
                let a = Closure::wrap(Box::new({
                    let handler = Rc::clone(&handler);
                    let connection_num = Rc::clone(&connection_num);
                    move |data_connection: DataConnection| {
                        let data_connection = Rc::new(data_connection);
                        let received_msg_num = Rc::new(Cell::new(0));

                        // それぞれのユーザーからデータが送られてくるごとに発生
                        let a = Closure::wrap(Box::new({
                            let handler = Rc::clone(&handler);
                            let data_connection = Rc::clone(&data_connection);
                            let received_msg_num = Rc::clone(&received_msg_num);
                            move |receive_data: Option<JsObject>| {
                                let msg = receive_data
                                    .map(|receive_data| skyway::Msg::from(receive_data))
                                    .and_then(|msg| {
                                        if let skyway::Msg::None = msg {
                                            None
                                        } else {
                                            Some(msg)
                                        }
                                    });
                                if let Some(msg) = msg {
                                    received_msg_num.set(received_msg_num.get() + 1);
                                    web_sys::console::log_1(&JsValue::from(format!(
                                        "Receive:{}, {}",
                                        &msg,
                                        received_msg_num.get()
                                    )));
                                    let h = handler.borrow_mut().take();
                                    if let Some(mut h) = h {
                                        h(Msg::ReceiveMsg(msg));
                                        let _ = handler.replace(Some(h));
                                    }
                                } else {
                                    web_sys::console::log_1(&JsValue::from(
                                        "faild to deserialize message",
                                    ));
                                }

                                if received_msg_num.get() >= 3 {
                                    data_connection.close(false);
                                    let h = handler.borrow_mut().take();
                                    if let Some(mut h) = h {
                                        h(Msg::SetLoadingState(false));
                                        let _ = handler.replace(Some(h));
                                    }
                                }
                            }
                        })
                            as Box<dyn FnMut(Option<JsObject>)>);
                        data_connection.on("data", Some(a.as_ref().unchecked_ref()));
                        a.forget();

                        let a = Closure::wrap(Box::new({
                            let data_connection = Rc::clone(&data_connection);
                            let connection_num = Rc::clone(&connection_num);
                            let received_msg_num = Rc::clone(&received_msg_num);
                            move || {
                                let cn = connection_num.get();
                                if cn == 0 {
                                    data_connection.send(&JsString::from("FirstConnection"));
                                } else {
                                    received_msg_num.set(received_msg_num.get() + 2);
                                }
                                connection_num.set(cn + 1);
                            }
                        }) as Box<dyn FnMut()>);
                        data_connection.on("open", Some(a.as_ref().unchecked_ref()));
                        a.forget();

                        let h = handler.borrow_mut().take();
                        if let Some(mut h) = h {
                            h(Msg::SetLoadingState(true));
                            let _ = handler.replace(Some(h));
                        }
                    }
                }) as Box<dyn FnMut(DataConnection)>);
                peer.on("connection", Some(a.as_ref().unchecked_ref()));
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

fn get_device_pixel_ratio(pixel_ratio: f64) -> f64 {
    web_sys::window().unwrap().device_pixel_ratio() * pixel_ratio
}

fn get_table_position(state: &State, mouse_coord: &[f64; 2], pixel_ratio: f64) -> [f64; 2] {
    let dpr = get_device_pixel_ratio(pixel_ratio);
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
            let dpr = get_device_pixel_ratio(state.pixel_ratio);
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
            let dpr = get_device_pixel_ratio(state.pixel_ratio);
            let canvas_size = [
                canvas.client_width() as f64 * dpr,
                canvas.client_height() as f64 * dpr,
            ];
            canvas.set_width(canvas_size[0] as u32);
            canvas.set_height(canvas_size[1] as u32);
            state.canvas_size = canvas_size;
            update(state, Msg::Render)
        }
        Msg::Render => {
            if let Some(renderer) = &mut state.renderer {
                renderer.render(&mut state.world, &state.camera, &state.resource);
            }
            state.cmd_queue.dequeue()
        }
        Msg::SetLowLoadingMode(flag) => {
            if state.is_low_loading_mode != flag {
                state.is_low_loading_mode = flag;
                if flag {
                    state.pixel_ratio = 0.5;
                } else {
                    state.pixel_ratio = 1.0;
                }
                update(state, Msg::WindowResized)
            } else {
                state.cmd_queue.dequeue()
            }
        }

        //メッセージの伝搬
        Msg::TransportContextMenuMsg(msg) => {
            contextmenu::update(&mut state.contextmenu.state, msg);
            state.cmd_queue.dequeue()
        }
        Msg::PickColor(color, color_picker_type) => match color_picker_type {
            ColorPickerType::TablemaskColor(obj_id) => {
                update(state, Msg::SetTablemaskColor(obj_id, color))
            }
        },

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
            let position = get_table_position(&state, &mouse_coord, state.pixel_ratio);
            let position = [position[0], position[1], 0.0];
            let mut character = Character::new();
            character.set_position(position);
            if state.world.table().is_bind_to_grid() {
                character.bind_to_grid();
            }
            update(state, Msg::AddChracater(character))
        }
        Msg::AddTablemaskWithMouseCoord(mouse_coord) => {
            let position = get_table_position(&state, &mouse_coord, state.pixel_ratio);
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
            let table_coord = get_table_position(&state, &mouse_coord, state.pixel_ratio);
            let table_coord = [table_coord[0], table_coord[1]];
            let table = state.world.table_mut();
            match state.table_state.selecting_tool {
                TableTool::Pen => {
                    table.draw_cursor(
                        &table_coord,
                        0.25,
                        ColorSystem::gray(255, 9),
                        ColorSystem::gray(255, 9),
                    );
                }
                TableTool::Eracer => {
                    table.draw_cursor(
                        &table_coord,
                        0.5,
                        ColorSystem::gray(255, 9),
                        ColorSystem::gray(255, 1),
                    );
                }
                TableTool::Measure(_) => {
                    table.draw_cursor(
                        &table_coord,
                        0.125,
                        ColorSystem::red(255, 5),
                        ColorSystem::red(255, 5),
                    );
                }
                _ => {}
            }
            state.table_state.last_mouse_coord = mouse_coord;
            if let Some(renderer) = &mut state.renderer {
                renderer.render(&mut state.world, &camera, &state.resource);
                let dpr = get_device_pixel_ratio(state.pixel_ratio);
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
            let rotation_factor = 3.0 / long_edge * get_device_pixel_ratio(state.pixel_ratio);
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
            let movement_factor = 50.0 / long_edge * get_device_pixel_ratio(state.pixel_ratio);
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
            let start_point = get_table_position(
                &state,
                &state.table_state.last_mouse_coord,
                state.pixel_ratio,
            );
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord, state.pixel_ratio);
            let end_point = [end_point[0], end_point[1]];
            state.world.table_mut().draw_line(
                &start_point,
                &end_point,
                ColorSystem::gray(255, 9),
                0.5,
            );
            state
                .room
                .send(&skyway::Msg::DrawLineToTable(start_point, end_point));
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::EraceLineWithMouseCoord(mouse_coord) => {
            let start_point = get_table_position(
                &state,
                &state.table_state.last_mouse_coord,
                state.pixel_ratio,
            );
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord, state.pixel_ratio);
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
            let start_point = get_table_position(&state, &start_point, state.pixel_ratio);
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord, state.pixel_ratio);
            let end_point = [end_point[0], end_point[1]];
            let measure_length = state.world.table_mut().draw_measure(
                &start_point,
                &end_point,
                ColorSystem::red(255, 5),
                0.2,
            );
            state.table_state.measure_length = Some(measure_length);
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::SetObjectPositionWithMouseCoord(object_id, mouse_coord) => {
            let movement = {
                let a = get_table_position(
                    &state,
                    &state.table_state.last_mouse_coord,
                    state.pixel_ratio,
                );
                let b = get_table_position(&state, &mouse_coord, state.pixel_ratio);
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
            if let Some(address) = state.object_modeless_address.get(&object_id).map(|a| *a) {
                state.modelesses[address].0.is_showing = true;
                update(state, Msg::FocusModeless(address))
            } else {
                state.modelesses.push((
                    ModelessState::new(true),
                    Modeless::Object {
                        tabs: vec![object_id],
                        focused: 0,
                    },
                ));
                state
                    .object_modeless_address
                    .insert(object_id, state.modelesses.len() - 1);
                state.cmd_queue.dequeue()
            }
        }
        Msg::OpenChatModeless => {
            if let Some(address) = state.chat_modeless_address {
                state.modelesses[address].0.is_showing = true;
                update(state, Msg::FocusModeless(address))
            } else {
                let mut modeless_state = ModelessState::new(true);
                modeless_state.loc_a = [2, 2];
                modeless_state.loc_b = [8, 14];
                state.modelesses.push((modeless_state, Modeless::Chat));
                state.chat_modeless_address = Some(state.modelesses.len() - 1);
                state.cmd_queue.dequeue()
            }
        }
        Msg::CloseModeless(modeless_idx) => {
            state.modelesses[modeless_idx].0.is_showing = false;
            state.cmd_queue.dequeue()
        }
        Msg::GrubModeless(modeless_idx, grubbed) => {
            state.modelesses[modeless_idx].0.grubbed = grubbed;
            update(state, Msg::FocusModeless(modeless_idx))
        }
        Msg::FocusModeless(modeless_idx) => {
            let modeless = state.modelesses.remove(modeless_idx);
            state.modelesses.push(modeless);
            let mut idx = 0;
            for modeless in &state.modelesses {
                match &modeless.1 {
                    Modeless::Object { tabs, .. } => {
                        for object_id in tabs {
                            state.object_modeless_address.insert(*object_id, idx);
                        }
                    }
                    Modeless::Chat => {
                        state.chat_modeless_address = Some(idx);
                    }
                }
                idx += 1;
            }
            state.cmd_queue.dequeue()
        }
        Msg::OpenModelessModal(modeless_idx) => {
            if let Some((modeless, ..)) = state.modelesses.get_mut(modeless_idx) {
                if let Some(resizable) = modeless.grubbed {
                    let props = modeless_modal::Props {
                        origin: modeless.loc_a.clone(),
                        corner: modeless.loc_b.clone(),
                        resizable: resizable.clone(),
                    };
                    state.editing_modeless = Some((
                        modeless_idx,
                        Rc::new(RefCell::new(modeless_modal::State::new(&props))),
                    ));
                }
            }
            state.cmd_queue.dequeue()
        }
        Msg::CloseModelessModal => {
            let modeless = state
                .editing_modeless
                .as_ref()
                .map(|(idx, ..)| *idx)
                .and_then(|idx| state.modelesses.get_mut(idx));
            if let Some((modeless, ..)) = modeless {
                modeless.grubbed = None;
            }
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
                modeless.grubbed = None;
            }
            state.cmd_queue.dequeue()
        }
        Msg::CloseModelessModalWithProps(props) => {
            let cmd = update(state, Msg::ReflectModelessModal(props));
            state.cmd_queue.enqueue(cmd);
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

        // PersonalData
        Msg::SetPersonalDataWithPlayerName(player_name) => {
            state.personal_data.name = player_name;
            state.cmd_queue.dequeue()
        }
        Msg::SetPersonalDataWithIconImage(r_id) => {
            state.personal_data.icon = Some(r_id);
            update(state, Msg::CloseModal)
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
                state.cmd_queue.enqueue(Cmd::task(|r| r(Msg::Render)));
                update(state, Msg::CloseModal)
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::SetCharacterSize(character_id, width, height) => {
            let world = &mut state.world;
            let resource = &state.resource;
            if let Some(character) = world.character_mut(&character_id) {
                if let (Some(width), Some(height)) = (width, height) {
                    character.set_size([width, height]);
                } else if let Some(width) = width {
                    if let Some(img) = character
                        .texture_id()
                        .and_then(|id| resource.get_as_image(&id))
                    {
                        let height = width * img.height() as f64 / img.width() as f64;
                        character.set_size([width, height]);
                    }
                } else if let Some(height) = height {
                    if let Some(img) = character
                        .texture_id()
                        .and_then(|id| resource.get_as_image(&id))
                    {
                        let width = height * img.width() as f64 / img.height() as f64;
                        character.set_size([width, height]);
                    }
                }
            }
            update(state, Msg::Render)
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
        Msg::SetTablemaskSizeWithStyle(tablemask_id, size, is_rounded) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_is_rounded(is_rounded);
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
        Msg::SetTablemaskColor(tablemask_id, color) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_background_color(color);
            }
            update(state, Msg::Render)
        }

        // チャット周り
        Msg::SetSelectingChatTabIdx(tab_idx) => {
            state.chat_data.selecting_tab_idx = tab_idx;
            state.cmd_queue.dequeue()
        }
        Msg::InputChatMessage(message) => {
            state.chat_data.inputing_message = message;
            state.cmd_queue.dequeue()
        }
        Msg::SendChatItem => {
            let tab_idx = state.chat_data.selecting_tab_idx;
            let message = state.chat_data.inputing_message.as_str().trim_end().into();

            state.chat_data.inputing_message = "".into();

            let chat_item = ChatItem {
                display_name: state.personal_data.name.clone(),
                peer_id: state.peer.id(),
                icon: state
                    .personal_data
                    .icon
                    .map(|r_id| Icon::Resource(r_id))
                    .unwrap_or(Icon::DefaultUser),
                payload: message,
            };

            state.chat_data.tabs[tab_idx].items.push(chat_item);
            state.cmd_queue.dequeue()
        }
        Msg::SetChatSender(sender_idx) => {
            if sender_idx < state.chat_data.senders.len() {
                state.chat_data.selecting_sender_idx = sender_idx;
            }
            state.cmd_queue.dequeue()
        }

        // リソース
        Msg::LoadFromFileList(file_list) => {
            let len = file_list.length();
            let mut blobs = HashMap::new();
            for i in 0..len {
                if let Some(file) = file_list.item(i) {
                    let blob: web_sys::Blob = file.into();
                    let data_id = random_id::u128val();
                    blobs.insert(data_id, Rc::new(blob));
                }
            }
            update(state, Msg::LoadFromBlobs(blobs, true))
        }
        Msg::LoadFromBlobs(blobs, transport) => {
            let mut transport_data = HashMap::new();
            for (data_id, blob) in blobs {
                let cmd = Cmd::task({
                    let blob = Rc::clone(&blob);
                    move |resolve| {
                        let blob_type = blob.type_();
                        let blob_type: Vec<&str> = blob_type.split('/').collect();
                        let blob_type = blob_type.first().map(|x| x as &str).unwrap_or("");
                        if blob_type == "image" {
                            let image = Rc::new(crate::util::html_image_element());
                            let a = {
                                let image = Rc::clone(&image);
                                let blob = Rc::clone(&blob);
                                Closure::once(Box::new(move || {
                                    let object_url =
                                        web_sys::Url::create_object_url_with_blob(&blob)
                                            .unwrap_or("".into());
                                    resolve(Msg::LoadReasource(
                                        data_id,
                                        Data::Image(image, blob, Rc::new(object_url)),
                                    ));
                                }))
                            };
                            image.set_onload(Some(&a.as_ref().unchecked_ref()));
                            if let Ok(object_url) = web_sys::Url::create_object_url_with_blob(&blob)
                            {
                                image.set_src(&object_url);
                            }
                            a.forget();
                        }
                    }
                });
                state.cmd_queue.enqueue(cmd);
                state.loading_resource_num += 1;
                if transport {
                    transport_data.insert(data_id, blob);
                }
            }
            if transport {
                state
                    .room
                    .send(&skyway::Msg::SetResource(ResourceData::from(
                        transport_data,
                    )));
            }
            state.cmd_queue.dequeue()
        }

        Msg::LoadReasource(resource_id, data) => {
            state.resource.insert(resource_id, data);
            state.loading_resource_num -= 1;
            if state.loading_resource_num == 0 {
                state.loaded_resource_num = 0;
            } else {
                state.loaded_resource_num += 1;
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
                    ColorSystem::gray(255, 9),
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
                update(state, Msg::LoadFromBlobs(resource_data.into(), false))
            }
            skyway::Msg::SetConnection(peers) => {
                state.peers = peers;
                state.cmd_queue.dequeue()
            }
            skyway::Msg::RemoveObject(object_id) => {
                update(state, Msg::RemoveObjectWithObjectId(object_id, false))
            }
            skyway::Msg::None => state.cmd_queue.dequeue(),
        },
        Msg::PeerJoin(peer_id) => {
            if state.loading_state != 0 {
                state
                    .cmd_queue
                    .enqueue(Cmd::task(move |r| r(Msg::PeerJoin(peer_id))));
                Cmd::none()
            } else {
                let data_connect = Rc::new(state.peer.connect(&peer_id));
                let world_data = state.world.to_data();

                let my_peer_id = state.peer.id();

                let stride = state.peers.len();
                let n = {
                    let mut i = 0;
                    for peer_id in &state.peers {
                        if my_peer_id == peer_id as &str {
                            break;
                        }
                        i += 1;
                    }
                    i
                };

                web_sys::console::log_2(&JsValue::from(stride as u32), &JsValue::from(n as u32));

                let resource_data = state.resource.to_data_with_n_and_stride(n, stride);
                state.peers.insert(peer_id);

                let a = Closure::once(Box::new({
                    let data_connect = Rc::clone(&data_connect);
                    move || {
                        web_sys::console::log_1(&JsValue::from("send resource data"));
                        data_connect.send(&skyway::Msg::SetResource(resource_data).as_object());
                    }
                }) as Box<dyn FnOnce()>);
                data_connect.on("open", Some(a.as_ref().unchecked_ref()));
                a.forget();

                let a = Closure::once(Box::new({
                    let data_connect = Rc::clone(&data_connect);
                    let peers = state.peers.clone();
                    move || {
                        web_sys::console::log_1(&JsValue::from("send world data"));
                        data_connect.send(&skyway::Msg::SetWorld(world_data).as_object());
                        data_connect.send(&skyway::Msg::SetConnection(peers).as_object());
                    }
                }) as Box<dyn FnOnce()>);
                data_connect.on("data", Some(a.as_ref().unchecked_ref()));
                a.forget();

                state.cmd_queue.dequeue()
            }
        }
        Msg::PeerLeave(peer_id) => {
            state.peers.remove(&peer_id);
            state.cmd_queue.dequeue()
        }
        Msg::SetLoadingState(is_loading) => {
            if is_loading {
                state.loading_state += 1;
            } else {
                state.loading_state -= 1;
            }
            web_sys::console::log_1(&JsValue::from(state.loading_state as i32));
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
            .style("grid-template-rows", "max-content 1fr")
            .style("grid-template-columns", "max-content 1fr"),
        Events::new().on("drop", |e| {
            e.prevent_default();
            e.stop_propagation();
            Msg::NoOp
        }),
        vec![
            vec![
                render_header_menu(
                    &state.room.id,
                    &state.table_state.selecting_tool,
                    state.world.table().is_bind_to_grid(),
                    state.is_2d_mode,
                    state.is_low_loading_mode,
                ),
                render_side_menu(),
                render_canvas_container(&state),
                render_loading_state(state.loading_resource_num, state.loaded_resource_num),
                render_context_menu(&state.contextmenu, &state.focused_object_id, &state.world),
            ],
            render_modals(
                &state.modals,
                &state.world,
                &state.personal_data,
                &state.chat_data,
                &state.resource,
            ),
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn render_header_menu(
    room_id: &String,
    selecting_tool: &TableTool,
    is_bind_to_grid: bool,
    is_2d_mode: bool,
    is_low_loading_mode: bool,
) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .style("grid-column", "span 2")
            .class("panel grid"),
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
            Html::div(
                Attributes::new()
                    .class("grid-w-18")
                    .class("justify-r")
                    .class("centering-h"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![
                        btn::primary(
                            Attributes::new().title("画像データや音楽データなどの管理"),
                            Events::new().on_click(|_| Msg::OpenModal(Modal::Resource)),
                            vec![awesome::i("fa-layer-group"), Html::text(" リソース")],
                        ),
                        btn::primary(
                            Attributes::new().title("プレイヤー名やアイコンなどの管理"),
                            Events::new().on_click(|_| Msg::OpenModal(Modal::PersonalSetting)),
                            vec![awesome::i("fa-user-cog"), Html::text(" 個人設定")],
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
                Attributes::new()
                    .class("grid-w-12")
                    .class("linear-h")
                    .class("container-a")
                    .class("centering-v-i"),
                Events::new(),
                vec![
                    btn::selectable(
                        selecting_tool.is_selector(),
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Selector)),
                        vec![awesome::i("fa-mouse-pointer"), Html::text(" 選択")],
                    ),
                    btn::selectable(
                        selecting_tool.is_pen(),
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Pen)),
                        vec![awesome::i("fa-pen"), Html::text(" ペン")],
                    ),
                    btn::selectable(
                        selecting_tool.is_eracer(),
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Eracer)),
                        vec![awesome::i("fa-eraser"), Html::text(" 消しゴム")],
                    ),
                    btn::selectable(
                        selecting_tool.is_measure(),
                        Attributes::new(),
                        Events::new()
                            .on_click(|_| Msg::SetSelectingTableTool(TableTool::Measure(None))),
                        vec![awesome::i("fa-ruler"), Html::text(" 計測")],
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
                Attributes::new()
                    .class("grid-w-12")
                    .class("justify-r")
                    .class("centering-h"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class("keyvalue"),
                            Events::new(),
                            vec![
                                Html::span(
                                    Attributes::new().class("text-label"),
                                    Events::new(),
                                    vec![Html::text("低負荷モード")],
                                ),
                                btn::toggle(
                                    is_low_loading_mode,
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetLowLoadingMode(!is_low_loading_mode)
                                    }),
                                ),
                            ],
                        ),
                        Html::div(
                            Attributes::new().class("keyvalue"),
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
                        ),
                    ],
                )],
            ),
        ],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().class("panel linear-v"),
        Events::new(),
        vec![btn::primary(
            Attributes::new()
                .class("linear-v")
                .style("font-size", "0.8em"),
            Events::new().on_click(|_| Msg::OpenChatModeless),
            vec![
                Html::span(
                    Attributes::new().style("font-size", "2em"),
                    Events::new(),
                    vec![awesome::i("fa-comment")],
                ),
                Html::text("チャット"),
            ],
        )],
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
                &state.chat_data,
                &state.personal_data,
                &state.modelesses,
            ),
            state
                .editing_modeless
                .as_ref()
                .map(|(_, props)| {
                    Html::component(modeless_modal::new(Rc::clone(props)).subscribe(
                        |sub| match sub {
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
        Attributes::new().id("table").class("cover cover-a"),
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
    chat_tabs: &ChatDataCollection,
    personal_data: &PersonalData,
    modelesses: &ModelessCollection,
) -> Html<Msg> {
    modeless_container(
        Attributes::new().class("cover cover-a"),
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
                            modeless::object(idx, state, tabs, *focused, world, resource)
                        }
                        Modeless::Chat => {
                            modeless::chat(idx, state, chat_tabs, personal_data, world, resource)
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
                    Attributes::new()
                        .class("pure-menu-item")
                        .class("pure-menu-has-children"),
                    Events::new(),
                    vec![
                        btn::contextmenu_text(Attributes::new(), Events::new(), "サイズ"),
                        Html::ul(
                            Attributes::new().class("pure-menu-children"),
                            Events::new(),
                            vec![Html::li(
                                Attributes::new()
                                    .class("pure-menu-item")
                                    .class("linear-h")
                                    .style("display", "grid"),
                                Events::new(),
                                vec![
                                    Html::ul(
                                        Attributes::new().class("pure-menu-list"),
                                        Events::new(),
                                        vec![
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [2., 2.],
                                                        true,
                                                    )
                                                }),
                                                "半径1",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [4., 4.],
                                                        true,
                                                    )
                                                }),
                                                "半径2",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [6., 6.],
                                                        true,
                                                    )
                                                }),
                                                "半径3",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [8., 8.],
                                                        true,
                                                    )
                                                }),
                                                "半径4",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [10., 10.],
                                                        true,
                                                    )
                                                }),
                                                "半径5",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [12., 12.],
                                                        true,
                                                    )
                                                }),
                                                "半径6",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [14., 14.],
                                                        true,
                                                    )
                                                }),
                                                "半径7",
                                            ),
                                        ],
                                    ),
                                    Html::ul(
                                        Attributes::new().class("pure-menu-list"),
                                        Events::new(),
                                        vec![
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [1., 1.],
                                                        false,
                                                    )
                                                }),
                                                "矩形1×1",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [2., 2.],
                                                        false,
                                                    )
                                                }),
                                                "矩形2×2",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [3., 3.],
                                                        false,
                                                    )
                                                }),
                                                "矩形3×3",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [4., 4.],
                                                        false,
                                                    )
                                                }),
                                                "矩形4×4",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [5., 5.],
                                                        false,
                                                    )
                                                }),
                                                "矩形5×5",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [6., 6.],
                                                        false,
                                                    )
                                                }),
                                                "矩形6×6",
                                            ),
                                            btn::contextmenu_text(
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetTablemaskSizeWithStyle(
                                                        object_id,
                                                        [7., 7.],
                                                        false,
                                                    )
                                                }),
                                                "矩形7×7",
                                            ),
                                        ],
                                    ),
                                ],
                            )],
                        ),
                    ],
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| {
                        Msg::OpenModal(Modal::ColorPicker(ColorPickerType::TablemaskColor(
                            object_id,
                        )))
                    }),
                    "色を変更",
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

fn render_loading_state(loading_resource_num: u64, loaded_resource_num: u64) -> Html<Msg> {
    if loading_resource_num == 0 {
        Html::none()
    } else {
        Html::div(
            Attributes::new()
                .class("text-color-light")
                .style("position", "fixed")
                .style("top", "0em")
                .style("right", "0em"),
            Events::new(),
            vec![Html::text(format!(
                "Loading：{} / {}",
                loaded_resource_num,
                loading_resource_num + loaded_resource_num
            ))],
        )
    }
}

fn render_hint() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("text-color-secondary-d")
            .style("position", "absolute")
            .style("bottom", "5em")
            .style("right", "5em"),
        Events::new(),
        vec![Html::text("Ctrl + ドラッグ or Alt + ドラッグで視界を回転")],
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

fn render_modals(
    modals: &Vec<Modal>,
    world: &World,
    personal_data: &PersonalData,
    chat_data: &ChatDataCollection,
    resource: &Resource,
) -> Vec<Html<Msg>> {
    let mut children = vec![];
    for modal in modals {
        let child = match modal {
            Modal::Resource => modal::resource(resource),
            Modal::SelectImage(modal_type) => modal::select_image(resource, modal_type),
            Modal::PersonalSetting => modal::personal_setting(personal_data, resource),
            Modal::ColorPicker(color_picker_type) => modal::color_picker(color_picker_type.clone()),
            Modal::CharacterSelecter(character_selecter_type) => match character_selecter_type {
                CharacterSelecterType::ChatSender => modal::character_selecter(
                    chat_data
                        .senders
                        .iter()
                        .filter_map(|s| s.as_character())
                        .collect(),
                    world,
                    resource,
                ),
            },
        };
        children.push(child);
    }
    children
}
