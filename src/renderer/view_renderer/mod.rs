mod character_collection_renderer;
mod table_renderer;

use super::webgl::WebGlRenderingContext;
use crate::model::{Camera, World};
use character_collection_renderer::CharacterCollectionRenderer;
use table_renderer::TableRenderer;

pub struct ViewRenderer {
    character_collection_renderer: CharacterCollectionRenderer,
    table_renderer: TableRenderer,
}

impl ViewRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        let character_collection_renderer = CharacterCollectionRenderer::new(gl);
        let table_renderer = TableRenderer::new(gl);

        Self {
            character_collection_renderer,
            table_renderer,
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        canvas_size: &[f64; 2],
        camera: &Camera,
        world: &mut World,
    ) {
        let vp_matrix = camera
            .view_matrix()
            .dot(&camera.perspective_matrix(&canvas_size));
        gl.viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        gl.disable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        self.table_renderer
            .render(gl, camera, &vp_matrix, world.table_mut());
        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        self.character_collection_renderer
            .render(gl, camera, &vp_matrix, world.characters_mut());

        gl.flush();
    }
}
