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

    uniform vec4 u_bgColor;
    varying vec2 v_textureCoord;
    uniform sampler2D u_texture0;
    uniform sampler2D u_texture1;
    uniform int u_flagTexture1;

    vec4 blend(vec4 bg, vec4 fr) {
        float dist_a = bg.w;
        float src_a = fr.w;
        float out_a = src_a + dist_a * (1.0 - src_a);
        vec3 out_rgb  = (fr.xyz * src_a + bg.xyz * dist_a * (1.0 - src_a)) / out_a;
        return vec4(out_rgb, out_a);
    }

    void main() {
        vec4 smpColor0 = texture2D(u_texture0, v_textureCoord);
        vec4 smpColor1 = u_flagTexture1 != 0 ? texture2D(u_texture1, v_textureCoord) : vec4(0.0,0.0,0.0,0.0);
        vec4 color_a = u_bgColor;
        vec4 color_b = blend(color_a, smpColor1);
        vec4 color_c = blend(color_b, smpColor0);
        gl_FragColor = color_c;
    }
"#;

pub struct TableTextureProgram {
    program: web_sys::WebGlProgram,
    pub a_vertex_location: WebGlAttributeLocation,
    pub a_texture_coord_location: WebGlAttributeLocation,
    pub u_translate_location: WebGlUniformLocation,
    pub u_bg_color_location: WebGlUniformLocation,
    pub u_texture_0_location: WebGlUniformLocation,
    pub u_texture_1_location: WebGlUniformLocation,
    pub u_flag_texture_1_location: WebGlUniformLocation,
}

impl TableTextureProgram {
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
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();
        let u_texture_0_location = gl.get_uniform_location(&program, "u_texture0").unwrap();
        let u_texture_1_location = gl.get_uniform_location(&program, "u_texture1").unwrap();
        let u_flag_texture_1_location =
            gl.get_uniform_location(&program, "u_flagTexture1").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_bg_color_location,
            u_texture_0_location,
            u_texture_1_location,
            u_flag_texture_1_location,
        }
    }

    pub fn use_program(&self, gl: &WebGlRenderingContext) {
        gl.use_program(Some(&self.program));
    }
}
