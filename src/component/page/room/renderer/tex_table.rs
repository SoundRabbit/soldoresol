use super::webgl::WebGlRenderingContext;
use std::collections::HashMap;

pub struct TexTable {
    max_tex: usize,
}

impl TexTable {
    fn new(gl: &WebGlRenderingContext) -> Self {
        let max_tex = gl
            .get_parameter(web_sys::WebGlRenderingContext::MAX_TEXTURE_IMAGE_UNITS)
            .unwrap()
            .as_f64()
            .unwrap() as usize;
        Self { max_tex }
    }
}
