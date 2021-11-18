use super::*;
use crate::libs::random_id::U128Id;

impl Renderer {
    pub fn reset_canvas_size(canvas: &web_sys::HtmlCanvasElement, dpr: f64) -> [f64; 2] {
        let bb = canvas.get_bounding_client_rect();
        let w = bb.width() * dpr;
        let h = bb.height() * dpr;

        canvas.set_width(w as u32);
        canvas.set_height(h as u32);

        crate::debug::log_2(w, h);

        [w, h]
    }

    pub fn resize_renderbuffer(
        gl: &WebGlRenderingContext,
        buf: &web_sys::WebGlRenderbuffer,
        width: i32,
        height: i32,
    ) {
        gl.bind_renderbuffer(web_sys::WebGlRenderingContext::RENDERBUFFER, Some(&buf));
        gl.renderbuffer_storage(
            web_sys::WebGlRenderingContext::RENDERBUFFER,
            web_sys::WebGlRenderingContext::DEPTH_COMPONENT16,
            width,
            height,
        );
    }

    pub fn resize_texturebuffer(
        gl: &WebGlRenderingContext,
        buf: &web_sys::WebGlTexture,
        tex_id: &U128Id,
        tex_table: &mut tex_table::TexTable,
        width: i32,
        height: i32,
    ) {
        let (_, tex_flag) = tex_table.use_custom(tex_id);
        gl.active_texture(tex_flag);
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&buf));
        let _ = gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                width,
                height,
                0,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                None,
            );
    }

    pub fn create_screen_texture(
        gl: &WebGlRenderingContext,
        tex_table: &mut tex_table::TexTable,
        width: i32,
        height: i32,
        filter: Option<u32>,
    ) -> (web_sys::WebGlTexture, U128Id) {
        let tex_buf = gl.create_texture().unwrap();
        let tex_id = U128Id::new();
        let (_, tex_flag) = tex_table.use_custom(&tex_id);
        let filter = filter.unwrap_or(web_sys::WebGlRenderingContext::LINEAR);
        gl.active_texture(tex_flag);
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            filter as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
            filter as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        Self::resize_texturebuffer(&gl, &tex_buf, &tex_id, tex_table, width, height);
        (tex_buf, tex_id)
    }
}
