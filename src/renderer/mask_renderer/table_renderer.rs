use super::super::program::MaskProgram;
use super::super::webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use super::super::ModelMatrix;
use crate::model::{Camera, Color, Table};
use ndarray::Array2;

pub struct TableRenderer {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl TableRenderer {
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
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        Self {
            vertexis_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        program: &MaskProgram,
        camera: &Camera,
        vp_matrix: &Array2<f64>,
        table: &Table,
        table_id: u32,
    ) {
        gl.set_attribute(&self.vertexis_buffer, &program.a_vertex_location, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        let s = table.size();
        let model_matrix: Array2<f64> = ModelMatrix::new().with_scale(&[s[0], s[1], 1.0]).into();
        let mvp_matrix = model_matrix.dot(vp_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&program.u_translate_location),
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
            Some(&program.u_mask_color_location),
            &Color::from(table_id).to_f32array(),
        );
        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            6,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}
