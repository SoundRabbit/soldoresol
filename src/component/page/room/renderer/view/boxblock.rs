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
        let normals_buffer = gl.create_vbo_with_f32array(
            &[
                Self::n(0.0, 0.0, 1.0),
                Self::n(0.0, 0.0, 1.0),
                Self::n(0.0, 0.0, 1.0),
                Self::n(0.0, 0.0, 1.0),
                Self::n(0.0, 1.0, 0.0),
                Self::n(0.0, 1.0, 0.0),
                Self::n(0.0, 1.0, 0.0),
                Self::n(0.0, 1.0, 0.0),
                Self::n(1.0, 0.0, 0.0),
                Self::n(1.0, 0.0, 0.0),
                Self::n(1.0, 0.0, 0.0),
                Self::n(1.0, 0.0, 0.0),
                Self::n(-1.0, 0.0, 0.0),
                Self::n(-1.0, 0.0, 0.0),
                Self::n(-1.0, 0.0, 0.0),
                Self::n(-1.0, 0.0, 0.0),
                Self::n(0.0, -1.0, 0.0),
                Self::n(0.0, -1.0, 0.0),
                Self::n(0.0, -1.0, 0.0),
                Self::n(0.0, -1.0, 0.0),
                Self::n(0.0, 0.0, -1.0),
                Self::n(0.0, 0.0, -1.0),
                Self::n(0.0, 0.0, -1.0),
                Self::n(0.0, 0.0, -1.0),
            ]
            .concat(),
        );
        let poly_index_buffer = gl.create_ibo_with_i16array(
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
        gl.set_unif_shade_intensity(0.5);
        gl.set_unif_env_light_intensity(1.0);

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
