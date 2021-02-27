use super::id_table::{IdTable, ObjectId};
use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use ndarray::Array2;
use ordered_float::OrderedFloat;

pub struct Character {
    vertexes_buffer_xy: WebGlF32Vbo,
    vertexes_buffer_xz: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl Character {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexes_buffer_xy = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 1.0 / 128.0],
                [-0.5, 0.5, 1.0 / 128.0],
                [0.5, -0.5, 1.0 / 128.0],
                [-0.5, -0.5, 1.0 / 128.0],
            ]
            .concat(),
        );
        let vertexes_buffer_xz = gl.create_vbo_with_f32array(
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
            vertexes_buffer_xy,
            vertexes_buffer_xz,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &mut IdTable,
        camera: &CameraMatrix,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        resource_arena: &resource::Arena,
        character_ids: impl Iterator<Item = BlockId>,
    ) {
        let characters = block_arena
            .iter_map_with_ids(
                character_ids,
                |character_id, character: &block::character::Character| {
                    let size = character.size();
                    let pos = character.position().clone();
                    let tex_scale = character.current_tex_scale();
                    let tex_size = character
                        .current_tex_id()
                        .and_then(|tex_id| resource_arena.get_as::<resource::ImageData>(tex_id))
                        .map(|img| img.size().clone());
                    let id = id_table.len() as u32 | 0xFF000000;
                    id_table.insert(id, ObjectId::Character(BlockId::clone(&character_id)));
                    let color = crate::libs::color::Color::from(id).to_f32array();
                    (color, size, pos, tex_scale, tex_size)
                },
            )
            .collect::<Vec<_>>();

        gl.use_program(ProgramType::OffscreenProgram);
        gl.set_attr_vertex(&self.vertexes_buffer_xy, 3, 0);
        gl.set_attr_tex_coord(&self.texture_coord_buffer, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        gl.set_unif_flag_round(1);

        for (color, size, pos, _, _) in &characters {
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(&[*size, *size, 1.0])
                .with_movement(pos)
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            gl.set_unif_translate(mvp_matrix.reversed_axes());
            gl.set_unif_bg_color(color);
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }

        gl.set_attr_vertex(&self.vertexes_buffer_xz, 3, 0);
        gl.set_unif_flag_round(0);

        for (color, size, pos, tex_scale, tex_size) in &characters {
            if let Some(tex_size) = tex_size.as_ref() {
                let size = (*size) * tex_scale;
                let model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_scale(&[size, 1.0, size * tex_size[1] / tex_size[0]])
                    .with_x_axis_rotation(camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                    .with_z_axis_rotation(camera.z_axis_rotation())
                    .with_movement(pos)
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_unif_translate(mvp_matrix.reversed_axes());
                gl.set_unif_bg_color(color);
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
