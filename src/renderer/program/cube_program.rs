use super::webgl::{WebGlAttributeLocation, WebGlRenderingContext};
use web_sys::WebGlUniformLocation;

const VERTEX_SHADER: &str = r#"
    attribute vec4 a_vertex;
    uniform mat4 u_translate;

    void main() {
        vec4 p = u_translate * a_vertex;
        gl_Position = p;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform vec4 u_maskColor;

    void main() {
        gl_FragColor = u_maskColor;
    }
"#;

pub struct CubeProgram {
    program: web_sys::WebGlProgram,
    pub a_vertex_location: WebGlAttributeLocation,
    pub u_translate_location: WebGlUniformLocation,
    pub u_mask_color_location: WebGlUniformLocation,
}

impl CubeProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let v_shader = super::compile_shader(
            gl,
            VERTEX_SHADER,
            web_sys::WebGlRenderingContext::VERTEX_SHADER,
        )
        .unwrap();
        let f_shader = super::compile_shader(
            gl,
            FRAGMENT_SHADER,
            web_sys::WebGlRenderingContext::FRAGMENT_SHADER,
        )
        .map_err(|e| crate::debug::log_1(e))
        .unwrap();
        let program = super::link_program(gl, &v_shader, &f_shader)
            .map_err(|e| crate::debug::log_1(e))
            .unwrap();
        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_mask_color_location = gl.get_uniform_location(&program, "u_maskColor").unwrap();

        Self {
            program,
            a_vertex_location,
            u_translate_location,
            u_mask_color_location,
        }
    }

    pub fn use_program(&self, gl: &WebGlRenderingContext) {
        gl.use_program(Some(&self.program));
    }
}
