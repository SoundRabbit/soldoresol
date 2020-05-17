mod character_collection_renderer;
// mod table_renderer;
mod tablemask_collection_renderer;

use super::program::MaskProgram;
use super::webgl::WebGlRenderingContext;
use crate::model::{Camera, World};
pub use character_collection_renderer::CharacterCollectionRenderer;
// use table_renderer::TableRenderer;
use tablemask_collection_renderer::TablemaskCollectionRenderer;
use wasm_bindgen::JsCast;

pub struct MaskRenderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: WebGlRenderingContext,
    mask_program: MaskProgram,
    // table_renderer: TableRenderer,
    character_collection_renderer: CharacterCollectionRenderer,
    tablemask_collection_renderer: TablemaskCollectionRenderer,
    id_map: Vec<u128>,
}

impl MaskRenderer {
    pub fn new() -> Self {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let gl = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        let gl = WebGlRenderingContext(gl);

        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ZERO,
        );

        let character_collection_renderer = CharacterCollectionRenderer::new(&gl);
        // let table_renderer = TableRenderer::new(&gl);
        let tablemask_collection_renderer = TablemaskCollectionRenderer::new(&gl);

        let mask_program = MaskProgram::new(&gl);
        mask_program.use_program(&gl);

        Self {
            canvas,
            gl,
            mask_program,
            // table_renderer,
            character_collection_renderer,
            tablemask_collection_renderer,
            id_map: Vec::new(),
        }
    }

    pub fn table_object_id(&self, position: &[f64; 2]) -> u128 {
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
        self.id_map[u32::from_be_bytes([pixel[3], pixel[0], pixel[1], pixel[2]]) as usize]
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

        // self.table_renderer.render(
        //     gl,
        //     &self.mask_program,
        //     camera,
        //     &vp_matrix,
        //     world.table(),
        //     world.table_id(),
        //     &mut self.id_map,
        // );

        self.id_map.push(world.table_id());

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

        gl.flush();
    }
}
