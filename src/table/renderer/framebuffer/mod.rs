use super::libs;
use crate::libs::random_id::U128Id;
use libs::tex_table::TexTable;
use libs::webgl::WebGlRenderingContext;

pub mod idmap;
pub mod screen;
pub mod shadowmap;
pub mod view;

pub use idmap::Idmap;
pub use screen::Screen;
pub use shadowmap::Shadowmap;
pub use view::View;

fn resize_depthbuffer(
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

fn resize_depth_stencilbuffer(
    gl: &WebGlRenderingContext,
    buf: &web_sys::WebGlRenderbuffer,
    width: i32,
    height: i32,
) {
    gl.bind_renderbuffer(web_sys::WebGlRenderingContext::RENDERBUFFER, Some(&buf));
    gl.renderbuffer_storage(
        web_sys::WebGlRenderingContext::RENDERBUFFER,
        web_sys::WebGlRenderingContext::DEPTH_STENCIL,
        width,
        height,
    );
}

fn resize_texturebuffer(
    gl: &WebGlRenderingContext,
    buf: &web_sys::WebGlTexture,
    tex_id: &U128Id,
    tex_table: &mut TexTable,
    width: i32,
    height: i32,
) {
    let (_, tex_flag) = tex_table.use_custom(tex_id);
    gl.active_texture(tex_flag);
    gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&buf));
    let _ = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
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

fn create_screen_texture(
    gl: &WebGlRenderingContext,
    tex_table: &mut TexTable,
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
    resize_texturebuffer(&gl, &tex_buf, &tex_id, tex_table, width, height);
    (tex_buf, tex_id)
}
