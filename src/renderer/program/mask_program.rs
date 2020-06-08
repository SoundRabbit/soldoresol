use super::webgl::{WebGlAttributeLocation, WebGlRenderingContext};
use web_sys::WebGlUniformLocation;

const VERTEX_SHADER: &str = r#"
    attribute vec4 a_vertex;
    attribute vec2 a_textureCoord;
    uniform mat4 u_translate;
    varying vec2 v_textureCoord;

    void main() {
        v_textureCoord = a_textureCoord;
        gl_Position = u_translate * a_vertex;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform vec4 u_maskColor;
    uniform int u_flagRound;
    varying vec2 v_textureCoord;

    void main() {
        float x = (v_textureCoord.x - 0.5) * 2.0;
        float y = (v_textureCoord.y - 0.5) * 2.0;
        gl_FragColor = u_flagRound != 0 ? (x * x + y * y > 1.0 ? vec4(0.0, 0.0, 0.0, 0.0) : u_maskColor) : u_maskColor;
    }
"#;

pub struct MaskProgram {
    program: web_sys::WebGlProgram,
    pub a_vertex_location: WebGlAttributeLocation,
    pub a_texture_coord_location: WebGlAttributeLocation,
    pub u_translate_location: WebGlUniformLocation,
    pub u_mask_color_location: WebGlUniformLocation,
    pub u_flag_round_location: WebGlUniformLocation,
}

impl MaskProgram {
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
        .unwrap();
        let program = super::link_program(gl, &v_shader, &f_shader).unwrap();
        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_mask_color_location = gl.get_uniform_location(&program, "u_maskColor").unwrap();
        let u_flag_round_location = gl.get_uniform_location(&program, "u_flagRound").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_mask_color_location,
            u_flag_round_location,
        }
    }

    pub fn use_program(&self, gl: &WebGlRenderingContext) {
        gl.use_program(Some(&self.program));
    }
}
