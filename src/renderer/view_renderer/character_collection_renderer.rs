use super::super::{
    program::{CharacterProgram, MaskProgram},
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use super::Camera;
use crate::{
    block::{self, BlockId},
    resource::Data,
    Color, Resource,
};
use ndarray::{arr1, Array2};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct CharacterCollectionRenderer {
    img_vertexis_buffer: WebGlF32Vbo,
    img_texture_coord_buffer: WebGlF32Vbo,
    img_index_buffer: WebGlI16Ibo,
    mask_vertexis_buffer: WebGlF32Vbo,
    mask_texture_coord_buffer: WebGlF32Vbo,
    mask_index_buffer: WebGlI16Ibo,
    character_program: CharacterProgram,
    mask_program: MaskProgram,
}

impl CharacterCollectionRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let img_vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 1.0, 0.0],
                [-0.5, 1.0, 0.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
            ]
            .concat(),
        );
        let img_texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());
        let img_index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        let mask_vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );
        let mask_texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let mask_index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        let character_program = CharacterProgram::new(gl);
        let mask_program = MaskProgram::new(gl);

        Self {
            img_vertexis_buffer,
            img_texture_coord_buffer,
            img_index_buffer,
            mask_vertexis_buffer,
            mask_texture_coord_buffer,
            mask_index_buffer,
            character_program,
            mask_program,
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
    ) {
        self.mask_program.use_program(gl);
        gl.set_attribute(
            &self.mask_vertexis_buffer,
            &self.mask_program.a_vertex_location,
            3,
            0,
        );
        gl.set_attribute(
            &self.mask_texture_coord_buffer,
            &self.mask_program.a_texture_coord_location,
            2,
            0,
        );
        gl.uniform1i(Some(&self.mask_program.u_flag_round_location), 1);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.mask_index_buffer),
        );

        let mut z_index: BTreeMap<OrderedFloat<f32>, Vec<(Array2<f32>, &block::Character)>> =
            BTreeMap::new();
        for (_, character) in block_field.listed::<block::Character>(characters.collect()) {
            let s = character.size();
            let p = character.position();
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(&[s[0], s[0], 1.0])
                .with_movement(&p)
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
                &Color::from([0.0, 0.0, 0.0, 0.75]).to_f32array(),
            );
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );

            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_x_axis_rotation(camera.x_axis_rotation())
                .with_z_axis_rotation(camera.z_axis_rotation())
                .with_movement(&p)
                .into();
            let mvp_matrix = model_matrix.dot(vp_matrix);

            let s = mvp_matrix.t().dot(&arr1(&[0.0, 0.0, 0.0, 1.0]));
            let key = OrderedFloat(-s[2] / s[3]);
            let value = (mvp_matrix, character);
            if let Some(v) = z_index.get_mut(&key) {
                v.push(value);
            } else {
                z_index.insert(key, vec![value]);
            }
        }

        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);

        self.character_program.use_program(gl);
        gl.set_attribute(
            &self.img_vertexis_buffer,
            &self.character_program.a_vertex_location,
            3,
            0,
        );
        gl.set_attribute(
            &self.img_texture_coord_buffer,
            &self.character_program.a_texture_coord_location,
            2,
            0,
        );
        gl.uniform1i(Some(&self.character_program.u_texture_location), 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.img_index_buffer),
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
                        textures.insert(gl, *texture_id, texture_data);
                    }
                    if let Some(texture) = textures.get(&texture_id) {
                        let s = character.size();
                        let model_matrix: Array2<f32> =
                            ModelMatrix::new().with_scale(&[s[0], s[1], 1.0]).into();
                        let mvp_matrix = model_matrix.dot(&mvp_matrix);

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
