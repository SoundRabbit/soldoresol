use super::ShaderSource;

const VERTEX_SHADER: &str = r#"
    attribute vec4 position;
    void main() {
        gl_Position = position;
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
