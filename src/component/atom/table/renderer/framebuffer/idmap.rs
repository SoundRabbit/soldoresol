use super::libs::tex_table::TexTable;
use super::libs::webgl::WebGlRenderingContext;
use crate::libs::random_id::U128Id;

pub struct Idmap {
    depth_buffer: web_sys::WebGlRenderbuffer,
    screen_tex: (web_sys::WebGlTexture, U128Id),
    frame_buffer: web_sys::WebGlFramebuffer,
}

impl Idmap {
    pub fn new(
        gl: &WebGlRenderingContext,
        width: i32,
        height: i32,
        tex_table: &mut TexTable,
    ) -> Self {
        let depth_buffer = gl.create_renderbuffer().unwrap();
        super::resize_renderbuffer(&gl, &depth_buffer, width, height);

        let screen_tex = super::create_screen_texture(&gl, tex_table, width, height, None);
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
        gl.framebuffer_texture_2d(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&screen_tex.0),
            0,
        );

        Self {
            depth_buffer,
            screen_tex,
            frame_buffer,
        }
    }

    pub fn reset_size(
        &self,
        gl: &WebGlRenderingContext,
        width: i32,
        height: i32,
        tex_table: &mut TexTable,
    ) {
        super::resize_renderbuffer(&gl, &self.depth_buffer, width, height);
        super::resize_texturebuffer(
            &gl,
            &self.screen_tex.0,
            &self.screen_tex.1,
            tex_table,
            width,
            height,
        );
    }

    pub fn bind_self(&self, gl: &WebGlRenderingContext) {
        gl.bind_framebuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            Some(&self.frame_buffer),
        );
    }

    pub fn begin_to_render(&self, gl: &WebGlRenderingContext, tex_table: &TexTable) {
        if let Some((_, tex_flag)) = tex_table.try_use_custom(&self.screen_tex.1) {
            gl.active_texture(tex_flag);
            gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, None);
        }
    }

    pub fn screen_tex(&self) -> &(web_sys::WebGlTexture, U128Id) {
        &self.screen_tex
    }
}
