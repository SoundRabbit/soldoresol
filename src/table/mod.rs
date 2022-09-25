use crate::arena::{block, component, ArenaMut, ArenaRef, BlockKind, BlockMut, BlockRef};
use crate::libs::random_id::U128Id;
use nusa::v_node::v_element::VEvent;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub mod table_tool;
mod table_tool_state;
mod three;

use table_tool::TableTool;
use table_tool_state::TableToolState;
use three::Three;

pub struct UpdatedBlocks {
    pub update: HashSet<U128Id>,
    pub insert: HashSet<U128Id>,
}

pub struct Table {
    three: Rc<RefCell<Three>>,

    is_2d_mode: bool,

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
            three: Rc::new(RefCell::new(Three::new())),

            is_2d_mode: false,

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

    pub fn set_camera_mode(&mut self, is_2d_mode: bool) {
        self.is_2d_mode = is_2d_mode;
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
        self.three.borrow().canvas()
    }

    pub fn reserve_rendering(&mut self) {
        self.is_reserve_rendering = true;
    }

    pub fn render_reserved(&mut self, world: BlockRef<block::World>) {
        if self.is_reserve_rendering {
            self.is_reserve_rendering = false;
            let three = Rc::clone(&self.three);
            let is_2d_mode = self.is_2d_mode;

            let a = Closure::once(Box::new(move || {
                three
                    .borrow_mut()
                    .render(is_2d_mode, BlockRef::clone(&world));
            }));

            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(a.as_ref().unchecked_ref());

            a.forget();
        }
    }

    pub fn reset_size(&mut self) {
        self.is_reserve_rendering = true;
        self.three.borrow_mut().reset_size();
    }

    fn selecting_scene(world: BlockRef<block::World>) -> Option<BlockMut<block::Scene>> {
        world.map(|world| BlockMut::clone(world.selecting_scene()))
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

    /// is_bind_to_gridに基づいた座標
    fn grid_position(is_bind_to_grid: bool, p: &[f64; 3]) -> [f64; 3] {
        if is_bind_to_grid {
            [
                (p[0] * 2.0).round() / 2.0,
                (p[1] * 2.0).round() / 2.0,
                (p[2] * 2.0).round() / 2.0,
            ]
        } else {
            [p[0], p[1], p[2]]
        }
    }

    /// 画面上の座標をキャンバス内の座標に変換
    pub fn mouse_coord(&self, page_x: f64, page_y: f64) -> [f64; 2] {
        let rect = self.canvas().get_bounding_client_rect();
        let client_x = page_x - rect.left();
        let client_y = page_y - rect.top();
        [client_x, client_y]
    }

    pub fn terran_coord(craftboard: &block::Craftboard, p: &[f64; 3], n: &[f64; 3]) -> [i32; 3] {
        let cs = craftboard.size();
        let cp = craftboard.position();
        let vd = craftboard.voxel_density();
        let offset = [
            cp[0] + (cs[0].rem_euclid(2.0) - 1.0) * 0.5,
            cp[1] + (cs[1].rem_euclid(2.0) - 1.0) * 0.5,
            cp[2] + 0.5,
        ];
        let p = [
            (p[0] - offset[0]) * vd[0] + n[0] * 0.5,
            (p[1] - offset[1]) * vd[1] + n[1] * 0.5,
            (p[2] - offset[2]) * vd[2] + n[2] * 0.5,
        ];
        [
            p[0].round() as i32,
            p[1].round() as i32,
            p[2].round() as i32,
        ]
    }

    /// BlockMut<Table>を更新する
    pub fn update_table(
        scene: BlockRef<block::Scene>,
        mut table: BlockMut<block::Table>,
        mut f: impl FnMut(&mut block::Table),
    ) -> HashSet<U128Id> {
        let mut updated_blocks = HashSet::new();

        updated_blocks.insert(table.id());
        table.update(&mut f);

        scene.map(|scene| {
            if table.id() == scene.master_table().id() {
                for table in scene.tables() {
                    let mut table = BlockMut::clone(table);
                    updated_blocks.insert(table.id());
                    table.update(&mut f);
                }
            }
        });

        updated_blocks
    }

    /// Boxblockを作成して配置する
    pub fn create_boxblock(
        &mut self,
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::Boxblock,
    ) {
        let scene = unwrap!(Self::selecting_scene(world.as_ref()));
        let table = unwrap!(Self::selecting_table(world.as_ref()));
        let (p, n) = self
            .three
            .borrow_mut()
            .get_focused_position(mouse_coord, &self.ignored_id());
        let n = Self::n_cube(&n, &option.size);
        let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];

        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);

        let p = Self::grid_position(is_bind_to_grid, &p);
        let mut boxblock = block::Boxblock::new(is_bind_to_grid);

        crate::debug::log_1(format!("add boxblock to {:?}", &p));

        boxblock.set_size(option.size.clone());
        boxblock.set_position(p);
        boxblock.set_color(option.color);
        boxblock.set_texture(option.texture.as_ref().map(|block| BlockRef::clone(block)));
        boxblock.set_shape(option.shape.clone());

        let boxblock = arena.insert(boxblock);
        let boxblock_id = boxblock.id();

        let updated_blocks = Self::update_table(scene.as_ref(), table, |table| {
            table.push_boxblock(BlockMut::clone(&boxblock));
        });

        self.reserve_rendering();
        for updated_block in updated_blocks {
            self.updated_blocks.update.insert(updated_block);
        }
        self.updated_blocks.insert.insert(boxblock_id);
    }

    pub fn create_character(
        &mut self,
        mut arena: ArenaMut,
        mut world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::Character,
    ) {
        let table = unwrap!(Self::selecting_table(world.as_ref()));
        let (p, n) = self
            .three
            .borrow_mut()
            .get_focused_position(mouse_coord, &self.ignored_id());
        let p = [
            p[0] + n[0] / 128.0,
            p[1] + n[1] / 128.0,
            p[2] + n[2] / 128.0,
        ];

        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);

        let p = Self::grid_position(is_bind_to_grid, &p);
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
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::Craftboard,
    ) {
        let scene = unwrap!(Self::selecting_scene(world.as_ref()));
        let table = unwrap!(Self::selecting_table(world.as_ref()));
        let (p, _) = self
            .three
            .borrow_mut()
            .get_focused_position(mouse_coord, &self.ignored_id());

        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);
        let p = Self::grid_position(is_bind_to_grid, &p);
        let terran = arena.insert(block::Terran::new());
        let terran_id = terran.id();
        let mut craftboard = block::Craftboard::new(is_bind_to_grid, p, terran);

        craftboard.set_size(option.size.clone());

        let craftboard = arena.insert(craftboard);
        let craftboard_id = craftboard.id();
        let updated_blocks = Self::update_table(scene.as_ref(), table, |table| {
            table.push_craftboard(BlockMut::clone(&craftboard));
        });

        self.reserve_rendering();
        for updated_block in updated_blocks {
            self.updated_blocks.update.insert(updated_block);
        }
        self.updated_blocks.insert.insert(terran_id);
        self.updated_blocks.insert.insert(craftboard_id);
    }

    pub fn create_terran_block(
        &mut self,
        mut arena: ArenaMut,
        _world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        option: &table_tool::TerranBlock,
    ) {
        let (block_id, p, n) = self
            .three
            .borrow_mut()
            .get_focused_object_and_position(mouse_coord, &self.ignored_id());

        let craftboard = match arena.kind_of(&block_id) {
            BlockKind::Craftboard => arena.get_mut::<block::Craftboard>(&block_id),
            _ => {
                return;
            }
        };

        let terran = craftboard
            .as_ref()
            .and_then(|x| x.map(|craftboard| BlockMut::clone(craftboard.terran())));

        if let Some((craftboard, mut terran)) = join_some!(craftboard, terran) {
            craftboard.map(|craftboard| {
                let p = Self::terran_coord(&craftboard, &p, &n);

                terran.update(|terran| {
                    terran.set_texture(
                        option
                            .texture
                            .as_ref()
                            .map(|tex| BlockMut::clone(tex))
                            .unwrap_or_else(|| BlockMut::none()),
                    );
                    terran.insert_block(
                        p,
                        block::terran::TerranVoxel::new(option.allocater_state.tex_idx),
                    );
                });

                self.reserve_rendering();
                self.updated_blocks.update.insert(terran.id());
            });
        }
    }

    pub fn create_textboard(
        &mut self,
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        _option: &table_tool::Textboard,
    ) {
        let scene = unwrap!(Self::selecting_scene(world.as_ref()));
        let table = unwrap!(Self::selecting_table(world.as_ref()));
        let (p, _) = self
            .three
            .borrow_mut()
            .get_focused_position(mouse_coord, &self.ignored_id());
        let is_bind_to_grid = table
            .map(|table| table.default_is_bind_to_grid())
            .unwrap_or(true);
        let p = Self::grid_position(is_bind_to_grid, &p);
        let textboard = block::Textboard::new(is_bind_to_grid, p);
        let textboard = arena.insert(textboard);
        let textboard_id = textboard.id();
        let updated_blocks = Self::update_table(scene.as_ref(), table, |table| {
            table.push_textboard(BlockMut::clone(&textboard));
        });

        self.reserve_rendering();
        for updated_block in updated_blocks {
            self.updated_blocks.update.insert(updated_block);
        }
        self.updated_blocks.insert.insert(textboard_id);
    }

    pub fn rotate_camera(&mut self, movement: &[f64; 2]) {
        let h_rot = -movement[0] / 500.0;
        let v_rot = -movement[1] / 500.0;
        let rotation = self.three.borrow().camera().rotation().clone();

        let x_rot = rotation[0] + v_rot;
        let z_rot = rotation[2] + h_rot;

        self.three
            .borrow_mut()
            .camera_mut()
            .set_x_axis_rotation(x_rot);

        self.three
            .borrow_mut()
            .camera_mut()
            .set_z_axis_rotation(z_rot);

        self.reserve_rendering();
    }

    pub fn allocate_boxblock(
        &mut self,
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        mut component: BlockMut<component::BoxblockComponent>,
    ) {
        if let Some(mut boxblock) = component.create_clone() {
            let scene = unwrap!(Self::selecting_scene(world.as_ref()));
            let table = unwrap!(Self::selecting_table(world.as_ref()));
            let (p, n) = self
                .three
                .borrow_mut()
                .get_focused_position(mouse_coord, &self.ignored_id());

            let n = Self::n_cube(&n, boxblock.size());
            let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];
            boxblock.set_position(p);

            let boxblock = arena.insert(boxblock);
            let boxblock_id = boxblock.id();

            component.update(|component| {
                component.push(BlockMut::clone(&boxblock));
            });

            let updated_blocks = Self::update_table(scene.as_ref(), table, |table| {
                table.push_boxblock(BlockMut::clone(&boxblock));
            });

            self.reserve_rendering();
            self.updated_blocks.update.insert(component.id());
            for updated_block in updated_blocks {
                self.updated_blocks.update.insert(updated_block);
            }
            self.updated_blocks.insert.insert(boxblock_id);
        }
    }

    pub fn allocate_craftboard(
        &mut self,
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        mut component: BlockMut<component::CraftboardComponent>,
    ) {
        if let Some(mut craftboard) = component.create_clone() {
            let scene = unwrap!(Self::selecting_scene(world.as_ref()));
            let table = unwrap!(Self::selecting_table(world.as_ref()));
            let (p, ..) = self
                .three
                .borrow_mut()
                .get_focused_position(mouse_coord, &self.ignored_id());

            craftboard.set_position(p);

            let craftboard = arena.insert(craftboard);
            let craftboard_id = craftboard.id();

            component.update(|component| {
                component.push(BlockMut::clone(&craftboard));
            });

            let updated_blocks = Self::update_table(scene.as_ref(), table, |table| {
                table.push_craftboard(BlockMut::clone(&craftboard));
            });

            self.reserve_rendering();
            self.updated_blocks.update.insert(component.id());
            for updated_block in updated_blocks {
                self.updated_blocks.update.insert(updated_block);
            }
            self.updated_blocks.insert.insert(craftboard_id);
        }
    }

    pub fn allocate_textboard(
        &mut self,
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        mut component: BlockMut<component::TextboardComponent>,
    ) {
        if let Some(mut textboard) = component.create_clone() {
            let scene = unwrap!(Self::selecting_scene(world.as_ref()));
            let table = unwrap!(Self::selecting_table(world.as_ref()));
            let (p, ..) = self
                .three
                .borrow_mut()
                .get_focused_position(mouse_coord, &self.ignored_id());

            textboard.set_position(p);

            let textboard = arena.insert(textboard);
            let textboard_id = textboard.id();

            component.update(|component| {
                component.push(BlockMut::clone(&textboard));
            });

            let updated_blocks = Self::update_table(scene.as_ref(), table, |table| {
                table.push_textboard(BlockMut::clone(&textboard));
            });

            self.reserve_rendering();
            self.updated_blocks.update.insert(component.id());
            for updated_block in updated_blocks {
                self.updated_blocks.update.insert(updated_block);
            }
            self.updated_blocks.insert.insert(textboard_id);
        }
    }

    pub fn remove_terran_block(
        &mut self,
        mut arena: ArenaMut,
        _world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
        _option: &table_tool::TerranBlock,
    ) {
        let (block_id, p, n) = self
            .three
            .borrow_mut()
            .get_focused_object_and_position(mouse_coord, &self.ignored_id());

        let craftboard = match arena.kind_of(&block_id) {
            BlockKind::Craftboard => arena.get_mut::<block::Craftboard>(&block_id),
            _ => {
                return;
            }
        };

        let terran = craftboard
            .as_ref()
            .and_then(|x| x.map(|craftboard| BlockMut::clone(craftboard.terran())));

        if let Some((craftboard, mut terran)) = join_some!(craftboard, terran) {
            craftboard.map(|craftboard| {
                let p = Self::terran_coord(&craftboard, &p, &[-n[0], -n[1], -n[2]]);

                terran.update(|terran| {
                    terran.remove_block(&p);
                });

                self.reserve_rendering();
                self.updated_blocks.update.insert(terran.id());
            });
        }
    }

    pub fn move_camera_xy(&mut self, movement: &[f64; 2]) {
        let h_mov = -movement[0] / 50.0;
        let v_mov = movement[1] / 50.0;

        let p = self.three.borrow().camera().movement().clone();

        self.three
            .borrow_mut()
            .camera_mut()
            .set_movement([p[0] + h_mov, p[1] + v_mov, p[2]]);

        self.reserve_rendering();
    }

    pub fn move_camera_z(&mut self, movement: f64) {
        let p = self.three.borrow().camera().movement().clone();

        self.three
            .borrow_mut()
            .camera_mut()
            .set_movement([p[0], p[1], p[2] + movement / 16.0]);

        self.reserve_rendering();
    }

    pub fn drag_block(
        &mut self,
        mut arena: ArenaMut,
        _world: BlockMut<block::World>,
        mouse_coord: &[f64; 2],
    ) {
        let ignored_id = self.ignored_id();
        let (block_kind, block_id) =
            unwrap!(self.tool_state.selecter_mut().grabbed_object.as_ref());
        let (p, n) = self
            .three
            .borrow_mut()
            .get_focused_position(mouse_coord, &ignored_id);

        match block_kind {
            BlockKind::Boxblock => {
                if let Some(mut block) = arena.get_mut::<block::Boxblock>(block_id) {
                    let block_id = block.id();
                    block.update(|block| {
                        let n = Self::n_cube(&n, block.size());
                        let p = Self::grid_position(block.is_bind_to_grid(), &p);
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
                        let p = Self::grid_position(block.is_bind_to_grid(), &p);

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
                        let p = Self::grid_position(block.is_bind_to_grid(), &p);

                        block.set_position(p);

                        self.reserve_rendering();
                        self.updated_blocks.update.insert(block_id);
                    });
                }
            }
            BlockKind::Textboard => {
                if let Some(mut block) = arena.get_mut::<block::Textboard>(block_id) {
                    let block_id = block.id();
                    block.update(|block| {
                        let p = Self::grid_position(block.is_bind_to_grid(), &p);
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
        mut arena: ArenaMut,
        world: BlockMut<block::World>,
        e: VEvent<web_sys::MouseEvent>,
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
            TableTool::TerranBlock(tool) => match tool.kind {
                table_tool::TerranBlockKind::Allocater => {
                    self.create_terran_block(arena, world, &mouse_coord, tool);
                }
                table_tool::TerranBlockKind::Eraser => {
                    self.remove_terran_block(arena, world, &mouse_coord, tool);
                }
            },
            TableTool::Textboard(tool) => {
                self.create_textboard(arena, world, &mouse_coord, tool);
            }
            TableTool::ComponentAllocater(tool) => match arena.kind_of(&tool.component) {
                BlockKind::BoxblockComponent => {
                    arena
                        .get_mut::<component::BoxblockComponent>(&tool.component)
                        .map(|block| {
                            self.allocate_boxblock(arena, world, &mouse_coord, block);
                        });
                }
                BlockKind::CraftboardComponent => {
                    arena
                        .get_mut::<component::CraftboardComponent>(&tool.component)
                        .map(|block| {
                            self.allocate_craftboard(arena, world, &mouse_coord, block);
                        });
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn on_wheel(&mut self, e: VEvent<web_sys::WheelEvent>, tool: &TableTool) {
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
        _world: BlockMut<block::World>,
        e: VEvent<web_sys::MouseEvent>,
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
                        let (block_kind, block_id) =
                            self.focused_block(page_x, page_y, arena.as_ref());
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
                            BlockKind::Textboard => {
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

    pub fn on_mouseup(&mut self, e: VEvent<web_sys::MouseEvent>, tool: &TableTool) {
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
        e: VEvent<web_sys::MouseEvent>,
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

    pub fn focused_block(&self, page_x: f64, page_y: f64, arena: ArenaRef) -> (BlockKind, U128Id) {
        let [px_x, px_y] = self.mouse_coord(page_x, page_y);
        let block_id = self
            .three
            .borrow_mut()
            .get_focused_object(&[px_x, px_y], &self.ignored_id());
        (arena.kind_of(&block_id), block_id)
    }

    pub fn ignored_id(&self) -> U128Id {
        if let TableToolState::Selecter(state) = &self.tool_state {
            state
                .grabbed_object
                .as_ref()
                .map(|(_, block_id)| U128Id::clone(block_id))
                .unwrap_or(U128Id::none())
        } else {
            U128Id::none()
        }
    }
}
