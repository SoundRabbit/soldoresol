use super::super::{
    program::MaskProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use super::TableBlock;
use crate::{
    block::{self, BlockId},
    Color,
};
use ndarray::Array2;
use std::collections::HashMap;

pub struct TablemaskCollectionRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl TablemaskCollectionRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        program: &MaskProgram,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        tablemasks: impl Iterator<Item = &'a BlockId>,
        id_map: &mut HashMap<u32, TableBlock>,
    ) {
        gl.set_attribute(&self.vertexis_buffer, &program.a_vertex_location, 3, 0);
        gl.set_attribute(
            &self.texture_coord_buffer,
            &program.a_texture_coord_location,
            2,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for (tablemask_id, tablemask) in
            block_field.listed::<block::table_object::Tablemask>(tablemasks.collect())
        {
            let model_matrix: Array2<f32> = ModelMatrix::new()
                .with_scale(tablemask.size())
                .with_movement(tablemask.position())
                .into();
            let mvp_matrix = vp_matrix.dot(&model_matrix);
            let mvp_matrix = mvp_matrix.t();

            gl.uniform_matrix4fv_with_f32_array(
                Some(&program.u_translate_location),
                false,
                &[
                    mvp_matrix.row(0).to_vec(),
                    mvp_matrix.row(1).to_vec(),
                    mvp_matrix.row(2).to_vec(),
                    mvp_matrix.row(3).to_vec(),
                ]
                .concat()
                .into_iter()
                .map(|a| a as f32)
                .collect::<Vec<f32>>(),
            );
            let color = Color::from(id_map.len() as u32 | 0xFF000000);
            gl.uniform4fv_with_f32_array(
                Some(&program.u_mask_color_location),
                &color.to_f32array(),
            );
            gl.uniform1i(Some(&program.u_flag_round_location), 1);
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );

            id_map.insert(color.to_u32(), TableBlock::new(tablemask_id.clone(), 0));
        }
    }
}
