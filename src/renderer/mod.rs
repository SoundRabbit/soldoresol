mod mask_renderer;
mod model_matrix;
mod program;
mod view_renderer;
mod webgl;

use crate::model::Camera;
use crate::model::World;
use mask_renderer::MaskRenderer;
use model_matrix::ModelMatrix;
use view_renderer::ViewRenderer;
use wasm_bindgen::JsCast;
use webgl::WebGlRenderingContext;

pub struct Renderer {
    gl: WebGlRenderingContext,
    view_renderer: ViewRenderer,
    mask_renderer: MaskRenderer,
}

impl Renderer {
    pub fn new(gl: web_sys::WebGlRenderingContext) -> Self {
        let gl = WebGlRenderingContext(gl);
        let view_renderer = ViewRenderer::new(&gl);
        let mask_renderer = MaskRenderer::new();
        Self {
            gl,
            view_renderer,
            mask_renderer,
        }
    }

    pub fn table_object_id(&self, position: &[f64; 2]) -> u32 {
        self.mask_renderer.table_object_id(position)
    }

    pub fn render(&mut self, world: &mut World, camera: &Camera) {
        let canvas = self
            .gl
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let canvas_size = [canvas.width() as f64, canvas.height() as f64];

        self.view_renderer
            .render(&self.gl, &canvas_size, &camera, world);

        self.mask_renderer.render(&canvas_size, &camera, world);
    }
}
