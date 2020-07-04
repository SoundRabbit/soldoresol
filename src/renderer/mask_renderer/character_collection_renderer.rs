use super::super::{
    program::MaskProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use super::{Camera, TableBlock};
use crate::{
    block::{self, BlockId},
    Color,
};
use ndarray::Array2;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

pub struct CharacterCollectionRenderer {
    vertexis_buffer_xy: WebGlF32Vbo,
    vertexis_buffer_xz: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl CharacterCollectionRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer_xy = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );
        let vertexis_buffer_xz = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.0, 1.0],
                [-0.5, 0.0, 1.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        Self {
            vertexis_buffer_xy,
            vertexis_buffer_xz,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        program: &MaskProgram,
        camera: &Camera,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        characters: impl Iterator<Item = &'a BlockId>,
        id_map: &mut HashMap<u32, TableBlock>,
    ) {
        gl.set_attribute(&self.vertexis_buffer_xy, &program.a_vertex_location, 3, 0);
        gl.set_attribute(
            &self.texture_coord_buffer,
            &program.a_texture_coord_location,
            2,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        let mut mvp_matrixies: Vec<(Array2<f32>, [f32; 4])> = vec![];

        for (character_id, character) in
            block_field.listed::<block::Character>(characters.collect())
        {
            let s = character.size();
            let p = character.position();
            let model_matrix: Array2<f32> =
                ModelMatrix::new().with_scale(s).with_movement(p).into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();
            let color = Color::from(id_map.len() as u32 | 0xFF000000);

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
            gl.uniform1i(Some(&program.u_flag_round_location), 1);
            gl.uniform4fv_with_f32_array(
                Some(&program.u_mask_color_location),
                &color.to_f32array(),
            );

            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );

            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(s)
                .with_x_axis_rotation(camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                .with_z_axis_rotation(camera.z_axis_rotation())
                .with_movement(p)
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);

            mvp_matrixies.push((mvp_matrix, color.to_f32array()));

            id_map.insert(color.to_u32(), TableBlock::new(character_id.clone(), 0));
        }

        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.set_attribute(&self.vertexis_buffer_xz, &program.a_vertex_location, 3, 0);

        for (mvp_matrix, color) in mvp_matrixies {
            let mvp_matrix = mvp_matrix.t();
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
            gl.uniform1i(Some(&program.u_flag_round_location), 0);
            gl.uniform4fv_with_f32_array(Some(&program.u_mask_color_location), &color);
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}
