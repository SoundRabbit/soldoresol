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
    varying vec2 v_textureCoord;
    void main() {
        vec4 smpColor = texture2D(u_texture, v_textureCoord);
        gl_FragColor  = smpColor;
    }
"#;

pub fn vertex_shader() -> ShaderSource {
    ShaderSource::VertexShader(String::from(VERTEX_SHADER))
}

pub fn fragment_shader() -> ShaderSource {
    ShaderSource::FragmentShader(String::from(FRAGMENT_SHADER))
}
