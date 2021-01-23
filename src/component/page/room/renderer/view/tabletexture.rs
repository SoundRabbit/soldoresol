use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;

pub struct Tabletexture {
    polygon_vertexis_buffer: WebGlF32Vbo,
    polygon_index_buffer: WebGlI16Ibo,
    polygon_texture_coord_buffer: WebGlF32Vbo,
    polygon_texture_buffer: web_sys::WebGlTexture,
    texture_update_time: f64,
    last_texture_id: BlockId,
}

impl Tabletexture {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let polygon_vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );
        let polygon_texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());
        let polygon_index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        let polygon_texture_buffer = gl.create_texture().unwrap();

        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&polygon_texture_buffer),
        );
        gl.pixel_storei(web_sys::WebGlRenderingContext::PACK_ALIGNMENT, 1);
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

        Self {
            polygon_vertexis_buffer,
            polygon_texture_coord_buffer,
            polygon_index_buffer,
            polygon_texture_buffer,
            texture_update_time: 0.0,
            last_texture_id: BlockId::none(),
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        table: &block::table::Table,
    ) {
        let table_size = {
            let sz = table.size();
            [sz[0], sz[1]]
        };

        gl.use_program(ProgramType::TabletextureProgram);

        gl.set_attr_vertex(&self.polygon_vertexis_buffer, 3, 0);
        gl.set_attr_tex_coord(&self.polygon_texture_coord_buffer, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.polygon_index_buffer),
        );

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE0);
        gl.set_unif_texture_1(0);
        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.polygon_texture_buffer),
        );
        let drawing_texture_id = table.drawing_texture_id();
        let texture_update_time = block_arena.timestamp_of(drawing_texture_id).unwrap_or(0.0);
        if self.last_texture_id != *drawing_texture_id
            || self.texture_update_time < texture_update_time
        {
            self.texture_update_time = texture_update_time;

            block_arena.map(
                drawing_texture_id,
                |table_texture: &block::table::texture::Texture| {
                    let _ = gl.tex_image_2d_with_u32_and_u32_and_canvas(
                        web_sys::WebGlRenderingContext::TEXTURE_2D,
                        0,
                        web_sys::WebGlRenderingContext::RGBA as i32,
                        web_sys::WebGlRenderingContext::RGBA,
                        web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                        table_texture.element(),
                    );
                },
            );
        }

        let model_matrix: Array2<f32> = ModelMatrix::new()
            .with_scale(&[table_size[0], table_size[1], 1.0])
            .into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);

        gl.set_unif_translate(mvp_matrix.reversed_axes());
        gl.set_unif_bg_color(&crate::libs::color::color_system::blue(100, 5).to_f32array());
        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            6,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}
