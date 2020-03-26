use super::ShaderSource;

const VERTEX_SHADER: &str = r#"
    attribute vec4 position;
    uniform mat4 u_translate;
    void main() {
        gl_Position = u_translate * position;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

pub fn vertex_shader() -> ShaderSource {
    ShaderSource::VertexShader(String::from(VERTEX_SHADER))
}

pub fn fragment_shader() -> ShaderSource {
    ShaderSource::FragmentShader(String::from(FRAGMENT_SHADER))
}
