use crate::arena::{block, BlockMut};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::component::Cmd;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod renderer;

use renderer::{CameraMatrix, ObjectId, Renderer};

pub struct Props {
    pub world: BlockMut<block::World>,
}

pub enum Msg {
    RenderAwait,
    Render,
}

pub enum On {}

pub struct Table {
    canvas: Option<Rc<web_sys::HtmlCanvasElement>>,
    renderer: Option<Renderer>,
    camera_matrix: CameraMatrix,
    grabbed_object: ObjectId,
}

impl Component for Table {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Table {
    pub fn new() -> PrepackedComponent<Self> {
        PrepackedComponent::new(Self {
            canvas: None,
            renderer: None,
            camera_matrix: CameraMatrix::new(),
            grabbed_object: ObjectId::None,
        })
    }
}

impl Update for Table {
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
            Msg::RenderAwait => Cmd::task(|resolve| {
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
            }),
            Msg::Render => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render(
                        props.world.as_ref(),
                        &self.camera_matrix,
                        &self.grabbed_object,
                    );
                }
                Cmd::none()
            }
        }
    }
}

impl Table {
    fn set_renderer(&mut self, node: web_sys::Node) -> Cmd<Self> {
        if let Ok(canvas) = node.dyn_into::<web_sys::HtmlCanvasElement>() {
            let canvas = Rc::new(canvas);
            self.canvas = Some(Rc::clone(&canvas));
            let renderer = Renderer::new(canvas);
            self.renderer = Some(renderer);
            Cmd::chain(Msg::RenderAwait)
        } else {
            Cmd::none()
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
