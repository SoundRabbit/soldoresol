use super::WebGlAttributeLocation;
use super::WebGlRenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation;

pub trait Program {
    fn as_program(&self) -> &WebGlProgram;
    fn attr_vertex(&self) -> Option<&WebGlAttributeLocation>;
    fn attr_tex_coord(&self) -> Option<&WebGlAttributeLocation>;
}

pub struct CharacterProgram {
    program: WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
    u_texture_location: WebGlUniformLocation,
}

fn compile_shader(
    context: &web_sys::WebGlRenderingContext,
    shader_source: &str,
    shader_type: u32,
) -> Result<web_sys::WebGlShader, String> {
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
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &web_sys::WebGlRenderingContext,
    vertex_shader: &web_sys::WebGlShader,
    fragment_shader: &web_sys::WebGlShader,
) -> Result<web_sys::WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

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
}

fn create_program(
    gl: &web_sys::WebGlRenderingContext,
    vert: &str,
    frag: &str,
) -> web_sys::WebGlProgram {
    let vert = compile_shader(gl, vert, web_sys::WebGlRenderingContext::VERTEX_SHADER).unwrap();
    let frag = compile_shader(gl, frag, web_sys::WebGlRenderingContext::FRAGMENT_SHADER).unwrap();
    let program = link_program(gl, &vert, &frag).unwrap();
    program
}

impl CharacterProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/character.vert");
        let frag = include_str!("./shader/character.frag");
        let program = create_program(gl, vert, frag);

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
}

impl Program for CharacterProgram {
    fn as_program(&self) -> &web_sys::WebGlProgram {
        &self.program
    }

    fn attr_vertex(&self) -> Option<&WebGlAttributeLocation> {
        Some(&self.a_vertex_location)
    }

    fn attr_tex_coord(&self) -> Option<&WebGlAttributeLocation> {
        Some(&self.a_texture_coord_location)
    }
}
