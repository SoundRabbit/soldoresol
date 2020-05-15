mod character_collection_renderer;

use super::webgl::WebGlRenderingContext;
use crate::model::{Camera, World};
use character_collection_renderer::CharacterCollectionRenderer;

pub struct ViewRenderer {
    character_collection_renderer: CharacterCollectionRenderer,
}

impl ViewRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let character_collection_renderer = CharacterCollectionRenderer::new(gl);

        Self {
            character_collection_renderer,
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
        self.character_collection_renderer
            .render(gl, camera, &vp_matrix, world.characters_mut());

        gl.flush();
    }
}
