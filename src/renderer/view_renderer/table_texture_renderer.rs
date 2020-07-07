use super::super::{
    program::TableTextureProgram,
    webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext},
    ModelMatrix,
};
use crate::{
    block::{self},
    resource::Data,
    Color, Resource,
};
use ndarray::Array2;

pub struct TableTextureRenderer {
    polygon_vertexis_buffer: WebGlF32Vbo,
    polygon_index_buffer: WebGlI16Ibo,
    polygon_texture_coord_buffer: WebGlF32Vbo,
    polygon_texture_buffer: web_sys::WebGlTexture,
    table_texture_program: TableTextureProgram,
    texture_update_time: f64,
}

impl TableTextureRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let polygon_vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );
        let polygon_texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]].concat());
        let polygon_index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        let polygon_texture_buffer = gl.create_texture().unwrap();

        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&polygon_texture_buffer),
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

        let table_texture_program = TableTextureProgram::new(gl);
        Self {
            polygon_vertexis_buffer,
            polygon_texture_coord_buffer,
            polygon_index_buffer,
            polygon_texture_buffer,
            table_texture_program,
            texture_update_time: 0.0,
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        block_field: &block::Field,
        table: &block::Table,
        textures: &mut super::TextureCollection,
        resource: &Resource,
    ) {
        let [height, width] = table.size();
        let (height, width) = (*height, *width);

        self.table_texture_program.use_program(gl);

        gl.set_attribute(
            &self.polygon_vertexis_buffer,
            &self.table_texture_program.a_vertex_location,
            3,
            0,
        );
        gl.set_attribute(
            &self.polygon_texture_coord_buffer,
            &self.table_texture_program.a_texture_coord_location,
            2,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.polygon_index_buffer),
        );

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE0);
        gl.uniform1i(Some(&self.table_texture_program.u_texture_0_location), 0);
        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.polygon_texture_buffer),
        );

        let drawing_texture_id = table.drawing_texture_id();
        if block_field
            .timestamp(drawing_texture_id)
            .map(|t| {
                if *t > self.texture_update_time {
                    self.texture_update_time = *t;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false)
        {
            if let Some(texture) = block_field.get::<block::table::Texture>(drawing_texture_id) {
                gl.tex_image_2d_with_u32_and_u32_and_canvas(
                    web_sys::WebGlRenderingContext::TEXTURE_2D,
                    0,
                    web_sys::WebGlRenderingContext::RGBA as i32,
                    web_sys::WebGlRenderingContext::RGBA,
                    web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                    texture.element(),
                )
                .unwrap();
            }
        }

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE1);
        let mut texture_1_is_active = false;
        if let Some(texture_id) = table.image_texture_id() {
            if let (
                None,
                Some(Data::Image {
                    element: texture_data,
                    ..
                }),
            ) = (textures.get(texture_id), resource.get(texture_id))
            {
                textures.insert(gl, *texture_id, &texture_data);
            }
            if let Some(texture) = textures.get(texture_id) {
                gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(texture));
                texture_1_is_active = true;
            }
        }
        if texture_1_is_active {
            gl.uniform1i(Some(&self.table_texture_program.u_texture_1_location), 1);
            gl.uniform1i(
                Some(&self.table_texture_program.u_flag_texture_1_location),
                1,
            );
        } else {
            gl.uniform1i(
                Some(&self.table_texture_program.u_flag_texture_1_location),
                0,
            );
        }

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE0);

        let model_matrix: Array2<f32> = ModelMatrix::new().with_scale(&[height, width, 1.0]).into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);
        let mvp_matrix = mvp_matrix.t();

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.table_texture_program.u_translate_location),
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
        gl.uniform4fv_with_f32_array(
            Some(&self.table_texture_program.u_bg_color_location),
            &Color::from([255, 255, 255, 0]).to_f32array(),
        );
        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::TRIANGLES,
            6,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}
