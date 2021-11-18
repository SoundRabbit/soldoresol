use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use ndarray::Array2;

pub struct Pointlight {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl Pointlight {
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
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        pointlight_ids: impl Iterator<Item = BlockId>,
        is_transparent: bool,
    ) {
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::DefaultProgram);

        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        let _ = block_arena.iter_map_with_ids(
            pointlight_ids,
            |_, pointlight: &block::pointlight::Pointlight| {
                let s = [1.0, 1.0, 1.0];
                let p = pointlight.position();
                let model_matrix: Array2<f32> =
                    ModelMatrix::new().with_scale(&s).with_movement(p).into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_unif_translate(mvp_matrix.reversed_axes());
                gl.set_unif_bg_color(&if is_transparent {
                    [0.0, 0.0, 0.0, 0.0]
                } else {
                    pointlight.color().to_color().to_f32array()
                });
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
