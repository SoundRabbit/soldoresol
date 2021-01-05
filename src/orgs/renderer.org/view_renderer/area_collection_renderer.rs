use super::super::{
    program::MaskCheckProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use crate::block::{self, BlockId};
use ndarray::Array2;

pub struct AreaCollectionRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    mask_check_program: MaskCheckProgram,
}

impl AreaCollectionRenderer {
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

        let mask_check_program = MaskCheckProgram::new(gl);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
            mask_check_program,
        }
    }

    pub fn render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        areas: impl Iterator<Item = &'a BlockId>,
    ) {
        self.mask_check_program.use_program(gl);

        gl.set_attribute(
            &self.vertexis_buffer,
            &self.mask_check_program.a_vertex_location,
            3,
            0,
        );
        gl.set_attribute(
            &self.texture_coord_buffer,
            &self.mask_check_program.a_texture_coord_location,
            2,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (_, area) in block_field.listed::<block::table_object::Area>(areas.collect()) {
            let (model_matrix, is_rounded, size) =
                if let block::table_object::area::Type::Line(line_width) = area.type_() {
                    let line_width = *line_width as f32;
                    let o = area.org().clone();
                    let v = area.vec().clone();

                    let len = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();

                    let zr = v[1].atan2(v[0]);

                    let mm: Array2<f32> = ModelMatrix::new()
                        .with_scale(&[len, line_width, 0.0])
                        .with_z_axis_rotation(zr)
                        .with_movement(&[o[0] + v[0] / 2.0, o[1] + v[1] / 2.0, o[2] + v[2] / 2.0])
                        .into();
                    (mm, false, [len, line_width])
                } else {
                    let o = area.org();
                    let v = area.vec().clone();

                    let len = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt() * 2.0;

                    let mm: Array2<f32> = ModelMatrix::new()
                        .with_scale(&[len, len, 0.0])
                        .with_movement(o)
                        .into();
                    (mm, true, [len, len])
                };

            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.mask_check_program.u_translate_location),
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
                Some(&self.mask_check_program.u_mask_color_1_location),
                &area.color_1().to_f32array(),
            );
            gl.uniform4fv_with_f32_array(
                Some(&self.mask_check_program.u_mask_color_2_location),
                &area.color_2().to_f32array(),
            );
            gl.uniform2fv_with_f32_array(
                Some(&self.mask_check_program.u_mask_size_location),
                &size,
            );
            gl.uniform1i(
                Some(&self.mask_check_program.u_flag_round_location),
                if is_rounded { 1 } else { 0 },
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
