mod render;
mod state;

use crate::{
    block::{self, BlockId},
    model::modeless::ModelessId,
    renderer::{Camera, Renderer},
    skyway,
};
use kagura::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

pub type State = state::State<Msg, Sub>;

pub enum Msg {
    NoOp,
    SetTableContext,
    ResizeCanvas,

    // Tick
    Tick1000ms,

    // Contextmenu
    OpenContextmenu([f64; 2], [f64; 2], state::Contextmenu),
    CloseContextmenu,

    // Modeless
    OpenModeless(state::Modeless),
    FocusModeless(ModelessId),
    GrubModeless(ModelessId, [f64; 2], [bool; 4]),
    DragModeless(ModelessId, [f64; 2]),
    DropModeless(ModelessId),
    CloseModeless(ModelessId),

    // Modal
    OpenModal(state::Modal),
    CloseModal,

    // UI for table
    AddChracaterWithMousePositionToCloseContextmenu([f32; 2]),
    AddTablemaskWithMousePositionToCloseContextmenu([f32; 2]),
    CloneCharacterToCloseContextmenu(BlockId),
    CloneTablemaskToCloseContextmenu(BlockId),
    RemoveCharacterToCloseContextmenu(BlockId),
    RemoveTablemaskToCloseContextmenu(BlockId),

    // Mouse
    SetLastMousePosition([f32; 2]),
    SetLastMouseDownPosition([f32; 2]),
    SetLastMouseUpPosition([f32; 2]),
    SetCameraRotationWithMouseMovement([f32; 2]),
    SetCameraMovementWithMouseMovement([f32; 2]),
    SetCameraMovementWithMouseWheel(f32),
    SetSelectingTableTool(state::table::Tool),
    SetTableObjectPositionWithMousePosition(BlockId, [f32; 2]),
    DrawLineWithMousePosition([f32; 2], [f32; 2]),
    EraceLineWithMousePosition([f32; 2], [f32; 2]),

    // World
    AddTable,

    // Table
    SetTableSize(BlockId, [f64; 2]),
    SetTableImage(BlockId, BlockId),

    // テーブル操作の制御
    SetCursorWithMouseCoord([f64; 2]),
    EraceLineWithMouseCoord([f64; 2]),
    SetMeasureStartPointAndEndPointWithMouseCoord(f64, bool, [f64; 2], [f64; 2]),
    SetObjectPositionWithMouseCoord(u128, [f64; 2]),
    BindObjectToTableGrid(u128),
    SetIs2dMode(bool),

    // PersonalData
    SetPersonalDataWithPlayerName(String),
    SetPersonalDataWithIconImage(u128),

    // table object
    SetCharacterName(BlockId, String),
    SetCharacterSize([Option<f64>; 2]),

    // property

    // チャット関係
    SetInputingChatMessage(String),
    SendInputingChatMessage,
    InsertChatItem(BlockId, block::chat::Item),
    SetChatSender(usize),
    AddChatTab,
    SetChatTabName(BlockId, String),
    RemoveChatTab(BlockId),

    // リソース管理
    LoadFromFileListToTransport(web_sys::FileList),
    LoadFromBlobsToTransport(HashMap<BlockId, Rc<web_sys::Blob>>),
    LoadFromBlobs(HashMap<BlockId, Rc<web_sys::Blob>>),
    LoadReasource(BlockId, block::Resource),

    // 接続に関する操作
    SendPacks(HashMap<BlockId, JsValue>),
    ReceiveMsg(skyway::Msg),
    PeerJoin(String),
    PeerLeave(String),
    DisconnectFromRoom,
}

pub enum Sub {
    DisconnectFromRoom,
}

pub fn new(
    peer: Rc<skyway::Peer>,
    room: Rc<skyway::Room>,
    common_database: Rc<web_sys::IdbDatabase>,
    room_database: Rc<web_sys::IdbDatabase>,
) -> Component<Msg, State, Sub> {
    let init = {
        let peer = Rc::clone(&peer);
        let room = Rc::clone(&room);
        move || {
            let state = State::new(peer, room, common_database, room_database);
            let task = Cmd::task(|handler| {
                handler(Msg::SetTableContext);
            });
            (state, task)
        }
    };
    Component::new(init, update, render)
        .batch(|mut handler| {
            let a = Closure::wrap(Box::new(move || {
                handler(Msg::ResizeCanvas);
            }) as Box<dyn FnMut()>);
            web_sys::window()
                .unwrap()
                .set_onresize(Some(a.as_ref().unchecked_ref()));
            a.forget();
        })
        .batch(batch::time::tick(1000, || Msg::Tick1000ms))
        .batch({
            let room = Rc::clone(&room);
            move |mut handler| {
                let a = Closure::wrap(Box::new({
                    move |receive_data: Option<skyway::ReceiveData>| {
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
                            web_sys::console::log_1(&JsValue::from(msg.type_name()));
                            handler(Msg::ReceiveMsg(msg));
                        } else {
                            web_sys::console::log_1(&JsValue::from("faild to deserialize message"));
                        }
                    }
                })
                    as Box<dyn FnMut(Option<skyway::ReceiveData>)>);
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

fn get_canvas_pixel_ratio(pixel_ratio: f32) -> f32 {
    web_sys::window().unwrap().device_pixel_ratio() as f32 * pixel_ratio
}

fn reset_canvas_size(pixel_ratio: f32) -> [f32; 2] {
    let canvas = get_table_canvas_element();
    let dpr = get_canvas_pixel_ratio(pixel_ratio);
    let canvas_size = [
        canvas.client_width() as f32 * dpr,
        canvas.client_height() as f32 * dpr,
    ];
    canvas.set_width(canvas_size[0] as u32);
    canvas.set_height(canvas_size[1] as u32);
    canvas_size
}

fn get_table_position(state: &State, screen_position: &[f32; 2], pixel_ratio: f32) -> [f32; 2] {
    let dpr = get_canvas_pixel_ratio(pixel_ratio);
    let mouse_coord = [screen_position[0] * dpr, screen_position[1] * dpr];
    let p = state
        .camera()
        .collision_point_on_xy_plane(state.canvas_size(), &screen_position);
    [p[0], p[1]]
}

fn render_canvas(state: &mut State) {
    if let Some(renderer) = state.renderer_mut() {
        renderer.render(
            &mut state.world,
            &state.camera,
            &state.resource,
            &state.canvas_size,
        );
    }
}

fn send_pack_cmd(block_field: &block::Field, packs: Vec<&BlockId>) -> Cmd<Msg, Sub> {
    let packs = block_field.pack_listed(packs);

    Cmd::task(move |resolve| {
        packs.then(|packs| {
            if let Ok(packs) = packs {
                resolve(Msg::SendPacks(packs.into_iter().collect()));
            }
        })
    })
}

fn clone_prop(block_field: &mut block::Field, prop: &block::Property) -> block::Property {
    let mut prop = prop.clone();

    if let block::property::Value::Children(children) = prop.value() {
        let mut new_children = vec![];

        for child in children {
            if let Some(child) = block_field.get::<block::Property>(child) {
                let child = clone_prop(block_field, child);
                let child = block_field.add(child);
                new_children.push(child);
            }
        }

        prop.set_value(block::property::Value::Children(new_children));
    }

    prop
}

fn trace_prop_id(block_field: &block::Field, prop: &BlockId) -> Vec<BlockId> {
    let mut prop_ids = vec![prop.clone()];

    if let Some(block::property::Value::Children(children)) =
        block_field.get::<block::Property>(&prop).map(|p| p.value())
    {
        for child in children {
            let child_children = trace_prop_id(block_field, child);
            for child_child in child_children {
                prop_ids.push(child_child);
            }
        }
    }

    prop_ids
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => state.dequeue(),
        Msg::SetTableContext => {
            state.set_canvas_size(reset_canvas_size(state.pixel_ratio()));
            let canvas = get_table_canvas_element();
            let gl = canvas.get_context("webgl").unwrap().unwrap();
            let gl = gl.dyn_into::<web_sys::WebGlRenderingContext>().unwrap();
            state.set_renderer(Renderer::new(gl));
            render_canvas(state);
            state.dequeue()
        }
        Msg::ResizeCanvas => {
            state.set_canvas_size(reset_canvas_size(state.pixel_ratio()));
            state.dequeue()
        }

        // Tick
        Msg::Tick1000ms => state.dequeue(),

        // Contextmenu
        Msg::OpenContextmenu(page_mouse_position, offset_mouse_position, contextmenu) => {
            state.open_contextmenu(page_mouse_position, offset_mouse_position, contextmenu);
            state.dequeue()
        }
        Msg::CloseContextmenu => {
            state.close_contextmenu();
            state.dequeue()
        }

        // Modeless
        Msg::OpenModeless(modeless) => {
            state.open_modeless(modeless);
            state.dequeue()
        }
        Msg::FocusModeless(modeless_id) => {
            state.focus_modeless(modeless_id);
            state.dequeue()
        }
        Msg::GrubModeless(modeless_id, mouse_position, movable) => {
            state.grub_modeless(modeless_id, mouse_position, movable);
            state.dequeue()
        }
        Msg::DragModeless(modeless_id, mouse_position) => {
            state.drag_modeless(modeless_id, mouse_position);
            state.dequeue()
        }
        Msg::DropModeless(modeless_id) => {
            state.drop_modeless(modeless_id);
            state.dequeue()
        }
        Msg::CloseModeless(modeless_id) => {
            state.close_modeless(modeless_id);
            state.dequeue()
        }

        // Modal
        Msg::OpenModal(modal) => {
            state.open_modal(modal);
            state.dequeue()
        }

        Msg::CloseModal => {
            state.close_modal();
            state.dequeue()
        }

        // UI
        Msg::AddChracaterWithMousePositionToCloseContextmenu(mouse_position) => {
            state.close_contextmenu();

            let [x, y] = get_table_position(state, &mouse_position, state.pixel_ratio());

            let mut prop_hp = block::Property::new("HP");
            prop_hp.set_value(block::property::Value::Num(0.0));
            let prop_hp = state.block_field_mut().add(prop_hp);

            let mut prop_mp = block::Property::new("MP");
            prop_mp.set_value(block::property::Value::Num(0.0));
            let prop_mp = state.block_field_mut().add(prop_mp);

            let mut prop_root = block::Property::new("");
            prop_root.set_value(block::property::Value::Children(vec![
                prop_hp.clone(),
                prop_mp.clone(),
            ]));
            let prop_root = state.block_field_mut().add(prop_root);

            let mut character = block::Character::new(prop_root.clone(), "キャラクター");
            character.set_position([x, y, 0.0]);
            let character = state.block_field_mut().add(character);

            let world = state.world();
            state.block_field_mut().update(
                world,
                js_sys::Date::now() as u32,
                |world: &mut block::World| {
                    world.add_character(character.clone());
                },
            );

            send_pack_cmd(
                state.block_field(),
                vec![&prop_hp, &prop_mp, &prop_root, &character, world],
            )
        }

        Msg::AddTablemaskWithMousePositionToCloseContextmenu(mouse_position) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table() {
                let [x, y] = get_table_position(state, &mouse_position, state.pixel_ratio());

                let mut prop_root = block::Property::new("");
                let prop_root = state.block_field_mut().add(prop_root);

                let tablemask = block::table_object::Tablemask::new(prop_root.clone());
                tablemask.set_position([x, y]);
                let tablemask = state.block_field_mut().add(tablemask);

                state.block_field_mut().update(
                    selecting_table,
                    js_sys::Date::now() as u32,
                    |table: &mut block::Table| {
                        table.add_tablemask(tablemask.clone());
                    },
                );

                send_pack_cmd(state.block_field(), vec![&tablemask, selecting_table])
            } else {
                state.dequeue()
            }
        }

        Msg::CloneCharacterToCloseContextmenu(character) => {
            state.close_contextmenu();

            if let Some(character) = state.block_field().get::<block::Character>(&character) {
                let mut character = character.clone();
                let prop = if let Some(prop) = state.block_field().get(character.property_id()) {
                    clone_prop(state.block_field_mut(), prop)
                } else {
                    block::Property::new("")
                };
                let prop = state.block_field_mut().add(prop);

                let mut props = trace_prop_id(state.block_field(), &prop);

                character.set_property_id(prop);
                let character = state.block_field_mut().add(character);

                let world = state.world();
                state.block_field_mut().update(
                    world,
                    js_sys::Date::now() as u32,
                    |world: &mut block::World| {
                        world.add_character(character.clone());
                    },
                );

                props.push(character);
                props.push(world.clone());

                send_pack_cmd(state.block_field(), props.iter().collect())
            } else {
                state.dequeue()
            }
        }

        Msg::CloneTablemaskToCloseContextmenu(tablemask) => {
            state.close_contextmenu();

            if let (Some(tablemask), Some(selecting_table)) = (
                state
                    .block_field()
                    .get::<block::table_object::Tablemask>(&tablemask),
                state.selecting_table(),
            ) {
                let mut tablemask = tablemask.clone();
                let prop = if let Some(prop) = state.block_field().get(tablemask.property_id()) {
                    clone_prop(state.block_field_mut(), prop)
                } else {
                    block::Property::new("")
                };
                let prop = state.block_field_mut().add(prop);

                let mut props = trace_prop_id(state.block_field(), &prop);

                tablemask.set_property_id(prop);
                let tablemask = state.block_field_mut().add(tablemask);

                state.block_field_mut().update(
                    selecting_table,
                    js_sys::Date::now() as u32,
                    |table: &mut block::Table| {
                        table.add_tablemask(tablemask.clone());
                    },
                );

                send_pack_cmd(state.block_field(), vec![&tablemask, selecting_table])
            } else {
                state.dequeue()
            }
        }

        Msg::RemoveCharacterToCloseContextmenu(character) => {
            state.close_contextmenu();

            let world = state.world();
            state.block_field_mut().update(
                world,
                js_sys::Date::now() as u32,
                |world: &mut block::World| {
                    world.remove_character(&character);
                },
            );

            send_pack_cmd(state.block_field(), vec![world])
        }

        Msg::RemoveTablemaskToCloseContextmenu(tablemask) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table() {
                state.block_field_mut().update(
                    selecting_table,
                    js_sys::Date::now() as u32,
                    |table: &mut block::Table| {
                        table.remove_tablemask(&tablemask);
                    },
                );

                send_pack_cmd(state.block_field(), vec![selecting_table])
            } else {
                state.dequeue()
            }
        }

        // Mouse
        Msg::SetLastMousePosition(mouse_position) => {
            state.table_mut().set_last_mouse_position(mouse_position);
            state.dequeue()
        }

        Msg::SetLastMouseDownPosition(mouse_position) => {
            state
                .table_mut()
                .set_last_mouse_down_position(mouse_position);
            state.dequeue()
        }

        Msg::SetLastMouseUpPosition(mouse_position) => {
            state.table_mut().set_last_mouse_up_position(mouse_position);
            state.dequeue()
        }

        Msg::SetCameraRotationWithMouseMovement(mouse_position) => {
            let dx = mouse_position[0] - state.table().last_mouse_position()[0];
            let dy = mouse_position[1] - state.table().last_mouse_position()[1];
            let long_edge = state.canvas_size()[0].max(state.canvas_size()[1]);

            let factor = 3.0 / long_edge * get_canvas_pixel_ratio(state.pixel_ratio());

            let camera = state.camera_mut();

            camera.set_x_axis_rotation(camera.x_axis_rotation() + dy * factor);
            camera.set_z_axis_rotation(camera.z_axis_rotation() + dx * factor);

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetCameraMovementWithMouseMovement(mouse_position) => {
            let dx = mouse_position[0] - state.table().last_mouse_position()[0];
            let dy = mouse_position[1] - state.table().last_mouse_position()[1];
            let long_edge = state.canvas_size()[0].max(state.canvas_size()[1]);

            let factor = 50.0 / long_edge * get_canvas_pixel_ratio(state.pixel_ratio());

            let camera = state.camera_mut();

            let mov = camera.movement();
            let mov = [mov[0] + dx * factor, mov[1] - dy * factor, mov[2]];

            camera.set_movement(mov);

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetCameraMovementWithMouseWheel(delta_y) => {
            let factor = 0.02;

            let camera = state.camera_mut();
            let mov = camera.movement();
            let mov = [mov[0], mov[1], mov[2] - factor * delta_y];

            camera.set_movement(mov);

            render_canvas(state);

            state.dequeue()
        }

        //テーブル操作の制御
        Msg::SetSelectingTableTool(table_tool) => {
            match &table_tool {
                TableTool::Measure(.., Option::None, _) => {
                    state.table_state.measure_length = None;
                    state
                        .world
                        .selecting_table_mut()
                        .map(|table| table.clear_measure());
                }
                _ => {}
            }
            state.table_state.selecting_tool = table_tool;
            update(state, Msg::Render)
        }
        Msg::SetIsBindToGridToTransport(is_bind_to_grid) => {
            let room = &state.room;
            room.send(skyway::Msg::SetIsBindToGrid(is_bind_to_grid));
            update(state, Msg::SetIsBindToGrid(is_bind_to_grid))
        }
        Msg::SetIsBindToGrid(is_bind_to_grid) => {
            state
                .world
                .selecting_table_mut()
                .map(|table| table.set_is_bind_to_grid(is_bind_to_grid));
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
            state.world.selecting_table_mut().map(|table| {
                table.draw_line(&start_point, &end_point, ColorSystem::gray(255, 9), 0.5)
            });
            state
                .room
                .send(skyway::Msg::DrawLineToTable(start_point, end_point));
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
                .selecting_table_mut()
                .map(|table| table.erace_line(&start_point, &end_point, 1.0));
            state
                .room
                .send(skyway::Msg::EraceLineToTable(start_point, end_point));
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::SetMeasureStartPointAndEndPointWithMouseCoord(
            line_width,
            rounded,
            start_point,
            mouse_coord,
        ) => {
            let start_point = get_table_position(&state, &start_point, state.pixel_ratio);
            let start_point = [start_point[0], start_point[1]];
            let end_point = get_table_position(&state, &mouse_coord, state.pixel_ratio);
            let end_point = [end_point[0], end_point[1]];
            let measure_length = state.world.selecting_table_mut().map(|table| {
                table.draw_measure(
                    &start_point,
                    &end_point,
                    ColorSystem::red(255, 5),
                    line_width,
                    rounded,
                )
            });
            state.table_state.measure_length = Some(measure_length.unwrap_or(0.0));
            update(state, Msg::SetCursorWithMouseCoord(mouse_coord))
        }
        Msg::SetObjectPositionWithMouseCoord(object_id, mouse_coord) => {
            let position = get_table_position(
                &state,
                &state.table_state.last_mouse_coord,
                state.pixel_ratio,
            );
            state.table_state.last_mouse_coord = mouse_coord;
            update(
                state,
                Msg::SetObjectPositionToTransport(object_id, [position[0], position[1], 0.0]),
            )
        }
        Msg::SetIs2dMode(is_2d_mode) => {
            if is_2d_mode {
                state.camera.set_x_axis_rotation(0.0);
                state.camera.set_z_axis_rotation(0.0);
            }
            state.is_2d_mode = is_2d_mode;
            update(state, Msg::Render)
        }
        Msg::SetTableSizeToTransport(size) => {
            let room = &state.room;
            room.send(skyway::Msg::SetTableSize(size.clone()));
            update(state, Msg::SetTableSize(size))
        }
        Msg::SetTableSize(size) => {
            state
                .world
                .selecting_table_mut()
                .map(|table| table.set_size(size));
            update(state, Msg::Render)
        }
        Msg::SetTableImageToTransport(resource_id) => {
            let room = &state.room;
            room.send(skyway::Msg::SetTableImage(resource_id));
            update(state, Msg::SetTableImage(resource_id))
        }
        Msg::SetTableImage(resource_id) => {
            if let Some(table) = state.world.selecting_table_mut() {
                table.set_image_texture_id(resource_id);
                update(state, Msg::Render)
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::AddTablemaskWithPointABToTransport(line_width, begin, end, is_rounded) => {
            let begin = get_table_position(&state, &begin, state.pixel_ratio);
            let begin = if state.world.selecting_table().is_bind_to_grid() {
                [
                    (2.0 * begin[0]).round() / 2.0,
                    (2.0 * begin[1]).round() / 2.0,
                ]
            } else {
                [begin[0], begin[1]]
            };
            let end = get_table_position(&state, &end, state.pixel_ratio);
            let end = if state.world.selecting_table().is_bind_to_grid() {
                [(2.0 * end[0]).round() / 2.0, (2.0 * end[1]).round() / 2.0]
            } else {
                [end[0], end[1]]
            };
            let r = ((end[0] - begin[0]).powi(2) + (end[1] - begin[1]).powi(2)).sqrt();
            let mut table_mask = Tablemask::new();
            if is_rounded {
                table_mask.set_is_rounded(true);
                table_mask.set_position([begin[0], begin[1], 0.0]);
                table_mask.set_size([2.0 * r, 2.0 * r]);
                table_mask.set_is_fixed(true);
            } else {
                let z_rotation = (end[1] - begin[1]).atan2(end[0] - begin[0]);
                let width = r;
                let height = line_width;
                let position = [(end[0] + begin[0]) / 2.0, (end[1] + begin[1]) / 2.0, 0.0];
                table_mask.set_is_rounded(false);
                table_mask.set_position(position);
                table_mask.set_size([width, height]);
                table_mask.set_z_rotation(z_rotation);
                table_mask.set_is_fixed(true);
                table_mask.set_background_color(ColorSystem::red(255, 5));
            }
            let cmd = update(state, Msg::AddTablemaskToTransport(table_mask));
            state.cmd_queue.enqueue(cmd);
            update(
                state,
                Msg::SetSelectingTableTool(TableTool::Measure(line_width, is_rounded, None, true)),
            )
        }

        // モードレス
        Msg::OpenObjectModeless(object_id) => {
            let modeless_id = state
                .object_id_to_modeless_id_map
                .get(&object_id)
                .map(|modeless_id| *modeless_id)
                .unwrap_or(random_id::u128val());

            if !state.modelesses.contains_key(&modeless_id) {
                state.modelesses.insert(
                    modeless_id,
                    (
                        ModelessState::new(),
                        Modeless::Object {
                            tabs: vec![object_id],
                            focused: 0,
                        },
                    ),
                );
                state
                    .object_id_to_modeless_id_map
                    .insert(object_id, modeless_id);
            }

            if let Some(insert_point) = state.modeless_dom.iter().position(|x| x.is_none()) {
                state.modeless_dom[insert_point] = Some(modeless_id);
            } else {
                state.modeless_dom.push(Some(modeless_id));
            }

            update(state, Msg::FocusModeless(modeless_id))
        }
        Msg::OpenChatModeless => {
            let modeless_id = state
                .chat_to_modeless_id_map
                .unwrap_or(random_id::u128val());

            if !state.modelesses.contains_key(&modeless_id) {
                let mut modeless_state = ModelessState::new();
                modeless_state.loc_a = [2, 2];
                modeless_state.loc_b = [8, 14];
                state
                    .modelesses
                    .insert(modeless_id, (modeless_state, Modeless::Chat));
                state.chat_to_modeless_id_map = Some(modeless_id);
            }

            if let Some(insert_point) = state.modeless_dom.iter().position(|x| x.is_none()) {
                state.modeless_dom[insert_point] = Some(modeless_id);
            } else {
                state.modeless_dom.push(Some(modeless_id));
            }
            update(state, Msg::FocusModeless(modeless_id))
        }
        Msg::CloseModeless(modeless_id) => {
            if let Some(close_point) = state
                .modeless_dom
                .iter()
                .position(|x| x.map(|x| x == modeless_id).unwrap_or(false))
            {
                state.modeless_dom[close_point] = None;
            }
            state.cmd_queue.dequeue()
        }
        Msg::GrubModeless(modeless_id, grubbed) => {
            state
                .modelesses
                .get_mut(&modeless_id)
                .map(|modeless| modeless.0.grubbed = grubbed);
            update(state, Msg::FocusModeless(modeless_id))
        }
        Msg::FocusModeless(modeless_id) => {
            if let Some(modeless) = state.modelesses.get_mut(&modeless_id) {
                state.modeless_max_z_index += 1;
                modeless.0.z_index = state.modeless_max_z_index;
            }

            state.cmd_queue.dequeue()
        }
        Msg::OpenModelessModal(modeless_id) => {
            if let Some((modeless, ..)) = state.modelesses.get_mut(&modeless_id) {
                if let Some(resizable) = modeless.grubbed {
                    let props = modeless_modal::Props {
                        origin: modeless.loc_a.clone(),
                        corner: modeless.loc_b.clone(),
                        resizable: resizable.clone(),
                    };
                    state.editing_modeless = Some((
                        modeless_id,
                        Rc::new(RefCell::new(modeless_modal::State::new(&props))),
                    ));
                }
            }
            state.cmd_queue.dequeue()
        }
        Msg::CloseModelessModal => {
            let editing_modeless = &state.editing_modeless;
            let modelesses = &mut state.modelesses;

            let modeless = editing_modeless
                .as_ref()
                .and_then(|(modeless_id, ..)| modelesses.get_mut(modeless_id));
            if let Some((modeless, ..)) = modeless {
                modeless.grubbed = None;
            }
            state.editing_modeless = None;
            state.cmd_queue.dequeue()
        }
        Msg::ReflectModelessModal(props) => {
            let editing_modeless = &state.editing_modeless;
            let modelesses = &mut state.modelesses;

            let modeless = editing_modeless
                .as_ref()
                .and_then(|(modeless_id, ..)| modelesses.get_mut(modeless_id));
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
        Msg::SetCharacterImageToTransport(character_id, data_id) => {
            if state.world.character(&character_id).is_some() {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterImage(character_id, data_id));
                state.cmd_queue.enqueue(Cmd::task(|r| r(Msg::CloseModal)));
            }
            update(state, Msg::SetCharacterImage(character_id, data_id))
        }
        Msg::SetCharacterImage(character_id, data_id) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.set_image_id(data_id);
                if let Some(img) = state.resource.get_as_image(&data_id) {
                    let width = character.size()[0];
                    let height = width * img.height() as f64 / img.width() as f64;
                    character.set_size([width, height]);
                }
                update(state, Msg::Render)
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::SetCharacterSizeToTransport(character_id, width, height) => {
            let cmd = update(state, Msg::SetCharacterSize(character_id, width, height));
            if let Some(character) = state.world.character(&character_id) {
                state.room.send(skyway::Msg::SetCharacterSize(
                    character_id,
                    character.size().clone(),
                ));
            }
            cmd
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
        Msg::SetCharacterNameToTransport(character_id, name) => {
            if state.world.character(&character_id).is_some() {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterName(character_id, name.clone()));
            }
            update(state, Msg::SetCharacterName(character_id, name))
        }
        Msg::SetCharacterName(character_id, name) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.set_name(name);
            }
            state.cmd_queue.dequeue()
        }
        Msg::AddChracaterToTransport(character) => {
            let character_data = character.as_data();
            let character_id = state.world.add_character(character);
            let room = &state.room;
            room.send(skyway::Msg::CreateCharacterToTable(
                character_id,
                character_data,
            ));
            update(state, Msg::Render)
        }
        Msg::AddTablemaskToTransport(tablemask) => {
            let tablemask_data = tablemask.as_data();
            let tablemask_id = state.world.add_tablemask(tablemask);
            let room = &state.room;
            room.send(skyway::Msg::CreateTablemaskToTable(
                tablemask_id,
                tablemask_data,
            ));
            update(state, Msg::Render)
        }
        Msg::SetTablemaskSizeWithStyleToTransport(tablemask_id, size, is_rounded, is_fixed) => {
            if state.world.tablemask(&tablemask_id).is_some() {
                let room = &state.room;
                room.send(skyway::Msg::SetTablemaskSizeWithStyle(
                    tablemask_id,
                    size.clone(),
                    is_rounded,
                    is_fixed,
                ));
            }
            update(
                state,
                Msg::SetTablemaskSizeWithStyle(tablemask_id, size, is_rounded, is_fixed),
            )
        }
        Msg::SetTablemaskSizeWithStyle(tablemask_id, size, is_rounded, is_fixed) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_is_rounded(is_rounded);
                tablemask.set_size(size);
                tablemask.set_is_fixed(is_fixed);
            }
            update(state, Msg::Render)
        }
        Msg::SetTablemaskSizeIsBindedToTransport(tablemask_id, is_binded) => {
            update(
                state,
                Msg::SetTablemaskSizeIsBinded(tablemask_id, is_binded),
            );
            todo!();
        }
        Msg::SetTablemaskSizeIsBinded(tablemask_id, is_binded) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_size_is_binded(is_binded);
            }
            update(state, Msg::Render)
        }
        Msg::SetTablemaskColorToTransport(tablemask_id, color) => {
            if state.world.tablemask(&tablemask_id).is_some() {
                let room = &state.room;
                room.send(skyway::Msg::SetTablemaskColor(tablemask_id, color.to_u32()));
            }
            update(state, Msg::SetTablemaskColor(tablemask_id, color))
        }
        Msg::SetTablemaskColor(tablemask_id, color) => {
            if let Some(tablemask) = state.world.tablemask_mut(&tablemask_id) {
                tablemask.set_background_color(color);
            }
            update(state, Msg::Render)
        }
        Msg::SetTablemaskTransparentToTransport(tablemask_id, tranparent) => {
            if let Some(tablemask) = state.world.tablemask(&tablemask_id) {
                let mut color = Color::from(tablemask.background_color().to_u32());
                color.alpha = (255.0 * tranparent) as u8;
                let msg = Msg::SetTablemaskColorToTransport(tablemask_id, color);
                update(state, msg)
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::SetObjectPositionToTransport(object_id, position) => {
            let room = &state.room;
            room.send(skyway::Msg::SetObjectPosition(object_id, position.clone()));
            update(state, Msg::SetObjectPosition(object_id, position))
        }
        Msg::SetObjectPosition(object_id, position) => {
            if let Some(character) = state.world.character_mut(&object_id) {
                character.set_position(position);
            }
            if let Some(tablemask) = state.world.tablemask_mut(&object_id) {
                if !tablemask.is_fixed() {
                    tablemask.set_position(position)
                }
            }
            update(state, Msg::Render)
        }
        Msg::BindObjectToTableGridToTransport(object_id) => {
            if state.world.selecting_table().is_bind_to_grid() {
                let room = &state.room;
                room.send(skyway::Msg::BindObjectToTableGrid(object_id));
            }
            update(state, Msg::BindObjectToTableGrid(object_id))
        }
        Msg::BindObjectToTableGrid(object_id) => {
            if state.world.selecting_table().is_bind_to_grid() {
                if let Some(character) = state.world.character_mut(&object_id) {
                    character.bind_to_grid();
                }
                if let Some(tablemask) = state.world.tablemask_mut(&object_id) {
                    tablemask.bind_to_grid();
                }
            }
            update(state, Msg::Render)
        }
        Msg::SetCharacterPropertyNameToTransport(character_id, property_id, property_name) => {
            let cmd = update(
                state,
                Msg::SetCharacterPropertyName(character_id, property_id, property_name),
            );
            if let Some(character) = state.world.character(&character_id) {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterProperty(
                    character_id,
                    character.property.as_object(),
                ));
            }
            cmd
        }
        Msg::SetCharacterPropertyName(character_id, property_id, property_name) => {
            if let Some(property) = state
                .world
                .character_mut(&character_id)
                .and_then(|c| c.property.get_mut(&property_id))
            {
                property.set_name(property_name);
            }
            state.cmd_queue.dequeue()
        }
        Msg::SetCharacterPropertyValueToTransport(character_id, property_id, property_value) => {
            let cmd = update(
                state,
                Msg::SetCharacterPropertyValue(character_id, property_id, property_value),
            );
            if let Some(character) = state.world.character(&character_id) {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterProperty(
                    character_id,
                    character.property.as_object(),
                ));
            }
            cmd
        }
        Msg::SetCharacterPropertyValue(character_id, property_id, property_value) => {
            if let Some(property) = state
                .world
                .character_mut(&character_id)
                .and_then(|c| c.property.get_mut(&property_id))
            {
                property.set_value(property_value);
            }
            state.cmd_queue.dequeue()
        }
        Msg::AddChildToCharacterPropertyToTransport(character_id, property_id, child_property) => {
            update(
                state,
                Msg::AddChildToCharacterProperty(character_id, property_id, child_property),
            );
            if let Some(character) = state.world.character(&character_id) {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterProperty(
                    character_id,
                    character.property.as_object(),
                ));
            }
            state.cmd_queue.dequeue()
        }
        Msg::AddChildToCharacterProperty(character_id, property_id, child_property) => {
            if let Some(property) = state
                .world
                .character_mut(&character_id)
                .and_then(|c| c.property.get_mut(&property_id))
            {
                property.push(child_property);
            }
            state.cmd_queue.dequeue()
        }
        Msg::RemoveCharacterPropertyToTransport(character_id, property_id) => {
            update(
                state,
                Msg::RemoveCharacterProperty(character_id, property_id),
            );
            if let Some(character) = state.world.character(&character_id) {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterProperty(
                    character_id,
                    character.property.as_object(),
                ));
            }
            state.cmd_queue.dequeue()
        }
        Msg::RemoveCharacterProperty(character_id, property_id) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                character.property.remove(property_id);
            }
            state.cmd_queue.dequeue()
        }
        Msg::SetCharacterPropertyIsSelectedToShowToTransport(
            character_id,
            property_id,
            is_selected_to_show,
        ) => {
            update(
                state,
                Msg::SetCharacterPropertyIsSelectedToShow(
                    character_id,
                    property_id,
                    is_selected_to_show,
                ),
            );
            if let Some(character) = state.world.character(&character_id) {
                let room = &state.room;
                room.send(skyway::Msg::SetCharacterProperty(
                    character_id,
                    character.property.as_object(),
                ));
            }
            state.cmd_queue.dequeue()
        }
        Msg::SetCharacterPropertyIsSelectedToShow(
            character_id,
            property_id,
            is_selected_to_show,
        ) => {
            if let Some(character) = state.world.character_mut(&character_id) {
                if let Some(property) = character.property.get_mut(&property_id) {
                    property.set_is_selected_to_show(is_selected_to_show);
                }
            }
            state.cmd_queue.dequeue()
        }
        Msg::SetSelectingTableToTransport(table_id) => {
            let room = &state.room;
            room.send(skyway::Msg::SetSelectingTable(table_id));
            update(state, Msg::SetSelectingTable(table_id))
        }
        Msg::SetSelectingTable(table_id) => {
            state.world.set_selecting_table_id(table_id);
            update(state, Msg::Render)
        }
        Msg::SetTableNameToTransport(table_id, name) => {
            let room = &state.room;
            room.send(skyway::Msg::SetTableName(table_id, name.clone()));
            update(state, Msg::SetTableName(table_id, name))
        }
        Msg::SetTableName(table_id, name) => {
            if let Some(table) = state.world.table_mut(&table_id) {
                table.set_name(name);
            }
            state.cmd_queue.dequeue()
        }
        Msg::AddTableToTransport => {
            let table_id = random_id::u128val();
            let room = &state.room;
            room.send(skyway::Msg::CreateTable(table_id));
            update(state, Msg::AddTable(table_id))
        }
        Msg::AddTable(table_id) => {
            state.world.add_table_with_id(table_id);
            state.cmd_queue.dequeue()
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
        Msg::SendChatItemToTransport => {
            let sender = &state.chat_data.senders[state.chat_data.selecting_sender_idx];
            let message: String = state.chat_data.inputing_message.drain(..).collect();
            let message: String = message.as_str().trim_end().into();

            if message.as_str().len() > 0 {
                let sender = match sender {
                    ChatSender::Player => {
                        state.dice_bot.run_time.set_ref(sainome::Ref::new(None));
                        Some((
                            state.personal_data.name.clone(),
                            None,
                            state
                                .personal_data
                                .icon
                                .map(|r_id| Icon::Resource(r_id))
                                .unwrap_or(Icon::DefaultUser),
                        ))
                    }
                    ChatSender::Character(character_id) => {
                        if let Some(character) = state.world.character(character_id) {
                            let r = character.property.as_sainome_ref();
                            state.dice_bot.run_time.set_ref(r);
                            Some((
                                character.name().clone(),
                                Some(*character_id),
                                character
                                    .texture_id()
                                    .map(|r_id| Icon::Resource(r_id))
                                    .unwrap_or(Icon::DefaultUser),
                            ))
                        } else {
                            None
                        }
                    }
                };

                if let Some((display_name, character_id, icon)) = sender {
                    let tab_idx = state.chat_data.selecting_tab_idx;

                    let (bot_msg, chat_cmd) = {
                        state.dice_bot.run_time.clear_log();

                        let run_time = &state.dice_bot.run_time;
                        let config = &state.dice_bot.config;
                        let chat_cmd = message.as_str().split_whitespace().collect::<Vec<&str>>();
                        let chat_cmd = chat_cmd
                            .get(0)
                            .map(|x| dice_bot::cmd_with_config(x.to_string(), &config));
                        let chat_cmd_result = chat_cmd
                            .as_ref()
                            .and_then(move |x| sainome::exec(x, &run_time).0);

                        let bot_msg = if let Some(result) = chat_cmd_result {
                            match result {
                                sainome::ExecResult::Err(..) => None,
                                _ => {
                                    let mut msgs = run_time.log().clone();
                                    msgs.push(format!("{}", result));
                                    Some(msgs.join(" → "))
                                }
                            }
                        } else {
                            None
                        };

                        (bot_msg, chat_cmd)
                    };

                    let chat_item =
                        ChatItem::new(display_name, state.peer.id(), character_id, icon, message);

                    state.room.send(skyway::Msg::InsertChatItem(
                        tab_idx as u32,
                        chat_item.as_object(),
                    ));

                    let cmd = update(state, Msg::InsertChatItem(tab_idx, chat_item));
                    state.cmd_queue.enqueue(cmd);

                    if let (Some(chat_cmd), Some(bot_msg)) = (chat_cmd, bot_msg) {
                        let chat_item = ChatItem::new(
                            "DiceBot",
                            state.peer.id(),
                            None,
                            Icon::None,
                            chat_cmd + " → " + &bot_msg,
                        );
                        state.room.send(skyway::Msg::InsertChatItem(
                            tab_idx as u32,
                            chat_item.as_object(),
                        ));
                        let cmd = update(state, Msg::InsertChatItem(tab_idx, chat_item));
                        state.cmd_queue.enqueue(cmd);
                    }
                }
            }
            Cmd::task(|_| {
                if let Some(el) = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("chat-area")
                {
                    el.set_scroll_top(el.scroll_height());
                }
            })
        }
        Msg::InsertChatItem(tab_idx, chat_item) => {
            let tabs = &mut state.chat_data.tabs;
            let world = &state.world;
            let canvas_size = &state.canvas_size;
            let camera = &state.camera;
            let pixel_ratio = state.pixel_ratio;

            if let Some(speech_bubble) = tabs.get_mut(tab_idx).and_then(|tab| {
                let speech_bubble = chat_item
                    .character_id()
                    .and_then(|character_id| world.character(&character_id))
                    .map(|character| {
                        let vertex = [0.0, character.size()[1], 0.0];
                        let position = Renderer::table_position(
                            &vertex,
                            character.position(),
                            camera,
                            canvas_size,
                            true,
                        );
                        let dpr = get_device_pixel_ratio(pixel_ratio);
                        let x = (position[0] + 1.0) / 2.0 * canvas_size[0] / dpr;
                        let y = -(position[1] - 1.0) / 2.0 * canvas_size[1] / dpr;
                        SpeechBubble {
                            texture_id: character.texture_id(),
                            position: [x, y],
                            message: chat_item.payload().clone(),
                        }
                    });
                tab.push(chat_item);
                speech_bubble
            }) {
                update(state, Msg::EnqueueSpeechBubble(speech_bubble))
            } else {
                state.cmd_queue.dequeue()
            }
        }
        Msg::SetChatSender(sender_idx) => {
            if sender_idx < state.chat_data.senders.len() {
                state.chat_data.selecting_sender_idx = sender_idx;
            }
            state.cmd_queue.dequeue()
        }
        Msg::AddChatSender(sender) => {
            state.chat_data.senders.push(sender);
            state.cmd_queue.dequeue()
        }
        Msg::RemoveChatSender(sender) => {
            let old_senders = state.chat_data.senders.drain(..);
            state.chat_data.senders = old_senders.into_iter().filter(|s| *s != sender).collect();
            state.cmd_queue.dequeue()
        }
        Msg::EnqueueSpeechBubble(sppech_bubble) => {
            state.speech_bubble_queue.push_back(sppech_bubble);
            Cmd::task(|resolve| {
                let a = Closure::once(
                    Box::new(|| resolve(Msg::DequeueSpeechBubble)) as Box<dyn FnOnce()>
                );
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        a.as_ref().unchecked_ref(),
                        2500,
                    );
                a.forget();
            })
        }
        Msg::DequeueSpeechBubble => {
            state.speech_bubble_queue.pop_front();
            state.cmd_queue.dequeue()
        }
        Msg::AddChatTabTotransport => {
            let room = &state.room;
            room.send(skyway::Msg::AddChatTab);
            update(state, Msg::AddChatTab)
        }
        Msg::AddChatTab => {
            let chat_tab = ChatTab::new("タブ");
            state.chat_data.tabs.push(chat_tab);
            state.cmd_queue.dequeue()
        }
        Msg::SetChatTabNameToTransport(idx, name) => {
            let room = &state.room;
            room.send(skyway::Msg::SetChatTabName(idx as u32, name.clone()));
            update(state, Msg::SetChatTabName(idx, name))
        }
        Msg::SetChatTabName(idx, name) => {
            if let Some(tab) = state.chat_data.tabs.get_mut(idx) {
                tab.set_name(name);
            }
            state.cmd_queue.dequeue()
        }
        Msg::RemoveChatTabToTransport(idx) => {
            let room = &state.room;
            room.send(skyway::Msg::RemoveChatTab(idx as u32));
            update(state, Msg::RemoveChatTab(idx))
        }
        Msg::RemoveChatTab(idx) => {
            if state.chat_data.tabs.len() > 1 {
                if state.chat_data.selecting_tab_idx == state.chat_data.tabs.len() - 1 {
                    state.chat_data.selecting_tab_idx -= 1;
                }
                state.chat_data.tabs.remove(idx);
            }
            state.cmd_queue.dequeue()
        }

        // リソース
        Msg::LoadFromFileListToTransport(file_list) => {
            let len = file_list.length();
            let mut blobs = HashMap::new();
            for i in 0..len {
                if let Some(file) = file_list.item(i) {
                    let blob: web_sys::Blob = file.into();
                    let data_id = random_id::u128val();
                    blobs.insert(data_id, Rc::new(blob));
                }
            }
            update(state, Msg::LoadFromBlobsToTransport(blobs))
        }
        Msg::LoadFromBlobsToTransport(blobs) => {
            let mut transport_data = HashMap::new();
            for (data_id, blob) in &blobs {
                let data_id = *data_id;
                let blob = Rc::clone(blob);
                transport_data.insert(data_id, blob);
            }
            if !transport_data.is_empty() {
                let room = &state.room;
                room.send(skyway::Msg::SetResource(ResourceData::from(transport_data)));
                update(state, Msg::LoadFromBlobs(blobs))
            } else {
                Cmd::none()
            }
        }
        Msg::LoadFromBlobs(blobs) => {
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

        // IndexedDB
        Msg::AddCharacterToDb(character_id) => {
            if let Some(character) = state.world.character(&character_id) {
                let texture_id = character.texture_id();
                let character: JsObject = character.as_data().into();
                let character: js_sys::Object = character.into();
                idb::query(
                    &state.common_database,
                    "characters",
                    idb::Query::Add(
                        &JsValue::from(character_id.to_string()),
                        &JsValue::from(&character),
                    ),
                    {
                        let texture_id = texture_id.clone();
                        move |_| {
                            texture_id
                                .map(|texture_id| Msg::AddResourceToDb(texture_id))
                                .unwrap_or(Msg::NoOp)
                        }
                    },
                    move |_| Msg::PutCharacterToDb(character_id, character, texture_id),
                )
            } else {
                Cmd::none()
            }
        }
        Msg::PutCharacterToDb(character_id, character, texture_id) => idb::query(
            &state.common_database,
            "characters",
            idb::Query::Put(
                &JsValue::from(character_id.to_string()),
                &JsValue::from(character),
            ),
            {
                let texture_id = texture_id.clone();
                move |_| {
                    texture_id
                        .map(|texture_id| Msg::AddResourceToDb(texture_id))
                        .unwrap_or(Msg::NoOp)
                }
            },
            |_| Msg::NoOp,
        ),
        Msg::AddResourceToDb(resource_id) => {
            if let Some(data) = state.resource.get_blob(&resource_id) {
                let data = data.as_ref().clone().dyn_into::<js_sys::Object>().unwrap();
                idb::query(
                    &state.common_database,
                    "resources",
                    idb::Query::Add(
                        &JsValue::from(resource_id.to_string()),
                        &JsValue::from(&data),
                    ),
                    |_| Msg::NoOp,
                    move |_| Msg::PutResourceToDb(resource_id, data),
                )
            } else {
                Cmd::none()
            }
        }
        Msg::PutResourceToDb(resource_id, data) => idb::query(
            &state.common_database,
            "resources",
            idb::Query::Put(
                &JsValue::from(resource_id.to_string()),
                &JsValue::from(data),
            ),
            |_| Msg::NoOp,
            |_| Msg::NoOp,
        ),
        Msg::GetCharacterListFromDbToOpenModal => idb::query(
            &state.common_database,
            "characters",
            idb::Query::GetAllKeys,
            move |x| Msg::FinishToGetCharacterListFromDbToOpenModal(x),
            |_| Msg::NoOp,
        ),
        Msg::FinishToGetCharacterListFromDbToOpenModal(keys) => {
            let raw_keys = js_sys::Array::from(&keys).to_vec();
            let mut keys = vec![];

            for raw_key in raw_keys {
                if let Some(key) = raw_key.as_string().and_then(|x| x.parse().ok()) {
                    keys.push(key);
                }
            }

            state.loading_character_keys = keys;
            get_character_with_key_from_db_to_open_modal(
                &state.common_database,
                state.loading_character_keys.pop(),
            )
        }
        Msg::FinishToGetCharacterWithKeyFromDbToOpenModal(x) => {
            if let Ok(x) = x.dyn_into::<JsObject>() {
                let character = model::CharacterData::from(x);
                let character: Character = character.into();
                state.loading_characters.push(character);
            }
            get_character_with_key_from_db_to_open_modal(
                &state.common_database,
                state.loading_character_keys.pop(),
            )
        }

        // 接続に関する操作
        Msg::ReceiveMsg(msg) => match msg {
            skyway::Msg::SetSelectingTable(table_id) => {
                update(state, Msg::SetSelectingTable(table_id))
            }
            skyway::Msg::SetTableName(table_id, name) => {
                update(state, Msg::SetTableName(table_id, name))
            }
            skyway::Msg::CreateTable(table_id) => update(state, Msg::AddTable(table_id)),
            skyway::Msg::CreateCharacterToTable(character_id, character) => {
                let character: Character = character.into();
                state.world.add_character_with_id(character_id, character);
                update(state, Msg::Render)
            }
            skyway::Msg::CreateTablemaskToTable(tablemask_id, tablemask) => {
                let tablemask: Tablemask = tablemask.into();
                state.world.add_tablemask_with_id(tablemask_id, tablemask);
                update(state, Msg::Render)
            }
            skyway::Msg::SetTableSize(size) => update(state, Msg::SetTableSize(size)),
            skyway::Msg::SetTableImage(data_id) => update(state, Msg::SetTableImage(data_id)),
            skyway::Msg::DrawLineToTable(start_point, end_point) => {
                if let Some(table) = state.world.selecting_table_mut() {
                    table.draw_line(&start_point, &end_point, ColorSystem::gray(255, 9), 0.5);
                }
                update(state, Msg::Render)
            }
            skyway::Msg::EraceLineToTable(start_point, end_point) => {
                if let Some(table) = state.world.selecting_table_mut() {
                    table.erace_line(&start_point, &end_point, 1.0);
                }
                update(state, Msg::Render)
            }
            skyway::Msg::SetCharacterImage(character_id, data_id) => {
                update(state, Msg::SetCharacterImage(character_id, data_id))
            }
            skyway::Msg::SetCharacterSize(character_id, size) => update(
                state,
                Msg::SetCharacterSize(character_id, Some(size[0]), Some(size[1])),
            ),
            skyway::Msg::SetCharacterName(character_id, name) => {
                update(state, Msg::SetCharacterName(character_id, name))
            }
            skyway::Msg::SetCharacterProperty(character_id, prop) => {
                let prop = Property::from(prop);
                if let Some(character) = state.world.character_mut(&character_id) {
                    character.property = prop;
                }
                state.cmd_queue.dequeue()
            }
            skyway::Msg::SetTablemaskSizeWithStyle(tablemask_id, size, is_rounded, is_fixed) => {
                update(
                    state,
                    Msg::SetTablemaskSizeWithStyle(tablemask_id, size, is_rounded, is_fixed),
                )
            }
            skyway::Msg::SetTablemaskColor(tablemask_id, color) => update(
                state,
                Msg::SetTablemaskColor(tablemask_id, Color::from(color)),
            ),
            skyway::Msg::SetObjectPosition(object_id, position) => {
                update(state, Msg::SetObjectPosition(object_id, position))
            }
            skyway::Msg::BindObjectToTableGrid(object_id) => {
                update(state, Msg::BindObjectToTableGrid(object_id))
            }
            skyway::Msg::SetIsBindToGrid(is_bind_to_grid) => {
                update(state, Msg::SetIsBindToGrid(is_bind_to_grid))
            }
            skyway::Msg::SetWorld(world_data) => {
                state.world = world_data.into();
                update(state, Msg::Render)
            }
            skyway::Msg::SetResource(resource_data) => {
                update(state, Msg::LoadFromBlobs(resource_data.into()))
            }
            skyway::Msg::SetChat(chat) => {
                state.chat_data.tabs = Chat::from(chat);
                Cmd::none()
            }
            skyway::Msg::SetConnection(peers) => {
                state.peers = peers;
                state.cmd_queue.dequeue()
            }
            skyway::Msg::RemoveObject(object_id) => {
                update(state, Msg::RemoveObjectWithObjectId(object_id))
            }
            skyway::Msg::InsertChatItem(tab_idx, item) => update(
                state,
                Msg::InsertChatItem(tab_idx as usize, ChatItem::from(item)),
            ),
            skyway::Msg::AddChatTab => update(state, Msg::AddChatTab),
            skyway::Msg::SetChatTabName(tab_idx, name) => {
                update(state, Msg::SetChatTabName(tab_idx as usize, name))
            }
            skyway::Msg::RemoveChatTab(tab_idx) => {
                update(state, Msg::RemoveChatTab(tab_idx as usize))
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
                let world_data = state.world.as_data();
                let chat = state.chat_data.tabs.as_object();

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

                // let resource_data = state.resource.to_data_with_n_and_stride(n, stride);
                let resource_data = state.resource.to_data();
                state.peers.insert(peer_id);

                let a = RefCell::new(Some(Box::new({
                    let data_connect = Rc::clone(&data_connect);
                    move || {
                        web_sys::console::log_1(&JsValue::from("send resource data"));
                        let msg: JsObject = skyway::Msg::SetResource(resource_data).into();
                        data_connect.send(&msg);
                    }
                }) as Box<dyn FnOnce()>));
                let a = Closure::wrap(Box::new(move || {
                    if let Some(a) = a.borrow_mut().take() {
                        a();
                    }
                }) as Box<dyn FnMut()>);
                data_connect.on("open", Some(a.as_ref().unchecked_ref()));
                a.forget();

                let a = RefCell::new(Some(Box::new({
                    let data_connect = Rc::clone(&data_connect);
                    let peers = state.peers.clone();
                    move || {
                        web_sys::console::log_1(&JsValue::from("send world data"));
                        let msg: JsObject = skyway::Msg::SetWorld(world_data).into();
                        data_connect.send(&msg);
                        let msg: JsObject = skyway::Msg::SetConnection(peers).into();
                        data_connect.send(&msg);
                        let msg: JsObject = skyway::Msg::SetChat(chat).into();
                        data_connect.send(&msg);
                    }
                }) as Box<dyn FnOnce()>));
                let a = Closure::wrap(Box::new(move || {
                    if let Some(a) = a.borrow_mut().take() {
                        a();
                    }
                }) as Box<dyn FnMut()>);
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
            .class("fullscreen")
            .class("unselectable")
            .class("app")
            .style("grid-template-columns", "max-content 1fr"),
        Events::new()
            .on("dragover", |e| {
                e.prevent_default();
                Msg::NoOp
            })
            .on("drop", |e| {
                e.prevent_default();
                e.stop_propagation();
                let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                e.data_transfer()
                    .unwrap()
                    .files()
                    .map(|files| Msg::LoadFromFileListToTransport(files))
                    .unwrap_or(Msg::NoOp)
            }),
        vec![
            render_header_menu(
                &state.room.id,
                &state.table_state.selecting_tool,
                state.is_2d_mode,
                state.is_low_loading_mode,
            ),
            render_side_menu(),
            render_canvas_container(&state),
            render_loading_state(state.loading_resource_num, state.loaded_resource_num),
            render_context_menu(&state.contextmenu, &state.focused_object_id, &state.world),
            render_modals(
                &state.modals,
                &state.world,
                &state.personal_data,
                &state.chat_data,
                &state.resource,
            ),
        ],
    )
}

fn render_header_menu(
    room_id: &String,
    selecting_tool: &TableTool,
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
                    .class("centering-v-i")
                    .class("pure-form"),
                Events::new(),
                vec![
                    vec![
                        btn::selectable(
                            selecting_tool.is_selector(),
                            Attributes::new(),
                            Events::new()
                                .on_click(|_| Msg::SetSelectingTableTool(TableTool::Selector)),
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
                            Events::new()
                                .on_click(|_| Msg::SetSelectingTableTool(TableTool::Eracer)),
                            vec![awesome::i("fa-eraser"), Html::text(" 消しゴム")],
                        ),
                        btn::selectable(
                            selecting_tool.is_measure(),
                            Attributes::new(),
                            Events::new().on_click(|_| {
                                Msg::SetSelectingTableTool(TableTool::Measure(
                                    0.2, false, None, false,
                                ))
                            }),
                            vec![awesome::i("fa-ruler"), Html::text(" 計測")],
                        ),
                    ],
                    table_tool_option(selecting_tool),
                ]
                .into_iter()
                .flatten()
                .collect(),
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

fn table_tool_option(selecting_tool: &TableTool) -> Vec<Html<Msg>> {
    match selecting_tool {
        TableTool::Selector => vec![],
        TableTool::Pen => vec![],
        TableTool::Eracer => vec![],
        TableTool::Measure(line_width, rounded, start_point, with_table_mask) => {
            let rounded = *rounded;
            let line_width = *line_width;
            let with_table_mask = *with_table_mask;
            vec![
                Html::div(
                    Attributes::new().class("keyvalue"),
                    Events::new(),
                    vec![
                        Html::span(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("太さ")],
                        ),
                        Html::input(
                            Attributes::new()
                                .value(line_width.to_string())
                                .type_("number")
                                .string("step", "0.1"),
                            Events::new().on_input({
                                let start_point = start_point.clone();
                                move |w| {
                                    w.parse()
                                        .map(|w| {
                                            Msg::SetSelectingTableTool(TableTool::Measure(
                                                w,
                                                rounded,
                                                start_point,
                                                with_table_mask,
                                            ))
                                        })
                                        .unwrap_or(Msg::NoOp)
                                }
                            }),
                            vec![],
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
                            vec![Html::text("円弧")],
                        ),
                        btn::toggle(
                            rounded,
                            Attributes::new(),
                            Events::new().on_click({
                                let start_point = start_point.clone();
                                move |_| {
                                    Msg::SetSelectingTableTool(TableTool::Measure(
                                        line_width,
                                        !rounded,
                                        start_point,
                                        with_table_mask,
                                    ))
                                }
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
                            vec![Html::text("マップマスクを生成")],
                        ),
                        btn::toggle(
                            with_table_mask,
                            Attributes::new(),
                            Events::new().on_click({
                                let start_point = start_point.clone();
                                move |_| {
                                    Msg::SetSelectingTableTool(TableTool::Measure(
                                        line_width,
                                        rounded,
                                        start_point,
                                        !with_table_mask,
                                    ))
                                }
                            }),
                        ),
                    ],
                ),
            ]
        }
    }
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().class("panel linear-v"),
        Events::new(),
        vec![
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenChatModeless),
                vec![awesome::i("fa-comments"), Html::text("チャット")],
            ),
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenModal(Modal::TableSetting)),
                vec![awesome::i("fa-layer-group"), Html::text("テーブル設定")],
            ),
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenModal(Modal::Resource)),
                vec![awesome::i("fa-folder"), Html::text("画像")],
            ),
        ],
    )
}

fn render_canvas_container(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .style("position", "relative")
            .style("z-index", "0"),
        Events::new(),
        vec![
            render_canvas(),
            render_speech_bubble_queue(&state.speech_bubble_queue, &state.resource),
            render_measure_length(&state.table_state.measure_length),
            render_hint(),
            render_table_character_list(
                state.world.characters().map(|(_, x)| x).collect(),
                &state.resource,
            ),
            render_canvas_overlaper(
                &state.table_state,
                &state.focused_object_id,
                state.is_2d_mode,
                &state.world,
                &state.resource,
                &state.chat_data,
                &state.personal_data,
                &state.modelesses,
                &state.modeless_dom,
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

fn render_speech_bubble_queue(
    speech_bubble_queue: &VecDeque<SpeechBubble>,
    resource: &Resource,
) -> Html<Msg> {
    modeless_container(
        Attributes::new().class("cover cover-a"),
        Events::new(),
        speech_bubble_queue
            .iter()
            .map(|speech_bubble| {
                Html::div(
                    Attributes::new()
                        .class("speechbubble")
                        .style("position", "absolute")
                        .style("left", format!("{}px", speech_bubble.position[0]))
                        .style("top", format!("{}px", speech_bubble.position[1])),
                    Events::new(),
                    vec![
                        speech_bubble
                            .texture_id
                            .and_then(|texture_id| resource.get_as_image_url(&texture_id))
                            .map(|image_url| {
                                Html::img(
                                    Attributes::new()
                                        .class("pure-img")
                                        .class("speechbubble-img")
                                        .string("src", image_url.as_str()),
                                    Events::new(),
                                    vec![],
                                )
                            })
                            .unwrap_or(Html::none()),
                        Html::pre(
                            Attributes::new().class("speechbubble-message"),
                            Events::new(),
                            vec![Html::text(&speech_bubble.message)],
                        ),
                    ],
                )
            })
            .collect(),
    )
}

fn render_table_character_list(characters: Vec<&Character>, resource: &Resource) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .class("cover-a")
            .class("flex-v"),
        Events::new(),
        characters
            .into_iter()
            .map(|character| render_table_character_list_item(character, resource))
            .collect(),
    )
}

fn render_table_character_list_item(character: &Character, resource: &Resource) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("chat-item")
            .class("bg-color-light-t")
            .class("container-a"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new()
                    .class("chat-icon linear-v")
                    .style("justify-items", "center"),
                Events::new(),
                vec![{
                    let icon = character
                        .texture_id()
                        .map(|r_id| Icon::Resource(r_id))
                        .unwrap_or(Icon::DefaultUser);
                    common::chat_icon(
                        Attributes::new().class("icon-medium"),
                        &icon,
                        character.name(),
                        resource,
                    )
                }],
            ),
            Html::div(
                Attributes::new().class("chat-args"),
                Events::new(),
                vec![Html::text(character.name())],
            ),
            Html::div(
                Attributes::new()
                    .class("chat-payload")
                    .class("keyvalue")
                    .class("keyvalue-align-start"),
                Events::new(),
                render_table_character_list_item_payload(character.property.selecteds()),
            ),
        ],
    )
}

fn render_table_character_list_item_payload(props: Vec<&Property>) -> Vec<Html<Msg>> {
    props
        .into_iter()
        .map(|prop| match prop.value() {
            PropertyValue::Children(children) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(
                    Attributes::new()
                        .class("keyvalue")
                        .class("keyvalue-align-start"),
                    Events::new(),
                    render_table_character_list_item_payload(children.iter().collect()),
                ),
            ],
            PropertyValue::None => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(Attributes::new(), Events::new(), vec![]),
            ],
            PropertyValue::Num(x) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(x.to_string())],
                ),
            ],
            PropertyValue::Str(x) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(Attributes::new(), Events::new(), vec![Html::text(x)]),
            ],
        })
        .flatten()
        .collect()
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
    modeless_dom: &Vec<Option<u128>>,
) -> Html<Msg> {
    let focused_object_id = focused_object_id.clone().and_then(|o_id| {
        if world.character(&o_id).is_some() {
            Some(o_id)
        } else if world
            .tablemask(&o_id)
            .map(|t| !t.is_fixed())
            .unwrap_or(false)
        {
            Some(o_id)
        } else {
            None
        }
    });
    modeless_container(
        Attributes::new()
            .class("cover cover-a")
            .style("z-index", "0"),
        Events::new()
            .on_mousemove({
                let selecting_tool = table_state.selecting_tool.clone();
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
                            TableTool::Measure(line_width, rounded, Some(start_point), _) => {
                                Msg::SetMeasureStartPointAndEndPointWithMouseCoord(
                                    line_width,
                                    rounded,
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
                        TableTool::Measure(line_width, rounded, _, with_table_mask) => {
                            let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                            Msg::SetSelectingTableTool(TableTool::Measure(
                                line_width,
                                rounded,
                                Some(mouse_coord),
                                with_table_mask,
                            ))
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
                    let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                    match selecting_tool {
                        TableTool::Selector => match focused_object_id {
                            Some(object_id) => Msg::BindObjectToTableGridToTransport(object_id),
                            None => Msg::NoOp,
                        },
                        TableTool::Measure(line_width, rounded, Some(start_point), true) => {
                            Msg::AddTablemaskWithPointABToTransport(
                                line_width,
                                start_point,
                                mouse_coord,
                                rounded,
                            )
                        }
                        TableTool::Measure(line_width, rounded, _, with_table_mask) => {
                            Msg::SetSelectingTableTool(TableTool::Measure(
                                line_width,
                                rounded,
                                None,
                                with_table_mask,
                            ))
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
        modeless_dom
            .iter()
            .map(|modeless_id| {
                if let Some((state, modeless)) =
                    modeless_id.and_then(|modeless_id| modelesses.get(&modeless_id))
                {
                    match modeless {
                        Modeless::Object { focused, tabs } => modeless::object(
                            modeless_id.unwrap(),
                            state,
                            tabs,
                            *focused,
                            world,
                            resource,
                        ),
                        Modeless::Chat => modeless::chat(
                            modeless_id.unwrap(),
                            state,
                            chat_tabs,
                            personal_data,
                            world,
                            resource,
                        ),
                    }
                } else {
                    Html::div(Attributes::new(), Events::new(), vec![])
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
        if let Some(tablemask) = world.tablemask(focused_object_id) {
            render_context_menu_tablemask(contextmenu, *focused_object_id, tablemask)
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
                    Events::new()
                        .on_click(move |_| Msg::CloneObjectWithObjectIdToTransport(object_id)),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::RemoveObjectWithObjectIdToTransport(object_id)),
                    "削除",
                ),
            ],
        )],
    )
}

fn render_context_menu_tablemask(
    contextmenu: &Contextmenu,
    object_id: u128,
    tablemask: &Tablemask,
) -> Html<Msg> {
    let is_fixed = tablemask.is_fixed();
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
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [2., 2.],
                                                true,
                                                is_fixed,
                                                "半径1",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [4., 4.],
                                                true,
                                                is_fixed,
                                                "半径2",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [6., 6.],
                                                true,
                                                is_fixed,
                                                "半径3",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [8., 8.],
                                                true,
                                                is_fixed,
                                                "半径4",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [10., 10.],
                                                true,
                                                is_fixed,
                                                "半径5",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [12., 12.],
                                                true,
                                                is_fixed,
                                                "半径6",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [14., 14.],
                                                true,
                                                is_fixed,
                                                "半径7",
                                            ),
                                        ],
                                    ),
                                    Html::ul(
                                        Attributes::new().class("pure-menu-list"),
                                        Events::new(),
                                        vec![
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [1., 1.],
                                                false,
                                                is_fixed,
                                                "矩形1×1",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [2., 2.],
                                                false,
                                                is_fixed,
                                                "矩形2×2",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [3., 3.],
                                                false,
                                                is_fixed,
                                                "矩形3×3",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [4., 4.],
                                                false,
                                                is_fixed,
                                                "矩形4×4",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [5., 5.],
                                                false,
                                                is_fixed,
                                                "矩形5×5",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [6., 6.],
                                                false,
                                                is_fixed,
                                                "矩形6×6",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [7., 7.],
                                                false,
                                                is_fixed,
                                                "矩形7×7",
                                            ),
                                        ],
                                    ),
                                ],
                            )],
                        ),
                    ],
                ),
                Html::li(
                    Attributes::new()
                        .class("pure-menu-item")
                        .class("pure-menu-has-children"),
                    Events::new(),
                    vec![
                        btn::contextmenu_text(Attributes::new(), Events::new(), "不透明度"),
                        Html::ul(
                            Attributes::new().class("pure-menu-children"),
                            Events::new(),
                            vec![
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 1.0)
                                    }),
                                    "100%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 0.8)
                                    }),
                                    "80%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 0.6)
                                    }),
                                    "60%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 0.4)
                                    }),
                                    "40%",
                                ),
                            ],
                        ),
                    ],
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let alpha = tablemask.background_color().alpha;
                        move |_| {
                            Msg::OpenModal(Modal::ColorPicker(ColorPickerType::TablemaskColor(
                                object_id, alpha,
                            )))
                        }
                    }),
                    "色を変更",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let size = tablemask.size().clone();
                        let is_rounded = tablemask.is_rounded();
                        move |_| {
                            Msg::SetTablemaskSizeWithStyleToTransport(
                                object_id, size, is_rounded, !is_fixed,
                            )
                        }
                    }),
                    String::from("固定") + if is_fixed { "解除" } else { "する" },
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::CloneObjectWithObjectIdToTransport(object_id)),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::RemoveObjectWithObjectIdToTransport(object_id)),
                    "削除",
                ),
            ],
        )],
    )
}

fn render_context_menu_tablemask_resizer(
    object_id: u128,
    size: [f64; 2],
    is_rounded: bool,
    is_fixed: bool,
    text: impl Into<String>,
) -> Html<Msg> {
    btn::contextmenu_text(
        Attributes::new(),
        Events::new().on_click(move |_| {
            Msg::SetTablemaskSizeWithStyleToTransport(object_id, size, is_rounded, is_fixed)
        }),
        text,
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
) -> Html<Msg> {
    let mut children = vec![];
    for modal in modals {
        let child = match modal {
            Modal::Resource => modal::resource(resource),
            Modal::SelectImage(modal_type) => modal::select_image(resource, modal_type),
            Modal::PersonalSetting => modal::personal_setting(personal_data, resource),
            Modal::TableSetting => modal::table_setting(
                world.selecting_table_id(),
                &world.selecting_table(),
                world.tables(),
                &resource,
            ),
            Modal::ColorPicker(color_picker_type) => modal::color_picker(color_picker_type.clone()),
            Modal::CharacterSelecter(character_selecter_type) => match character_selecter_type {
                CharacterSelecterType::ChatSender => modal::character_selecter(
                    character_selecter_type.clone(),
                    chat_data
                        .senders
                        .iter()
                        .filter_map(|s| s.as_character())
                        .collect(),
                    world,
                    resource,
                ),
            },
            Modal::ChatLog => modal::chat_log(chat_data, resource),
            Modal::ChatTabEditor => modal::chat_tab_editor(chat_data),
        };
        children.push(child);
    }
    Html::div(
        Attributes::new()
            .style("position", "fixied")
            .style("z-index", "1"),
        Events::new(),
        children,
    )
}
