use super::ShaderSource;

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

    uniform sampler2D u_texture;
    uniform vec4 u_bgColor;
    varying vec2 v_textureCoord;
    void main() {
        vec4 smpColor = texture2D(u_texture, v_textureCoord);
        float dist_a = u_bgColor.w;
        float src_a = smpColor.w;
        float out_a = src_a + dist_a * (1.0 - src_a);
        vec3 out_rgb  = (smpColor.xyz * src_a + u_bgColor.xyz * dist_a * (1.0 - src_a)) / out_a;
        gl_FragColor = vec4(out_rgb, out_a);
    }
"#;

pub fn vertex_shader() -> ShaderSource {
    ShaderSource::VertexShader(String::from(VERTEX_SHADER))
}

pub fn fragment_shader() -> ShaderSource {
    ShaderSource::FragmentShader(String::from(FRAGMENT_SHADER))
}
