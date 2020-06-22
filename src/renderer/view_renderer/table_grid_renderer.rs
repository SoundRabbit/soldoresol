use super::super::program::{TableGridProgram, TableTextureProgram};
use super::super::webgl::{WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use super::super::ModelMatrix;
use crate::model::{Camera, Color, Resource, Table};
use ndarray::Array2;

pub struct TableGridRenderer {
    table_size: [u64; 2],
    grid_index_len: i32,
    grid_vertexis_buffer: WebGlF32Vbo,
    grid_index_buffer: WebGlI16Ibo,
    table_grid_program: TableGridProgram,
}

impl TableGridRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let table_size = [20, 20];

        let (grid_vertexis_buffer, grid_index_buffer, grid_index_len) =
            Self::create_grid_buffers(&gl, &table_size);

        let polygon_vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [0.5, 0.5, 0.0],
                [-0.5, 0.5, 0.0],
                [0.5, -0.5, 0.0],
                [-0.5, -0.5, 0.0],
            ]
            .concat(),
        );

        let table_grid_program = TableGridProgram::new(gl);
        Self {
            table_size,
            grid_index_len,
            grid_vertexis_buffer,
            grid_index_buffer,
            table_grid_program,
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        _camera: &Camera,
        vp_matrix: &Array2<f64>,
        table: &mut Table,
    ) {
        let table_size = table.size();
        let table_size = [table_size[0].floor() as u64, table_size[1].floor() as u64];
        if table_size[0] != self.table_size[0] || table_size[1] != self.table_size[1] {
            let (grid_vertexis_buffer, grid_index_buffer, grid_index_len) =
                Self::create_grid_buffers(&gl, &table_size);

            self.grid_vertexis_buffer = grid_vertexis_buffer;
            self.grid_index_buffer = grid_index_buffer;
            self.grid_index_len = grid_index_len;
            self.table_size = table_size;
        }

        self.table_grid_program.use_program(gl);
        gl.set_attribute(
            &self.grid_vertexis_buffer,
            &self.table_grid_program.a_vertex_location,
            3,
            0,
        );
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.grid_index_buffer),
        );
        let model_matrix: Array2<f64> = ModelMatrix::new().into();
        let mvp_matrix = model_matrix.dot(vp_matrix);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.table_grid_program.u_translate_location),
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
            Some(&self.table_grid_program.u_mask_color_location),
            &Color::from([0, 0, 0, 191]).to_f32array(),
        );
        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::LINES,
            self.grid_index_len,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        table.rendered();
    }

    fn create_grid_buffers(
        gl: &WebGlRenderingContext,
        table_size: &[u64; 2],
    ) -> (WebGlF32Vbo, WebGlI16Ibo, i32) {
        let width = table_size[0];
        let height = table_size[1];

        let x_offset = width as f32 / 2.0;
        let y_offset = height as f32 / 2.0;

        let mut grid_vertexis = vec![];

        for x in 0..(width + 1) {
            let x = x as f32 - x_offset;
            grid_vertexis.push(vec![x, -y_offset, 0.0]);
            grid_vertexis.push(vec![x, y_offset, 0.0]);
        }

        for y in 0..(height + 1) {
            let y = y as f32 - y_offset;
            grid_vertexis.push(vec![-x_offset, y, 0.0]);
            grid_vertexis.push(vec![x_offset, y, 0.0]);
        }

        let mut grid_idx = vec![];

        for idx in 0..grid_vertexis.len() {
            grid_idx.push(idx as i16);
        }

        let grid_vertexis: Vec<f32> = grid_vertexis.into_iter().flatten().collect();

        let grid_vertexis_buffer = gl.create_vbo_with_f32array(&grid_vertexis);

        let grid_index_buffer = gl.create_ibo_with_i16array(&grid_idx);

        (
            grid_vertexis_buffer,
            grid_index_buffer,
            grid_idx.len() as i32,
        )
    }
}
