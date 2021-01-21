use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

mod matrix;
mod webgl;

use webgl::WebGlRenderingContext;

pub struct Renderer {
    view_canvas: Rc<web_sys::HtmlCanvasElement>,
    view_gl: WebGlRenderingContext,
    offscreen_canvas: Rc<web_sys::HtmlCanvasElement>,
    offscreen_gl: WebGlRenderingContext,
}

impl Renderer {
    pub fn new(view_canvas: Rc<web_sys::HtmlCanvasElement>) -> Self {
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
        offscreen_canvas.set_width(view_canvas.width());
        offscreen_canvas.set_height(view_canvas.height());
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

        Self {
            view_canvas,
            view_gl,
            offscreen_canvas,
            offscreen_gl,
        }
    }
}
