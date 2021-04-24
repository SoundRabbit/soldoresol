use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::tex_table::TexTable;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use ndarray::{arr1, Array2};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct Boxblock {
    vertexis_buffer: WebGlF32Vbo,
    normals_buffer: WebGlF32Vbo,
    poly_index_buffer: WebGlI16Ibo,
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
        let normals_buffer = gl.create_vbo_with_f32array(
            &[
                Self::n(0.5, 0.5, 0.5),
                Self::n(-0.5, 0.5, 0.5),
                Self::n(0.5, -0.5, 0.5),
                Self::n(-0.5, -0.5, 0.5),
                Self::n(0.5, 0.5, -0.5),
                Self::n(-0.5, 0.5, -0.5),
                Self::n(0.5, -0.5, -0.5),
                Self::n(-0.5, -0.5, -0.5),
            ]
            .concat(),
        );
        let poly_index_buffer = gl.create_ibo_with_i16array(
            &[
                [0, 1, 2, 3, 2, 1],
                [4, 1, 0, 1, 4, 5],
                [0, 2, 4, 6, 4, 2],
                [5, 3, 1, 3, 5, 7],
                [2, 3, 6, 7, 6, 3],
                [6, 5, 4, 5, 6, 7],
            ]
            .concat(),
        );

        Self {
            vertexis_buffer,
            poly_index_buffer,
            normals_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        boxblock_ids: impl Iterator<Item = BlockId>,
    ) {
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::BoxblockProgram);

        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);

        gl.set_attr_normal(&self.normals_buffer, 3, 0);

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.poly_index_buffer),
        );

        gl.set_unif_light(&[0.5, -2.0, 1.0]);
        gl.set_unif_shade_intensity(0.2);

        let _ = block_arena.iter_map_with_ids(
            boxblock_ids,
            |_, boxblock: &block::boxblock::Boxblock| {
                let s = boxblock.size();
                let p = boxblock.position();
                let model_matrix: Array2<f32> =
                    ModelMatrix::new().with_scale(s).with_movement(p).into();
                let inv_model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_movement(&[-p[0], p[1], -p[2]])
                    .with_scale(&[1.0 / s[0], 1.0 / s[1], 1.0 / s[2]])
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_unif_translate(mvp_matrix.reversed_axes());
                gl.set_unif_inv_model(inv_model_matrix.reversed_axes());
                gl.set_unif_bg_color(&boxblock.color().to_color().to_f32array());
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
