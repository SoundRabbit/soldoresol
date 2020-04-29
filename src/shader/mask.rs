use super::ShaderSource;

const VERTEX_SHADER: &str = r#"
    attribute vec4 a_vertex;
    uniform mat4 u_translate;
    void main() {
        gl_Position = u_translate * a_vertex;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform vec4 u_maskColor;
    void main() {
        gl_FragColor  = u_maskColor;
    }
"#;

pub fn vertex_shader() -> ShaderSource {
    ShaderSource::VertexShader(String::from(VERTEX_SHADER))
}

pub fn fragment_shader() -> ShaderSource {
    ShaderSource::FragmentShader(String::from(FRAGMENT_SHADER))
}
