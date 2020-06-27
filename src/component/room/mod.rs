mod render;
mod state;

use crate::{
    block::{self, BlockId},
    color_system,
    model::modeless::ModelessId,
    random_id,
    renderer::{Camera, Renderer},
    resource::{Data, ResourceId},
    skyway, Promise,
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
    SetCharacterName(BlockId, String),
    SetCharacterSize(BlockId, [Option<f32>; 2]),

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

fn timestamp() -> Option<u32> {
    Some(js_sys::Date::now() as u32)
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
            state
                .block_field_mut()
                .update(world, timestamp(), |world: &mut block::World| {
                    world.add_character(character.clone());
                });

            render_canvas(state);

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
                    timestamp(),
                    |table: &mut block::Table| {
                        table.add_tablemask(tablemask.clone());
                    },
                );

                render_canvas(state);

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
                state
                    .block_field_mut()
                    .update(world, timestamp(), |world: &mut block::World| {
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
                    timestamp(),
                    |table: &mut block::Table| {
                        table.add_tablemask(tablemask.clone());
                    },
                );

                render_canvas(state);

                send_pack_cmd(state.block_field(), vec![&tablemask, selecting_table])
            } else {
                state.dequeue()
            }
        }

        Msg::RemoveCharacterToCloseContextmenu(character) => {
            state.close_contextmenu();

            let world = state.world();
            state
                .block_field_mut()
                .update(world, timestamp(), |world: &mut block::World| {
                    world.remove_character(&character);
                });

            render_canvas(state);

            send_pack_cmd(state.block_field(), vec![world])
        }

        Msg::RemoveTablemaskToCloseContextmenu(tablemask) => {
            state.close_contextmenu();

            if let Some(selecting_table) = state.selecting_table() {
                state.block_field_mut().update(
                    selecting_table,
                    timestamp(),
                    |table: &mut block::Table| {
                        table.remove_tablemask(&tablemask);
                    },
                );

                render_canvas(state);

                send_pack_cmd(state.block_field(), vec![selecting_table])
            } else {
                state.dequeue()
            }
        }

        // Mouse
        Msg::SetLastMousePosition(mouse_position) => {
            if let Some(block_id) = state
                .renderer()
                .and_then(|r| r.table_object_id(&mouse_position))
            {
                let block_id = state.block_field().block_id(*block_id);
                state.table_mut().set_focused(Some(block_id));
            } else {
                state.table_mut().set_focused(None);
            }

            if let Some(block_id) = state.table().focused() {
                let bf = state.block_field_mut();
                bf.update(block_id, None, |character: &mut block::Character| {})
                    .and_then(|bf| {
                        bf.update(
                            block_id,
                            None,
                            |tablemask: &mut block::table_object::Tablemask| {},
                        )
                    })
                    .and_then(|bf| {
                        bf.update(block_id, None, |route: &mut block::table_object::Route| {})
                    });
            }

            state.table_mut().set_last_mouse_position(mouse_position);

            render_canvas(state);

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

        Msg::SetSelectingTableTool(table_tool) => {
            state.table_mut().set_selecting_tool(table_tool);
            state.dequeue()
        }

        Msg::SetTableObjectPositionWithMousePosition(block_id, mouse_position) => {
            let [x, y] = get_table_position(state, &mouse_position, state.pixel_ratio());
            let timestamp = timestamp();

            let updated = state
                .block_field_mut()
                .update(&block_id, timestamp, |character: &mut block::Character| {
                    character.set_position([x, y, 0.0]);
                })
                .and_then(|bf| {
                    bf.update(
                        &block_id,
                        timestamp,
                        |tablemask: &mut block::table_object::Tablemask| {
                            tablemask.set_position([x, y]);
                        },
                    )
                })
                .is_none();

            if updated {
                render_canvas(state);
                send_pack_cmd(state.block_field(), vec![&block_id])
            } else {
                state.dequeue()
            }
        }

        Msg::DrawLineWithMousePosition(a, b) => {
            if let Some(texture_id) = state
                .selecting_table()
                .and_then(|table_id| state.block_field_mut().get::<block::Table>(&table_id))
                .map(|table| table.drawing_texture_id())
            {
                let [ax, ay] = get_table_position(state, &a, state.pixel_ratio());
                let [bx, by] = get_table_position(state, &b, state.pixel_ratio());

                state.block_field_mut().update(
                    texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        let [ax, ay] = texture.texture_position(&[ax as f64, ay as f64]);
                        let [bx, by] = texture.texture_position(&[bx as f64, by as f64]);

                        let context = texture.context();

                        context.set_line_width(0.5);
                        context.set_line_cap("round");
                        context.set_stroke_style(&color_system::gray(0, 9).to_jsvalue());
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

                send_pack_cmd(state.block_field(), vec![&texture_id])
            } else {
                state.dequeue()
            }
        }

        Msg::EraceLineWithMousePosition(a, b) => {
            if let Some(texture_id) = state
                .selecting_table()
                .and_then(|table_id| state.block_field_mut().get::<block::Table>(&table_id))
                .map(|table| table.drawing_texture_id())
            {
                let [ax, ay] = get_table_position(state, &a, state.pixel_ratio());
                let [bx, by] = get_table_position(state, &b, state.pixel_ratio());

                state.block_field_mut().update(
                    texture_id,
                    timestamp(),
                    |texture: &mut block::table::Texture| {
                        let [ax, ay] = texture.texture_position(&[ax as f64, ay as f64]);
                        let [bx, by] = texture.texture_position(&[bx as f64, by as f64]);

                        let context = texture.context();

                        context.set_line_width(1.0);
                        context.set_line_cap("round");
                        context.set_stroke_style(&color_system::gray(0, 9).to_jsvalue());
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

                send_pack_cmd(state.block_field(), vec![&texture_id])
            } else {
                state.dequeue()
            }
        }

        // World
        Msg::AddTable => {
            let texture = block::table::Texture::new(&[4096, 4096], [20.0, 20.0]);
            let texture = state.block_field_mut().add(texture);
            let table = block::Table::new(texture.clone(), [20.0, 20.0], "テーブル");
            let table = state.block_field_mut().add(table);

            state.block_field_mut().update(
                state.world(),
                timestamp(),
                |world: &mut block::World| world.add_table(table.clone()),
            );

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
                        character.set_size([w, 0.0, h]);
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
                            character.set_size([w, 0.0, h]);
                        },
                    );
                }
                if let Some(h) = h {
                    let w = h / ih * iw;
                    state.block_field_mut().update(
                        &character,
                        timestamp(),
                        |character: &mut block::Character| {
                            character.set_size([w, 0.0, h]);
                        },
                    );
                }
            }

            send_pack_cmd(state.block_field(), vec![&character])
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
                            if let Ok(data) = data {
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
                promise.then(|data| {
                    if let Ok(data) = data {
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
