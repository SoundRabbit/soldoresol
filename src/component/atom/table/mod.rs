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

use renderer::{CameraMatrix, ObjectId, Renderer};
use table_tool::TableTool;

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
    grabbed_object: ObjectId,

    arena: ArenaMut,
    world: BlockMut<block::World>,

    is_2d_mode: bool,
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
            grabbed_object: ObjectId::None,
            arena,
            world,
            is_2d_mode: false,
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

        if self.is_2d_mode != props.is_2d_mode {
            self.is_2d_mode = props.is_2d_mode;
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
                    renderer.render(
                        props.is_debug_mode,
                        props.world.as_ref(),
                        &self.camera_matrix,
                        &self.grabbed_object,
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

    pub fn table_coord(&self, e: &web_sys::MouseEvent) -> [f64; 2] {
        let page_x = e.page_x() as f64;
        let page_y = e.page_y() as f64;
        let rect = unwrap_or!(self.canvas.as_ref().map(|x| x.get_bounding_client_rect()); [page_x, page_y]);
        let client_x = page_x - rect.left();
        let client_y = page_y - rect.top();
        [client_x, client_y]
    }

    pub fn create_boxblock(
        &self,
        px_x: f64,
        px_y: f64,
        option: &table_tool::Boxblock,
    ) -> Option<block::Boxblock> {
        let renderer = unwrap!(self.renderer.as_ref());
        let (p, n) = renderer.get_focused_position(&self.camera_matrix, px_x, px_y);
        let n = Self::n_cube(&n, &option.size);
        let p = [p[0] + n[0], p[1] + n[1], p[2] + n[2]];

        let mut boxblock = block::Boxblock::new();

        crate::debug::log_1(format!("add boxblock to {:?}", &p));

        boxblock.set_size(option.size.clone());
        boxblock.set_position(p);
        boxblock.set_color(option.color);
        boxblock.set_texture(option.texture.as_ref().map(|block| BlockMut::clone(block)));
        boxblock.set_shape(option.shape.clone());

        Some(boxblock)
    }

    pub fn on_click(&mut self, e: web_sys::MouseEvent, tool: &TableTool) {
        let page_x = e.page_x() as f64;
        let page_y = e.page_y() as f64;
        let rect = unwrap_or!(self.canvas.as_ref().map(|x| x.get_bounding_client_rect()); ());
        let client_x = page_x - rect.left();
        let client_y = page_y - rect.top();

        match &tool {
            TableTool::Boxblock(tool) => {
                if let Some((boxblock, mut table)) = join_some!(
                    self.create_boxblock(client_x, client_y, &tool),
                    self.selecting_table()
                ) {
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
            }
            _ => {}
        }
    }

    pub fn focused_block(&self, px_x: f64, px_y: f64) -> (BlockKind, U128Id) {
        let renderer = unwrap_or!(self.renderer.as_ref(); (BlockKind::None, U128Id::none()));

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
        Self::styled(
            Html::canvas(
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![],
            )
            .ref_name("canvas"),
        )
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
