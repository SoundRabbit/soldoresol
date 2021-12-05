use super::*;
use id_table::IdTableBuilder;
use wasm_bindgen::{prelude::*, JsCast};

impl Renderer {
    pub fn new(canvas: Rc<web_sys::HtmlCanvasElement>) -> Self {
        let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio();
        let canvas_size = Self::reset_canvas_size(&canvas, device_pixel_ratio);

        let option: JsValue = object! {
            "preserveDrawingBuffer": true,
            "stenchil": true
        }
        .into();
        let gl = canvas
            .get_context_with_context_options("webgl", &option)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        gl.get_extension("EXT_frag_depth")
            .map_err(|err| crate::debug::log_1(&err))
            .unwrap()
            .unwrap();
        let mut gl = WebGlRenderingContext::new(gl);

        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        gl.cull_face(web_sys::WebGlRenderingContext::BACK);
        gl.enable(web_sys::WebGlRenderingContext::STENCIL_TEST);

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear_stencil(0);
        gl.stencil_func(web_sys::WebGlRenderingContext::ALWAYS, 0, 0);

        let mut tex_table = tex_table::TexTable::new(&gl);
        let id_table = id_table::IdTable::from(IdTableBuilder::new());

        let sw = canvas_size[0] as i32;
        let sh = canvas_size[1] as i32;

        let view_frame = framebuffer::View::new();
        let screen_frame = framebuffer::Screen::new(&gl, sw, sh, &mut tex_table);
        let idmap_frame = framebuffer::Idmap::new(&gl, sw, sh, &mut tex_table);
        let craftboard_idmap_frame = framebuffer::Idmap::new(&gl, sw, sh, &mut tex_table);
        let shadomap_frame = framebuffer::Shadowmap::new(&gl, &mut tex_table);

        let screen_mesh = mesh::Screen::new(&gl);
        let craftboard_grid_mesh = mesh::CraftboardGrid::new(&gl);
        let boxblock_mesh = mesh::Boxblock::new(&gl);
        let nameplate_mesh = mesh::Nameplate::new(&gl);
        let character_mesh = mesh::Character::new(&gl);
        let character_base_mesh = mesh::CharacterBase::new(&gl);

        Self {
            canvas,
            gl,

            canvas_size,
            device_pixel_ratio,

            tex_table,
            id_table,

            view_frame,
            screen_frame,
            idmap_frame,
            craftboard_idmap_frame,
            shadomap_frame,

            screen_mesh,
            craftboard_grid_mesh,
            boxblock_mesh,
            nameplate_mesh,
            character_mesh,
            character_base_mesh,
        }
    }
}
