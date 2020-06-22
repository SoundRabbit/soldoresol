use super::super::{
    program::MaskProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use crate::{
    model::{Camera, Character, Color},
    random_id,
};
use ndarray::Array2;
use std::collections::{hash_map, HashMap};

#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

pub struct CharacterCollectionRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl CharacterCollectionRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 1.0, 0.0],
                [-0.5, 1.0, 0.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        program: &MaskProgram,
        camera: &Camera,
        vp_matrix: &Array2<f64>,
        characters: hash_map::Iter<u128, Character>,
        id_map: &mut HashMap<u32, u128>,
    ) {
        gl.set_attribute(&self.vertexis_buffer, &program.a_vertex_location, 3, 0);
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

        let mut mvp_matrixies: Vec<(Array2<f64>, [f32; 4])> = vec![];

        for (character_id, character) in characters {
            let s = character.size();
            let p = character.position();
            let model_matrix: Array2<f64> = ModelMatrix::new()
                .with_scale(&[s[0], s[0], 1.0])
                .with_movement(&[p[0], (p[1] - 0.5 * s[0]), p[2]])
                .into();
            let mvp_matrix = model_matrix.dot(vp_matrix);
            let color = Color::from(random_id::u32val());

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

            let model_matrix: Array2<f64> = ModelMatrix::new()
                .with_scale(&[s[0], s[1], 1.0])
                .with_x_axis_rotation(camera.x_axis_rotation())
                .with_z_axis_rotation(camera.z_axis_rotation())
                .with_movement(&p)
                .into();
            let mvp_matrix = model_matrix.dot(vp_matrix);

            mvp_matrixies.push((mvp_matrix, color.to_f32array()));

            id_map.insert(color.to_u32(), *character_id);
        }

        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);

        for (mvp_matrix, color) in mvp_matrixies {
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
