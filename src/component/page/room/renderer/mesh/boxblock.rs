use super::libs::id_table::{IdColor, IdTable, IdTableBuilder, ObjectId, Surface};
use super::libs::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::libs::tex_table::TexTable;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::libs::random_id::U128Id;
use ndarray::Array2;

pub enum LightingMode<'a> {
    PointLight {
        position: &'a [f32; 3],
        id_map: &'a [(web_sys::WebGlTexture, U128Id, Array2<f32>); 6],
        light_attenation: f32,
    },
    AmbientLight {
        direction: &'a [f32; 3],
    },
}

pub enum RenderingMode<'a> {
    IdMap {
        grabbed: &'a ObjectId,
    },
    View {
        lighting: LightingMode<'a>,
        light_color: &'a crate::libs::color::Pallet,
        light_intensity: f32,
        camera: &'a CameraMatrix,
    },
}

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

        //テクスチャ座標用のメモ
        //[0.00,1.00][0.25,1.00][0.50,1.00][0.75,1.00][1.00,1.00]
        //[0.00,0.70][0.25,0.70][0.50,0.70][0.75,0.70][1.00,0.70]
        //[0.00,0.65][0.25,0.65][0.50,0.65][0.75,0.65][1.00,0.65]
        //[0.00,0.45][0.25,0.45][0.50,0.45][0.75,0.45][1.00,0.45]
        //[0.00,0.30][0.25,0.30][0.50,0.30][0.75,0.30][1.00,0.30]
        //[0.00,0.00][0.25,0.00][0.50,0.00][0.75,0.00][1.00,0.00]

        //PZ
        //PX PY NX NY
        //NZ

        let texture_coord_buffer = gl.create_vbo_with_f32array(
            &[
                [[0.00, 1.00], [0.25, 1.00], [0.00, 0.70], [0.25, 0.70]].concat(), //PZ
                [[0.25, 0.65], [0.50, 0.65], [0.25, 0.45], [0.50, 0.45]].concat(), // PY
                [[0.00, 0.65], [0.25, 0.65], [0.00, 0.45], [0.25, 0.45]].concat(), // PX
                [[0.50, 0.65], [0.75, 0.65], [0.50, 0.45], [0.75, 0.45]].concat(), // NX
                [[0.75, 0.65], [1.00, 0.65], [0.75, 0.45], [1.00, 0.45]].concat(), // NY
                [[0.00, 0.30], [0.25, 0.30], [0.00, 0.00], [0.25, 0.00]].concat(), // NZ,
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
        let index_buffer = gl.create_ibo_with_i16array(
            &[
                [0, 1, 2, 3, 2, 1],       //PZ
                [4, 5, 6, 7, 6, 5],       //PY
                [8, 9, 10, 11, 10, 9],    //PX
                [12, 13, 14, 15, 14, 13], //NX
                [16, 17, 18, 19, 18, 17], //NY
                [20, 21, 22, 23, 22, 21], //NZ
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

    pub fn update_id<'a>(
        &self,
        builder: &mut IdTableBuilder,
        block_arena: &block::Arena,
        boxblock_ids: impl Iterator<Item = &'a BlockId>,
    ) {
        block_arena.iter_map_with_ids(
            boxblock_ids,
            |boxblock_id, boxblock: &block::boxblock::Boxblock| {
                for srfs in 0..6 {
                    builder.insert(
                        boxblock_id,
                        IdColor::from(srfs),
                        ObjectId::Boxblock(
                            BlockId::clone(&boxblock_id),
                            Self::surface_of(boxblock, srfs),
                        ),
                    );
                }
            },
        );
    }

    pub fn render<'a>(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &IdTable,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        boxblock_ids: impl Iterator<Item = &'a BlockId>,
        rendering_mode: &RenderingMode,
        tex_table: &'a mut TexTable,
    ) {
        gl.use_program(match rendering_mode {
            RenderingMode::IdMap { .. } => ProgramType::UnshapedProgram,
            RenderingMode::View { .. } => ProgramType::ShapedProgram,
        });
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
        gl.set_u_shape(program::SHAPE_3D_BOX);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_vp_matrix(vp_matrix.clone().reversed_axes());

        match rendering_mode {
            RenderingMode::IdMap { .. } => {
                gl.set_u_bg_color_1(program::COLOR_NONE);
                gl.set_u_id(program::ID_V_WRITE);
                gl.set_u_light(program::LIGHT_NONE);
            }
            RenderingMode::View {
                lighting,
                light_intensity,
                light_color,
                camera,
            } => {
                gl.set_u_id(program::ID_V_READ);
                gl.set_u_camera_position(&camera.position());
                gl.set_u_light_color(&light_color.to_color().to_f32array());
                gl.set_u_light_intensity(*light_intensity);
                match lighting {
                    LightingMode::AmbientLight { direction } => {
                        gl.set_u_light(program::LIGHT_AMBIENT);
                        gl.set_u_light_position(*direction);
                        gl.set_u_light_attenation(1.0);
                    }
                    LightingMode::PointLight {
                        position,
                        id_map,
                        light_attenation,
                    } => {
                        gl.set_u_light_vp_px(id_map[0].2.clone().reversed_axes());
                        gl.set_u_light_vp_py(id_map[1].2.clone().reversed_axes());
                        gl.set_u_light_vp_pz(id_map[2].2.clone().reversed_axes());
                        gl.set_u_light_vp_nx(id_map[3].2.clone().reversed_axes());
                        gl.set_u_light_vp_ny(id_map[4].2.clone().reversed_axes());
                        gl.set_u_light_vp_nz(id_map[5].2.clone().reversed_axes());
                        gl.set_u_light(program::LIGHT_POINT_WITH_ID);
                        gl.set_u_light_position(*position);
                        gl.set_u_light_attenation(*light_attenation);

                        let set_tex = [
                            WebGlRenderingContext::set_u_light_map_px,
                            WebGlRenderingContext::set_u_light_map_py,
                            WebGlRenderingContext::set_u_light_map_pz,
                            WebGlRenderingContext::set_u_light_map_nx,
                            WebGlRenderingContext::set_u_light_map_ny,
                            WebGlRenderingContext::set_u_light_map_nz,
                        ];

                        for i in 0..6 {
                            let (tex_idx, tex_flag) = tex_table.use_custom(&id_map[i].1);
                            gl.active_texture(tex_flag);
                            gl.bind_texture(
                                web_sys::WebGlRenderingContext::TEXTURE_2D,
                                Some(&id_map[i].0),
                            );
                            set_tex[i](gl, tex_idx);
                        }
                    }
                }
            }
        }

        let _ = block_arena.iter_map_with_ids(
            boxblock_ids,
            |boxblock_id, boxblock: &block::boxblock::Boxblock| {
                if let RenderingMode::IdMap { grabbed } = rendering_mode {
                    if grabbed.eq(&boxblock_id) {
                        return;
                    }
                }

                let id_offset_color = if let Some(x) = id_table.offset_color(boxblock_id) {
                    x
                } else {
                    return;
                };

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
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_u_translate(mvp_matrix.reversed_axes());
                gl.set_u_id_value(id_offset_color.value() as i32);
                gl.set_u_shape(boxblock.shape().as_num());

                if let RenderingMode::View { .. } = rendering_mode {
                    let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                        .with_movement(&[-p[0], -p[1], -p[2]])
                        .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
                        .into();
                    gl.set_u_model_matrix(model_matrix.reversed_axes());
                    gl.set_u_inv_model_matrix(inv_model_matrix.reversed_axes());
                    gl.set_u_bg_color_1(program::COLOR_SOME);
                    gl.set_u_bg_color_1_value(&boxblock.color().to_color().to_f32array());
                }

                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::TRIANGLES,
                    36,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            },
        );
    }

    fn surface_of(boxblock: &block::boxblock::Boxblock, idx: u32) -> Surface {
        let p = boxblock.position();
        let s = boxblock.size();

        match idx % 6 {
            0 => Surface {
                //PZ
                r: [p[0], p[1], p[2] + s[2] * 0.5],
                s: [1.0, 0.0, 0.0],
                t: [0.0, 1.0, 0.0],
            },
            1 => Surface {
                //PY
                r: [p[0], p[1] + s[1] * 0.5, p[2]],
                s: [0.0, 0.0, 1.0],
                t: [1.0, 0.0, 0.0],
            },
            2 => Surface {
                //PX
                r: [p[0] + s[0] * 0.5, p[1], p[2]],
                s: [0.0, 1.0, 0.0],
                t: [0.0, 0.0, 1.0],
            },
            3 => Surface {
                //NX
                r: [p[0] - s[0] * 0.5, p[1], p[2]],
                s: [0.0, 0.0, 1.0],
                t: [0.0, 1.0, 0.0],
            },
            4 => Surface {
                //NY
                r: [p[0], p[1] - s[1] * 0.5, p[2]],
                s: [1.0, 0.0, 0.0],
                t: [0.0, 0.0, 1.0],
            },
            5 => Surface {
                //NZ
                r: [p[0], p[1], p[2] - s[2] * 0.5],
                s: [0.0, 1.0, 0.0],
                t: [1.0, 0.0, 0.0],
            },
            _ => unreachable!(),
        }
    }

    fn n(x: f32, y: f32, z: f32) -> [f32; 3] {
        let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        [x / len, y / len, z / len]
    }
}
