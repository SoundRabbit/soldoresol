use super::webgl::{WebGlAttributeLocation, WebGlRenderingContext};
use web_sys::WebGlUniformLocation;

const VERTEX_SHADER: &str = r#"
    attribute vec4 a_vertex;
    attribute vec2 a_textureCoord;
    uniform mat4 u_translate;
    varying vec2 v_textureCoord;
    varying float v_fragDepth;

    void main() {
        v_textureCoord = a_textureCoord;
        gl_Position = u_translate * a_vertex;

        float far = gl_DepthRange.far;
        float near = gl_DepthRange.near;
        vec4 p =  u_translate * vec4(a_vertex.x, a_vertex.y - 0.5, a_vertex.zw);
        float ndc_depth = p.z / p.w;
        v_fragDepth = (((far-near) * ndc_depth) + near + far) / 2.0;
    }
"#;

const FRAGMENT_SHADER: &str = r##"
    precision mediump float;

    #extension GL_EXT_frag_depth : enable

    uniform vec4 u_bgColor;
    varying vec2 v_textureCoord;
    uniform sampler2D u_texture;
    varying float v_fragDepth;

    void main() {
        vec4 smpColor = texture2D(u_texture, v_textureCoord);
        float dist_a = u_bgColor.w;
        float src_a = smpColor.w;
        float out_a = src_a + dist_a * (1.0 - src_a);
        vec3 out_rgb  = (smpColor.xyz * src_a + u_bgColor.xyz * dist_a * (1.0 - src_a)) / out_a;
        gl_FragColor = vec4(out_rgb, out_a);
        gl_FragDepthEXT = v_fragDepth;
    }
"##;

pub struct CharacterProgram {
    program: web_sys::WebGlProgram,
    pub a_vertex_location: WebGlAttributeLocation,
    pub a_texture_coord_location: WebGlAttributeLocation,
    pub u_translate_location: WebGlUniformLocation,
    pub u_bg_color_location: WebGlUniformLocation,
    pub u_texture_location: WebGlUniformLocation,
}

impl CharacterProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let v_shader = super::compile_shader(
            gl,
            VERTEX_SHADER,
            web_sys::WebGlRenderingContext::VERTEX_SHADER,
        )
        .map_err(|err| crate::debug::log_1(err))
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
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();
        let u_texture_location = gl.get_uniform_location(&program, "u_texture").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_bg_color_location,
            u_texture_location,
        }
    }

    pub fn use_program(&self, gl: &WebGlRenderingContext) {
        gl.use_program(Some(&self.program));
    }
}
