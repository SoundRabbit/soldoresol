use super::webgl::WebGlF32Vbo;
use super::webgl::WebGlI16Ibo;
use crate::model::Camera;
use ndarray::Array2;

pub trait BasicRenderer {
    type Model;
    fn vertexis(&self) -> &WebGlF32Vbo;
    fn texture_coord(&self) -> &WebGlF32Vbo;
    fn index(&self) -> &WebGlI16Ibo;
    fn model_matrix(&self, _: &Camera, model: &Self::Model) -> Array2<f64>;
}
