use super::id_table::{IdColor, IdTable, ObjectId, Surface};
use super::matrix::model::ModelMatrix;
use super::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;
use std::collections::HashMap;

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

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &mut IdTable,
        id_value: &mut HashMap<BlockId, IdColor>,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        boxblock_ids: impl Iterator<Item = BlockId>,
        grabbed_object_id: &ObjectId,
    ) {
        gl.use_program(ProgramType::UnshapedProgram);
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
        gl.set_u_bg_color_1(program::COLOR_NONE);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_id(program::ID_V_WRITE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_light(program::LIGHT_NONE);

        let _ = block_arena.iter_map_with_ids(
            boxblock_ids,
            |boxblock_id, boxblock: &block::boxblock::Boxblock| {
                if grabbed_object_id.eq(&boxblock_id) {
                    return;
                }

                let s = boxblock.size();
                let p = boxblock.position();
                let model_matrix: Array2<f32> =
                    ModelMatrix::new().with_scale(s).with_movement(p).into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_u_translate(mvp_matrix.reversed_axes());
                let id_offset = id_table.len() as u32 | 0xFF000000;
                gl.set_u_id_value(id_offset as i32);
                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::TRIANGLES,
                    36,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );

                id_value.insert(BlockId::clone(&boxblock_id), IdColor::from(id_offset));
                for srfs in 0..6 {
                    id_table.insert(
                        IdColor::from(id_offset + srfs),
                        ObjectId::Boxblock(
                            BlockId::clone(&boxblock_id),
                            Self::surface_of(boxblock, srfs),
                        ),
                    );
                }
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
