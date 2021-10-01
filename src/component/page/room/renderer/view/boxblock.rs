use super::id_table::{IdColor, IdTable, ObjectId, Surface};
use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::tex_table::TexTable;
use super::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::libs::random_id::U128Id;
use ndarray::Array2;

pub struct Boxblock {
    vertex_buffer: WebGlF32Vbo,
    v_color_buffer: WebGlF32Vbo,
    id_color_buffer: WebGlF32Vbo,
    normal_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
}

impl Boxblock {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_buffer = gl.create_vbo_with_f32array(
            &[
                [
                    [0.5, 0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                ]
                .concat(), //PZ
                [
                    [0.5, 0.5, 0.5],
                    [0.5, 0.5, -0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, 0.5, -0.5],
                ]
                .concat(), // PY
                [
                    [0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                ]
                .concat(), // PX
                [
                    [-0.5, 0.5, 0.5],
                    [-0.5, 0.5, -0.5],
                    [-0.5, -0.5, 0.5],
                    [-0.5, -0.5, -0.5],
                ]
                .concat(), // NX
                [
                    [0.5, -0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, -0.5],
                    [-0.5, -0.5, -0.5],
                ]
                .concat(), // NY
                [
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                    [-0.5, -0.5, -0.5],
                ]
                .concat(), // NZ,
            ]
            .concat(),
        );
        let id_color_buffer = gl.create_vbo_with_f32array(
            &[
                IdColor::from(0).to_f32array(),
                IdColor::from(0).to_f32array(),
                IdColor::from(0).to_f32array(),
                IdColor::from(0).to_f32array(),
                IdColor::from(1).to_f32array(),
                IdColor::from(1).to_f32array(),
                IdColor::from(1).to_f32array(),
                IdColor::from(1).to_f32array(),
                IdColor::from(2).to_f32array(),
                IdColor::from(2).to_f32array(),
                IdColor::from(2).to_f32array(),
                IdColor::from(2).to_f32array(),
                IdColor::from(3).to_f32array(),
                IdColor::from(3).to_f32array(),
                IdColor::from(3).to_f32array(),
                IdColor::from(3).to_f32array(),
                IdColor::from(4).to_f32array(),
                IdColor::from(4).to_f32array(),
                IdColor::from(4).to_f32array(),
                IdColor::from(4).to_f32array(),
                IdColor::from(5).to_f32array(),
                IdColor::from(5).to_f32array(),
                IdColor::from(5).to_f32array(),
                IdColor::from(5).to_f32array(),
            ]
            .concat(),
        );
        let v_color_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
            .concat(),
        );
        let normal_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ]
            .concat(),
        );
        let texture_coord_buffer = gl.create_vbo_with_f32array(
            &[
                [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]].concat(), //PZ
                [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]].concat(), // PY
                [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]].concat(), // PX
                [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]].concat(), // NX
                [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]].concat(), // NY
                [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]].concat(), // NZ,
            ]
            .concat(),
        );
        let index_buffer = gl.create_ibo_with_i16array(
            &[
                [0, 1, 2, 3, 2, 1],
                [4, 5, 6, 7, 6, 5],
                [8, 9, 10, 11, 10, 9],
                [12, 13, 14, 15, 14, 13],
                [16, 17, 18, 19, 18, 17],
                [20, 21, 22, 23, 22, 21],
            ]
            .concat(),
        );

        Self {
            vertex_buffer,
            v_color_buffer,
            id_color_buffer,
            index_buffer,
            texture_coord_buffer,
            normal_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_value: &mut HashMap<BlockId, IdColor>,
        camera: &CameraMatrix,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        boxblock_ids: impl Iterator<Item = BlockId>,
        light: &[f32; 3],
        light_color: &crate::libs::color::Pallet,
        light_intensity: f32,
        mut tex_table: Option<&mut TexTable>,
        shadowmap: Option<&[(web_sys::WebGlTexture, U128Id); 6]>,
        light_vps: Option<&[Array2<f32>; 6]>,
        light_attenation: Option<f32>,
    ) {
        gl.use_program(ProgramType::ShapedProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.set_a_vertex(&self.vertex_buffer, 3, 0);
        gl.set_a_texture_coord(&self.texture_coord_buffer, 2, 0);
        gl.set_a_id_color(&self.id_color_buffer, 4, 0);
        gl.set_a_v_color(&self.v_color_buffer, 4, 0);
        gl.set_a_normal(&self.normal_buffer, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_id(program::ID_V_READ);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);

        if let (Some(light_vps), Some(light_attenation)) = (light_vps, light_attenation) {
            gl.set_u_light_vp_px(light_vps[0].clone().reversed_axes());
            gl.set_u_light_vp_py(light_vps[1].clone().reversed_axes());
            gl.set_u_light_vp_pz(light_vps[2].clone().reversed_axes());
            gl.set_u_light_vp_nx(light_vps[3].clone().reversed_axes());
            gl.set_u_light_vp_ny(light_vps[4].clone().reversed_axes());
            gl.set_u_light_vp_nz(light_vps[5].clone().reversed_axes());
            gl.set_u_light(program::LIGHT_POINT_WITH_ID);
            gl.set_u_light_attenation(light_attenation);
            gl.set_u_shade_intensity(1.0);
        } else {
            gl.set_u_light(program::LIGHT_AMBIENT);
            gl.set_u_light_attenation(1.0);
            gl.set_u_shade_intensity(0.5);
        }

        gl.set_u_camera_position(&camera.position());
        gl.set_u_light_position(light);
        gl.set_u_light_color(&light_color.to_color().to_f32array());
        gl.set_u_light_intensity(light_intensity);

        if let (Some(tex_table), Some(shadowmap)) = (tex_table.as_mut(), shadowmap.as_ref()) {
            for i in 0..6 {
                let (tex_idx, tex_flag) = tex_table.use_custom(&shadowmap[i].1);
                gl.active_texture(tex_flag);
                gl.bind_texture(
                    web_sys::WebGlRenderingContext::TEXTURE_2D,
                    Some(&shadowmap[i].0),
                );
                match i {
                    0 => {
                        gl.set_u_light_map_px(tex_idx);
                    }
                    1 => {
                        gl.set_u_light_map_py(tex_idx);
                    }
                    2 => {
                        gl.set_u_light_map_pz(tex_idx);
                    }
                    3 => {
                        gl.set_u_light_map_nx(tex_idx);
                    }
                    4 => {
                        gl.set_u_light_map_ny(tex_idx);
                    }
                    5 => {
                        gl.set_u_light_map_nz(tex_idx);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        gl.set_u_vp_matrix(vp_matrix.clone().reversed_axes());

        let _ = block_arena.iter_map_with_ids(
            boxblock_ids,
            |_, boxblock: &block::boxblock::Boxblock| {
                let s = {
                    let s = boxblock.size();
                    [
                        s[0].abs().max(1.0 / 128.0).copysign(s[0]),
                        s[1].abs().max(1.0 / 128.0).copysign(s[1]),
                        s[2].abs().max(1.0 / 128.0).copysign(s[2]),
                    ]
                };
                let p = boxblock.position();
                let model_matrix: Array2<f32> =
                    ModelMatrix::new().with_scale(&s).with_movement(p).into();
                let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_movement(&[-p[0], -p[1], -p[2]])
                    .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);

                gl.set_unif_model(model_matrix.reversed_axes());
                gl.set_unif_inv_model(inv_model_matrix.reversed_axes());
                gl.set_unif_translate(mvp_matrix.reversed_axes());
                gl.set_unif_bg_color(&boxblock.color().to_color().to_f32array());
                gl.set_unif_shape(boxblock.shape().as_num());

                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::TRIANGLES,
                    36,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            },
        );
    }

    fn n(x: f32, y: f32, z: f32) -> [f32; 3] {
        let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        [x / len, y / len, z / len]
    }
}
