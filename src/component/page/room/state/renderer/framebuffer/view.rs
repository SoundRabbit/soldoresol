use super::libs::tex_table::TexTable;
use super::libs::webgl::WebGlRenderingContext;
use crate::libs::random_id::U128Id;

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bind_self(&self, gl: &WebGlRenderingContext) {
        gl.bind_framebuffer(web_sys::WebGlRenderingContext::FRAMEBUFFER, None);
    }
}
