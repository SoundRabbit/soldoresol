use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::tex_table::TexTable;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use ndarray::{arr1, Array2};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct Character {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl Character {
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

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        tex_table: &mut TexTable,
        camera: &CameraMatrix,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        resource_arena: &resource::Arena,
        character_ids: impl Iterator<Item = BlockId>,
    ) {
        let mut z_index: BTreeMap<
            OrderedFloat<f32>,
            BTreeMap<OrderedFloat<f32>, Vec<block::character::Character>>,
        > = BTreeMap::new();

        let _ = block_arena.iter_map_with_ids(
            character_ids,
            |_, character: &block::character::Character| {
                let model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_movement(&character.position())
                    .into();
                let s = model_matrix.dot(&arr1(&[0.0, 0.0, 0.0, 1.0]));
                let s = vp_matrix.dot(&s);
                let z_key = OrderedFloat(-s[2] / s[3]);
                let y_key = OrderedFloat(-s[1] / s[3]);
                if let Some(y_index) = z_index.get_mut(&z_key) {
                    if let Some(v) = y_index.get_mut(&y_key) {
                        v.push(block::character::Character::clone(character));
                    } else {
                        y_index.insert(y_key, vec![block::character::Character::clone(character)]);
                    }
                } else {
                    let mut y_index = BTreeMap::new();
                    y_index.insert(y_key, vec![block::character::Character::clone(character)]);
                    z_index.insert(z_key, y_index);
                }
            },
        );

        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::CharacterProgram);
        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);
        gl.set_attr_tex_coord(&self.texture_coord_buffer, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (_, y_index) in z_index {
            for (_, character_list) in y_index {
                for character in character_list {
                    let tex_id = character.current_tex_id();
                    if let Some(tex_id) = tex_id {
                        let tex_idx = tex_table.use_resource(gl, resource_arena, tex_id);
                        if let Some((tex_idx, tex)) = join_some!(
                            tex_idx,
                            resource_arena.get_as::<resource::ImageData>(tex_id)
                        ) {
                            let tex_height = character.current_tex_height();
                            let tex_size = tex.size();
                            let model_matrix: Array2<f32> = ModelMatrix::new()
                                .with_scale(&[
                                    tex_height * tex_size[0] / tex_size[1],
                                    1.0,
                                    tex_height,
                                ])
                                .with_x_axis_rotation(
                                    camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2,
                                )
                                .with_z_axis_rotation(camera.z_axis_rotation())
                                .with_movement(&character.position())
                                .into();
                            let mvp_matrix = vp_matrix.dot(&model_matrix);

                            gl.set_unif_texture(tex_idx);
                            gl.set_unif_translate(mvp_matrix.reversed_axes());
                            gl.set_unif_text_color_1(&[0.0, 0.0, 0.0]);
                            gl.set_unif_text_color_2(&[1.0, 1.0, 1.0]);
                            gl.draw_elements_with_i32(
                                web_sys::WebGlRenderingContext::TRIANGLES,
                                6,
                                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                                0,
                            );
                        }
                    }
                    let display_name_tex = tex_table.use_string(gl, character.display_name());
                    let name_tex = tex_table.use_string(gl, character.name());
                    if let (
                        Some((name_tex_idx, name_tex_size)),
                        Some((display_name_tex_idx, display_name_tex_size)),
                    ) = (name_tex, display_name_tex)
                    {
                        let text_color = character.name_color();

                        if text_color.idx >= 6 {
                            let text_color = text_color.to_color().to_f32array();
                            gl.set_unif_text_color_1(&[
                                text_color[0],
                                text_color[1],
                                text_color[2],
                            ]);
                            gl.set_unif_text_color_2(&[1.0, 1.0, 1.0]);
                        } else {
                            let text_color = text_color.to_color().to_f32array();
                            gl.set_unif_text_color_1(&[0.0, 0.0, 0.0]);
                            gl.set_unif_text_color_2(&[
                                text_color[0],
                                text_color[1],
                                text_color[2],
                            ]);
                        }

                        let display_name_width =
                            ((0.5 * display_name_tex_size[0] / display_name_tex_size[1]) as f32)
                                .min(character.size() * 2.0);
                        let display_name_height = display_name_width
                            * (display_name_tex_size[1] / display_name_tex_size[0]) as f32;
                        let model_matrix: Array2<f32> = ModelMatrix::new()
                            .with_scale(&[display_name_width, 1.0, display_name_height])
                            .with_movement(&[0.0, 0.0, character.current_tex_height()])
                            .with_x_axis_rotation(
                                camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2,
                            )
                            .with_z_axis_rotation(camera.z_axis_rotation())
                            .with_movement(character.position())
                            .into();
                        let mvp_matrix = vp_matrix.dot(&model_matrix);
                        gl.set_unif_texture(display_name_tex_idx);
                        gl.set_unif_translate(mvp_matrix.reversed_axes());
                        gl.draw_elements_with_i32(
                            web_sys::WebGlRenderingContext::TRIANGLES,
                            6,
                            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                            0,
                        );

                        let name_width = ((0.3 * name_tex_size[0] / name_tex_size[1]) as f32)
                            .min(character.size() * 2.0);
                        let name_height = name_width * (name_tex_size[1] / name_tex_size[0]) as f32;
                        let model_matrix: Array2<f32> = ModelMatrix::new()
                            .with_scale(&[name_width, 1.0, name_height])
                            .with_movement(&[
                                0.0,
                                0.0,
                                character.current_tex_height() + display_name_height,
                            ])
                            .with_x_axis_rotation(
                                camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2,
                            )
                            .with_z_axis_rotation(camera.z_axis_rotation())
                            .with_movement(character.position())
                            .into();
                        let mvp_matrix = vp_matrix.dot(&model_matrix);
                        gl.set_unif_texture(name_tex_idx);
                        gl.set_unif_translate(mvp_matrix.reversed_axes());
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
