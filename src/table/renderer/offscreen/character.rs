use super::id_table::{IdColor, IdTable, ObjectId, Surface};
use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use ndarray::Array2;
use std::collections::HashMap;

pub struct Character {
    vertex_buffer_xy: WebGlF32Vbo,
    vertex_buffer_xz: WebGlF32Vbo,
    normal_buffer_xy: WebGlF32Vbo,
    normal_buffer_xz: WebGlF32Vbo,
    v_color_buffer: WebGlF32Vbo,
    id_color_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl Character {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_buffer_xy = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 1.0 / 128.0],
                [-0.5, 0.5, 1.0 / 128.0],
                [0.5, -0.5, 1.0 / 128.0],
                [-0.5, -0.5, 1.0 / 128.0],
            ]
            .concat(),
        );
        let vertex_buffer_xz = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.0, 1.0],
                [-0.5, 0.0, 1.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
            ]
            .concat(),
        );
        let normal_buffer_xy = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ]
            .concat(),
        );
        let normal_buffer_xz = gl.create_vbo_with_f32array(
            &[
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ]
            .concat(),
        );
        let v_color_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
            .concat(),
        );
        let id_color_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        Self {
            vertex_buffer_xy,
            vertex_buffer_xz,
            normal_buffer_xy,
            normal_buffer_xz,
            id_color_buffer,
            v_color_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &mut IdTable,
        id_value: &mut HashMap<BlockId, IdColor>,
        camera: &CameraMatrix,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        resource_arena: &resource::Arena,
        character_ids: impl Iterator<Item = BlockId>,
        grabbed_object_id: &ObjectId,
    ) {
        let characters = block_arena
            .iter_map_with_ids(
                character_ids.filter(|x| !grabbed_object_id.eq(x)),
                |character_id, character: &block::character::Character| {
                    let size = character.size();
                    let pos = character.position().clone();
                    let tex_height = character.current_tex_height();
                    let tex_size = character
                        .current_tex_id()
                        .and_then(|tex_id| resource_arena.get_as::<resource::ImageData>(tex_id))
                        .map(|img| img.size().clone());
                    let id = id_table.len() as u32 | 0xFF000000;
                    id_table.insert(
                        IdColor::from(id),
                        ObjectId::Character(
                            BlockId::clone(&character_id),
                            Surface {
                                r: pos.clone(),
                                s: [1.0, 0.0, 0.0],
                                t: [0.0, 1.0, 0.0],
                            },
                        ),
                    );
                    id_value.insert(BlockId::clone(&character_id), IdColor::from(id));
                    (id as i32, size, pos, tex_height, tex_size)
                },
            )
            .collect::<Vec<_>>();

        gl.use_program(ProgramType::UnshapedProgram);
        gl.set_a_vertex(&self.vertex_buffer_xy, 3, 0);
        gl.set_a_normal(&self.normal_buffer_xy, 3, 0);
        gl.set_a_texture_coord(&self.texture_coord_buffer, 2, 0);
        gl.set_a_id_color(&self.id_color_buffer, 4, 0);
        gl.set_a_v_color(&self.v_color_buffer, 4, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        gl.set_u_shape(program::SHAPE_2D_CIRCLE);
        gl.set_u_bg_color_1(program::COLOR_NONE);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_id(program::ID_U_WRITE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_light(program::LIGHT_NONE);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);

        for (id, size, pos, _, _) in &characters {
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(&[*size, *size, 1.0])
                .with_movement(pos)
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            gl.set_u_translate(mvp_matrix.reversed_axes());
            gl.set_u_id_value(*id);
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }

        gl.set_a_vertex(&self.vertex_buffer_xz, 3, 0);
        gl.set_a_normal(&self.normal_buffer_xz, 3, 0);
        gl.set_u_shape(program::SHAPE_2D_BOX);

        for (id, _, pos, tex_height, tex_size) in &characters {
            if let Some(tex_size) = tex_size.as_ref() {
                let model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_scale(&[*tex_height * tex_size[0] / tex_size[1], 1.0, *tex_height])
                    .with_x_axis_rotation(camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                    .with_z_axis_rotation(camera.z_axis_rotation())
                    .with_movement(pos)
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_u_translate(mvp_matrix.reversed_axes());
                gl.set_u_id_value(*id);
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
