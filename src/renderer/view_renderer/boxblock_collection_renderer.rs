use super::super::{
    program::CubeProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use crate::block::{self, BlockId};
use ndarray::Array2;

pub struct BoxblockCollectionRenderer {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    cube_program: CubeProgram,
}

impl BoxblockCollectionRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [0.5, 0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
            ]
            .concat(),
        );
        let index_buffer = gl.create_ibo_with_i16array(
            &[
                [0, 1, 2, 3, 2, 1],
                [0, 1, 4, 5, 4, 1],
                [0, 2, 4, 6, 4, 2],
                [1, 3, 5, 7, 5, 3],
                [2, 3, 6, 7, 6, 3],
                [4, 5, 6, 7, 6, 5],
            ]
            .concat(),
        );

        let cube_program = CubeProgram::new(gl);

        Self {
            vertexis_buffer,
            index_buffer,
            cube_program,
        }
    }

    pub fn render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        boxblocks: impl Iterator<Item = &'a BlockId>,
    ) {
        self.cube_program.use_program(gl);

        gl.set_attribute(
            &self.vertexis_buffer,
            &self.cube_program.a_vertex_location,
            3,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (_, boxblock) in
            block_field.listed::<block::table_object::Boxblock>(boxblocks.collect())
        {
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(boxblock.size())
                .with_movement(boxblock.position())
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.cube_program.u_translate_location),
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
                Some(&self.cube_program.u_mask_color_location),
                &boxblock.color().to_f32array(),
            );
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                36,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}
