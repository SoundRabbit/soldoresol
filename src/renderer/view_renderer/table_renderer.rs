use super::super::program::MaskProgram;
use super::super::webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use super::super::ModelMatrix;
use crate::model::{Camera, Color, Table};
use ndarray::Array2;

pub struct TableRenderer {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    mask_program: MaskProgram,
}

impl TableRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer =
            gl.create_vbo_with_f32array(&[[0.5, 0.0, 0.0], [-0.5, 0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1]);
        let mask_program = MaskProgram::new(gl);
        Self {
            vertexis_buffer,
            index_buffer,
            mask_program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        camera: &Camera,
        vp_matrix: &Array2<f64>,
        table: &Table,
    ) {
        self.mask_program.use_program(gl);
        gl.enable(web_sys::WebGlRenderingContext::BLEND);
        gl.blend_func(
            web_sys::WebGlRenderingContext::SRC_ALPHA,
            web_sys::WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        gl.set_attribute(
            &self.vertexis_buffer,
            &self.mask_program.a_vertex_location,
            3,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        let [height, width] = table.size();
        let (height, width) = (*height, *width);

        let (min_y, max_y) = (-(height / 2.0).floor(), (height / 2.0).floor());
        let (min_x, max_x) = (-(width / 2.0).floor(), (width / 2.0).floor());

        for d in 0..2 {
            let (min, max, x_flag, y_flag) = if d % 2 == 0 {
                (min_y as i32, max_y as i32, 0.0, 1.0)
            } else {
                (min_x as i32, max_x as i32, 1.0, 0.0)
            };
            for p in (min as i32)..(max as i32 + 1) {
                let p = p as f64;
                let model_matrix: Array2<f64> = ModelMatrix::new()
                    .with_scale(&[width, height, 1.0])
                    .with_z_axis_rotation(std::f64::consts::PI / 2.0 * x_flag)
                    .with_movement(&[p * x_flag, p * y_flag, 0.0])
                    .into();
                let mvp_matrix = model_matrix.dot(vp_matrix);
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
                    &Color::from([0, 0, 0, 255]).to_f32array(),
                );
                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::LINE_LOOP,
                    2,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            }
        }
    }
}
