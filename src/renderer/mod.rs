mod mask_renderer;
mod model_matrix;
mod program;
mod view_renderer;
mod webgl;

use crate::model::{Camera, Resource, World};
use mask_renderer::MaskRenderer;
use model_matrix::ModelMatrix;
use ndarray::{arr1, Array1, Array2};
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
    pub fn new(gl: web_sys::WebGlRenderingContext) -> Self {
        let gl = Rc::new(WebGlRenderingContext(gl));
        let view_renderer = ViewRenderer::new(&gl);
        let mask_renderer = MaskRenderer::new();
        Self {
            gl,
            view_renderer,
            mask_renderer,
        }
    }

    pub fn table_object_id(&self, position: &[f64; 2]) -> u128 {
        self.mask_renderer.table_object_id(position)
    }

    pub fn table_position(
        vertex: &[f64; 3],
        movement: &[f64; 3],
        camera: &Camera,
        canvas_size: &[f64; 2],
        is_billboard: bool,
    ) -> Array1<f64> {
        let vp_matrix = camera
            .view_matrix()
            .dot(&camera.perspective_matrix(&canvas_size));
        let model_matrix: Array2<f64> = if is_billboard {
            ModelMatrix::new()
                .with_x_axis_rotation(camera.x_axis_rotation())
                .with_z_axis_rotation(camera.z_axis_rotation())
                .with_movement(&movement)
                .into()
        } else {
            ModelMatrix::new().with_movement(&movement).into()
        };
        let mvp_matrix = model_matrix.dot(&vp_matrix);
        let screen_position = mvp_matrix.dot(&arr1(&[vertex[0], vertex[1], vertex[2], 1.0]));
        screen_position
    }

    pub fn render(
        &mut self,
        world: &mut World,
        camera: &Camera,
        resource: &Resource,
        canvas_size: &[f64; 2],
    ) {
        if Rc::strong_count(&self.gl) < 3 {
            self.view_renderer
                .render(&self.gl, &canvas_size, &camera, world, resource);

            self.mask_renderer.render(&canvas_size, &camera, world);

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
