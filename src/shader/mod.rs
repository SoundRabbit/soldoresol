pub mod default;

pub enum ShaderSource {
    VertexShader(String),
    FragmentShader(String),
}

pub enum Shader {
    VertexShader(web_sys::WebGlShader),
    FragmentShader(web_sys::WebGlShader),
}

pub fn compile_shader(
    context: &web_sys::WebGlRenderingContext,
    shader: &ShaderSource,
) -> Result<Shader, String> {
    let (shader_source, shader_type) = match shader {
        ShaderSource::VertexShader(src) => (src, web_sys::WebGlRenderingContext::VERTEX_SHADER),
        ShaderSource::FragmentShader(src) => (src, web_sys::WebGlRenderingContext::FRAGMENT_SHADER),
    };
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, shader_source);
    context.compile_shader(&shader);
    if context
        .get_shader_parameter(&shader, web_sys::WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        if shader_type == web_sys::WebGlRenderingContext::VERTEX_SHADER {
            Ok(Shader::VertexShader(shader))
        } else {
            Ok(Shader::FragmentShader(shader))
        }
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &web_sys::WebGlRenderingContext,
    vertex_shader: &Shader,
    fragment_shader: &Shader,
) -> Result<web_sys::WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    if let (Shader::VertexShader(vertex_shader), Shader::FragmentShader(fragment_shader)) =
        (vertex_shader, fragment_shader)
    {
        context.attach_shader(&program, vertex_shader);
        context.attach_shader(&program, fragment_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, web_sys::WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    } else {
        Err(String::from("Shader type is unmatched"))
    }
}
