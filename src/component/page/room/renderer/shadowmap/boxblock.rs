use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::libs::random_id::U128Id;
use ndarray::Array2;

pub struct Boxblock {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
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
            index_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        shadowmap: &[(web_sys::WebGlTexture, U128Id); 6],
        light_vps: &[Array2<f32>; 6],
        block_arena: &block::Arena,
        boxblock_ids: Vec<BlockId>,
    ) {
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::ShadowmapProgram);

        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for i in 0..6 {
            let tex_buf = &shadowmap[i].0;
            gl.framebuffer_texture_2d(
                web_sys::WebGlRenderingContext::FRAMEBUFFER,
                web_sys::WebGlRenderingContext::COLOR_ATTACHMENT0,
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                Some(tex_buf),
                0,
            );
            gl.clear(
                web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
                    | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT,
            );

            let vp_matrix = &light_vps[i];

            let _ = block_arena.iter_map_with_ids(
                boxblock_ids.iter().map(BlockId::clone),
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
                    let mvp_matrix = vp_matrix.dot(&model_matrix);
                    gl.set_unif_translate(mvp_matrix.reversed_axes());
                    gl.draw_elements_with_i32(
                        web_sys::WebGlRenderingContext::TRIANGLES,
                        36,
                        web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                        0,
                    );
                },
            );
        }
    }
}
