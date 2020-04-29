mod basic_renderer;
mod character_renderer;
mod model_matrix;
mod table_renderer;
mod webgl;

use crate::model::Camera;
use crate::model::World;
use crate::shader;
use basic_renderer::BasicRenderer;
use character_renderer::CharacterRenderer;
use ndarray::Array2;
use table_renderer::TableRenderer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use webgl::WebGlAttributeLocation;
use webgl::WebGlRenderingContext;

pub struct Renderer {
    gl: WebGlRenderingContext,
    program: web_sys::WebGlProgram,
    texture: web_sys::WebGlTexture,
    table_renderer: TableRenderer,
    character_renderer: CharacterRenderer,
    a_vertex_location: WebGlAttributeLocation,
    a_color_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_translate_location: web_sys::WebGlUniformLocation,
    u_texture_location: web_sys::WebGlUniformLocation,
}

impl Renderer {
    pub fn new(gl: web_sys::WebGlRenderingContext) -> Self {
        let gl = WebGlRenderingContext(gl);

        let vertex_shader = shader::compile_shader(&gl, &shader::default::vertex_shader()).unwrap();
        let fragment_shader =
            shader::compile_shader(&gl, &shader::default::fragment_shader()).unwrap();
        let program = shader::link_program(&gl, &vertex_shader, &fragment_shader).unwrap();
        gl.use_program(Some(&program));

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_color_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_color") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);

        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_texture_location = gl.get_uniform_location(&program, "u_texture").unwrap();

        let texture = gl.create_texture().unwrap();
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        gl.pixel_storei(web_sys::WebGlRenderingContext::PACK_ALIGNMENT, 1);

        let table_renderer = TableRenderer::new(&gl);
        let character_renderer = CharacterRenderer::new(&gl);

        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
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

        gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);

        gl.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        Self {
            gl,
            program,
            texture,
            table_renderer,
            character_renderer,
            a_vertex_location,
            a_color_location,
            a_texture_coord_location,
            u_translate_location,
            u_texture_location,
        }
    }

    pub fn render(&self, world: &mut World, camera: &Camera) {
        let gl = &self.gl;
        let canvas = gl
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let canvas_size = [canvas.width() as f64, canvas.height() as f64];
        let vp_matrix = camera
            .view_matrix()
            .dot(&camera.perspective_matrix(&canvas_size));
        gl.viewport(0, 0, canvas_size[0] as i32, canvas_size[1] as i32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT );

        // render table
        self.alloc_memory(&self.table_renderer);
        let table = world.table_mut();
        let texture = table.texture_element();
        gl.tex_image_2d_with_u32_and_u32_and_canvas(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            0,
            web_sys::WebGlRenderingContext::RGBA as i32,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            &texture,
        )
        .unwrap();
        self.draw_with_model(&camera, &vp_matrix, table, &self.table_renderer);
        table.flip();

        // render character
        self.alloc_memory(&self.character_renderer);
        for (_, character) in world.characters() {
            self.draw_with_model(&camera, &vp_matrix, character, &self.character_renderer);
        }

        // v-sync
        let gl = (*gl).clone();
        let a = Closure::once(Box::new(move || {
            gl.finish();
        }) as Box<dyn FnOnce()>);
        web_sys::window()
            .unwrap()
            .request_animation_frame(a.as_ref().unchecked_ref())
            .unwrap();
        a.forget();
    }

    fn alloc_memory(&self, renderer: &impl BasicRenderer) {
        let gl = &self.gl;
        let vertexis = renderer.vertexis();
        let color = renderer.color();
        let texture_coord = renderer.texture_coord();
        let index = renderer.index();
        gl.set_attribute(&vertexis, &self.a_vertex_location, 3, 0);
        gl.set_attribute(&color, &self.a_color_location, 4, 0);
        gl.set_attribute(&texture_coord, &self.a_texture_coord_location, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(index),
        );
        gl.uniform1i(Some(&self.u_texture_location), 0);
    }

    fn draw_with_model<M>(
        &self,
        camera: &Camera,
        vp_matrix: &Array2<f64>,
        model: &M,
        renderer: &impl BasicRenderer<Model = M>,
    ) {
        let gl = &self.gl;
        let mvp_matrix = renderer.model_matrix(&camera, &model).dot(vp_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_translate_location),
            false,
            &[
                mvp_matrix.row(0).to_vec(),
                mvp_matrix.row(1).to_vec(),
                mvp_matrix.row(2).to_vec(),
                mvp_matrix.row(3).to_vec(),
            ]
            .concat()
            .into_iter()
            .map(|a| a as f32)
            .collect::<Vec<f32>>(),
        );
        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            6,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}
