use super::model_matrix::ModelMatrix;
use super::webgl::WebGlF32Vbo;
use super::webgl::WebGlI16Ibo;
use super::webgl::WebGlRenderingContext;
use crate::model::Camera;
use crate::model::Table;
use ndarray::Array2;

pub struct TableRenderer {
    vertexis_buffer: WebGlF32Vbo,
    color_buffer: WebGlF32Vbo,
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
        let color_buffer = gl.create_vbo_with_f32array(
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
            .concat(),
        );
        let texture_coord_buffer =
            gl.create_vbo_with_f32array(&[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat());
        let index_buffer = gl.create_ibo_with_i16array(&[0, 1, 2, 3, 2, 1]);
        Self {
            vertexis_buffer,
            color_buffer,
            texture_coord_buffer,
            index_buffer,
        }
    }

    pub fn vertexis(&self) -> &WebGlF32Vbo {
        &self.vertexis_buffer
    }

    pub fn color(&self) -> &WebGlF32Vbo {
        &self.color_buffer
    }

    pub fn texture_coord(&self) -> &WebGlF32Vbo {
        &self.texture_coord_buffer
    }

    pub fn index(&self) -> &WebGlI16Ibo {
        &self.index_buffer
    }

    pub fn model_matrix(&self, _: &Camera, table: &Table) -> Array2<f64> {
        let s = table.size();
        ModelMatrix::new().with_scale(&[s[0], s[1], 1.0]).into()
    }
}
