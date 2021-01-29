use crate::arena::block::{self, BlockId};
use crate::arena::resource::{self};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod matrix;
mod tex_table;
mod view;
mod webgl;

use webgl::WebGlRenderingContext;

pub use matrix::camera::CameraMatrix;

pub struct Renderer {
    view_canvas: Rc<web_sys::HtmlCanvasElement>,
    view_gl: WebGlRenderingContext,
    offscreen_canvas: Rc<web_sys::HtmlCanvasElement>,
    offscreen_gl: WebGlRenderingContext,

    canvas_size: [f32; 2],
    device_pixel_ratio: f32,

    tex_table: tex_table::TexTable,

    render_view_tablegrid: view::tablegrid::Tablegrid,
    render_view_tabletexture: view::tabletexture::Tabletexture,
    render_view_character: view::character::Character,
}

impl Renderer {
    fn reset_canvas_size(canvas: &web_sys::HtmlCanvasElement, dpr: f32) -> [f32; 2] {
        let bb = canvas.get_bounding_client_rect();
        let w = bb.width() as f32 * dpr;
        let h = bb.height() as f32 * dpr;

        canvas.set_width(w as u32);
        canvas.set_height(h as u32);

        crate::debug::log_2(w, h);

        [w, h]
    }

    pub fn new(view_canvas: Rc<web_sys::HtmlCanvasElement>) -> Self {
        let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio() as f32;
        let canvas_size = Self::reset_canvas_size(&view_canvas, device_pixel_ratio);

        let option: JsValue = object! {stenchil: true}.into();
        let view_gl = view_canvas
            .get_context_with_context_options("webgl", &option)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        view_gl
            .get_extension("EXT_frag_depth")
            .map_err(|err| crate::debug::log_1(&err))
            .unwrap()
            .unwrap();
        let view_gl = WebGlRenderingContext::new(view_gl);

        view_gl.enable(web_sys::WebGlRenderingContext::BLEND);
        view_gl.blend_func_separate(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ONE,
        );
        view_gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        view_gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        view_gl.cull_face(web_sys::WebGlRenderingContext::BACK);
        view_gl.enable(web_sys::WebGlRenderingContext::STENCIL_TEST);

        view_gl.clear_color(0.0, 0.0, 0.0, 0.0);
        view_gl.clear_stencil(0);

        let offscreen_canvas = Rc::new(crate::libs::element::html_canvas_element());
        offscreen_canvas.set_width(canvas_size[0] as u32);
        offscreen_canvas.set_height(canvas_size[1] as u32);
        let option: JsValue = object! {preserveDrawingBuffer: true}.into();
        let offscreen_gl = offscreen_canvas
            .get_context_with_context_options("webgl", &option)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        let offscreen_gl = WebGlRenderingContext::new(offscreen_gl);

        offscreen_gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        offscreen_gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        offscreen_gl.enable(web_sys::WebGlRenderingContext::BLEND);
        offscreen_gl.blend_func(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        offscreen_gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
        offscreen_gl.cull_face(web_sys::WebGlRenderingContext::BACK);

        let mut tex_table = tex_table::TexTable::new(&view_gl);

        let render_view_tablegrid = view::tablegrid::Tablegrid::new(&view_gl);
        let render_view_tabletexture =
            view::tabletexture::Tabletexture::new(&view_gl, &mut tex_table);
        let render_view_character = view::character::Character::new(&view_gl);

        Self {
            view_canvas,
            view_gl,
            offscreen_canvas,
            offscreen_gl,
            canvas_size,
            device_pixel_ratio,
            tex_table,
            render_view_tablegrid,
            render_view_tabletexture,
            render_view_character,
        }
    }

    pub fn reset_size(&mut self) {
        let canvas_size = Self::reset_canvas_size(&self.view_canvas, self.device_pixel_ratio);

        self.offscreen_canvas.set_width(canvas_size[0] as u32);
        self.offscreen_canvas.set_height(canvas_size[1] as u32);

        self.view_gl
            .viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        self.offscreen_gl
            .viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);

        self.canvas_size = canvas_size;
    }

    pub fn render(
        &mut self,
        block_arena: &block::Arena,
        local_block_arena: &block::Arena,
        resource_arena: &resource::Arena,
        world_id: &BlockId,
        camera_matrix: &CameraMatrix,
    ) {
        block_arena.map(world_id, |world: &block::world::World| {
            self.view_gl.clear_color(0.0, 0.0, 0.0, 0.0);
            self.view_gl.clear_stencil(0);

            let vp_matrix = camera_matrix
                .perspective_matrix(&self.canvas_size)
                .dot(&camera_matrix.view_matrix());

            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                self.render_view_tabletexture.render(
                    &mut self.view_gl,
                    &mut self.tex_table,
                    &vp_matrix,
                    block_arena,
                    local_block_arena,
                    resource_arena,
                    table,
                );
            });

            block_arena.map(world.selecting_table(), |table: &block::table::Table| {
                self.render_view_tablegrid
                    .render(&mut self.view_gl, &vp_matrix, table)
            });

            self.render_view_character.render(
                &mut self.view_gl,
                &mut self.tex_table,
                camera_matrix,
                &vp_matrix,
                block_arena,
                resource_arena,
                world.characters().map(|x| BlockId::clone(x)),
            );
        });
    }
}
