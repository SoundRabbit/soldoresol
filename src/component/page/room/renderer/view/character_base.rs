use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;

pub struct CharacterBase {
    vertexes_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl CharacterBase {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexes_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 1.0 / 128.0],
                [-0.5, 0.5, 1.0 / 128.0],
                [0.5, -0.5, 1.0 / 128.0],
                [-0.5, -0.5, 1.0 / 128.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        Self {
            vertexes_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        character_ids: impl Iterator<Item = BlockId>,
    ) {
        gl.use_program(ProgramType::TablemaskProgram);
        gl.set_attr_vertex(&self.vertexes_buffer, 3, 0);
        gl.set_attr_tex_coord(&self.texture_coord_buffer, 2, 0);
        gl.set_unif_flag_round(1);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        let bg_color = crate::libs::color::Pallet::gray(9)
            .a(75)
            .to_color()
            .to_f32array();

        let _ = block_arena.iter_map_with_ids(
            character_ids,
            |_, character: &block::character::Character| {
                let sz = character.size();
                let model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_scale(&[sz, sz, 1.0])
                    .with_movement(character.position())
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_unif_translate(mvp_matrix.reversed_axes());
                gl.set_unif_bg_color(&bg_color);
                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::TRIANGLES,
                    6,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            },
        );
    }
}
