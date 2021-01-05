use super::webgl::{WebGlAttributeLocation, WebGlRenderingContext};
use web_sys::WebGlUniformLocation;

const VERTEX_SHADER: &str = r#"
    attribute vec4 a_vertex;
    attribute vec3 a_normal;
    uniform mat4 u_translate;
    uniform mat4 u_invModel;
    uniform vec3 u_light;
    uniform vec4 u_maskColor;
    uniform float u_shade;
    varying vec4 v_color;

    void main() {
        vec3 invLight = normalize(u_invModel * vec4(u_light, 0.0)).xyz;
        float diffuse = clamp(dot(a_normal, invLight) * u_shade + 1.0 - u_shade, 1.0 - u_shade, 1.0);
        v_color = u_maskColor * vec4(vec3(diffuse), 1.0);

        vec4 p = u_translate * a_vertex;
        gl_Position = p;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec4 v_color;

    void main() {
        gl_FragColor = v_color;
    }
"#;

pub struct CubeProgram {
    program: web_sys::WebGlProgram,
    pub a_vertex_location: WebGlAttributeLocation,
    pub a_normal_location: WebGlAttributeLocation,
    pub u_translate_location: WebGlUniformLocation,
    pub u_inv_model_location: WebGlUniformLocation,
    pub u_light_location: WebGlUniformLocation,
    pub u_mask_color_location: WebGlUniformLocation,
    pub u_shade_location: WebGlUniformLocation,
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
        let a_normal_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_normal") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_inv_model_location = gl.get_uniform_location(&program, "u_invModel").unwrap();
        let u_light_location = gl.get_uniform_location(&program, "u_light").unwrap();
        let u_mask_color_location = gl.get_uniform_location(&program, "u_maskColor").unwrap();
        let u_shade_location = gl.get_uniform_location(&program, "u_shade").unwrap();

        Self {
            program,
            a_vertex_location,
            a_normal_location,
            u_translate_location,
            u_inv_model_location,
            u_light_location,
            u_mask_color_location,
            u_shade_location,
        }
    }

    pub fn use_program(&self, gl: &WebGlRenderingContext) {
        gl.use_program(Some(&self.program));
    }
}
