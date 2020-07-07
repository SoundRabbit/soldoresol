use super::super::{
    program::CubeProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use crate::block::{self, BlockId};
use ndarray::Array2;

pub struct BoxblockCollectionRenderer {
    vertexis_buffer: WebGlF32Vbo,
    normals_buffer: WebGlF32Vbo,
    poly_index_buffer: WebGlI16Ibo,
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
        let normals_buffer = gl.create_vbo_with_f32array(
            &[
                Self::n(0.5, 0.5, 0.5),
                Self::n(-0.5, 0.5, 0.5),
                Self::n(0.5, -0.5, 0.5),
                Self::n(-0.5, -0.5, 0.5),
                Self::n(0.5, 0.5, -0.5),
                Self::n(-0.5, 0.5, -0.5),
                Self::n(0.5, -0.5, -0.5),
                Self::n(-0.5, -0.5, -0.5),
            ]
            .concat(),
        );
        let poly_index_buffer = gl.create_ibo_with_i16array(
            &[
                [0, 1, 2, 3, 2, 1],
                [4, 1, 0, 1, 4, 5],
                [0, 2, 4, 6, 4, 2],
                [5, 3, 1, 3, 5, 7],
                [2, 3, 6, 7, 6, 3],
                [6, 5, 4, 5, 6, 7],
            ]
            .concat(),
        );

        let cube_program = CubeProgram::new(gl);

        Self {
            vertexis_buffer,
            poly_index_buffer,
            normals_buffer,
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

        gl.set_attribute(
            &self.normals_buffer,
            &self.cube_program.a_normal_location,
            3,
            0,
        );

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.poly_index_buffer),
        );

        gl.uniform3fv_with_f32_array(Some(&self.cube_program.u_light_location), &[1.0, -1.0, 1.0]);

        for (_, boxblock) in
            block_field.listed::<block::table_object::Boxblock>(boxblocks.collect())
        {
            let s = boxblock.size();
            let p = boxblock.position();
            let model_matrix: Array2<f32> =
                ModelMatrix::new().with_scale(s).with_movement(p).into();
            let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                .with_movement(&[-p[0], p[1], -p[2]])
                .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
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
            let inv_model_matrix = inv_model_matrix.t();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&self.cube_program.u_inv_model_location),
                false,
                &[
                    inv_model_matrix.row(0).to_vec(),
                    inv_model_matrix.row(1).to_vec(),
                    inv_model_matrix.row(2).to_vec(),
                    inv_model_matrix.row(3).to_vec(),
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

    fn n(x: f32, y: f32, z: f32) -> [f32; 3] {
        let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        [x / len, y / len, z / len]
    }
}
