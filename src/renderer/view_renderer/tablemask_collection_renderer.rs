use super::super::{
    program::TablemaskProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use crate::block::{self, BlockId};
use ndarray::Array2;

pub struct TablemaskCollectionRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    program: TablemaskProgram,
}

impl TablemaskCollectionRenderer {
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

        let program = TablemaskProgram::new(gl);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
            program,
        }
    }

    pub fn render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        data_field: &block::Field,
        tablemasks: impl Iterator<Item = &'a BlockId>,
    ) {
        self.program.use_program(gl);

        gl.set_attribute(&self.vertexis_buffer, &self.program.a_vertex_location, 3, 0);
        gl.set_attribute(
            &self.texture_coord_buffer,
            &self.program.a_texture_coord_location,
            2,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (_, tablemask) in
            data_field.listed::<block::table_object::Tablemask>(tablemasks.collect())
        {
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(tablemask.size())
                .with_movement(tablemask.position())
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.program.u_translate_location),
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
                Some(&self.program.u_mask_color_location),
                &tablemask.background_color().to_f32array(),
            );
            gl.uniform1i(
                Some(&self.program.u_flag_round_location),
                if tablemask.is_rounded() { 1 } else { 0 },
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
