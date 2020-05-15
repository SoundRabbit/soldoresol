mod character_collection_renderer;

use character_collection_renderer::CharacterCollectionRenderer;

use super::webgl::{WebGlAttributeLocation, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use super::ModelMatrix;
use crate::model::{Camera, World};
use crate::shader;
use wasm_bindgen::JsCast;

pub struct MaskRenderer {
    canvas: web_sys::HtmlCanvasElement,
    gl: WebGlRenderingContext,
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    u_translate_location: web_sys::WebGlUniformLocation,
    u_mask_color_location: web_sys::WebGlUniformLocation,
    character_collection_renderer: CharacterCollectionRenderer,
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

        let vertex_shader = shader::compile_shader(&gl, &shader::mask::vertex_shader()).unwrap();
        let fragment_shader =
            shader::compile_shader(&gl, &shader::mask::fragment_shader()).unwrap();
        let program = shader::link_program(&gl, &vertex_shader, &fragment_shader).unwrap();
        gl.use_program(Some(&program));

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);

        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_mask_color_location = gl.get_uniform_location(&program, "u_maskColor").unwrap();

        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::ONE,
            web_sys::WebGlRenderingContext::ZERO,
        );

        let character_collection_renderer = CharacterCollectionRenderer::new(&gl);

        Self {
            canvas,
            gl,
            program,
            a_vertex_location,
            u_translate_location,
            u_mask_color_location,
            character_collection_renderer,
        }
    }

    pub fn table_object_id(&self, position: &[f64; 2]) -> u32 {
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
        u32::from_be_bytes([pixel[3], pixel[0], pixel[1], pixel[2]])
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

        self.character_collection_renderer.render(
            gl,
            camera,
            &vp_matrix,
            &self.a_vertex_location,
            &self.u_translate_location,
            &self.u_mask_color_location,
            world.characters(),
        );

        gl.flush();
    }
}
