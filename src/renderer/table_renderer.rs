use super::basic_renderer::BasicRenderer;
use super::model_matrix::ModelMatrix;
use super::webgl::WebGlF32Vbo;
use super::webgl::WebGlI16Ibo;
use super::webgl::WebGlRenderingContext;
use crate::model::Camera;
use crate::model::Table;
use ndarray::Array2;

pub struct TableRenderer {
    vertexis_buffer: WebGlF32Vbo,
    texture_coord_buffer: WebGlF32Vbo,
    index_buffer: WebGlI16Ibo,
}

impl TableRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertexis_buffer = gl.create_vbo_with_f32array(
            &[
                [1.0, 1.0, 0.0],
                [-1.0, 1.0, 0.0],
                [1.0, -1.0, 0.0],
                [-1.0, -1.0, 0.0],
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
}

impl BasicRenderer for TableRenderer {
    type Model = Table;

    fn vertexis(&self) -> &WebGlF32Vbo {
        &self.vertexis_buffer
    }

    fn texture_coord(&self) -> &WebGlF32Vbo {
        &self.texture_coord_buffer
    }

    fn index(&self) -> &WebGlI16Ibo {
        &self.index_buffer
    }

    fn model_matrix(&self, _: &Camera, table: &Table) -> Array2<f64> {
        let s = table.size();
        ModelMatrix::new()
            .with_scale(&[s[0] / 2.0, s[1] / 2.0, 1.0])
            .into()
    }
}
