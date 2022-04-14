mod renderer;
pub mod table_tool;
mod table_tool_state;
use crate::arena::{block, ArenaMut, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use crate::libs::window;
use renderer::{CameraMatrix, ObjectId, Renderer};
use std::cell::RefCell;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use table_tool::TableTool;
use table_tool_state::TableToolState;

pub struct UpdatedBlocks {
    update: HashSet<U128Id>,
    insert: HashSet<U128Id>,
}

pub struct Table {
    renderer: Rc<RefCell<Renderer>>,
    camera_matrix: Rc<RefCell<CameraMatrix>>,

    is_2d_mode: bool,
    is_debug_mode: bool,

    camera_state: CameraState,
    tool_state: TableToolState,
    last_cursor_position: [f64; 2],

    is_reserve_rendering: bool,

    updated_blocks: UpdatedBlocks,
}

struct CameraState {
    is_rotating: bool,
    is_moving: bool,
}

impl Table {
    pub fn new() -> Self {
        Self {
            renderer: Rc::new(RefCell::new(Renderer::new())),
            camera_matrix: Rc::new(RefCell::new(CameraMatrix::new())),

            is_2d_mode: false,
            is_debug_mode: false,

            camera_state: CameraState {
                is_rotating: false,
                is_moving: false,
            },
            tool_state: TableToolState::None,
            last_cursor_position: [0.0, 0.0],
            is_reserve_rendering: true,

            updated_blocks: UpdatedBlocks {
                insert: HashSet::new(),
                update: HashSet::new(),
            },
        }
    }

    pub fn take_updated(&mut self) -> UpdatedBlocks {
        let mut taked = UpdatedBlocks {
            insert: HashSet::new(),
            update: HashSet::new(),
        };

        std::mem::swap(&mut self.updated_blocks, &mut taked);

        taked
    }

    pub fn canvas(&self) -> Rc<web_sys::HtmlCanvasElement> {
        self.renderer.borrow().canvas()
    }

    pub fn reserve_rendering(&mut self) {
        self.is_reserve_rendering = true;
    }

    pub fn render_reserved(
        &mut self,
        world: BlockRef<block::World>,
    ) -> Pin<Box<dyn Future<Output = ()>>> {
        if self.is_reserve_rendering {
            self.is_reserve_rendering = false;
            let renderer = Rc::clone(&self.renderer);
            let camera_matrix = Rc::clone(&self.camera_matrix);
            let grabbed_object_id = if let TableToolState::Selecter(state) = &self.tool_state {
                state
                    .grabbed_object
                    .as_ref()
                    .map(|(_, block_id)| U128Id::clone(block_id))
                    .unwrap_or(U128Id::none())
            } else {
                U128Id::none()
            };
            let is_debug_mode = self.is_debug_mode;

            Box::pin(kagura::util::Task::new(move |resolve| {
                window::request_animation_frame(move || {
                    renderer.borrow_mut().render(
                        is_debug_mode,
                        world,
                        &camera_matrix.borrow(),
                        &grabbed_object_id,
                    );
                    resolve(());
                });
            }))
        } else {
            Box::pin(std::future::ready(()))
        }
    }

    fn selecting_table(world: BlockRef<block::World>) -> Option<BlockMut<block::Table>> {
        world
            .map(|world| {
                world
                    .selecting_scene()
                    .map(|secene| BlockMut::clone(secene.selecting_table()))
            })
            .unwrap_or(None)
    }

    /// nベクトルをcubeに接するように拡張する
    fn n_cube(n: &[f64; 3], cube: &[f64; 3]) -> [f64; 3] {
        let mut ratio = None;
        if n[0] != 0.0 {
            let x_ratio = (cube[0] * 0.5 / n[0]).abs();
            ratio = Some(x_ratio);
        }

        if n[1] != 0.0 {
            let y_ratio = (cube[1] * 0.5 / n[1]).abs();
            ratio = if let Some(ratio) = ratio {
                Some(ratio.min(y_ratio))
            } else {
                Some(y_ratio)
            };
        }

        if n[2] != 0.0 {
            let z_ratio = (cube[2] * 0.5 / n[2]).abs();
            ratio = if let Some(ratio) = ratio {
                Some(ratio.min(z_ratio))
            } else {
                Some(z_ratio)
            };
        }

        if let Some(ratio) = ratio {
            [n[0] * ratio, n[1] * ratio, n[2] * ratio]
        } else {
            n.clone()
        }
    }

    /// 画面上の座標をキャンバス内の座標に変換
    pub fn mouse_coord(&self, page_x: f64, page_y: f64) -> [f64; 2] {
        let rect = self.canvas().get_bounding_client_rect();
        let client_x = page_x - rect.left();
        let client_y = page_y - rect.top();
        [client_x, client_y]
    }

    /// Boxblockを作成して配置する
    pub fn create_boxblock(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::Boxblock,
    ) {
        let mut table = unwrap!(Self::selecting_table(world.as_ref()));
        let renderer = self.renderer.as_ref();
        let (p, n) = renderer.borrow().get_focused_position(
            &self.camera_matrix.borrow(),
            mouse_coord[0],
            mouse_coord[1],
        );
        let n = Self::n_cube(&n, &option.size);
        let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];

        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);

        let mut boxblock = block::Boxblock::new(is_bind_to_grid);

        crate::debug::log_1(format!("add boxblock to {:?}", &p));

        boxblock.set_size(option.size.clone());
        boxblock.set_position(p);
        boxblock.set_color(option.color);
        boxblock.set_texture(option.texture.as_ref().map(|block| BlockRef::clone(block)));
        boxblock.set_shape(option.shape.clone());

        let boxblock = arena.insert(boxblock);
        let boxblock_id = boxblock.id();
        table.update(|table| {
            table.push_boxblock(boxblock);
        });
        self.reserve_rendering();
        self.updated_blocks.update.insert(table.id());
        self.updated_blocks.insert.insert(boxblock_id);
    }

    pub fn create_character(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::Character,
    ) {
        let table = unwrap!(Self::selecting_table(world.as_ref()));
        let renderer = self.renderer.as_ref();
        let (p, _) = renderer.borrow().get_focused_position(
            &self.camera_matrix.borrow(),
            mouse_coord[0],
            mouse_coord[1],
        );
        let n =
            self.camera_matrix
                .borrow()
                .position_vec_n(&[p[0] as f32, p[1] as f32, p[2] as f32]);
        let p = [
            p[0] + n[0] as f64 / 128.0,
            p[1] + n[1] as f64 / 128.0,
            p[2] + n[2] as f64 / 128.0,
        ];

        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);

        let mut character = block::Character::new(is_bind_to_grid);

        character.set_size(option.size);
        character.set_position(p);
        character.set_tex_size(option.tex_size);
        character.set_color(option.color);
        character.set_texture_image(
            0,
            option.texture.as_ref().map(|block| BlockRef::clone(block)),
        );

        let character = arena.insert(character);
        let character_id = character.id();
        world.update(|world| {
            world.push_character(character);
        });
        self.reserve_rendering();
        self.updated_blocks.update.insert(world.id());
        self.updated_blocks.insert.insert(character_id);
    }

    pub fn create_craftboard(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::Craftboard,
    ) {
        let mut table = unwrap!(Self::selecting_table(world.as_ref()));
        let renderer = self.renderer.as_ref();
        let (p, _) = renderer.borrow().get_focused_position(
            &self.camera_matrix.borrow(),
            mouse_coord[0],
            mouse_coord[1],
        );

        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);
        let mut craftboard = block::Craftboard::new(is_bind_to_grid, p);

        craftboard.set_size(option.size.clone());

        let craftboard = arena.insert(craftboard);
        let craftboard_id = craftboard.id();
        table.update(|table| {
            table.push_craftboard(craftboard);
        });
        self.reserve_rendering();
        self.updated_blocks.update.insert(table.id());
        self.updated_blocks.insert.insert(craftboard_id);
    }

    pub fn rotate_camera(&mut self, movement: &[f64; 2]) {
        let h_rot = -movement[0] / 500.0;
        let v_rot = -movement[1] / 500.0;

        let z_rot = self.camera_matrix.borrow().z_axis_rotation() + h_rot as f32;
        let x_rot = self.camera_matrix.borrow().x_axis_rotation() + v_rot as f32;

        self.camera_matrix.borrow_mut().set_z_axis_rotation(z_rot);
        self.camera_matrix
            .borrow_mut()
            .set_x_axis_rotation(x_rot, false);

        self.reserve_rendering();
    }

    pub fn move_camera_xy(&mut self, movement: &[f64; 2]) {
        let h_mov = -movement[0] / 50.0;
        let v_mov = movement[1] / 50.0;

        let p = {
            let camera_matrix = self.camera_matrix.borrow();
            let p = camera_matrix.movement();
            [p[0] + h_mov as f32, p[1] + v_mov as f32, p[2]]
        };

        self.camera_matrix.borrow_mut().set_movement(p);

        self.reserve_rendering();
    }

    pub fn move_camera_z(&mut self, movement: f64) {
        let p = {
            let camera_matrix = self.camera_matrix.borrow();
            let p = camera_matrix.movement();
            [p[0], p[1], p[2] + movement as f32 / 16.0]
        };

        self.camera_matrix.borrow_mut().set_movement(p);

        self.reserve_rendering();
    }

    pub fn drag_block(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
    ) {
        let (block_kind, block_id) =
            unwrap!(self.tool_state.selecter_mut().grabbed_object.as_ref());
        let renderer = self.renderer.as_ref();
        let (p, n) = renderer.borrow().get_focused_position(
            &self.camera_matrix.borrow(),
            mouse_coord[0],
            mouse_coord[1],
        );

        match block_kind {
            BlockKind::Boxblock => {
                if let Some(mut block) = arena.get_mut::<block::Boxblock>(block_id) {
                    let block_id = block.id();
                    block.update(|block| {
                        let n = Self::n_cube(&n, block.size());
                        let p = if block.is_bind_to_grid() {
                            [
                                (p[0] * 2.0).round() / 2.0,
                                (p[1] * 2.0).round() / 2.0,
                                (p[2] * 2.0).round() / 2.0,
                            ]
                        } else {
                            p
                        };
                        let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];
                        block.set_position(p);

                        self.reserve_rendering();
                        self.updated_blocks.update.insert(block_id);
                    });
                }
            }
            BlockKind::Character => {
                if let Some(mut block) = arena.get_mut::<block::Character>(block_id) {
                    let block_id = block.id();
                    block.update(|block| {
                        let p = if block.is_bind_to_grid() {
                            [
                                (p[0] * 2.0).round() / 2.0,
                                (p[1] * 2.0).round() / 2.0,
                                (p[2] * 2.0).round() / 2.0,
                            ]
                        } else {
                            p
                        };
                        block.set_position(p);

                        self.reserve_rendering();
                        self.updated_blocks.update.insert(block_id);
                    });
                }
            }
            BlockKind::Craftboard => {
                if let Some(mut block) = arena.get_mut::<block::Craftboard>(block_id) {
                    let block_id = block.id();
                    block.update(|block| {
                        let p = if block.is_bind_to_grid() {
                            [
                                (p[0] * 2.0).round() / 2.0,
                                (p[1] * 2.0).round() / 2.0,
                                (p[2] * 2.0).round() / 2.0,
                            ]
                        } else {
                            p
                        };
                        block.set_position(p);

                        self.reserve_rendering();
                        self.updated_blocks.update.insert(block_id);
                    });
                }
            }
            _ => {}
        }
    }

    pub fn on_click(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        e: web_sys::MouseEvent,
        tool: &TableTool,
    ) {
        let mouse_coord = self.mouse_coord(e.page_x() as f64, e.page_y() as f64);

        match &tool {
            TableTool::Boxblock(tool) => {
                self.create_boxblock(arena, world, &mouse_coord, tool);
            }
            TableTool::Character(tool) => {
                self.create_character(arena, world, &mouse_coord, tool);
            }
            TableTool::Craftboard(tool) => {
                self.create_craftboard(arena, world, &mouse_coord, tool);
            }
            _ => {}
        }
    }

    pub fn on_wheel(&mut self, e: web_sys::WheelEvent, tool: &TableTool) {
        let delta_y = if e.delta_mode() == web_sys::WheelEvent::DOM_DELTA_PIXEL {
            e.delta_y()
        } else {
            e.delta_y() * 16.0
        };

        self.move_camera_z(delta_y);
    }

    pub fn on_mousedown(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        e: web_sys::MouseEvent,
        tool: &TableTool,
    ) {
        let page_x = e.page_x() as f64;
        let page_y = e.page_y() as f64;
        let mouse_coord = self.mouse_coord(page_x, page_y);
        let button = e.button();

        if button == 0 {
            if e.alt_key() {
                self.camera_state.is_rotating = true;
            } else {
                match tool {
                    TableTool::Selecter(..) => {
                        let (block_kind, block_id) = self.focused_block(page_x, page_y);
                        let mut camera_is_moving = self.camera_state.is_moving;

                        match block_kind {
                            BlockKind::Boxblock
                                if !arena
                                    .get::<block::Boxblock>(&block_id)
                                    .and_then(|x| x.map(|boxblock| boxblock.is_fixed_position()))
                                    .unwrap_or(true) =>
                            {
                                self.tool_state.selecter_mut().grabbed_object =
                                    Some((block_kind, block_id));
                                self.reserve_rendering();
                            }
                            BlockKind::Character
                                if !arena
                                    .get::<block::Character>(&block_id)
                                    .and_then(|x| x.map(|character| character.is_fixed_position()))
                                    .unwrap_or(true) =>
                            {
                                self.tool_state.selecter_mut().grabbed_object =
                                    Some((block_kind, block_id));
                                self.reserve_rendering();
                            }
                            BlockKind::Craftboard
                                if !arena
                                    .get::<block::Craftboard>(&block_id)
                                    .and_then(|x| {
                                        x.map(|craftboard| craftboard.is_fixed_position())
                                    })
                                    .unwrap_or(true) =>
                            {
                                self.tool_state.selecter_mut().grabbed_object =
                                    Some((block_kind, block_id));
                                self.reserve_rendering();
                            }
                            _ => {
                                camera_is_moving = true;
                            }
                        }

                        self.camera_state.is_moving = camera_is_moving;
                    }
                    _ => {}
                }
            }
        } else if button == 1 {
            self.camera_state.is_moving = true;
        }

        self.last_cursor_position = mouse_coord;
    }

    pub fn on_mouseup(&mut self, e: web_sys::MouseEvent, tool: &TableTool) {
        let mouse_coord = self.mouse_coord(e.page_x() as f64, e.page_y() as f64);
        let button = e.button();

        if button == 0 {
            self.camera_state.is_rotating = false;
            self.camera_state.is_moving = false;

            match tool {
                TableTool::Selecter(..) => {
                    self.tool_state.selecter_mut().grabbed_object = None;
                    self.reserve_rendering();
                }
                _ => {}
            }
        } else if button == 1 {
            self.camera_state.is_moving = false;
        }

        self.last_cursor_position = mouse_coord;
    }

    pub fn on_mousemove(
        &mut self,
        arena: ArenaMut,
        world: BlockMut<block::World>,
        e: web_sys::MouseEvent,
        tool: &TableTool,
    ) {
        let mouse_coord = self.mouse_coord(e.page_x() as f64, e.page_y() as f64);

        if self.camera_state.is_rotating {
            let x_mov = mouse_coord[0] - self.last_cursor_position[0];
            let y_mov = mouse_coord[1] - self.last_cursor_position[1];
            self.rotate_camera(&[x_mov, y_mov]);
        }

        if self.camera_state.is_moving {
            let x_mov = mouse_coord[0] - self.last_cursor_position[0];
            let y_mov = mouse_coord[1] - self.last_cursor_position[1];
            self.move_camera_xy(&[x_mov, y_mov]);
        }

        match tool {
            TableTool::Selecter(..) => {
                self.drag_block(arena, world, &mouse_coord);
            }
            _ => {}
        }

        self.last_cursor_position = mouse_coord;
    }

    pub fn focused_block(&self, page_x: f64, page_y: f64) -> (BlockKind, U128Id) {
        let [px_x, px_y] = self.mouse_coord(page_x, page_y);
        let renderer = self.renderer.as_ref();

        match renderer.borrow().get_object_id(px_x, px_y) {
            ObjectId::Boxblock(b_id, ..) => (BlockKind::Boxblock, b_id),
            ObjectId::Character(b_id, ..) => (BlockKind::Character, b_id),
            ObjectId::Craftboard(b_id, ..) => (BlockKind::Craftboard, b_id),
            _ => (BlockKind::None, U128Id::none()),
        }
    }
}
