use super::matrix::{camera::CameraMatrix, model::ModelMatrix};
use super::tex_table::TexTable;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, block_trait::DisplayNamed, BlockId};
use crate::libs::random_id::U128Id;
use ndarray::Array2;

pub struct Nameplate {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl Nameplate {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.0, 1.0],
                [-0.5, 0.0, 1.0],
                [0.5, 0.0, 0.0],
                [-0.5, 0.0, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn render(
        &self,
        gl: &mut WebGlRenderingContext,
        tex_table: &mut TexTable,
        camera: &CameraMatrix,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        block_ids: impl Iterator<Item = BlockId>,
    ) {
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.use_program(ProgramType::NamePlateProgram);
        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);
        gl.set_attr_tex_coord(&self.texture_coord_buffer, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        for block_id in block_ids {
            block_arena.map(&block_id, |block: &block::boxblock::Boxblock| {
                let name = block.display_name();
                let color = block.color().clone();
                let width = (block.size()[0].powi(2) + block.size()[1].powi(2)).sqrt();
                let offset = block.size()[2] / 2.0 + 0.5;
                let position = block.position();

                self.render_plate(
                    gl, tex_table, camera, vp_matrix, name, color, width, offset, position,
                );
            });
            block_arena.map(&block_id, |block: &block::character::Character| {
                let name = block.display_name();
                let color = block.name_color().clone();
                let width = block.size();
                let offset = block.current_tex_height();
                let position = block.position();

                self.render_plate(
                    gl, tex_table, camera, vp_matrix, name, color, width, offset, position,
                );
            });
        }
    }

    fn render_plate(
        &self,
        gl: &mut WebGlRenderingContext,
        tex_table: &mut TexTable,
        camera: &CameraMatrix,
        vp_matrix: &Array2<f32>,
        name: &String,
        mut color: crate::libs::color::Pallet,
        width: f32,
        offset: f32,
        position: &[f32; 3],
    ) {
        if name != "" {
            let name_tex = tex_table.use_string(gl, name);
            if let Some((name_tex_idx, name_tex_size)) = name_tex {
                color.idx = color.idx / 2 + 5;
                let color = color.to_color().to_f32array();
                let bg_color = crate::libs::color::Pallet::gray(0)
                    .a(100)
                    .to_color()
                    .to_f32array();
                gl.set_unif_text_color_1(&[color[0], color[1], color[2]]);
                gl.set_unif_text_color_2(&[bg_color[0], bg_color[1], bg_color[2]]);

                let name_width =
                    ((0.5 * name_tex_size[0] / name_tex_size[1]) as f32).min(width * 2.0);
                let name_height = name_width * (name_tex_size[1] / name_tex_size[0]) as f32;

                let model_matrix: Array2<f32> = ModelMatrix::new()
                    .with_scale(&[name_width, 1.0, name_height])
                    .with_x_axis_rotation(camera.x_axis_rotation() - std::f32::consts::FRAC_PI_2)
                    .with_z_axis_rotation(camera.z_axis_rotation())
                    .with_movement(&[0.0, 0.0, offset])
                    .with_movement(position)
                    .into();
                let mvp_matrix = vp_matrix.dot(&model_matrix);
                gl.set_unif_area_size(&[name_width, name_height]);
                gl.set_unif_texture(name_tex_idx);
                gl.set_unif_translate(mvp_matrix.reversed_axes());
                gl.draw_elements_with_i32(
                    web_sys::WebGlRenderingContext::TRIANGLES,
                    6,
                    web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                    0,
                );
            }
        }
    }
}
