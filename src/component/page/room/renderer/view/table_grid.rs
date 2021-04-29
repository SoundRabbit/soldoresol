use super::matrix::model::ModelMatrix;
use super::webgl::{ProgramType, WebGlF32Vbo, WebGlI16Ibo, WebGlRenderingContext};
use crate::arena::block;
use ndarray::Array2;

pub struct TableGrid {
    grid_index_buffer: WebGlI16Ibo,
    grid_index_len: i32,
    grid_vertexis_buffer: WebGlF32Vbo,
    table_size: [u64; 2],
}

impl TableGrid {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let table_size = [20, 20];

        let (grid_vertexis_buffer, grid_index_buffer, grid_index_len) =
            Self::create_grid_buffers(&gl, &table_size);

        Self {
            grid_index_buffer,
            grid_index_len,
            grid_vertexis_buffer,
            table_size,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut WebGlRenderingContext,
        vp_matrix: &Array2<f32>,
        table: &block::table::Table,
    ) {
        let table_size = {
            let sz = table.size();
            [sz[0].floor() as u64, sz[1].floor() as u64]
        };

        if table_size[0] != self.table_size[0] || table_size[1] != self.table_size[1] {
            let (grid_vertexis_buffer, grid_index_buffer, grid_index_len) =
                Self::create_grid_buffers(&gl, &table_size);

            self.grid_vertexis_buffer = grid_vertexis_buffer;
            self.grid_index_buffer = grid_index_buffer;
            self.grid_index_len = grid_index_len;
            self.table_size = table_size;
        }

        gl.use_program(ProgramType::TablegridProgram);
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);
        gl.set_attr_vertex(&self.grid_vertexis_buffer, 3, 0);
        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.grid_index_buffer),
        );

        let model_matrix: Array2<f32> = ModelMatrix::new().into();
        let mvp_matrix = vp_matrix.dot(&model_matrix);
        gl.set_unif_translate(mvp_matrix.reversed_axes());
        gl.set_unif_point_size(1.0);
        gl.line_width(5.0);
        gl.set_unif_bg_color(&table.grid_color().to_color().to_f32array());

        gl.draw_elements_with_i32(
            web_sys::WebGlRenderingContext::LINES,
            self.grid_index_len,
            web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
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
