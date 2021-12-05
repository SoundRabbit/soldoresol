use super::libs::matrix::camera::CameraMatrix;
use super::libs::matrix::model::ModelMatrix;
use super::libs::tex_table::TexTable;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::{block, BlockRef};
use ndarray::Array2;

pub struct Nameplate {
    vertex_buffer: WebGlF32Vbo,
    v_color_buffer: WebGlF32Vbo,
    id_buffer: WebGlF32Vbo,
    normal_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
}

impl Nameplate {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.0, 1.0],
                [-0.5, 0.0, 1.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
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
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
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

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        camera_position: &[f32; 3],
        camera_matrix: &CameraMatrix,
        boxblocks: impl Iterator<Item = BlockRef<block::Boxblock>>,
        characters: impl Iterator<Item = BlockRef<block::Character>>,
        is_2d_mode: bool,
        tex_table: &mut TexTable,
    ) {
        gl.use_program(ProgramType::UnshapedProgram);
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
        gl.set_u_camera_position(camera_position);
        gl.set_u_vp_matrix(vp_matrix.clone().reversed_axes());
        gl.set_u_bg_color_1(program::COLOR_NONE);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_texture_0(program::TEXTURE_TEXT);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_perspective(if is_2d_mode {
            program::PERSPECTIVE_PROJECTION
        } else {
            program::PERSPECTIVE_NORMAL
        });
        gl.set_u_id(program::ID_NONE);
        gl.set_u_light(program::LIGHT_NONE);
        gl.set_u_shape(program::SHAPE_2D_BOX);

        for boxblock in boxblocks {
            boxblock.map(|boxblock| {
                if !boxblock.display_name().0.is_empty() || !boxblock.display_name().1.is_empty() {
                    Self::render_plate(
                        gl,
                        vp_matrix,
                        camera_matrix,
                        tex_table,
                        boxblock.display_name(),
                        &boxblock.color().to_color(),
                        boxblock.position(),
                        boxblock.size()[2] as f32 * 0.5,
                    );
                }
            });
        }

        for character in characters {
            character.map(|character| {
                if !character.display_name().0.is_empty() || !character.display_name().1.is_empty()
                {
                    Self::render_plate(
                        gl,
                        vp_matrix,
                        camera_matrix,
                        tex_table,
                        character.display_name(),
                        &character.color().to_color(),
                        character.position(),
                        (character.size() * character.tex_size()) as f32,
                    );
                }
            });
        }
    }

    fn render_plate(
        gl: &WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        camera_matrix: &CameraMatrix,
        tex_table: &mut TexTable,
        name: &(String, String),
        color: &crate::libs::color::Color,
        position: &[f64; 3],
        offset: f32,
    ) {
        if let Some((tex_idx, s)) = tex_table.use_string(gl, name) {
            gl.set_u_texture_0_text_fill_color(&color.to_f32array());
            if color.v() > 0.95 {
                gl.set_u_texture_0_text_stroke_color(
                    &crate::libs::color::Pallet::gray(9).to_color().to_f32array(),
                );
            } else {
                gl.set_u_texture_0_text_stroke_color(
                    &crate::libs::color::Pallet::gray(0).to_color().to_f32array(),
                );
            }

            let s = [(s[0] / s[1] * 0.4) as f32, 0.0, 0.4];
            let p = [
                position[0] as f32,
                position[1] as f32,
                position[2] as f32 + offset,
            ];

            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(&s)
                .with_x_axis_rotation(camera_matrix.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                .with_z_axis_rotation(camera_matrix.z_axis_rotation())
                .with_movement(&p)
                .into();
            let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                .with_movement(&[-p[0], -p[1], -p[2]])
                .with_z_axis_rotation(-camera_matrix.z_axis_rotation())
                .with_x_axis_rotation(
                    -(camera_matrix.x_axis_rotation() - std::f32::consts::FRAC_PI_2),
                )
                .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
                .into();

            let mvp_matrix = vp_matrix.dot(&model_matrix);

            gl.set_u_translate(mvp_matrix.reversed_axes());

            gl.set_u_model_matrix(model_matrix.reversed_axes());
            gl.set_u_inv_model_matrix(inv_model_matrix.reversed_axes());
            gl.set_u_texture_0_sampler(tex_idx);
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}
