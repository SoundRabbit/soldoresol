use super::libs::tex_table::TexTable;
use super::libs::webgl::WebGlRenderingContext;
use crate::libs::random_id::U128Id;

pub struct Screen {
    depth_buffer: web_sys::WebGlRenderbuffer,
    stencil_biffer: web_sys::WebGlRenderbuffer,
    backscreen_tex: (web_sys::WebGlTexture, U128Id),
    frontscreen_tex: (web_sys::WebGlTexture, U128Id),
    frame_buffer: web_sys::WebGlFramebuffer,
}

impl Screen {
    pub fn new(
        gl: &WebGlRenderingContext,
        width: i32,
        height: i32,
        tex_table: &mut TexTable,
    ) -> Self {
        let backscreen_tex = super::create_screen_texture(&gl, tex_table, width, height, None);
        let frontscreen_tex = super::create_screen_texture(&gl, tex_table, width, height, None);

        let frame_buffer = gl.create_framebuffer().unwrap();
        gl.bind_framebuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            Some(&frame_buffer),
        );

        let depth_buffer = gl.create_renderbuffer().unwrap();
        super::resize_depthbuffer(&gl, &depth_buffer, width, height);
        gl.framebuffer_renderbuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::DEPTH_ATTACHMENT,
            web_sys::WebGlRenderingContext::RENDERBUFFER,
            Some(&depth_buffer),
        );

        let stencil_biffer = gl.create_renderbuffer().unwrap();
        super::resize_stencilbuffer(&gl, &depth_buffer, width, height);
        gl.framebuffer_renderbuffer(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::STENCIL_ATTACHMENT,
            web_sys::WebGlRenderingContext::RENDERBUFFER,
            Some(&stencil_biffer),
        );

        Self {
            depth_buffer,
            stencil_biffer,
            frontscreen_tex,
            backscreen_tex,
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
        super::resize_depthbuffer(&gl, &self.depth_buffer, width, height);
        super::resize_texturebuffer(
            &gl,
            &self.backscreen_tex.0,
            &self.backscreen_tex.1,
            tex_table,
            width,
            height,
        );
        super::resize_texturebuffer(
            &gl,
            &self.frontscreen_tex.0,
            &self.frontscreen_tex.1,
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

    pub fn begin_to_render_backscreen(&self, gl: &WebGlRenderingContext, tex_table: &TexTable) {
        if let Some((_, tex_flag)) = tex_table.try_use_custom(&self.backscreen_tex.1) {
            gl.active_texture(tex_flag);
            gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, None);
        }
        gl.framebuffer_texture_2d(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.backscreen_tex.0),
            0,
        );
    }

    pub fn begin_to_render_frontscreen(&self, gl: &WebGlRenderingContext, tex_table: &TexTable) {
        if let Some((_, tex_flag)) = tex_table.try_use_custom(&self.frontscreen_tex.1) {
            gl.active_texture(tex_flag);
            gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, None);
        }
        gl.framebuffer_texture_2d(
            web_sys::WebGlRenderingContext::FRAMEBUFFER,
            web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.frontscreen_tex.0),
            0,
        );
    }

    pub fn backscreen_tex(&self) -> &(web_sys::WebGlTexture, U128Id) {
        &self.backscreen_tex
    }

    pub fn frontscreen_tex(&self) -> &(web_sys::WebGlTexture, U128Id) {
        &self.frontscreen_tex
    }
}
