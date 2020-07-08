use crate::{
    block::{self, BlockId},
    color_system,
    dicebot::{self, bcdice},
    model::{self, modeless::ModelessId},
    renderer::Renderer,
    resource::{Data, ResourceId},
    skyway, Color,
};
use kagura::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};

mod render;
mod state;

use render::render;

pub type State = state::State<Msg, Sub>;

pub enum Msg {
    NoOp,
    InitDomDependents,
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
    SetModelessTabIdx(ModelessId, usize),
    GrubModelessTab(ModelessId, usize),
    DropModelessTabToModeless(ModelessId),
    DropModelessTab([f64; 2]),

    // Modal
    OpenModal(state::Modal),
    CloseModal,

    // UI for table
    AddChracaterWithMousePositionToCloseContextmenu([f32; 2]),
    AddTablemaskWithMousePositionToCloseContextmenu([f32; 2], [f32; 2], Color, bool, bool),
    AddBoxblockWithMousePositionToCloseContextmenu([f32; 2], [f32; 3], Color),
    CloneCharacterToCloseContextmenu(BlockId),
    CloneTablemaskToCloseContextmenu(BlockId),
    RemoveCharacterToCloseContextmenu(BlockId),
    RemoveTablemaskToCloseContextmenu(BlockId),
    RemoveAreaToCloseContextmenu(BlockId),
    RemoveBoxblockToCloseContextmenu(BlockId),

    // Mouse
    SetLastMousePosition(bool, [f32; 2]),
    SetLastMouseDownPosition([f32; 2]),
    SetLastMouseUpPosition([f32; 2]),
    SetCameraRotationWithMouseMovement([f32; 2]),
    SetCameraMovementWithMouseMovement([f32; 2]),
    SetCameraMovementWithMouseWheel(f32),
    SetSelectingTableTool(state::table::Tool),
    SetCharacterPositionWithMousePosition(BlockId, [f32; 2]),
    SetTablemaskPositionWithMousePosition(BlockId, [f32; 2]),
    SetBoxblockPositionWithMousePosition(BlockId, [f32; 2]),
    DrawLineWithMousePosition([f32; 2], [f32; 2], f64, Color),
    EraceLineWithMousePosition([f32; 2], [f32; 2], f64),
    ClearTable,
    MeasureLineWithMousePosition([f32; 2], [f32; 2], Option<BlockId>, Color),
    ClearMeasure,
    SetAreaWithMousePosition(
        [f32; 2],
        [f32; 2],
        Option<BlockId>,
        Color,
        Color,
        block::table_object::area::Type,
    ),

    // World
    AddTable,

    // Table
    SetTableSize(BlockId, [f32; 2]),
    SetTableImage(BlockId, Option<ResourceId>),

    // PersonalData
    SetPersonalDataWithPlayerName(String),
    SetPersonalDataWithIconImageToCloseModal(u128),

    // tablemask
    SetTablemaskSize(BlockId, [f32; 2]),
    SetTablemaskColor(BlockId, Color),
    SetTablemaskIsFixed(BlockId, bool),
    SetTablemaskIsRounded(BlockId, bool),
    SetTablemaskIsInved(BlockId, bool),

    // boxblock
    SetBoxblockSize(BlockId, [f32; 3]),
    SetBoxblockColor(BlockId, Color),
    SetBoxblockIsFixed(BlockId, bool),

    // character
    SetCharacterName(BlockId, String),
    SetCharacterSize(BlockId, [Option<f32>; 2]),
    SetCharacterTextrureToCloseModal(BlockId, Option<ResourceId>),
    SetCharacterPosition(BlockId, [f32; 3]),

    // property

    // チャット関係
    SetInputingChatMessage(String),
    SendInputingChatMessage,
    InsertChatItem(BlockId, block::chat::Item, f64),
    AddChatSender(BlockId),
    RemoveChatSender(BlockId),
    SetSelectingChatSenderIdx(usize),
    AddChatTab,
    SetSelectingChatTabIdx(usize),
    SetChatTabName(BlockId, String),
    RemoveChatTab(BlockId),

    // リソース管理
    LoadFromFileList(web_sys::FileList),
    LoadDataToResource(Data),

    // BCDice
    GetBcdiceServerList,
    SetBcdiceServerList(Vec<String>),
    GetBcdiceSystemNames(String),
    SetBcdiceSystemNames(bcdice::Names),

    // 接続に関する操作
    SendBlockPacks(HashMap<BlockId, JsValue>, HashSet<BlockId>),
    SendResourcePacks(HashMap<ResourceId, JsValue>),
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
                handler(Msg::InitDomDependents);
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

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => state.dequeue(),
        Msg::InitDomDependents => {
            state.set_canvas_size(reset_canvas_size(state.pixel_ratio()));
            let canvas = get_table_canvas_element();
            state.set_renderer(Renderer::new(canvas));
            render_canvas(state);

            let element = get_element_by_id("modeless-parent");
            state.set_modeless_parent(Some(element));

            Cmd::task(|resolve| resolve(Msg::GetBcdiceServerList))
        }
        Msg::ResizeCanvas => {
            state.set_canvas_size(reset_canvas_size(state.pixel_ratio()));
            render_canvas(state);
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

        Msg::SetModelessTabIdx(modeless_id, tab_idx) => {
            state.set_modeless_focused_tab(modeless_id, tab_idx);
            state.dequeue()
        }

        Msg::GrubModelessTab(modeless_id, tab_idx) => {
            state
                .table_mut()
                .set_moving_tab(Some((modeless_id, tab_idx)));
            state.grub_modeless(modeless_id, [0.0, 0.0], [false, false, false, false]);
            state.dequeue()
        }

        Msg::DropModelessTabToModeless(modeless_id) => {
            if let Some((from_id, tab_idx)) = state.table().moving_tab() {
                let from_id = *from_id;
                let tab_idx = *tab_idx;
                if modeless_id != from_id {
                    if let Some(block_id) = state.remove_modeless_tab(from_id, tab_idx) {
                        state.add_modeless_tab(modeless_id, block_id);
                    }
                    state.drop_modeless(from_id);
                }
            }
            state.dequeue()
        }

        Msg::DropModelessTab(mouse_pos) => {
            if let Some((from_id, tab_idx)) = state.table().moving_tab() {
                let from_id = *from_id;
                let tab_idx = *tab_idx;
                if let Some(block_id) = state.remove_modeless_tab(from_id, tab_idx) {
                    let modeless = state::Modeless::Object {
                        tabs: vec![block_id],
                        focused: 0,
                        outlined: None,
                    };
                    state.open_modeless_with_position(modeless, mouse_pos);
                }
                state.drop_modeless(from_id);
            }

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

            let [x, y] = get_table_position(state, false, &mouse_position, state.pixel_ratio());

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

            let world = state.world().clone();
            state
                .block_field_mut()
                .update(&world, timestamp(), |world: &mut block::World| {
                    world.add_character(character.clone());
                });

            render_canvas(state);

            send_pack_cmd(
                state.block_field(),
                vec![&prop_hp, &prop_mp, &prop_root, &character, &world],
                hash_set! {},
            )
        }

        Msg::AddTablemaskWithMousePositionToCloseContextmenu(
            mouse_position,
            size,
            color,
            is_rounded,
            is_inved,
        ) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                let [x, y] = get_table_position(state, false, &mouse_position, state.pixel_ratio());

                let prop_root = block::Property::new("");
                let prop_root = state.block_field_mut().add(prop_root);

                let mut tablemask = block::table_object::Tablemask::new(
                    prop_root.clone(),
                    &size,
                    color,
                    is_rounded,
                    is_inved,
                );
                tablemask.set_position([x, y]);
                let tablemask = state.block_field_mut().add(tablemask);

                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.add_tablemask(tablemask.clone());
                    },
                );

                render_canvas(state);

                send_pack_cmd(
                    state.block_field(),
                    vec![&tablemask, &selecting_table],
                    hash_set! {},
                )
            } else {
                state.dequeue()
            }
        }

        Msg::AddBoxblockWithMousePositionToCloseContextmenu(mouse_position, size, color) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                let [xw, yw, zw] = size;

                let [x, y, z] = get_focused_position(
                    state,
                    &mouse_position,
                    state.pixel_ratio(),
                    &[xw * 0.5, yw * 0.5, zw * 0.5],
                );

                let boxblock = block::table_object::Boxblock::new([x, y, z], [xw, yw, zw], color);
                let boxblock = state.block_field_mut().add(boxblock);

                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.add_boxblock(boxblock.clone());
                    },
                );

                render_canvas(state);

                send_pack_cmd(
                    state.block_field(),
                    vec![&boxblock, &selecting_table],
                    hash_set! {},
                )
            } else {
                state.dequeue()
            }
        }

        Msg::CloneCharacterToCloseContextmenu(character) => {
            state.close_contextmenu();

            if let Some(character) = state.block_field().get::<block::Character>(&character) {
                let mut character = character.clone();
                let prop = clone_prop(state.block_field_mut(), character.property_id())
                    .unwrap_or(block::Property::new(""));
                let prop = state.block_field_mut().add(prop);

                let mut props = trace_prop_id(state.block_field(), &prop);

                character.set_property_id(prop);
                let character = state.block_field_mut().add(character);

                let world = state.world().clone();
                state
                    .block_field_mut()
                    .update(&world, timestamp(), |world: &mut block::World| {
                        world.add_character(character.clone());
                    });

                props.push(character);
                props.push(world.clone());

                render_canvas(state);

                send_pack_cmd(state.block_field(), props.iter().collect(), hash_set! {})
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
                state.selecting_table().map(|t| t.clone()),
            ) {
                let mut tablemask = tablemask.clone();
                let prop = clone_prop(state.block_field_mut(), tablemask.property_id())
                    .unwrap_or(block::Property::new(""));
                let prop = state.block_field_mut().add(prop);

                let props = trace_prop_id(state.block_field(), &prop);

                tablemask.set_property_id(prop);
                let tablemask = state.block_field_mut().add(tablemask);

                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.add_tablemask(tablemask.clone());
                    },
                );

                render_canvas(state);

                send_pack_cmd(
                    state.block_field(),
                    vec![&tablemask, &selecting_table],
                    hash_set! {},
                )
            } else {
                state.dequeue()
            }
        }

        Msg::RemoveAreaToCloseContextmenu(area_id) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.remove_area(&area_id);
                    },
                );

                state.block_field_mut().remove(&area_id);

                render_canvas(state);

                send_pack_cmd(
                    state.block_field(),
                    vec![&selecting_table],
                    hash_set! {area_id},
                )
            } else {
                state.dequeue()
            }
        }

        Msg::RemoveBoxblockToCloseContextmenu(boxblock_id) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.remove_boxblock(&boxblock_id);
                    },
                );

                state.block_field_mut().remove(&boxblock_id);

                render_canvas(state);

                send_pack_cmd(
                    state.block_field(),
                    vec![&selecting_table],
                    hash_set! {boxblock_id},
                )
            } else {
                state.dequeue()
            }
        }

        Msg::RemoveCharacterToCloseContextmenu(character) => {
            state.close_contextmenu();

            let world = state.world().clone();
            state
                .block_field_mut()
                .update(&world, timestamp(), |world: &mut block::World| {
                    world.remove_character(&character);
                });

            state.block_field_mut().remove(&character);

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&world], hash_set! {character})
        }

        Msg::RemoveTablemaskToCloseContextmenu(tablemask) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.remove_tablemask(&tablemask);
                    },
                );

                state.block_field_mut().remove(&tablemask);

                render_canvas(state);

                send_pack_cmd(
                    state.block_field(),
                    vec![&selecting_table],
                    hash_set! {tablemask},
                )
            } else {
                state.dequeue()
            }
        }

        // Mouse
        Msg::SetLastMousePosition(check_focused, mouse_position) => {
            if check_focused {
                check_focused_object(state, &mouse_position);
            }

            state.table_mut().set_last_mouse_position(mouse_position);

            state.dequeue()
        }

        Msg::SetLastMouseDownPosition(mouse_position) => {
            check_focused_object(state, &mouse_position);

            state
                .table_mut()
                .set_last_mouse_down_position(mouse_position);

            match state.table().focused() {
                state::table::Focused::Character(table_block)
                | state::table::Focused::Tablemask(table_block)
                | state::table::Focused::Boxblock(table_block) => {
                    let block_id = table_block.block_id.clone();
                    state.table_mut().set_floating_object(Some(block_id));
                }
                _ => (),
            }

            state.dequeue()
        }

        Msg::SetLastMouseUpPosition(mouse_position) => {
            check_focused_object(state, &mouse_position);

            state.table_mut().set_last_mouse_up_position(mouse_position);

            state.table_mut().set_floating_object(None);

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetCameraRotationWithMouseMovement(mouse_position) => {
            let dx = mouse_position[0] - state.table().last_mouse_position()[0];
            let dy = mouse_position[1] - state.table().last_mouse_position()[1];
            let long_edge = state.canvas_size()[0].max(state.canvas_size()[1]);

            let factor = 3.0 / long_edge * get_canvas_pixel_ratio(state.pixel_ratio());

            let camera = state.camera_mut();

            camera.set_x_axis_rotation(camera.x_axis_rotation() - dy * factor);
            camera.set_z_axis_rotation(camera.z_axis_rotation() - dx * factor);

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
            let mov = [mov[0] - dx * factor, mov[1] + dy * factor, mov[2]];

            camera.set_movement(mov);

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetCameraMovementWithMouseWheel(delta_y) => {
            let factor = 0.02;

            let camera = state.camera_mut();
            let mov = camera.movement();
            let mov = [mov[0], mov[1], mov[2] + factor * delta_y];

            camera.set_movement(mov);

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetSelectingTableTool(table_tool) => {
            state.table_mut().set_selecting_tool(table_tool);
            state.dequeue()
        }

        Msg::SetCharacterPositionWithMousePosition(block_id, mouse_position) => {
            let position = get_focused_position(
                state,
                &mouse_position,
                state.pixel_ratio(),
                &[0.0, 0.0, 0.0],
            );
            let timestamp = timestamp();

            let updated = state
                .block_field_mut()
                .update(&block_id, timestamp, |character: &mut block::Character| {
                    character.set_position(position);
                })
                .is_none();

            if updated {
                render_canvas(state);
                send_pack_cmd(state.block_field(), vec![&block_id], hash_set! {})
            } else {
                state.dequeue()
            }
        }

        Msg::SetTablemaskPositionWithMousePosition(block_id, mouse_position) => {
            let is_fixied = state
                .block_field()
                .get::<block::table_object::Tablemask>(&block_id)
                .map(|tablemask| tablemask.is_fixed())
                .unwrap_or(true);

            if !is_fixied {
                let [x, y] = get_table_position(state, false, &mouse_position, state.pixel_ratio());
                let timestamp = timestamp();

                let updated = state
                    .block_field_mut()
                    .update(
                        &block_id,
                        timestamp,
                        |tablemask: &mut block::table_object::Tablemask| {
                            tablemask.set_position([x, y]);
                        },
                    )
                    .is_none();

                if updated {
                    render_canvas(state);
                    send_pack_cmd(state.block_field(), vec![&block_id], hash_set! {})
                } else {
                    state.dequeue()
                }
            } else {
                update(
                    state,
                    Msg::SetCameraMovementWithMouseMovement(mouse_position),
                )
            }
        }

        Msg::SetBoxblockPositionWithMousePosition(block_id, mouse_position) => {
            let is_fixied = state
                .block_field()
                .get::<block::table_object::Boxblock>(&block_id)
                .map(|boxblock| boxblock.is_fixed())
                .unwrap_or(true);

            if !is_fixied {
                let s = state
                    .block_field()
                    .get::<block::table_object::Boxblock>(&block_id)
                    .map(|boxblock| boxblock.size().clone())
                    .unwrap_or([0.0, 0.0, 0.0]);

                let position = get_focused_position(
                    state,
                    &mouse_position,
                    state.pixel_ratio(),
                    &[s[0] * 0.5, s[1] * 0.5, s[2] * 0.5],
                );
                let timestamp = timestamp();

                let updated = state
                    .block_field_mut()
                    .update(
                        &block_id,
                        timestamp,
                        |boxblock: &mut block::table_object::Boxblock| {
                            boxblock.set_position(position);
                        },
                    )
                    .is_none();
                if updated {
                    render_canvas(state);
                    send_pack_cmd(state.block_field(), vec![&block_id], hash_set! {})
                } else {
                    state.dequeue()
                }
            } else {
                update(
                    state,
                    Msg::SetCameraMovementWithMouseMovement(mouse_position),
                )
            }
        }

        Msg::DrawLineWithMousePosition(a, b, line_width, color) => {
            let selecting_table = state.selecting_table();
            let drawing_texture_id = if let Some(selecting_table) = selecting_table {
                state
                    .block_field()
                    .get::<block::Table>(&selecting_table)
                    .map(|table| table.drawing_texture_id().clone())
            } else {
                None
            };
            if let Some(texture_id) = drawing_texture_id {
                let [ax, ay] = get_table_position(state, true, &a, state.pixel_ratio());
                let [bx, by] = get_table_position(state, true, &b, state.pixel_ratio());

                state.block_field_mut().update(
                    &texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        let [ax, ay] = texture.texture_position(&[ax as f64, ay as f64]);
                        let [bx, by] = texture.texture_position(&[bx as f64, by as f64]);

                        let context = texture.context();

                        context.set_line_width(line_width);
                        context.set_line_cap("round");
                        context.set_stroke_style(&color.to_jsvalue());
                        context
                            .set_global_composite_operation("source-over")
                            .unwrap();
                        context.begin_path();
                        context.move_to(ax, ay);
                        context.line_to(bx, by);
                        context.fill();
                        context.stroke();
                    },
                );

                render_canvas(state);

                state.dequeue()
            } else {
                state.dequeue()
            }
        }

        Msg::EraceLineWithMousePosition(a, b, line_width) => {
            let selecting_table = state.selecting_table();
            let drawing_texture_id = if let Some(selecting_table) = selecting_table {
                state
                    .block_field()
                    .get::<block::Table>(&selecting_table)
                    .map(|table| table.drawing_texture_id().clone())
            } else {
                None
            };
            if let Some(texture_id) = drawing_texture_id {
                let [ax, ay] = get_table_position(state, true, &a, state.pixel_ratio());
                let [bx, by] = get_table_position(state, true, &b, state.pixel_ratio());

                state.block_field_mut().update(
                    &texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        let [ax, ay] = texture.texture_position(&[ax as f64, ay as f64]);
                        let [bx, by] = texture.texture_position(&[bx as f64, by as f64]);

                        let context = texture.context();

                        context.set_line_width(line_width);
                        context.set_line_cap("round");
                        context.set_stroke_style(&color_system::gray(255, 9).to_jsvalue());
                        context
                            .set_global_composite_operation("destination-out")
                            .unwrap();
                        context.begin_path();
                        context.move_to(ax, ay);
                        context.line_to(bx, by);
                        context.fill();
                        context.stroke();
                    },
                );

                render_canvas(state);

                state.dequeue()
            } else {
                state.dequeue()
            }
        }

        Msg::ClearTable => {
            let selecting_table = state.selecting_table();
            let drawing_texture_id = if let Some(selecting_table) = selecting_table {
                state
                    .block_field()
                    .get::<block::Table>(&selecting_table)
                    .map(|table| table.drawing_texture_id().clone())
            } else {
                None
            };
            if let Some(texture_id) = drawing_texture_id {
                state.block_field_mut().update(
                    &texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        texture.clear();
                    },
                );

                render_canvas(state);

                state.dequeue()
            } else {
                state.dequeue()
            }
        }

        Msg::MeasureLineWithMousePosition(a, b, block_id, color) => {
            let [ax, ay, az] =
                get_focused_position(state, &a, state.pixel_ratio(), &[0.0, 0.0, 0.0]);
            let [bx, by, bz] =
                get_focused_position(state, &b, state.pixel_ratio(), &[0.0, 0.0, 0.0]);
            let len = ((bx - ax).powi(2) + (by - ay).powi(2) + (bz - az).powi(2)).sqrt();

            state.table_mut().clear_info();
            state
                .table_mut()
                .add_info("始点", format!("({:.1},{:.1},{:.1})", ax, ay, az));
            state
                .table_mut()
                .add_info("終点", format!("({:.1},{:.1},{:.1})", bx, by, bz));
            state.table_mut().add_info("距離", format!("{:.1}", len));

            if let Some(block_id) = block_id {
                let block_id = block_id.clone();
                state.block_field_mut().update(
                    &block_id,
                    timestamp(),
                    |m: &mut block::table_object::Measure| {
                        m.set_org([ax, ay, az]);
                        m.set_vec([bx - ax, by - ay, bz - az]);
                        m.set_color(color);
                    },
                );
            } else {
                let measure = block::table_object::Measure::new(
                    [ax, ay, az],
                    [bx - ax, by - ay, bz - az],
                    color,
                );
                let bid = state.block_field_mut().add(measure);
                if let state::table::Tool::Measure { block_id, .. } =
                    state.table_mut().selecting_tool_mut()
                {
                    *block_id = Some(bid);
                }
            }

            render_canvas(state);

            state.dequeue()
        }

        Msg::ClearMeasure => {
            let measures = state
                .block_field()
                .all::<block::table_object::Measure>()
                .into_iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>();

            for measure_id in measures {
                state.block_field_mut().remove(&measure_id);
            }

            if let state::table::Tool::Measure { block_id, .. } =
                state.table_mut().selecting_tool_mut()
            {
                *block_id = None;
            }

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetAreaWithMousePosition(a, b, block_id, color_1, color_2, type_) => {
            let [ax, ay] = get_table_position(state, false, &a, state.pixel_ratio());
            let [bx, by] = get_table_position(state, false, &b, state.pixel_ratio());
            let len = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();

            state.table_mut().clear_info();
            state
                .table_mut()
                .add_info("始点", format!("({:.1},{:.1})", ax, ay));
            state
                .table_mut()
                .add_info("終点", format!("({:.1},{:.1})", bx, by));
            state.table_mut().add_info("距離", format!("{:.1}", len));

            if let Some(block_id) = block_id {
                let block_id = block_id.clone();
                state.block_field_mut().update(
                    &block_id,
                    timestamp(),
                    |a: &mut block::table_object::Area| {
                        a.set_org([ax, ay, 0.0]);
                        a.set_vec([bx - ax, by - ay, 0.0]);
                        a.set_color_1(color_1);
                        a.set_color_2(color_2);
                        a.set_type(type_);
                    },
                );
            } else if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                let area = block::table_object::Area::new(
                    [ax, ay, 0.0],
                    [bx - ax, by - ay, 0.0],
                    color_1,
                    color_2,
                    type_,
                );
                let bid = state.block_field_mut().add(area);
                state.block_field_mut().update(
                    &selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        crate::debug::log_1("add area");
                        table.add_area(bid.clone());
                    },
                );
                if let state::table::Tool::Area { block_id, .. } =
                    state.table_mut().selecting_tool_mut()
                {
                    *block_id = Some(bid);
                }
            }

            render_canvas(state);

            state.dequeue()
        }

        // World
        Msg::AddTable => {
            let texture = block::table::Texture::new(&[4096, 4096], [20.0, 20.0]);
            let texture = state.block_field_mut().add(texture);
            let table = block::Table::new(texture.clone(), [20.0, 20.0], "テーブル");
            let table = state.block_field_mut().add(table);

            let world = state.world().clone();

            state
                .block_field_mut()
                .update(&world, timestamp(), |world: &mut block::World| {
                    world.add_table(table.clone())
                });

            render_canvas(state);

            send_pack_cmd(
                state.block_field(),
                vec![&texture, &table, state.world()],
                hash_set! {},
            )
        }

        // Table
        Msg::SetTableSize(table, size) => {
            state
                .block_field_mut()
                .update(&table, timestamp(), |table: &mut block::Table| {
                    table.set_size(size);
                });

            send_pack_cmd(state.block_field(), vec![&table], hash_set! {})
        }

        Msg::SetTableImage(table, image) => {
            state
                .block_field_mut()
                .update(&table, timestamp(), |table: &mut block::Table| {
                    table.set_image_texture_id(image);
                });

            send_pack_cmd(state.block_field(), vec![&table], hash_set! {})
        }

        // PersonalData
        Msg::SetPersonalDataWithPlayerName(player_name) => {
            state.personal_data_mut().set_name(player_name);
            state.dequeue()
        }
        Msg::SetPersonalDataWithIconImageToCloseModal(r_id) => {
            state.personal_data_mut().set_icon(Some(r_id));
            state.close_modal();
            state.dequeue()
        }

        // Tablemask
        Msg::SetTablemaskSize(tablemask_id, size) => {
            state.block_field_mut().update(
                &tablemask_id,
                timestamp(),
                |tablemask: &mut block::table_object::Tablemask| {
                    tablemask.set_size(&size);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&tablemask_id], hash_set! {})
        }

        Msg::SetTablemaskColor(tablemask_id, color) => {
            state.block_field_mut().update(
                &tablemask_id,
                timestamp(),
                |tablemask: &mut block::table_object::Tablemask| {
                    tablemask.set_color(color);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&tablemask_id], hash_set! {})
        }

        Msg::SetTablemaskIsFixed(tablemask_id, is_fixed) => {
            state.block_field_mut().update(
                &tablemask_id,
                timestamp(),
                |tablemask: &mut block::table_object::Tablemask| {
                    tablemask.set_is_fixed(is_fixed);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&tablemask_id], hash_set! {})
        }

        Msg::SetTablemaskIsRounded(tablemask_id, is_rounded) => {
            state.block_field_mut().update(
                &tablemask_id,
                timestamp(),
                |tablemask: &mut block::table_object::Tablemask| {
                    tablemask.set_is_rounded(is_rounded);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&tablemask_id], hash_set! {})
        }

        Msg::SetTablemaskIsInved(tablemask_id, is_inved) => {
            state.block_field_mut().update(
                &tablemask_id,
                timestamp(),
                |tablemask: &mut block::table_object::Tablemask| {
                    tablemask.set_is_inved(is_inved);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&tablemask_id], hash_set! {})
        }

        // Boxblock
        Msg::SetBoxblockSize(boxblock_id, size) => {
            state.block_field_mut().update(
                &boxblock_id,
                timestamp(),
                |boxblock: &mut block::table_object::Boxblock| {
                    boxblock.set_size(size);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&boxblock_id], hash_set! {})
        }

        Msg::SetBoxblockColor(boxblock_id, color) => {
            state.block_field_mut().update(
                &boxblock_id,
                timestamp(),
                |boxblock: &mut block::table_object::Boxblock| {
                    boxblock.set_color(color);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&boxblock_id], hash_set! {})
        }

        Msg::SetBoxblockIsFixed(boxblock_id, is_fixed) => {
            state.block_field_mut().update(
                &boxblock_id,
                timestamp(),
                |boxblock: &mut block::table_object::Boxblock| {
                    boxblock.set_is_fixed(is_fixed);
                },
            );

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&boxblock_id], hash_set! {})
        }

        // Character
        Msg::SetCharacterName(character, name) => {
            state.block_field_mut().update(
                &character,
                timestamp(),
                |character: &mut block::Character| {
                    character.set_name(name);
                },
            );

            send_pack_cmd(state.block_field(), vec![&character], hash_set! {})
        }

        Msg::SetCharacterSize(character, [w, h]) => {
            if let (Some(w), Some(h)) = (w, h) {
                state.block_field_mut().update(
                    &character,
                    timestamp(),
                    |character: &mut block::Character| {
                        character.set_size([w, w, h]);
                    },
                );
            } else if let Some(Data::Image { element, .. }) = state
                .block_field()
                .get::<block::Character>(&character)
                .and_then(|character| character.texture_id())
                .and_then(|t_id| state.resource().get(t_id))
            {
                let iw = element.width() as f32;
                let ih = element.height() as f32;
                if let Some(w) = w {
                    let h = w / iw * ih;
                    state.block_field_mut().update(
                        &character,
                        timestamp(),
                        |character: &mut block::Character| {
                            character.set_size([w, w, h]);
                        },
                    );
                }
                if let Some(h) = h {
                    let w = h / ih * iw;
                    state.block_field_mut().update(
                        &character,
                        timestamp(),
                        |character: &mut block::Character| {
                            character.set_size([w, w, h]);
                        },
                    );
                }
            }

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&character], hash_set! {})
        }

        Msg::SetCharacterTextrureToCloseModal(character_id, texture_id) => {
            state.block_field_mut().update(
                &character_id,
                timestamp(),
                |character: &mut block::Character| {
                    character.set_texture_id(texture_id);
                },
            );

            render_canvas(state);

            state.dequeue()
        }

        Msg::SetCharacterPosition(character_id, pos) => {
            state.block_field_mut().update(
                &character_id,
                timestamp(),
                |character: &mut block::Character| {
                    character.set_position(pos);
                },
            );

            render_canvas(state);

            state.dequeue()
        }

        // Chat
        Msg::SetInputingChatMessage(msg) => {
            state.chat_mut().set_inputing_message(msg);
            state.dequeue()
        }

        Msg::SendInputingChatMessage => {
            use block::chat::item::Icon;
            use block::chat::item::Sender;

            let sender = state.chat().selecting_sender().clone();
            let (display_name, character_id, icon) = match &sender {
                Sender::User => (
                    Some(state.personal_data().name().clone()),
                    None,
                    state.personal_data().icon().map(|r_id| *r_id),
                ),
                Sender::Character(character_id) => {
                    if let Some(character) =
                        state.block_field().get::<block::Character>(character_id)
                    {
                        (
                            Some(character.name().clone()),
                            Some(*character_id),
                            character.texture_id().map(|r_id| *r_id),
                        )
                    } else {
                        (None, None, None)
                    }
                }
                _ => (None, None, None),
            };
            if let (Some(display_name), Some(tab)) = (display_name, state.selecting_chat_tab_id()) {
                let tab = *tab;
                let text = state.chat_mut().drain_inputing_message();
                let (left, right) = state.dicebot().delimit(&text);
                let [left, right] = [left.to_string(), right.to_string()];
                let item = block::chat::Item::new(
                    state.peer().id(),
                    display_name,
                    icon.map(|r_id| Icon::Resource(r_id))
                        .unwrap_or(Icon::DefaultUser),
                    sender.clone(),
                    text,
                    Some(left),
                );
                update(state, Msg::InsertChatItem(tab, item, js_sys::Date::now()))
            } else {
                state.dequeue()
            }
        }

        Msg::InsertChatItem(tab, item, timestamp) => {
            let item_id = state.block_field_mut().add(item);
            state
                .block_field_mut()
                .update(&tab, None, |tab: &mut block::chat::Tab| {
                    tab.insert(timestamp, item_id);
                });
            state.dequeue()
        }

        Msg::AddChatSender(character_id) => {
            state.chat_mut().add_sender(character_id);
            state.dequeue()
        }

        Msg::RemoveChatSender(character_id) => {
            state.chat_mut().remove_sender(&character_id);
            state.dequeue()
        }

        Msg::SetSelectingChatSenderIdx(idx) => {
            state.chat_mut().set_selecting_sender_idx(idx);
            state.dequeue()
        }

        Msg::AddChatTab => {
            let tab = block::chat::Tab::new("タブ");
            let tab_id = state.block_field_mut().add(tab);
            state.update_chat_block(timestamp(), |chat| {
                chat.push(tab_id);
            });
            state.dequeue()
        }

        Msg::SetSelectingChatTabIdx(idx) => {
            state.chat_mut().set_selecting_tab_idx(idx);
            state.dequeue()
        }

        Msg::SetChatTabName(tab, name) => {
            state
                .block_field_mut()
                .update(&tab, timestamp(), |tab: &mut block::chat::Tab| {
                    tab.set_name(name);
                });
            state.dequeue()
        }

        Msg::RemoveChatTab(tab) => {
            state.update_chat_block(timestamp(), |chat| {
                if let Some(tab_idx) = chat.iter().position(|t| tab == *t) {
                    if chat.len() > 1 {
                        chat.remove(tab_idx);
                    }
                }
            });
            state.dequeue()
        }

        // リソース
        Msg::LoadFromFileList(file_list) => {
            let len = file_list.length();
            for i in 0..len {
                if let Some(file) = file_list.item(i) {
                    let blob: web_sys::Blob = file.into();
                    let promise = Data::from_blob(blob);
                    let task = Cmd::task(move |resolve| {
                        promise.then(|data| {
                            if let Some(data) = data {
                                resolve(Msg::LoadDataToResource(data));
                            }
                        })
                    });
                    state.enqueue(task);
                }
            }
            state.dequeue()
        }

        Msg::LoadDataToResource(data) => {
            let promise = data.pack();
            let id = state.resource_mut().add(data);
            Cmd::task(move |resolve| {
                promise.then(move |data| {
                    if let Some(data) = data {
                        resolve(Msg::SendResourcePacks(
                            vec![(id, data)].into_iter().collect(),
                        ))
                    }
                })
            })
        }

        //BCDice
        Msg::GetBcdiceServerList => Cmd::task(task::http::get(
            r"https://raw.githubusercontent.com/bcdice/bcdice-api-servers/master/servers.yaml",
            task::http::Props::new(),
            |response| {
                if let Some(servers) = response
                    .ok()
                    .and_then(|r| r.text)
                    .and_then(|text| serde_yaml::from_str(&text).ok())
                {
                    Msg::SetBcdiceServerList(servers)
                } else {
                    Msg::NoOp
                }
            },
        )),

        Msg::SetBcdiceServerList(servers) => {
            for server in servers.iter() {
                crate::debug::log_1(server);
            }

            let url = servers.get(0).map(|url| url.clone());
            state.dicebot_mut().bcdice_mut().set_servers(servers);

            if let Some(url) = url {
                Cmd::task(|r| r(Msg::GetBcdiceSystemNames(url)))
            } else {
                state.dequeue()
            }
        }

        Msg::GetBcdiceSystemNames(url) => Cmd::task(task::http::get(
            url + r"/v1/names",
            task::http::Props::new(),
            |response| {
                if let Some(names) = response
                    .ok()
                    .and_then(|r| r.text)
                    .and_then(|text| serde_json::from_str(&text).ok())
                {
                    Msg::SetBcdiceSystemNames(names)
                } else {
                    Msg::NoOp
                }
            },
        )),

        Msg::SetBcdiceSystemNames(names) => {
            for name in names.iter() {
                crate::debug::log_1(name.name());
            }
            state.dicebot_mut().bcdice_mut().set_names(names);
            state.dequeue()
        }

        // 接続に関する操作
        Msg::SendBlockPacks(packs, removed) => {
            let packs = packs
                .into_iter()
                .map(|(id, data)| (id.to_u128(), data))
                .collect();
            state.room().send(skyway::Msg::SetBlockPacks(packs));
            state.dequeue()
        }

        Msg::SendResourcePacks(packs) => {
            state.room().send(skyway::Msg::SetResourcePacks(packs));
            state.dequeue()
        }

        Msg::ReceiveMsg(msg) => match msg {
            skyway::Msg::None => state.dequeue(),
            skyway::Msg::SetBlockPacks(packs) => state.dequeue(),
            skyway::Msg::SetResourcePacks(packs) => state.dequeue(),
        },
        Msg::PeerJoin(peer_id) => {
            state.peers_mut().insert(peer_id);
            state.dequeue()
        }
        Msg::PeerLeave(peer_id) => {
            state.peers_mut().remove(&peer_id);
            state.dequeue()
        }
        Msg::DisconnectFromRoom => Cmd::Sub(Sub::DisconnectFromRoom),
    }
}

fn get_element_by_id(id: &str) -> web_sys::Element {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
}

fn get_table_canvas_element() -> web_sys::HtmlCanvasElement {
    get_element_by_id("table")
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

fn get_table_position(
    state: &State,
    ignore_binding: bool,
    screen_position: &[f32; 2],
    pixel_ratio: f32,
) -> [f32; 2] {
    let dpr = get_canvas_pixel_ratio(pixel_ratio);
    let mouse_coord = [screen_position[0] * dpr, screen_position[1] * dpr];
    let p = state
        .camera()
        .collision_point_on_xy_plane(state.canvas_size(), &mouse_coord);
    if state
        .block_field()
        .get::<block::World>(state.world())
        .and_then(|w| state.block_field().get::<block::Table>(w.selecting_table()))
        .map(|t| t.is_bind_to_grid())
        .unwrap_or(false)
        && (!ignore_binding)
    {
        [(p[0] * 2.0).round() / 2.0, (p[1] * 2.0).round() / 2.0]
    } else {
        [p[0], p[1]]
    }
}

fn get_focused_position(
    state: &State,
    screen_position: &[f32; 2],
    pixel_ratio: f32,
    offset: &[f32; 3],
) -> [f32; 3] {
    let cpr = get_canvas_pixel_ratio(state.pixel_ratio());

    let canvas_pos = [screen_position[0] * cpr, screen_position[1] * cpr];

    let pos = if let Some(tableblock) = state.renderer().and_then(|r| {
        r.table_object_id(state.canvas_size(), &canvas_pos)
            .map(|t| t.clone())
    }) {
        if let Some(boxblock) = state
            .block_field()
            .get::<block::table_object::Boxblock>(&tableblock.block_id)
        {
            let (r, s, t) =
                box_surface(boxblock.position(), boxblock.size(), tableblock.surface_idx);
            let p = state
                .camera()
                .collision_point(state.canvas_size(), &canvas_pos, &r, &s, &t);
            let p = bind_to_grid(state, false, &p);
            let p = match tableblock.surface_idx {
                0 => [p[0], p[1], p[2] + offset[2]],
                1 => [p[0], p[1] + offset[1], p[2]],
                2 => [p[0] + offset[0], p[1], p[2]],
                3 => [p[0] - offset[0], p[1], p[2]],
                4 => [p[0], p[1] - offset[1], p[2]],
                5 => [p[0], p[1], p[2] - offset[2]],
                _ => unreachable!(),
            };
            Some(p)
        } else if let Some(character) = state
            .block_field()
            .get::<block::Character>(&tableblock.block_id)
        {
            let r = character.position();
            let s = [1.0, 0.0, 0.0];
            let t = [0.0, 1.0, 0.0];
            let p = state
                .camera()
                .collision_point(state.canvas_size(), &canvas_pos, &r, &s, &t);
            let p = bind_to_grid(state, false, &p);
            let p = [p[0], p[1], p[2] + offset[2]];
            Some(p)
        } else {
            None
        }
    } else {
        None
    };
    pos.unwrap_or({
        let [x, y] = get_table_position(state, false, screen_position, pixel_ratio);
        [x, y, offset[2]]
    })
}

fn box_surface(p: &[f32; 3], s: &[f32; 3], s_idx: usize) -> ([f32; 3], [f32; 3], [f32; 3]) {
    match s_idx {
        0 => {
            let r = [p[0], p[1], p[2] + s[2] * 0.5];
            let s = [1.0, 0.0, 0.0];
            let t = [0.0, 1.0, 0.0];
            (r, s, t)
        }
        1 => {
            let r = [p[0], p[1] + s[1] * 0.5, p[2]];
            let s = [1.0, 0.0, 0.0];
            let t = [0.0, 0.0, 1.0];
            (r, s, t)
        }
        2 => {
            let r = [p[0] + s[0] * 0.5, p[1], p[2]];
            let s = [0.0, 1.0, 0.0];
            let t = [0.0, 0.0, 1.0];
            (r, s, t)
        }
        3 => {
            let r = [p[0] - s[0] * 0.5, p[1], p[2]];
            let s = [0.0, 1.0, 0.0];
            let t = [0.0, 0.0, 1.0];
            (r, s, t)
        }
        4 => {
            let r = [p[0], p[1] - s[1] * 0.5, p[2]];
            let s = [1.0, 0.0, 0.0];
            let t = [0.0, 0.0, 1.0];
            (r, s, t)
        }
        5 => {
            let r = [p[0], p[1], p[2] - s[2] * 0.5];
            let s = [1.0, 0.0, 0.0];
            let t = [0.0, 1.0, 0.0];
            (r, s, t)
        }
        _ => unreachable!(),
    }
}

fn bind_to_grid(state: &State, ignore_binding: bool, p: &[f32; 3]) -> [f32; 3] {
    if state
        .block_field()
        .get::<block::World>(state.world())
        .and_then(|w| state.block_field().get::<block::Table>(w.selecting_table()))
        .map(|t| t.is_bind_to_grid())
        .unwrap_or(false)
        && (!ignore_binding)
    {
        [
            (p[0] * 2.0).round() / 2.0,
            (p[1] * 2.0).round() / 2.0,
            (p[2] * 2.0).round() / 2.0,
        ]
    } else {
        [p[0], p[1], p[2]]
    }
}

fn render_canvas(state: &mut State) {
    state.render();
}

fn timestamp() -> Option<f64> {
    Some(js_sys::Date::now())
}

fn send_pack_cmd(
    block_field: &block::Field,
    packs: Vec<&BlockId>,
    removed: HashSet<BlockId>,
) -> Cmd<Msg, Sub> {
    let packs = block_field.pack_listed(packs);

    Cmd::task(move |resolve| {
        packs.then(|packs| {
            if let Some(packs) = packs {
                resolve(Msg::SendBlockPacks(packs.into_iter().collect(), removed));
            }
        })
    })
}

fn clone_prop(block_field: &mut block::Field, prop: &BlockId) -> Option<block::Property> {
    let mut prop = if let Some(prop) = block_field.get::<block::Property>(prop) {
        Some(prop.clone())
    } else {
        None
    };

    if let Some(prop) = &mut prop {
        if let block::property::Value::Children(children) = prop.value() {
            let mut new_children = vec![];

            for child in children {
                let child = clone_prop(block_field, child);
                if let Some(child) = child {
                    let child = block_field.add(child);
                    new_children.push(child);
                }
            }

            prop.set_value(block::property::Value::Children(new_children));
        }
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

fn check_focused_object(state: &mut State, mouse_position: &[f32; 2]) {
    let cpr = get_canvas_pixel_ratio(state.pixel_ratio());

    let canvas_pos = [mouse_position[0] * cpr, mouse_position[1] * cpr];

    if let Some(tableblock) = state.renderer().and_then(|r| {
        r.table_object_id(state.canvas_size(), &canvas_pos)
            .map(|t| t.clone())
    }) {
        if state
            .block_field()
            .get::<block::Character>(&tableblock.block_id)
            .is_some()
        {
            let table = state.table_mut();
            table.set_focused(state::table::Focused::Character(tableblock.clone()));
        } else if state
            .block_field()
            .get::<block::table_object::Tablemask>(&tableblock.block_id)
            .is_some()
        {
            let table = state.table_mut();
            table.set_focused(state::table::Focused::Tablemask(tableblock.clone()));
        } else if state
            .block_field()
            .get::<block::table_object::Area>(&tableblock.block_id)
            .is_some()
        {
            let table = state.table_mut();
            table.set_focused(state::table::Focused::Area(tableblock.clone()));
        } else if state
            .block_field()
            .get::<block::table_object::Boxblock>(&tableblock.block_id)
            .is_some()
        {
            let table = state.table_mut();
            table.set_focused(state::table::Focused::Boxblock(tableblock.clone()));
        }
    } else {
        state.table_mut().set_focused(state::table::Focused::None);
    }
}
