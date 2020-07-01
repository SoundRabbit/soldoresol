use super::super::{
    program::MaskProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use super::Camera;
use crate::{
    block::{self, BlockId},
    Color,
};
use ndarray::Array2;

pub struct CharacterMaskRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    mask_program: MaskProgram,
}

impl CharacterMaskRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        let mask_program = MaskProgram::new(gl);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
            mask_program,
        }
    }

    pub fn render<'a>(
        &mut self,
        gl: &WebGlRenderingContext,
        _: &Camera,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        characters: impl Iterator<Item = &'a BlockId>,
    ) {
        self.mask_program.use_program(gl);
        gl.set_attribute(
            &self.vertexis_buffer,
            &self.mask_program.a_vertex_location,
            3,
            0,
        );
        gl.set_attribute(
            &self.texture_coord_buffer,
            &self.mask_program.a_texture_coord_location,
            2,
            0,
        );
        gl.uniform1i(Some(&self.mask_program.u_flag_round_location), 1);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (_, character) in block_field.listed::<block::Character>(characters.collect()) {
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(character.size())
                .with_movement(character.position())
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();

            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.mask_program.u_translate_location),
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
            gl.uniform4fv_with_f32_array(
                Some(&self.mask_program.u_mask_color_location),
                &Color::from([0.0, 0.0, 0.0, 0.75]).to_f32array(),
            );
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}
