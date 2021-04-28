use super::super::{
    super::util::cmds::Cmds,
    super::util::State,
    model::table::{
        BoxblockTool, CharacterTool, EraserTool, FillShapeTool, LineShapeTool, PenTool, ShapeTool,
        TableTool,
    },
    renderer::{CameraMatrix, ObjectId},
};
use super::{ElementId, Implement, KeyState, Modal, MouseBtnState, Msg, On, Overlay, Props};
use crate::arena::block;
use crate::arena::player::{self, Player};
use crate::arena::resource;
use crate::arena::Insert;
use crate::libs::color::Pallet;
use crate::libs::modeless_list::ModelessList;
use crate::libs::select_list::SelectList;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

impl Implement {
    pub fn constructor(props: Props, builder: &mut ComponentBuilder<Msg, On>) -> Self {
        let mut block_arena = block::Arena::new();
        let mut local_block_arena = block::Arena::new();

        let chat = block::chat::Chat::new(vec![
            block_arena.insert(block::chat::channel::Channel::new(
                String::from("メイン"),
                block::chat::channel::ChannelType::Public,
            )),
            block_arena.insert(block::chat::channel::Channel::new(
                String::from("サブ"),
                block::chat::channel::ChannelType::Public,
            )),
        ]);

        let chat_id = block_arena.insert(chat);

        let tex_size = [4096, 4096];
        let tbl_size = [20.0, 20.0];
        let drawing_texture_id =
            local_block_arena.insert(block::texture::Texture::new(&tex_size, tbl_size.clone()));
        let darwed_texture_id =
            block_arena.insert(block::texture::Texture::new(&tex_size, tbl_size.clone()));
        let table_id = block_arena.insert(block::table::Table::new(
            drawing_texture_id,
            darwed_texture_id,
            [tbl_size[0] as f32, tbl_size[1] as f32],
            "最初のテーブル",
        ));
        let world_id = block_arena.insert(block::world::World::new(table_id));

        let mut player_arena = player::Arena::new();
        player_arena.insert(Rc::clone(&props.client_id), Player::new());

        builder.add_batch(|mut resolve| {
            let a = Closure::wrap(Box::new(move || {
                resolve(Msg::ResetCanvasSize);
            }) as Box<dyn FnMut()>);
            web_sys::window()
                .unwrap()
                .set_onresize(Some(a.as_ref().unchecked_ref()));
            a.forget();
        });
        builder.add_batch(|mut resolve| {
            let a = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                resolve(Msg::UpdateKeyState {
                    e,
                    is_key_down: true,
                });
            }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
            web_sys::window()
                .unwrap()
                .set_onkeydown(Some(a.as_ref().unchecked_ref()));
            a.forget();
        });
        builder.add_batch(|mut resolve| {
            let a = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                resolve(Msg::UpdateKeyState {
                    e,
                    is_key_down: false,
                });
            }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
            web_sys::window()
                .unwrap()
                .set_onkeyup(Some(a.as_ref().unchecked_ref()));
            a.forget();
        });

        Self {
            cmds: Cmds::new(),

            peer: props.peer,
            peer_id: props.peer_id,
            room: props.room,
            room_id: props.room_id,
            client_id: props.client_id,

            element_id: ElementId {
                header_room_id: format!("{:X}", crate::libs::random_id::u128val()),
            },

            table_tools: State::new(SelectList::new(
                vec![
                    TableTool::Selector,
                    TableTool::TableEditor,
                    TableTool::Hr(Rc::new(String::from("描画"))),
                    TableTool::Pen(PenTool {
                        line_width: 1.0,
                        pallet: Pallet::gray(9).a(100),
                    }),
                    TableTool::Shape(SelectList::new(
                        vec![
                            ShapeTool::Line(LineShapeTool {
                                line_width: 0.5,
                                pallet: Pallet::gray(9).a(100),
                            }),
                            ShapeTool::Rect(FillShapeTool {
                                line_width: 0.5,
                                line_pallet: Pallet::gray(9).a(100),
                                fill_pallet: Pallet::gray(0).a(100),
                            }),
                            ShapeTool::Ellipse(FillShapeTool {
                                line_width: 0.5,
                                line_pallet: Pallet::gray(9).a(100),
                                fill_pallet: Pallet::gray(0).a(100),
                            }),
                        ],
                        0,
                    )),
                    TableTool::Eraser(EraserTool {
                        line_width: 2.0,
                        alpha: 100,
                    }),
                    TableTool::Character(CharacterTool {
                        size: 1.0,
                        height: 1.0,
                        tex_id: None,
                        name: String::from(""),
                    }),
                    TableTool::Boxblock(BoxblockTool {
                        size: [1.0, 1.0, 1.0],
                        color: Pallet::blue(5).a(100),
                    }),
                ],
                0,
            )),
            modeless_list: ModelessList::new(),
            modeless_container_element: None,
            dragging_modeless_tab: None,

            block_arena,
            local_block_arena,
            player_arena,
            resource_arena: resource::Arena::new(),

            renderer: None,
            camera_matrix: CameraMatrix::new(),

            chat_id,
            world_id,

            modal: Modal::None,
            overlay: Overlay::None,
            contextmenu: None,

            mouse_btn_state: MouseBtnState::new(),
            key_state: KeyState {
                alt_key: false,
                ctrl_key: false,
                shift_key: false,
                space_key: false,
            },
            canvas: None,
            canvas_pos: [0.0, 0.0],
            canvas_size: [1.0, 1.0],
            drawing_line: vec![],
            grabbed_object_id: ObjectId::None,
        }
    }
}
