use super::libs::id_table::{IdColor, IdTable, IdTableBuilder, ObjectId, Surface};
use super::libs::matrix::model::ModelMatrix;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::{block, BlockRef};
use crate::libs::random_id::U128Id;
use ndarray::Array2;

pub enum RenderingMode<'a> {
    IdMap { grabbed: &'a U128Id },
    View,
}

pub struct CharacterBase {
    vertex_buffer: WebGlF32Vbo,
    v_color_buffer: WebGlF32Vbo,
    id_buffer: WebGlF32Vbo,
    normal_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
}

impl CharacterBase {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );

        let id_buffer = gl.create_vbo_with_f32array(&[0.0, 0.0, 0.0, 0.0]);

        let v_color_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
            .concat(),
        );

        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());

        let normal_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ]
            .concat(),
        );

        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        Self {
            vertex_buffer,
            v_color_buffer,
            id_buffer,
            index_buffer,
            texture_coord_buffer,
            normal_buffer,
        }
    }

    pub fn update_id(
        &self,
        builder: &mut IdTableBuilder,
        characters: impl Iterator<Item = BlockRef<block::Character>>,
    ) {
        for character in characters {
            let block_id = character.id();
            character.map(|character| {
                let surface = Surface {
                    p: character.position().clone(),
                    r: [0.0, 0.0, 0.0],
                    s: [1.0, 0.0, 0.0],
                    t: [0.0, 1.0, 0.0],
                };
                builder.insert(
                    &block_id,
                    IdColor::from(2),
                    ObjectId::Character(U128Id::clone(&block_id), surface),
                );
            });
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &IdTable,
        vp_matrix: &Array2<f32>,
        camera_position: &[f32; 3],
        characters: impl Iterator<Item = BlockRef<block::Character>>,
        rendering_mode: &RenderingMode,
        is_2d_mode: bool,
    ) {
        gl.use_program(ProgramType::ShapedProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.set_a_vertex(&self.vertex_buffer, 3, 0);
        gl.set_a_texture_coord(&self.texture_coord_buffer, 2, 0);
        gl.set_a_id(&self.id_buffer, 1, 0);
        gl.set_a_v_color(&self.v_color_buffer, 4, 0);
        gl.set_a_normal(&self.normal_buffer, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        gl.set_u_expand(0.0);
        gl.set_u_v_color_mask(program::V_COLOR_MASK_NONE);
        gl.set_u_camera_position(camera_position);
        gl.set_u_vp_matrix(vp_matrix.clone().reversed_axes());
        gl.set_u_bg_color_1(program::COLOR_SOME);
        gl.set_u_bg_color_1_value(
            &crate::libs::color::Pallet::gray(9)
                .a(75)
                .to_color()
                .to_f32array(),
        );
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_perspective(if is_2d_mode {
            program::PERSPECTIVE_PROJECTION
        } else {
            program::PERSPECTIVE_NORMAL
        });
        gl.set_u_light(program::LIGHT_NONE);

        match rendering_mode {
            RenderingMode::IdMap { .. } => {
                gl.set_u_shape(program::SHAPE_2D_CIRCLE);
                gl.set_u_id(program::ID_V_WRITE);
            }
            RenderingMode::View => {
                gl.set_u_shape(program::SHAPE_2D_RING);
                gl.set_u_shape_line_width(0.05);
                gl.set_u_id(program::ID_V_READ);
            }
        }

        for character in characters {
            let character_id = character.id();
            character.map(|character| {
                if let RenderingMode::IdMap { grabbed } = rendering_mode {
                    if character_id == **grabbed {
                        return;
                    }
                }

                let id_offset_color = unwrap!(id_table.offset_color(&character_id));
                let s = character.size();
                let s = [s as f32, s as f32, 0.0];
                let p = character.position();
                let p = [p[0] as f32, p[1] as f32, p[2] as f32 + 1.0 / 128.0];

                let model_matrix: Array2<f32> =
                    ModelMatrix::new().with_scale(&s).with_movement(&p).into();

                let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_movement(&[-p[0], -p[1], -p[2]])
                    .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
                    .into();

                let mvp_matrix = vp_matrix.dot(&model_matrix);

                if let RenderingMode::View = rendering_mode {
                    gl.set_u_shape_scale(&[s[0], s[1], 0.0]);
                }
                gl.set_u_translate(mvp_matrix.reversed_axes());
                gl.set_u_model_matrix(model_matrix.reversed_axes());
                gl.set_u_inv_model_matrix(inv_model_matrix.reversed_axes());
                gl.set_u_id_value(id_offset_color.value() as i32);
                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::TRIANGLES,
                    6,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            });
        }
    }
}