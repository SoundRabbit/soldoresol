mod camera;
mod mask_renderer;
mod model_matrix;
mod program;
mod view_renderer;
mod webgl;

use crate::{
    block::{self, BlockId},
    Resource,
};
pub use camera::Camera;
use mask_renderer::MaskRenderer;
pub use mask_renderer::TableBlock;
use model_matrix::ModelMatrix;
use ndarray::{arr1, Array2};
use std::rc::Rc;
use view_renderer::ViewRenderer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use webgl::WebGlRenderingContext;

pub struct Renderer {
    gl: Rc<WebGlRenderingContext>,
    view_renderer: ViewRenderer,
    mask_renderer: MaskRenderer,
}

impl Renderer {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        let option = object! {stencil: true};
        let option: js_sys::Object = option.into();
        let gl = canvas
            .get_context_with_context_options("webgl", &option.into())
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        gl.get_extension("EXT_frag_depth")
            .map_err(|err| crate::debug::log_1(&err))
            .unwrap()
            .unwrap();
        let gl = Rc::new(WebGlRenderingContext(gl));
        let view_renderer = ViewRenderer::new(&gl);
        let mask_renderer = MaskRenderer::new();
        Self {
            gl,
            view_renderer,
            mask_renderer,
        }
    }

    pub fn table_object_id(
        &self,
        canvas_size: &[f32; 2],
        position: &[f32; 2],
    ) -> Option<&TableBlock> {
        self.mask_renderer.table_object_id(canvas_size, position)
    }

    pub fn table_position(
        vertex: &[f32; 3],
        movement: &[f32; 3],
        camera: &Camera,
        canvas_size: &[f32; 2],
        is_billboard: bool,
    ) -> [f32; 3] {
        let vp_matrix = camera
            .perspective_matrix(&canvas_size)
            .dot(&camera.view_matrix());
        let model_matrix: Array2<f32> = if is_billboard {
            ModelMatrix::new()
                .with_x_axis_rotation(camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                .with_z_axis_rotation(camera.z_axis_rotation())
                .with_movement(&movement)
                .into()
        } else {
            ModelMatrix::new().with_movement(&movement).into()
        };
        let mvp_matrix = vp_matrix.dot(&model_matrix);
        let screen_position = mvp_matrix.dot(&arr1(&[vertex[0], vertex[1], vertex[2], 1.0]));

        [
            screen_position[0] / screen_position[3],
            screen_position[1] / screen_position[3],
            screen_position[2],
        ]
    }

    pub fn render(
        &mut self,
        block_field: &block::Field,
        world: &BlockId,
        camera: &Camera,
        resource: &Resource,
        canvas_size: &[f32; 2],
        floating_object: &Option<&BlockId>,
        client_id: &String,
    ) {
        if Rc::strong_count(&self.gl) < 3 {
            if let Some(world) = block_field.get::<block::World>(world) {
                self.view_renderer.render(
                    &self.gl,
                    &canvas_size,
                    &camera,
                    block_field,
                    world,
                    resource,
                    client_id,
                );

                self.mask_renderer.render(
                    &canvas_size,
                    &camera,
                    block_field,
                    world,
                    floating_object,
                    client_id,
                );

                let view_gl = Rc::clone(&self.gl);
                let mask_gl = self.mask_renderer.gl();

                let a = Closure::once(Box::new(move || {
                    view_gl.flush();
                    mask_gl.flush();
                }) as Box<dyn FnOnce()>);

                let _ = web_sys::window()
                    .unwrap()
                    .request_animation_frame(a.as_ref().unchecked_ref());

                a.forget();
            }
        }
    }
}
