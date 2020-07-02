use crate::{
    block::{self, BlockId},
    color_system,
    model::{self, modeless::ModelessId},
    renderer::Renderer,
    resource::{Data, ResourceId},
    skyway,
};
use kagura::prelude::*;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

mod render;
mod state;

use render::render;

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
    SetModelessTabIdx(ModelessId, usize),
    GrubModelessTab(ModelessId, usize),
    DropModelessTabToModeless(ModelessId),
    DropModelessTab([f64; 2]),

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
    SetLastMousePosition(bool, [f32; 2]),
    SetLastMouseDownPosition([f32; 2]),
    SetLastMouseUpPosition([f32; 2]),
    SetCameraRotationWithMouseMovement([f32; 2]),
    SetCameraMovementWithMouseMovement([f32; 2]),
    SetCameraMovementWithMouseWheel(f32),
    SetSelectingTableTool(state::table::Tool),
    SetCharacterPositionWithMousePosition(BlockId, [f32; 2]),
    SetTablemaskPositionWithMousePosition(BlockId, [f32; 2]),
    DrawLineWithMousePosition([f32; 2], [f32; 2]),
    EraceLineWithMousePosition([f32; 2], [f32; 2]),
    MeasureLineWithMousePosition([f32; 2], [f32; 2]),

    // World
    AddTable,

    // Table
    SetTableSize(BlockId, [f32; 2]),
    SetTableImage(BlockId, Option<ResourceId>),

    // PersonalData
    SetPersonalDataWithPlayerName(String),
    SetPersonalDataWithIconImageToCloseModal(u128),

    // table object

    // character
    SetCharacterName(BlockId, String),
    SetCharacterSize(BlockId, [Option<f32>; 2]),
    SetCharacterTextrureToCloseModal(BlockId, Option<ResourceId>),

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
    LoadFromFileList(web_sys::FileList),
    LoadDataToResource(Data),

    // 接続に関する操作
    SendBlockPacks(HashMap<BlockId, JsValue>),
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
            )
        }

        Msg::AddTablemaskWithMousePositionToCloseContextmenu(mouse_position) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table().map(|t| t.clone()) {
                let [x, y] = get_table_position(state, &mouse_position, state.pixel_ratio());

                let prop_root = block::Property::new("");
                let prop_root = state.block_field_mut().add(prop_root);

                let mut tablemask = block::table_object::Tablemask::new(prop_root.clone());
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

                send_pack_cmd(state.block_field(), vec![&tablemask, &selecting_table])
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

                send_pack_cmd(state.block_field(), vec![&tablemask, &selecting_table])
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

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![&world])
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

                render_canvas(state);

                send_pack_cmd(state.block_field(), vec![&selecting_table])
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
            let [x, y] = get_table_position(state, &mouse_position, state.pixel_ratio());
            let timestamp = timestamp();

            let updated = state
                .block_field_mut()
                .update(&block_id, timestamp, |character: &mut block::Character| {
                    character.set_position([x, y, 0.0]);
                })
                .is_none();

            if updated {
                render_canvas(state);
                send_pack_cmd(state.block_field(), vec![&block_id])
            } else {
                state.dequeue()
            }
        }

        Msg::SetTablemaskPositionWithMousePosition(block_id, mouse_position) => {
            let [x, y] = get_table_position(state, &mouse_position, state.pixel_ratio());
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
                send_pack_cmd(state.block_field(), vec![&block_id])
            } else {
                state.dequeue()
            }
        }

        Msg::DrawLineWithMousePosition(a, b) => {
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
                let [ax, ay] = get_table_position(state, &a, state.pixel_ratio());
                let [bx, by] = get_table_position(state, &b, state.pixel_ratio());

                state.block_field_mut().update(
                    &texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        let [ax, ay] = texture.texture_position(&[ax as f64, ay as f64]);
                        let [bx, by] = texture.texture_position(&[bx as f64, by as f64]);

                        let context = texture.context();

                        context.set_line_width(0.5);
                        context.set_line_cap("round");
                        context.set_stroke_style(&color_system::gray(255, 9).to_jsvalue());
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

        Msg::EraceLineWithMousePosition(a, b) => {
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
                let [ax, ay] = get_table_position(state, &a, state.pixel_ratio());
                let [bx, by] = get_table_position(state, &b, state.pixel_ratio());

                state.block_field_mut().update(
                    &texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        let [ax, ay] = texture.texture_position(&[ax as f64, ay as f64]);
                        let [bx, by] = texture.texture_position(&[bx as f64, by as f64]);

                        let context = texture.context();

                        context.set_line_width(1.0);
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

        Msg::MeasureLineWithMousePosition(a, b) => {
            let [ax, ay] = get_table_position(state, &a, state.pixel_ratio());
            let [bx, by] = get_table_position(state, &b, state.pixel_ratio());
            let len = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();

            state.table_mut().clear_info();
            state
                .table_mut()
                .add_info("始点", format!("({:.1},{:.1})", ax, ay));
            state
                .table_mut()
                .add_info("終点", format!("({:.1},{:.1})", bx, by));
            state.table_mut().add_info("距離", format!("{:.1}", len));

            match state.table().selecting_tool() {
                state::table::Tool::Measure(Some(block_id)) => {
                    let block_id = block_id.clone();
                    state.block_field_mut().update(
                        &block_id,
                        timestamp(),
                        |m: &mut block::table_object::Measure| {
                            m.set_org([ax, ay, 0.0]);
                            m.set_vec([bx - ax, by - ay, 0.0]);
                        },
                    );
                }
                state::table::Tool::Measure(None) => {
                    let measure =
                        block::table_object::Measure::new([ax, ay, 0.0], [bx - ax, by - ay, 0.0]);
                    let bid = state.block_field_mut().add(measure);
                    state
                        .table_mut()
                        .set_selecting_tool(state::table::Tool::Measure(Some(bid)));
                }
                _ => (),
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

            send_pack_cmd(state.block_field(), vec![&texture, &table, state.world()])
        }

        // Table
        Msg::SetTableSize(table, size) => {
            state
                .block_field_mut()
                .update(&table, timestamp(), |table: &mut block::Table| {
                    table.set_size(size);
                });

            send_pack_cmd(state.block_field(), vec![&table])
        }

        Msg::SetTableImage(table, image) => {
            state
                .block_field_mut()
                .update(&table, timestamp(), |table: &mut block::Table| {
                    table.set_image_texture_id(image);
                });

            send_pack_cmd(state.block_field(), vec![&table])
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

        // Character
        Msg::SetCharacterName(character, name) => {
            state.block_field_mut().update(
                &character,
                timestamp(),
                |character: &mut block::Character| {
                    character.set_name(name);
                },
            );

            send_pack_cmd(state.block_field(), vec![&character])
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

            send_pack_cmd(state.block_field(), vec![&character])
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

        // Chat
        Msg::SetInputingChatMessage(msg) => {
            state.chat_mut().set_inputing_message(msg);
            state.dequeue()
        }

        Msg::SendInputingChatMessage => state.dequeue(),

        Msg::InsertChatItem(tab, item) => state.dequeue(),

        Msg::SetChatSender(idx) => state.dequeue(),

        Msg::AddChatTab => state.dequeue(),

        Msg::SetChatTabName(tab, name) => state.dequeue(),

        Msg::RemoveChatTab(tab) => state.dequeue(),

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

        // 接続に関する操作
        Msg::SendBlockPacks(packs) => {
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
        .collision_point_on_xy_plane(state.canvas_size(), &mouse_coord);
    [p[0], p[1]]
}

fn render_canvas(state: &mut State) {
    state.render();
}

fn timestamp() -> Option<f64> {
    Some(js_sys::Date::now())
}

fn send_pack_cmd(block_field: &block::Field, packs: Vec<&BlockId>) -> Cmd<Msg, Sub> {
    let packs = block_field.pack_listed(packs);

    Cmd::task(move |resolve| {
        packs.then(|packs| {
            if let Some(packs) = packs {
                resolve(Msg::SendBlockPacks(packs.into_iter().collect()));
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

    if let Some(block_id) = state.renderer().and_then(|r| {
        r.table_object_id(state.canvas_size(), &canvas_pos)
            .map(|t| t.clone())
    }) {
        web_sys::console::log_1(&JsValue::from(block_id.to_string()));
        if state
            .block_field()
            .get::<block::Character>(&block_id)
            .is_some()
        {
            let table = state.table_mut();
            table.set_focused(state::table::Focused::Character(block_id.clone()));
        } else if state
            .block_field()
            .get::<block::table_object::Tablemask>(&block_id)
            .is_some()
        {
            let table = state.table_mut();
            table.set_focused(state::table::Focused::Tablemask(block_id.clone()));
        }
    } else {
        state.table_mut().set_focused(state::table::Focused::None);
    }
}
