mod area_collection_renderer;
mod character_collection_renderer;
mod tablemask_collection_renderer;

use super::{program::MaskProgram, webgl::WebGlRenderingContext, Camera};
use crate::block::{self, BlockId};
use area_collection_renderer::AreaCollectionRenderer;
pub use character_collection_renderer::CharacterCollectionRenderer;
use std::{collections::HashMap, rc::Rc};
use tablemask_collection_renderer::TablemaskCollectionRenderer;
use wasm_bindgen::JsCast;

pub struct MaskRenderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: Rc<WebGlRenderingContext>,
    mask_program: MaskProgram,
    area_collection_renderer: AreaCollectionRenderer,
    character_collection_renderer: CharacterCollectionRenderer,
    tablemask_collection_renderer: TablemaskCollectionRenderer,
    id_map: HashMap<u32, BlockId>,
}

impl MaskRenderer {
    pub fn new() -> Self {
        let canvas = crate::util::html_canvas_element();
        let option = object! {
            preserveDrawingBuffer: true
        };
        let option: js_sys::Object = option.into();
        let gl = canvas
            .get_context_with_context_options("webgl", &option.into())
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        let gl = Rc::new(WebGlRenderingContext(gl));

        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        let area_collection_renderer = AreaCollectionRenderer::new(&gl);
        let character_collection_renderer = CharacterCollectionRenderer::new(&gl);
        let tablemask_collection_renderer = TablemaskCollectionRenderer::new(&gl);

        let mask_program = MaskProgram::new(&gl);
        mask_program.use_program(&gl);

        Self {
            canvas,
            gl,
            mask_program,
            area_collection_renderer,
            character_collection_renderer,
            tablemask_collection_renderer,
            id_map: HashMap::new(),
        }
    }

    pub fn gl(&self) -> Rc<WebGlRenderingContext> {
        Rc::clone(&self.gl)
    }

    pub fn table_object_id(&self, canvas_size: &[f32; 2], position: &[f32; 2]) -> Option<&BlockId> {
        let mut pixel = [0, 0, 0, 0];
        self.gl
            .read_pixels_with_opt_u8_array(
                position[0] as i32,
                (canvas_size[1] - position[1]) as i32,
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

    pub fn render(
        &mut self,
        canvas_size: &[f32; 2],
        camera: &Camera,
        block_field: &block::Field,
        world: &block::World,
    ) {
        crate::debug::log_2(self.canvas.width(), self.canvas.height());

        let gl = &self.gl;
        let canvas = &self.canvas;
        canvas.set_width(canvas_size[0] as u32);
        canvas.set_height(canvas_size[1] as u32);
        let vp_matrix = camera
            .perspective_matrix(&canvas_size)
            .dot(&camera.view_matrix());
        gl.viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(
            web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        self.id_map.clear();

        self.id_map
            .insert(0xFF000000, world.selecting_table().clone());

        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);

        if let Some(table) = block_field.get::<block::Table>(world.selecting_table()) {
            self.tablemask_collection_renderer.render(
                gl,
                &self.mask_program,
                &vp_matrix,
                block_field,
                table.tablemasks(),
                &mut self.id_map,
            );
            self.area_collection_renderer.render(
                gl,
                &self.mask_program,
                &vp_matrix,
                block_field,
                table.areas(),
                &mut self.id_map,
            );
        }

        self.character_collection_renderer.render(
            gl,
            &self.mask_program,
            camera,
            &vp_matrix,
            block_field,
            world.characters(),
            &mut self.id_map,
        );
    }
}
