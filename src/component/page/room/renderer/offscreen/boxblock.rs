use super::id_table::{IdTable, ObjectId, Surface};
use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;

pub struct Boxblock {
    vertexis_buffer: WebGlF32Vbo,
    colors_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
}

impl Boxblock {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                [0.5, 0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
            ]
            .concat(),
        );
        let colors_buffer = gl.create_vbo_with_f32array(
            &[
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
                [0.0, 0.0],
                [0.0, 0.0],
                [0.0, 0.0],
                [0.0, 0.0],
                [0.0, 0.0],
                [0.0, 0.0],
                [0.0, 0.0],
                [0.0, 0.0],
            ]
            .concat(),
        );
        let index_buffer = gl.create_ibo_with_i16array(
            &[
                [0, 1, 2, 3, 2, 1], // 上
                [4, 1, 0, 1, 4, 5], // 奥
                [0, 2, 4, 6, 4, 2], // 右
                [5, 3, 1, 3, 5, 7], // 左
                [2, 3, 6, 7, 6, 3], // 前
                [6, 5, 4, 5, 6, 7], // 下
            ]
            .concat(),
        );

        Self {
            vertexis_buffer,
            colors_buffer,
            index_buffer,
            texture_coord_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        id_table: &mut IdTable,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        boxblock_ids: impl Iterator<Item = BlockId>,
        grabbed_object_id: &ObjectId,
    ) {
        gl.use_program(ProgramType::OffscreenProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);
        gl.set_attr_tex_coord(&self.texture_coord_buffer, 2, 0);
        gl.set_attr_color(&self.colors_buffer, 4, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );
        gl.set_unif_flag_round(0);

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
                gl.set_unif_translate(mvp_matrix.reversed_axes());

                for srfs in 0..6 {
                    let color = crate::libs::color::Color::from(id_table.len() as u32 | 0xFF000000);
                    gl.set_unif_bg_color(&color.to_f32array());
                    gl.draw_elements_with_i32(
                        web_sys::WebGlRenderingContext::TRIANGLES,
                        6,
                        web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                        6 * 2 * srfs,
                    );
                    id_table.insert(
                        color.to_u32(),
                        ObjectId::Boxblock(
                            BlockId::clone(&boxblock_id),
                            Self::surface_of(boxblock, srfs),
                        ),
                    );
                }
            },
        );
    }

    fn surface_of(boxblock: &block::boxblock::Boxblock, idx: i32) -> Surface {
        let p = boxblock.position();
        let s = boxblock.size();

        match idx % 6 {
            0 => Surface {
                r: [p[0], p[1], p[2] + s[2] * 0.5],
                s: [1.0, 0.0, 0.0],
                t: [0.0, 1.0, 0.0],
            },
            1 => Surface {
                r: [p[0], p[1] + s[1] * 0.5, p[2]],
                s: [0.0, 0.0, 1.0],
                t: [1.0, 0.0, 0.0],
            },
            2 => Surface {
                r: [p[0] + s[0] * 0.5, p[1], p[2]],
                s: [0.0, 1.0, 0.0],
                t: [0.0, 0.0, 1.0],
            },
            3 => Surface {
                r: [p[0] - s[0] * 0.5, p[1], p[2]],
                s: [0.0, 0.0, 1.0],
                t: [0.0, 1.0, 0.0],
            },
            4 => Surface {
                r: [p[0], p[1] - s[1] * 0.5, p[2]],
                s: [1.0, 0.0, 0.0],
                t: [0.0, 0.0, 1.0],
            },
            5 => Surface {
                r: [p[0], p[1], p[2] - s[2] * 0.5],
                s: [0.0, 1.0, 0.0],
                t: [1.0, 0.0, 0.0],
            },
            _ => unreachable!(),
        }
    }
}
