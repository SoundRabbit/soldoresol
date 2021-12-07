use super::libs::id_table::{IdColor, IdTable, IdTableBuilder, ObjectId, Surface};
use super::libs::matrix::model::ModelMatrix;
use super::libs::tex_table::TexTable;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::{block, BlockRef};
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
    },
}

pub struct Boxblock {
    vertex_buffer: WebGlF32Vbo,
    v_color_buffer: WebGlF32Vbo,
    id_buffer: WebGlF32Vbo,
    normal_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
}

impl Boxblock {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_buffer = gl.create_vbo_with_f32array(
            &[
                [
                    //PZ
                    [0.5, 0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                ]
                .concat(),
                [
                    // PY
                    [0.5, 0.5, 0.5],
                    [0.5, 0.5, -0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, 0.5, -0.5],
                ]
                .concat(),
                [
                    // PX
                    [0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                ]
                .concat(),
                [
                    // NX
                    [-0.5, -0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, -0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                ]
                .concat(),
                [
                    // NY
                    [0.5, -0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, -0.5],
                    [-0.5, -0.5, -0.5],
                ]
                .concat(),
                [
                    // NZ
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                    [-0.5, -0.5, -0.5],
                ]
                .concat(),
                [
                    // 斜面（上半分）
                    [-0.5, 0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                    [0.0, 0.5, 0.0],
                    [0.0, -0.5, 0.0],
                ]
                .concat(),
                [
                    // 斜面（下半分）
                    [0.0, 0.5, 0.0],
                    [0.0, -0.5, 0.0],
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                ]
                .concat(),
            ]
            .concat(),
        );
        let id_buffer = gl.create_vbo_with_f32array(&[
            0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 4.0, 4.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 8.0,
            8.0, 8.0, 8.0, 10.0, 10.0, 10.0, 10.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0,
        ]);
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
        //[0.00,0.00][0.25,0.00][0.50,0.00][0.75,0.00][1.00,0.00]
        //[0.00,0.30][0.25,0.30][0.50,0.30][0.75,0.30][1.00,0.30]
        //[0.00,0.35][0.25,0.35][0.50,0.35][0.75,0.35][1.00,0.35]
        //[0.00,0.65][0.25,0.65][0.50,0.65][0.75,0.65][1.00,0.65]
        //[0.00,0.70][0.25,0.70][0.50,0.70][0.75,0.70][1.00,0.70]
        //[0.00,1.00][0.25,1.00][0.50,1.00][0.75,1.00][1.00,1.00]

        //PZ
        //NY PX PY NX
        //NZ

        let texture_coord_buffer = gl.create_vbo_with_f32array(
            &[
                [[0.25, 0.00], [0.00, 0.00], [0.25, 0.30], [0.00, 0.30]].concat(), // PZ,
                [[0.75, 0.35], [0.50, 0.35], [0.75, 0.65], [0.50, 0.65]].concat(), // PY
                [[0.50, 0.35], [0.25, 0.35], [0.50, 0.65], [0.25, 0.65]].concat(), // PX
                [[1.00, 0.35], [0.75, 0.35], [1.00, 0.65], [0.75, 0.65]].concat(), // NX
                [[0.25, 0.35], [0.00, 0.35], [0.25, 0.65], [0.00, 0.65]].concat(), // NY
                [[0.25, 0.70], [0.00, 0.70], [0.25, 1.00], [0.00, 1.00]].concat(), // NZ
                [[0.00, 0.00], [0.00, 0.30], [0.25, 0.00], [0.25, 0.30]].concat(), // 斜面（上半分）
                [[0.50, 0.35], [0.25, 0.35], [0.50, 0.65], [0.25, 0.65]].concat(), // 斜面（下半分）
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
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
                Self::n(&[1.0, 0.0, 1.0]),
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
                [4, 7, 6, 19, 18, 17],    //斜面PY&NY
                [12, 13, 14, 15, 14, 13], //NX
                [20, 21, 22, 23, 22, 21], //NZ
                [24, 25, 26, 27, 26, 25], //斜面（上半分）
                [28, 29, 30, 31, 30, 29], //斜面（下半分）
            ]
            .concat(),
        );

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
        boxblocks: impl Iterator<Item = BlockRef<block::Boxblock>>,
    ) {
        for boxblock in boxblocks {
            let block_id = boxblock.id();
            boxblock.map(|boxblock| {
                for srfs in 0..7 {
                    if let Some(surface) = Self::surface_of(boxblock, srfs) {
                        builder.insert(
                            &block_id,
                            IdColor::from(srfs * 2),
                            ObjectId::Boxblock(U128Id::clone(&block_id), surface),
                        );
                    }
                }
            });
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &IdTable,
        vp_matrix: &Array2<f32>,
        camera_position: &[f32; 3],
        boxblocks: impl Iterator<Item = BlockRef<block::Boxblock>>,
        rendering_mode: &RenderingMode,
        is_2d_mode: bool,
        tex_table: &mut TexTable,
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
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_perspective(if is_2d_mode {
            program::PERSPECTIVE_PROJECTION
        } else {
            program::PERSPECTIVE_NORMAL
        });

        match rendering_mode {
            RenderingMode::IdMap { .. } => {
                gl.set_u_bg_color_1(program::COLOR_NONE);
                gl.set_u_texture_0(program::TEXTURE_NONE);
                gl.set_u_id(program::ID_V_WRITE);
                gl.set_u_light(program::LIGHT_NONE);
            }
            RenderingMode::View {
                lighting,
                light_intensity,
                light_color,
            } => {
                gl.set_u_id(program::ID_V_READ);
                gl.set_u_light_color(&light_color.to_color().to_f32array());
                gl.set_u_light_intensity(*light_intensity);
                gl.set_u_shade_intensity(1.0);
                gl.set_u_bg_color_1(program::COLOR_SOME);
                match lighting {
                    LightingMode::AmbientLight { direction } => {
                        gl.set_u_light(program::LIGHT_AMBIENT);
                        gl.set_u_light_position(*direction);
                        gl.set_u_light_attenation(0.0);
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

        for boxblock in boxblocks {
            let boxblock_id = boxblock.id();
            boxblock.map(|boxblock| {
                if let RenderingMode::IdMap { grabbed } = rendering_mode {
                    if grabbed.is(&boxblock_id) {
                        return;
                    }
                }

                let id_offset_color = unwrap!(id_table.offset_color(&boxblock_id));

                let s = boxblock.size();
                let s = match rendering_mode {
                    RenderingMode::IdMap { .. } => [s[0] as f32, s[1] as f32, s[2] as f32],
                    RenderingMode::View { .. } => [
                        s[0].abs().max(1.0 / 128.0).copysign(s[0]) as f32,
                        s[1].abs().max(1.0 / 128.0).copysign(s[1]) as f32,
                        s[2].abs().max(1.0 / 128.0).copysign(s[2]) as f32,
                    ],
                };

                let p = boxblock.position();
                let p = [p[0] as f32, p[1] as f32, p[2] as f32];

                let shape = boxblock.shape();

                let model_matrix: Array2<f32> =
                    ModelMatrix::new().with_scale(&s).with_movement(&p).into();

                let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_movement(&[-p[0], -p[1], -p[2]])
                    .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
                    .into();

                let mvp_matrix = vp_matrix.dot(&model_matrix);

                gl.set_u_translate(mvp_matrix.reversed_axes());
                gl.set_u_id_value(id_offset_color.value() as i32);
                gl.set_u_model_matrix(model_matrix.reversed_axes());
                gl.set_u_inv_model_matrix(inv_model_matrix.reversed_axes());
                gl.set_u_shape(match shape {
                    block::boxblock::Shape::Cube => program::SHAPE_3D_BOX,
                    block::boxblock::Shape::Slope => program::SHAPE_3D_BOX,
                    block::boxblock::Shape::Cylinder => program::SHAPE_3D_CYLINDER,
                    block::boxblock::Shape::Sphere => program::SHAPE_3D_SPHERE,
                });

                if let RenderingMode::View { .. } = rendering_mode {
                    gl.set_u_bg_color_1_value(&boxblock.color().to_color().to_f32array());

                    if let Some(tex_idx) = boxblock
                        .texture()
                        .and_then(|texture| tex_table.use_blocktexture(gl, texture.as_ref()))
                    {
                        gl.set_u_texture_0(program::TEXTURE_NORMAL);
                        gl.set_u_texture_0_sampler(tex_idx);
                    } else {
                        gl.set_u_texture_0(program::TEXTURE_NONE);
                    }
                }
                match shape {
                    block::boxblock::Shape::Slope => {
                        gl.draw_elements_with_i32(
                            web_sys::WebGlRenderingContext::TRIANGLES,
                            30,
                            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                            36 * 2,
                        );
                    }
                    _ => {
                        gl.draw_elements_with_i32(
                            web_sys::WebGlRenderingContext::TRIANGLES,
                            36,
                            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                            0,
                        );
                    }
                }
            });
        }
    }

    fn surface_of(boxblock: &block::boxblock::Boxblock, idx: u32) -> Option<Surface> {
        let p = boxblock.position();
        let s = boxblock.size();

        Some(match idx % 6 {
            0 => Surface {
                //PZ
                p: p.clone(),
                r: [0.0, 0.0, s[2] * 0.5],
                s: [1.0, 0.0, 0.0],
                t: [0.0, 1.0, 0.0],
            },
            1 => Surface {
                //PY
                p: p.clone(),
                r: [0.0, s[1] * 0.5, 0.0],
                s: [0.0, 0.0, 1.0],
                t: [1.0, 0.0, 0.0],
            },
            2 => Surface {
                //PX
                p: p.clone(),
                r: [s[0] * 0.5, 0.0, 0.0],
                s: [0.0, 1.0, 0.0],
                t: [0.0, 0.0, 1.0],
            },
            3 => Surface {
                //NX
                p: p.clone(),
                r: [-s[0] * 0.5, 0.0, 0.0],
                s: [0.0, 0.0, 1.0],
                t: [0.0, 1.0, 0.0],
            },
            4 => Surface {
                //NY
                p: p.clone(),
                r: [0.0, -s[1] * 0.5, 0.0],
                s: [1.0, 0.0, 0.0],
                t: [0.0, 0.0, 1.0],
            },
            5 => Surface {
                //NZ
                p: p.clone(),
                r: [0.0, 0.0, -s[2] * 0.5],
                s: [0.0, 1.0, 0.0],
                t: [1.0, 0.0, 0.0],
            },
            6 => Surface {
                //斜面（上半分）
                p: p.clone(),
                r: [0.0, 0.0, 0.0],
                s: [0.0, 1.0, 0.0],
                t: [-1.0, 0.0, 1.0],
            },
            7 => Surface {
                //斜面（下半分）
                p: p.clone(),
                r: [0.0, 0.0, 0.0],
                s: [0.0, 1.0, 0.0],
                t: [-1.0, 0.0, 1.0],
            },
            _ => unreachable!(),
        })
    }

    fn n(v: &[f32; 3]) -> [f32; 3] {
        let len = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
        [v[0] / len, v[1] / len, v[2] / len]
    }
}
