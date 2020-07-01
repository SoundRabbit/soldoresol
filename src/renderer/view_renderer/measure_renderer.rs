use super::super::{
    program::TableGridProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo},
    ModelMatrix,
};
use super::WebGlRenderingContext;
use crate::block;
use ndarray::Array2;

pub struct MeasureRenderer {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    table_grid_program: TableGridProgram,
}

impl MeasureRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer =
            gl.create_vbo_with_f32array(&[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]].concat());

        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        let table_grid_program = TableGridProgram::new(gl);

        Self {
            vertexis_buffer,
            index_buffer,
            table_grid_program,
        }
    }

    pub fn render<'a>(
        &mut self,
        gl: &WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        data_field: &block::Field,
    ) {
        self.table_grid_program.use_program(gl);

        gl.set_attribute(
            &self.vertexis_buffer,
            &self.table_grid_program.a_vertex_location,
            3,
            0,
        );

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        gl.uniform1f(Some(&self.table_grid_program.u_point_size_location), 8.0);

        for (_, measure) in data_field.all::<block::table_object::Measure>() {
            let s = measure.vec();
            let p = measure.org();
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(&[s[0], s[1], s[2]])
                .with_movement(&p)
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.table_grid_program.u_translate_location),
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
                Some(&self.table_grid_program.u_mask_color_location),
                &measure.color().to_f32array(),
            );
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::LINES,
                2,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::POINTS,
                2,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}
