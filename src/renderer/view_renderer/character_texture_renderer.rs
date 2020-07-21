use super::super::{
    program::CharacterProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use super::Camera;
use crate::{
    block::{self, BlockId},
    resource::Data,
    Resource,
};
use ndarray::{arr1, Array2};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct CharacterTextureRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    character_program: CharacterProgram,
}

impl CharacterTextureRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.0, 1.0],
                [-0.5, 0.0, 1.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        let character_program = CharacterProgram::new(gl);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
            character_program,
        }
    }

    pub fn render<'a>(
        &mut self,
        gl: &WebGlRenderingContext,
        camera: &Camera,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        characters: impl Iterator<Item = &'a BlockId>,
        textures: &mut super::TextureCollection,
        resource: &Resource,
        client_id: &String,
    ) {
        let mut z_index: BTreeMap<OrderedFloat<f32>, Vec<(Array2<f32>, &block::Character)>> =
            BTreeMap::new();
        for (_, character) in block_field.listed::<block::Character>(characters.collect()) {
            if character.is_showable(client_id) {
                let s = character.size();
                let model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_scale(s)
                    .with_x_axis_rotation(camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                    .with_z_axis_rotation(camera.z_axis_rotation())
                    .with_movement(&character.position())
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);

                let s = mvp_matrix.dot(&arr1(&[0.0, 0.0, 0.0, 1.0]));
                let key = OrderedFloat(-s[2] / s[3]);
                let value = (mvp_matrix, character);
                if let Some(v) = z_index.get_mut(&key) {
                    v.push(value);
                } else {
                    z_index.insert(key, vec![value]);
                }
            }
        }

        self.character_program.use_program(gl);
        gl.set_attribute(
            &self.vertexis_buffer,
            &self.character_program.a_vertex_location,
            3,
            0,
        );
        gl.set_attribute(
            &self.texture_coord_buffer,
            &self.character_program.a_texture_coord_location,
            2,
            0,
        );
        gl.uniform1i(Some(&self.character_program.u_texture_location), 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (_, character_list) in z_index {
            for (mvp_matrix, character) in character_list {
                let texture_id = character.texture_id();
                if let Some(texture_id) = texture_id {
                    if let (
                        None,
                        Some(Data::Image {
                            element: texture_data,
                            ..
                        }),
                    ) = (textures.get(texture_id), resource.get(texture_id))
                    {
                        textures.insert(gl, texture_id.clone(), texture_data);
                    }
                    if let Some(texture) = textures.get(&texture_id) {
                        let mvp_matrix = mvp_matrix.t();
                        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&texture));
                        gl.uniform_matrix4fv_with_f32_array(
                            Some(&self.character_program.u_translate_location),
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
                            Some(&self.character_program.u_bg_color_location),
                            &character.background_color().to_f32array(),
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
        }
    }
}
