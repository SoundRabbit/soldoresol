mod character_collection_renderer;
mod tablemask_collection_renderer;

use super::{program::MaskProgram, webgl::WebGlRenderingContext};
use crate::model::{Camera, World};
pub use character_collection_renderer::CharacterCollectionRenderer;
use std::{collections::HashMap, rc::Rc};
use tablemask_collection_renderer::TablemaskCollectionRenderer;
use wasm_bindgen::JsCast;

pub struct MaskRenderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: Rc<WebGlRenderingContext>,
    mask_program: MaskProgram,
    character_collection_renderer: CharacterCollectionRenderer,
    tablemask_collection_renderer: TablemaskCollectionRenderer,
    id_map: HashMap<u32, u128>,
}

impl MaskRenderer {
    pub fn new() -> Self {
        let canvas = crate::util::html_canvas_element();
        let gl = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        let gl = Rc::new(WebGlRenderingContext(gl));

        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ZERO,
        );

        let character_collection_renderer = CharacterCollectionRenderer::new(&gl);
        let tablemask_collection_renderer = TablemaskCollectionRenderer::new(&gl);

        let mask_program = MaskProgram::new(&gl);
        mask_program.use_program(&gl);

        Self {
            canvas,
            gl,
            mask_program,
            character_collection_renderer,
            tablemask_collection_renderer,
            id_map: HashMap::new(),
        }
    }

    pub fn gl(&self) -> Rc<WebGlRenderingContext> {
        Rc::clone(&self.gl)
    }

    pub fn table_object_id(&self, position: &[f64; 2]) -> Option<&u128> {
        let mut pixel = [0, 0, 0, 0];
        self.gl
            .read_pixels_with_opt_u8_array(
                position[0] as i32,
                position[1] as i32,
                1,
                1,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&mut pixel),
            )
            .unwrap();
        self.id_map.get(&u32::from_be_bytes([
            pixel[3], pixel[0], pixel[1], pixel[2],
        ]))
    }

    pub fn render(&mut self, canvas_size: &[f64; 2], camera: &Camera, world: &mut World) {
        let gl = &self.gl;
        let canvas = &self.canvas;
        canvas.set_width(canvas_size[0] as u32);
        canvas.set_height(canvas_size[1] as u32);
        let vp_matrix = camera
            .view_matrix()
            .dot(&camera.perspective_matrix(&canvas_size));
        gl.viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        self.id_map.clear();

        self.id_map.insert(0, world.table_id());

        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);

        self.tablemask_collection_renderer.render(
            gl,
            &self.mask_program,
            camera,
            &vp_matrix,
            world.tablemasks(),
            &mut self.id_map,
        );

        self.character_collection_renderer.render(
            gl,
            &self.mask_program,
            camera,
            &vp_matrix,
            world.characters(),
            &mut self.id_map,
        );
    }
}
