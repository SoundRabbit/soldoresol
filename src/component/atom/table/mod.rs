use crate::arena::{block, ArenaMut, BlockKind, BlockMut};
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Cmd;
use kagura::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod renderer;
pub mod table_tool;
mod table_tool_state;

use renderer::{CameraMatrix, ObjectId, Renderer};
use table_tool::TableTool;
use table_tool_state::TableToolState;

pub struct Props {
    pub is_debug_mode: bool,
    pub arena: ArenaMut,
    pub world: BlockMut<block::World>,
    pub is_2d_mode: bool,
}

pub enum Msg {
    NoOp,
    Render,
    Resize,
}

pub enum On {
    UpdateBlocks {
        insert: HashSet<U128Id>,
        update: HashSet<U128Id>,
    },
}

pub struct Table {
    cmds: Vec<Cmd<Self>>,
    canvas: Option<Rc<web_sys::HtmlCanvasElement>>,
    renderer: Option<Renderer>,
    camera_matrix: CameraMatrix,

    arena: ArenaMut,
    world: BlockMut<block::World>,

    is_2d_mode: bool,
    is_debug_mode: bool,

    camera_state: CameraState,
    tool_state: TableToolState,
    last_cursor_position: [f64; 2],
}

struct CameraState {
    is_rotating: bool,
    is_moving: bool,
}

impl Component for Table {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Table {
    pub fn new(arena: ArenaMut, world: BlockMut<block::World>) -> PrepackedComponent<Self> {
        PrepackedComponent::new(Self {
            cmds: vec![],
            canvas: None,
            renderer: None,
            camera_matrix: CameraMatrix::new(),
            arena,
            world,
            is_2d_mode: false,
            is_debug_mode: false,

            camera_state: CameraState {
                is_rotating: false,
                is_moving: false,
            },
            tool_state: TableToolState::None,
            last_cursor_position: [0.0, 0.0],
        })
    }
}

impl Update for Table {
    fn on_assemble(&mut self, props: &Props) -> Cmd<Self> {
        self.cmds.push(Cmd::batch(move |mut handle| {
            let a = Closure::wrap(Box::new(move || handle(Msg::Resize)) as Box<dyn FnMut()>);
            let _ = web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", a.as_ref().unchecked_ref());
            a.forget();
        }));

        self.on_load(props)
    }

    fn on_load(&mut self, props: &Props) -> Cmd<Self> {
        self.arena = ArenaMut::clone(&props.arena);
        self.world = BlockMut::clone(&props.world);

        if self.is_2d_mode != props.is_2d_mode || self.is_debug_mode != props.is_debug_mode {
            self.is_2d_mode = props.is_2d_mode;
            self.is_debug_mode = props.is_debug_mode;
            self.cmds.push(Self::render());
        }

        Cmd::list(self.cmds.drain(..).collect())
    }

    fn ref_node(&mut self, _props: &Props, ref_name: String, node: web_sys::Node) -> Cmd<Self> {
        if ref_name == "canvas" {
            if let Some(canvas) = self.canvas.as_ref() {
                let canvas_ref: &JsValue = &canvas;
                let node_ref: &JsValue = &node;
                if *canvas_ref != *node_ref {
                    return self.set_renderer(node);
                }
            } else {
                return self.set_renderer(node);
            }
        }
        Cmd::none()
    }

    fn update(&mut self, props: &Props, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::NoOp => Cmd::none(),
            Msg::Render => {
                if let Some(renderer) = self.renderer.as_mut() {
                    self.camera_matrix.set_is_2d_mode(self.is_2d_mode);

                    let grabbed_object_id =
                        if let TableToolState::Selecter(state) = &self.tool_state {
                            state
                                .grabbed_object
                                .as_ref()
                                .map(|(_, block_id)| U128Id::clone(block_id))
                                .unwrap_or(U128Id::none())
                        } else {
                            U128Id::none()
                        };

                    renderer.render(
                        self.is_debug_mode,
                        props.world.as_ref(),
                        &self.camera_matrix,
                        &grabbed_object_id,
                    );
                }
                Cmd::none()
            }
            Msg::Resize => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.reset_size();
                    Self::render()
                } else {
                    Cmd::none()
                }
            }
        }
    }
}

impl Table {
    fn render() -> Cmd<Self> {
        Cmd::task(|resolve| {
            let mut resolve = Some(resolve);
            let a = Closure::wrap(Box::new(move || {
                if let Some(resolve) = resolve.take() {
                    resolve(Msg::Render);
                }
            }) as Box<dyn FnMut()>);
            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(a.as_ref().unchecked_ref());
            a.forget();
        })
    }

    fn set_renderer(&mut self, node: web_sys::Node) -> Cmd<Self> {
        if let Ok(canvas) = node.dyn_into::<web_sys::HtmlCanvasElement>() {
            let canvas = Rc::new(canvas);
            self.canvas = Some(Rc::clone(&canvas));
            let renderer = Renderer::new(Rc::clone(&canvas));
            self.renderer = Some(renderer);

            Cmd::list(vec![Self::render()])
        } else {
            Cmd::none()
        }
    }

    fn selecting_table(&self) -> Option<BlockMut<block::Table>> {
        self.world
            .map(|world| {
                world
                    .selecting_scene()
                    .map(|secene| BlockMut::clone(secene.selecting_table()))
            })
            .unwrap_or(None)
    }

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

    pub fn need_rendering(&mut self) {
        self.cmds.push(Self::render());
    }

    pub fn mouse_coord(&self, page_x: f64, page_y: f64) -> Option<[f64; 2]> {
        let rect = unwrap!(self.canvas.as_ref().map(|x| x.get_bounding_client_rect()); None);
        let client_x = page_x - rect.left();
        let client_y = page_y - rect.top();
        Some([client_x, client_y])
    }

    pub fn create_boxblock(&mut self, mouse_coord: &[f64; 2], option: &table_tool::Boxblock) {
        let mut table = unwrap!(self.selecting_table());
        let renderer = unwrap!(self.renderer.as_ref());
        let (p, n) =
            renderer.get_focused_position(&self.camera_matrix, mouse_coord[0], mouse_coord[1]);
        let n = Self::n_cube(&n, &option.size);
        let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];

        let mut boxblock = block::Boxblock::new();

        crate::debug::log_1(format!("add boxblock to {:?}", &p));

        boxblock.set_size(option.size.clone());
        boxblock.set_position(p);
        boxblock.set_color(option.color);
        boxblock.set_texture(option.texture.as_ref().map(|block| BlockMut::clone(block)));
        boxblock.set_shape(option.shape.clone());

        let boxblock = self.arena.insert(boxblock);
        let boxblock_id = boxblock.id();
        table.update(|table| {
            table.push_boxblock(boxblock);
        });
        self.cmds.push(Self::render());
        self.cmds.push(Cmd::sub(On::UpdateBlocks {
            update: set! { table.id() },
            insert: set! { boxblock_id },
        }));
    }

    pub fn create_character(&mut self, mouse_coord: &[f64; 2], option: &table_tool::Character) {
        let renderer = unwrap!(self.renderer.as_ref());
        let (p, _) =
            renderer.get_focused_position(&self.camera_matrix, mouse_coord[0], mouse_coord[1]);
        let n = self
            .camera_matrix
            .position_vec_n(&[p[0] as f32, p[1] as f32, p[2] as f32]);
        let p = [
            p[0] + n[0] as f64 / 128.0,
            p[1] + n[1] as f64 / 128.0,
            p[2] + n[2] as f64 / 128.0,
        ];

        let mut character = block::Character::new();

        character.set_size(option.size);
        character.set_position(p);
        character.set_tex_size(option.tex_size);
        character.set_color(option.color);
        character.set_texture(option.texture.as_ref().map(|block| BlockMut::clone(block)));

        let character = self.arena.insert(character);
        let character_id = character.id();
        self.world.update(|world| {
            world.push_character(character);
        });
        self.cmds.push(Self::render());
        self.cmds.push(Cmd::sub(On::UpdateBlocks {
            update: set! { self.world.id() },
            insert: set! { character_id },
        }));
    }

    pub fn create_craftboard(&mut self, mouse_coord: &[f64; 2], option: &table_tool::Craftboard) {
        let mut table = unwrap!(self.selecting_table());
        let renderer = unwrap!(self.renderer.as_ref());
        let (p, _) =
            renderer.get_focused_position(&self.camera_matrix, mouse_coord[0], mouse_coord[1]);

        let mut craftboard = block::Craftboard::new(p);

        craftboard.set_size(option.size.clone());

        let craftboard = self.arena.insert(craftboard);
        let craftboard_id = craftboard.id();
        table.update(|table| {
            table.push_craftboard(craftboard);
        });
        self.cmds.push(Self::render());
        self.cmds.push(Cmd::sub(On::UpdateBlocks {
            update: set! { table.id() },
            insert: set! { craftboard_id },
        }));
    }

    pub fn rotate_camera(&mut self, movement: &[f64; 2]) {
        let h_rot = -movement[0] / 500.0;
        let v_rot = -movement[1] / 500.0;

        self.camera_matrix
            .set_z_axis_rotation(self.camera_matrix.z_axis_rotation() + h_rot as f32);
        self.camera_matrix
            .set_x_axis_rotation(self.camera_matrix.x_axis_rotation() + v_rot as f32, false);

        self.cmds.push(Self::render());
    }

    pub fn move_camera(&mut self, movement: &[f64; 2]) {
        let h_mov = -movement[0] / 50.0;
        let v_mov = movement[1] / 50.0;
        let p = self.camera_matrix.movement();
        let p = [p[0] + h_mov as f32, p[1] + v_mov as f32, p[2]];

        self.camera_matrix.set_movement(p);

        self.cmds.push(Self::render());
    }

    pub fn zoom_camera(&mut self, movement: f64) {
        let p = self.camera_matrix.movement();
        let p = [p[0], p[1], p[2] + movement as f32 / 16.0];

        self.camera_matrix.set_movement(p);

        self.cmds.push(Self::render());
    }

    pub fn drag_block(&mut self, mouse_coord: &[f64; 2]) {
        let (block_kind, block_id) =
            unwrap!(self.tool_state.selecter_mut().grabbed_object.as_ref());
        let renderer = unwrap!(self.renderer.as_ref());
        let (p, n) =
            renderer.get_focused_position(&self.camera_matrix, mouse_coord[0], mouse_coord[1]);

        match block_kind {
            BlockKind::Boxblock => {
                if let Some(mut block) = self.arena.get_mut::<block::Boxblock>(block_id) {
                    let block_id = block.id();
                    block.update(|block| {
                        let n = Self::n_cube(&n, block.size());
                        let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];
                        block.set_position(p);

                        self.cmds.push(Self::render());
                        self.cmds.push(Cmd::sub(On::UpdateBlocks {
                            insert: set! {},
                            update: set! { block_id },
                        }));
                    });
                }
            }
            _ => {}
        }
    }

    pub fn on_click(&mut self, e: web_sys::MouseEvent, tool: &TableTool) {
        let mouse_coord = unwrap!(self.mouse_coord(e.page_x() as f64, e.page_y() as f64));

        match &tool {
            TableTool::Boxblock(tool) => {
                self.create_boxblock(&mouse_coord, tool);
            }
            TableTool::Character(tool) => {
                self.create_character(&mouse_coord, tool);
            }
            TableTool::Craftboard(tool) => {
                self.create_craftboard(&mouse_coord, tool);
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

        self.zoom_camera(delta_y);
    }

    pub fn on_mousedown(&mut self, e: web_sys::MouseEvent, tool: &TableTool) {
        let page_x = e.page_x() as f64;
        let page_y = e.page_y() as f64;
        let mouse_coord = unwrap!(self.mouse_coord(page_x, page_y));
        let button = e.button();

        if button == 0 {
            if e.alt_key() {
                self.camera_state.is_rotating = true;
            } else {
                match tool {
                    TableTool::Selecter(..) => {
                        let (block_kind, block_id) = self.focused_block(page_x, page_y);

                        if block_kind != BlockKind::None {
                            self.tool_state.selecter_mut().grabbed_object =
                                Some((block_kind, block_id));
                        } else {
                            self.camera_state.is_rotating = true;
                        }
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
        let mouse_coord = unwrap!(self.mouse_coord(e.page_x() as f64, e.page_y() as f64));
        let button = e.button();

        if button == 0 {
            self.camera_state.is_rotating = false;

            match tool {
                TableTool::Selecter(..) => {
                    self.tool_state.selecter_mut().grabbed_object = None;
                    self.cmds.push(Self::render());
                }
                _ => {}
            }
        } else if button == 1 {
            self.camera_state.is_moving = false;
        }

        self.last_cursor_position = mouse_coord;
    }

    pub fn on_mousemove(&mut self, e: web_sys::MouseEvent, tool: &TableTool) {
        let mouse_coord = unwrap!(self.mouse_coord(e.page_x() as f64, e.page_y() as f64));

        if self.camera_state.is_rotating {
            let x_mov = mouse_coord[0] - self.last_cursor_position[0];
            let y_mov = mouse_coord[1] - self.last_cursor_position[1];
            self.rotate_camera(&[x_mov, y_mov]);
        }

        if self.camera_state.is_moving {
            let x_mov = mouse_coord[0] - self.last_cursor_position[0];
            let y_mov = mouse_coord[1] - self.last_cursor_position[1];
            self.move_camera(&[x_mov, y_mov]);
        }

        match tool {
            TableTool::Selecter(..) => {
                self.drag_block(&mouse_coord);
            }
            _ => {}
        }

        self.last_cursor_position = mouse_coord;
    }

    pub fn focused_block(&self, page_x: f64, page_y: f64) -> (BlockKind, U128Id) {
        let [px_x, px_y] =
            unwrap!(self.mouse_coord(page_x, page_y); (BlockKind::None, U128Id::none()));
        let renderer = unwrap!(self.renderer.as_ref(); (BlockKind::None, U128Id::none()));

        match renderer.get_object_id(px_x, px_y) {
            ObjectId::Boxblock(b_id, ..) => (BlockKind::Boxblock, b_id),
            ObjectId::Character(b_id, ..) => (BlockKind::Character, b_id),
            ObjectId::Craftboard(b_id, ..) => (BlockKind::Craftboard, b_id),
            _ => (BlockKind::None, U128Id::none()),
        }
    }
}

impl Render for Table {
    fn render(&self, _props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Self::styled(Html::canvas(
            Attributes::new()
                .ref_name("canvas")
                .class(Self::class("base")),
            Events::new(),
            vec![],
        ))
    }
}

impl Styled for Table {
    fn style() -> Style {
        style! {
            ".base" {
                "width": "100%";
                "height": "100%";
            }
        }
    }
}
