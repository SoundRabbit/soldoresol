use super::matrix::model::ModelMatrix;
use super::tex_table::TexTable;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block::{self, BlockId};
use crate::arena::resource;
use crate::libs::random_id::U128Id;
use ndarray::Array2;

pub struct Tabletexture {
    vertexis_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
    texture_coord_buffer: WebGlF32Vbo,
    drawing_texture_buffer: web_sys::WebGlTexture,
    drawing_texture_buffer_id: U128Id,
    drawing_texture_update_time: f64,
    last_drawing_texture_id: BlockId,
    drawed_texture_buffer: web_sys::WebGlTexture,
    drawed_texture_buffer_id: U128Id,
    drawed_texture_update_time: f64,
    last_drawed_texture_id: BlockId,
}

impl Tabletexture {
    pub fn new(gl: &WebGlRenderingContext, tex_table: &mut TexTable) -> Self {
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
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);

        let (drawing_texture_buffer_id, drawing_texture_buffer) =
            Self::create_texture(gl, tex_table);
        let (drawed_texture_buffer_id, drawed_texture_buffer) = Self::create_texture(gl, tex_table);

        Self {
            vertexis_buffer,
            texture_coord_buffer,
            index_buffer,
            drawing_texture_buffer,
            drawing_texture_buffer_id,
            drawing_texture_update_time: 0.0,
            drawed_texture_buffer,
            drawed_texture_buffer_id,
            drawed_texture_update_time: 0.0,
            last_drawing_texture_id: BlockId::none(),
            last_drawed_texture_id: BlockId::none(),
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        tex_table: &mut TexTable,
        vp_matrix: &Array2<f32>,
        block_arena: &block::Arena,
        local_block_arena: &block::Arena,
        resource_arena: &resource::Arena,
        table: &block::table::Table,
    ) {
        let table_size = {
            let sz = table.size();
            [sz[0], sz[1]]
        };

        gl.use_program(ProgramType::TabletextureProgram);

        gl.set_attr_vertex(&self.vertexis_buffer, 3, 0);
        gl.set_attr_tex_coord(&self.texture_coord_buffer, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        let (tex_idx, tex_update_time) = Self::update_texture(
            gl,
            local_block_arena,
            tex_table,
            &self.last_drawing_texture_id,
            table.drawing_texture_id(),
            self.drawing_texture_update_time,
            &self.drawing_texture_buffer_id,
            &self.drawing_texture_buffer,
        );
        gl.set_unif_texture(tex_idx);
        if let Some(tex_update_time) = tex_update_time {
            self.drawing_texture_update_time = tex_update_time;
            self.last_drawing_texture_id = BlockId::clone(table.drawing_texture_id());
        }

        let (tex_idx, tex_update_time) = Self::update_texture(
            gl,
            block_arena,
            tex_table,
            &self.last_drawed_texture_id,
            table.drawed_texture_id(),
            self.drawed_texture_update_time,
            &self.drawed_texture_buffer_id,
            &self.drawed_texture_buffer,
        );
        gl.set_unif_texture_1(tex_idx);
        if let Some(tex_update_time) = tex_update_time {
            self.drawed_texture_update_time = tex_update_time;
            self.last_drawed_texture_id = BlockId::clone(table.drawed_texture_id());
        }

        gl.set_unif_texture_2_is_available(0);

        let model_matrix: Array2<f32> = ModelMatrix::new()
            .with_scale(&[table_size[0], table_size[1], 1.0])
            .into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);

        gl.set_unif_translate(mvp_matrix.reversed_axes());
        gl.set_unif_bg_color(&table.background_color().to_color().to_f32array());
        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            6,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn create_texture(
        gl: &WebGlRenderingContext,
        tex_table: &mut TexTable,
    ) -> (U128Id, web_sys::WebGlTexture) {
        let texture_buffer = gl.create_texture().unwrap();
        let texture_id = U128Id::new();
        let (_, tex_flag) = tex_table.use_custom(&texture_id);
        gl.active_texture(tex_flag);
        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&texture_buffer),
        );
        gl.pixel_storei(web_sys::WebGlRenderingContext::PACK_ALIGNMENT, 1);
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        (texture_id, texture_buffer)
    }

    fn update_texture(
        gl: &WebGlRenderingContext,
        block_arena: &block::Arena,
        tex_table: &mut TexTable,
        last_tex_id: &BlockId,
        tex_id: &BlockId,
        last_tex_update_time: f64,
        tex_buf_id: &U128Id,
        tex_buf: &web_sys::WebGlTexture,
    ) -> (i32, Option<f64>) {
        let (tex_idx, tex_flag) = tex_table.use_custom(tex_buf_id);
        gl.active_texture(tex_flag);
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&tex_buf));
        let tex_update_time = block_arena.timestamp_of(tex_id).unwrap_or(0.0);
        if *last_tex_id != *tex_id || last_tex_update_time < tex_update_time {
            block_arena.map(tex_id, |table_texture: &block::table::texture::Texture| {
                let _ = gl.tex_image_2d_with_u32_and_u32_and_canvas(
                    web_sys::WebGlRenderingContext::TEXTURE_2D,
                    0,
                    web_sys::WebGlRenderingContext::RGBA as i32,
                    web_sys::WebGlRenderingContext::RGBA,
                    web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                    table_texture.element(),
                );
            });

            (tex_idx, Some(tex_update_time))
        } else {
            (tex_idx, None)
        }
    }
}
