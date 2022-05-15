mod craftboard_box;
mod craftboard_grid;
mod craftboard_texture;

use super::libs::matrix::model::ModelMatrix;
use super::libs::tex_table::TexTable;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::{block, BlockRef};
use ndarray::Array2;

use craftboard_box::CraftboardBox;
use craftboard_grid::CraftboardGrid;
use craftboard_texture::CraftboardTexture;

pub struct Craftboard {
    craftboard_box_mesh: CraftboardBox,
    craftboard_grid_mesh: CraftboardGrid,
    craftboard_texture_mesh: CraftboardTexture,
}

impl Craftboard {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let craftboard_box_mesh = CraftboardBox::new(gl);
        let craftboard_grid_mesh = CraftboardGrid::new(gl);
        let craftboard_texture_mesh = CraftboardTexture::new(gl);

        Self {
            craftboard_box_mesh,
            craftboard_grid_mesh,
            craftboard_texture_mesh,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        camera_position: &[f32; 3],
        craftboards: impl Iterator<Item = BlockRef<block::Craftboard>>,
        is_2d_mode: bool,
        tex_table: &mut TexTable,
    ) {
        gl.use_program(ProgramType::UnshapedProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::LEQUAL);
        gl.set_u_expand(0.0);
        gl.set_u_v_color_mask(program::V_COLOR_MASK_NONE);
        gl.set_u_id(program::ID_NONE);

        gl.clear(web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT);
        gl.stencil_op(
            web_sys::WebGlRenderingContext::KEEP,
            web_sys::WebGlRenderingContext::KEEP,
            web_sys::WebGlRenderingContext::INCR,
        );

        for craftboard in craftboards {
            gl.stencil_func(web_sys::WebGlRenderingContext::ALWAYS, 0, 1);
            self.craftboard_box_mesh.render(
                gl,
                vp_matrix,
                camera_position,
                BlockRef::clone(&craftboard),
                is_2d_mode,
            );
            self.craftboard_grid_mesh.render(
                gl,
                vp_matrix,
                camera_position,
                BlockRef::clone(&craftboard),
            );
            gl.stencil_func(web_sys::WebGlRenderingContext::EQUAL, 0, 1);
            self.craftboard_texture_mesh.render(
                gl,
                vp_matrix,
                camera_position,
                BlockRef::clone(&craftboard),
                is_2d_mode,
                tex_table,
            );

            gl.clear(web_sys::WebGlRenderingContext::STENCIL_BUFFER_BIT);
        }

        gl.stencil_op(
            web_sys::WebGlRenderingContext::KEEP,
            web_sys::WebGlRenderingContext::KEEP,
            web_sys::WebGlRenderingContext::KEEP,
        );
    }
}
