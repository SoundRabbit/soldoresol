use super::libs::matrix::model::ModelMatrix;
use super::libs::webgl::{program, ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block;
use ndarray::Array2;

struct Buffer {
    index: WebGlI16Ibo,
    index_len: i32,
    vertex: WebGlF32Vbo,
    v_color: WebGlF32Vbo,
    id_color: WebGlF32Vbo,
    normal: WebGlF32Vbo,
    texture_coord: WebGlF32Vbo,
}

pub struct TableGrid {
    buffer: Buffer,
    table_size: [u64; 2],
}

impl TableGrid {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let table_size = [20, 20];
        let buffer = Self::create_grid_buffers(&gl, &table_size);

        Self { buffer, table_size }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        camera_position: &[f32; 3],
        table: &block::table::Table,
    ) {
        let table_size = {
            let sz = table.size();
            [sz[0].floor() as u64, sz[1].floor() as u64]
        };

        if table_size[0] != self.table_size[0] || table_size[1] != self.table_size[1] {
            let buffer = Self::create_grid_buffers(&gl, &table_size);

            self.buffer = buffer;
            self.table_size = table_size;
        }

        gl.line_width(5.0);

        gl.use_program(ProgramType::UnshapedProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.set_a_vertex(&self.buffer.vertex, 3, 0);
        gl.set_a_id_color(&self.buffer.id_color, 4, 0);
        gl.set_a_v_color(&self.buffer.v_color, 4, 0);
        gl.set_a_normal(&self.buffer.normal, 3, 0);
        gl.set_a_texture_coord(&self.buffer.texture_coord, 2, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.buffer.index),
        );

        let model_matrix: Array2<f32> = ModelMatrix::new().into();
        let inv_model_matrix: Array2<f32> = ModelMatrix::new().into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);
        gl.set_u_translate(mvp_matrix.reversed_axes());

        gl.set_u_camera_position(camera_position);
        gl.set_u_inv_model_matrix(inv_model_matrix.reversed_axes());
        gl.set_u_model_matrix(model_matrix.reversed_axes());
        gl.set_u_shape(program::SHAPE_2D_BOX);
        gl.set_u_vp_matrix(vp_matrix.clone().reversed_axes());
        gl.set_u_bg_color_1(program::COLOR_SOME);
        gl.set_u_bg_color_2(program::COLOR_NONE);
        gl.set_u_bg_color_1_value(&table.grid_color().to_color().to_f32array());
        gl.set_u_id(program::ID_NONE);
        gl.set_u_texture_0(program::TEXTURE_NONE);
        gl.set_u_texture_1(program::TEXTURE_NONE);
        gl.set_u_texture_2(program::TEXTURE_NONE);
        gl.set_u_light(program::LIGHT_NONE);

        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::LINES,
            self.buffer.index_len,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn create_grid_buffers(gl: &WebGlRenderingContext, table_size: &[u64; 2]) -> Buffer {
        let width = table_size[0];
        let height = table_size[1];

        let x_offset = width as f32 / 2.0;
        let y_offset = height as f32 / 2.0;

        let mut grid_vertex = vec![];
        let mut grid_texture_coord = vec![];
        let mut grid_idx_len = 0;

        for x in 0..(width + 1) {
            let x = x as f32 - x_offset;
            grid_vertex.append(&mut vec![x, -y_offset, 0.0]);
            grid_vertex.append(&mut vec![x, y_offset, 0.0]);
            grid_texture_coord.append(&mut vec![x, -y_offset]);
            grid_texture_coord.append(&mut vec![x, y_offset]);
            grid_idx_len += 2;
        }

        for y in 0..(height + 1) {
            let y = y as f32 - y_offset;
            grid_vertex.append(&mut vec![-x_offset, y, 0.0]);
            grid_vertex.append(&mut vec![x_offset, y, 0.0]);
            grid_texture_coord.append(&mut vec![-x_offset, y]);
            grid_texture_coord.append(&mut vec![x_offset, y]);
            grid_idx_len += 2;
        }

        let mut grid_idx = vec![];
        let mut grid_v_color = vec![];
        let mut grid_id_color = vec![];
        let mut grid_normal = vec![];

        for idx in 0..grid_idx_len {
            grid_idx.push(idx as i16);
            grid_v_color.append(&mut vec![0.0, 0.0, 0.0, 0.0]);
            grid_id_color.append(&mut vec![0.0, 0.0, 0.0, 0.0]);
            grid_normal.append(&mut vec![0.0, 0.0, 1.0]);
        }

        Buffer {
            index: gl.create_ibo_with_i16array(&grid_idx),
            index_len: grid_idx_len,
            vertex: gl.create_vbo_with_f32array(&grid_vertex),
            texture_coord: gl.create_vbo_with_f32array(&grid_texture_coord),
            v_color: gl.create_vbo_with_f32array(&grid_v_color),
            id_color: gl.create_vbo_with_f32array(&grid_id_color),
            normal: gl.create_vbo_with_f32array(&grid_normal),
        }
    }
}
