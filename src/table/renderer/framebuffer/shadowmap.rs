use super::libs::tex_table::TexTable;
use super::libs::webgl::WebGlRenderingContext;
use crate::libs::random_id::U128Id;

pub struct Shadowmap {
    depth_buffer: web_sys::WebGlRenderbuffer,
    screen_tex: [(web_sys::WebGlTexture, U128Id); 6],
    frame_buffer: web_sys::WebGlFramebuffer,
}

impl Shadowmap {
    pub fn size() -> [f32; 2] {
        [512.0, 512.0]
    }

    pub fn new(gl: &WebGlRenderingContext, tex_table: &mut TexTable) -> Self {
        let depth_buffer = gl.create_renderbuffer().unwrap();
        let [sw, sh] = Self::size();
        let sw = sw as i32;
        let sh = sh as i32;
        super::resize_depthbuffer(&gl, &depth_buffer, sw, sh);

        let filter = web_sys::WebGlRenderingContext::NEAREST;
        let screen_tex = [
            super::create_screen_texture(&gl, tex_table, sw, sh, Some(filter)),
            super::create_screen_texture(&gl, tex_table, sw, sh, Some(filter)),
            super::create_screen_texture(&gl, tex_table, sw, sh, Some(filter)),
            super::create_screen_texture(&gl, tex_table, sw, sh, Some(filter)),
            super::create_screen_texture(&gl, tex_table, sw, sh, Some(filter)),
            super::create_screen_texture(&gl, tex_table, sw, sh, Some(filter)),
        ];

        let frame_buffer = gl.create_framebuffer().unwrap();
        gl.bind_framebuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            Some(&frame_buffer),
        );

        gl.framebuffer_renderbuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::DEPTH_ATTACHMENT,
            web_sys::WebGlRenderingContext::RENDERBUFFER,
            Some(&depth_buffer),
        );

        Self {
            depth_buffer,
            screen_tex,
            frame_buffer,
        }
    }

    pub fn bind_self(&self, gl: &WebGlRenderingContext) {
        gl.bind_framebuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            Some(&self.frame_buffer),
        );
    }

    pub fn begin_to_render(&self, gl: &WebGlRenderingContext, idx: usize) {
        gl.framebuffer_texture_2d(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.screen_tex[idx].0),
            0,
        );
    }

    pub fn screen_tex(&self) -> &[(web_sys::WebGlTexture, U128Id); 6] {
        &self.screen_tex
    }
}
