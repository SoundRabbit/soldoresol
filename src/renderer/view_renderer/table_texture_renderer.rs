use super::super::program::TableTextureProgram;
use super::super::webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use super::super::ModelMatrix;
use crate::model::{Camera, Color, Resource, Table};
use ndarray::Array2;

pub struct TableTextureRenderer {
    polygon_vertexis_buffer: WebGlF32Vbo,
    polygon_index_buffer: WebGlI16Ibo,
    polygon_texture_coord_buffer: WebGlF32Vbo,
    polygon_texture_buffer_0: web_sys::WebGlTexture,
    polygon_texture_buffer_1: web_sys::WebGlTexture,
    table_texture_program: TableTextureProgram,
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
        let polygon_texture_buffer_0 = gl.create_texture().unwrap();
        let polygon_texture_buffer_1 = gl.create_texture().unwrap();

        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&polygon_texture_buffer_0),
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

        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&polygon_texture_buffer_1),
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
            polygon_texture_buffer_0,
            polygon_texture_buffer_1,
            table_texture_program,
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        _camera: &Camera,
        vp_matrix: &Array2<f64>,
        table: &mut Table,
        textures: &mut super::TextureCollection,
        resource: &Resource,
    ) {
        let [height, width] = table.size();
        let (height, width) = (*height, *width);

        self.table_texture_program.use_program(gl);

        gl.enable(web_sys::WebGlRenderingContext::CULL_FACE);
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
            Some(&self.polygon_texture_buffer_0),
        );
        if let Some(texture) = table.drawing_texture_element() {
            gl.tex_image_2d_with_u32_and_u32_and_canvas(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                texture,
            )
            .unwrap();
        }

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE1);
        gl.uniform1i(Some(&self.table_texture_program.u_texture_1_location), 1);
        gl.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            Some(&self.polygon_texture_buffer_1),
        );
        if let Some(texture) = table.measure_texture_element() {
            gl.tex_image_2d_with_u32_and_u32_and_canvas(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                texture,
            )
            .unwrap();
        }

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE2);
        let mut texture_2_is_active = false;
        if let Some(texture_id) = table.image_texture_id() {
            if let (None, Some(texture_data)) =
                (textures.get(texture_id), resource.get_as_image(texture_id))
            {
                textures.insert(gl, *texture_id, &texture_data);
            }
            if let Some(texture) = textures.get(&texture_id) {
                gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&texture));
                texture_2_is_active = true;
            }
        }
        if texture_2_is_active {
            gl.uniform1i(Some(&self.table_texture_program.u_texture_2_location), 2);
            gl.uniform1i(
                Some(&self.table_texture_program.u_flag_texture_2_location),
                1,
            );
        } else {
            gl.uniform1i(
                Some(&self.table_texture_program.u_flag_texture_2_location),
                0,
            );
        }

        gl.active_texture(web_sys::WebGlRenderingContext::TEXTURE0);

        let model_matrix: Array2<f64> = ModelMatrix::new().with_scale(&[height, width, 1.0]).into();
        let mvp_matrix = model_matrix.dot(vp_matrix);

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

        gl.disable(web_sys::WebGlRenderingContext::CULL_FACE);

        table.rendered();
    }
}
