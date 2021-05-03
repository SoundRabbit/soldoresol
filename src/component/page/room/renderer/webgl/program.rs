use super::WebGlAttributeLocation;
use super::WebGlRenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation;

macro_rules! accesser {
    (program) => {
        fn as_program(&self) -> &web_sys::WebGlProgram {
            &self.program
        }
    };

    (None as $a:ident : $t:ty) => {
        fn $a(&self) -> Option<&$t> {
            None
        }
    };

    ($n:ident as $a:ident : $t:ty) => {
        fn $a(&self) -> Option<&$t> {
            Some(&self.$n)
        }
    };
}

pub trait Program {
    fn as_program(&self) -> &WebGlProgram;

    accesser!(None as attr_normal: WebGlAttributeLocation);
    accesser!(None as attr_tex_coord: WebGlAttributeLocation);
    accesser!(None as attr_vertex: WebGlAttributeLocation);

    accesser!(None as unif_area_size: WebGlUniformLocation);
    accesser!(None as unif_attenation: WebGlUniformLocation);
    accesser!(None as unif_bg_color: WebGlUniformLocation);
    accesser!(None as unif_bg_color_1: WebGlUniformLocation);
    accesser!(None as unif_bg_color_2: WebGlUniformLocation);
    accesser!(None as unif_flag_round: WebGlUniformLocation);
    accesser!(None as unif_inv_model: WebGlUniformLocation);
    accesser!(None as unif_is_shadowmap: WebGlUniformLocation);
    accesser!(None as unif_light: WebGlUniformLocation);
    accesser!(None as unif_light_color: WebGlUniformLocation);
    accesser!(None as unif_light_intensity: WebGlUniformLocation);
    accesser!(None as unif_light_vp_px: WebGlUniformLocation);
    accesser!(None as unif_light_vp_py: WebGlUniformLocation);
    accesser!(None as unif_light_vp_pz: WebGlUniformLocation);
    accesser!(None as unif_light_vp_nx: WebGlUniformLocation);
    accesser!(None as unif_light_vp_ny: WebGlUniformLocation);
    accesser!(None as unif_light_vp_nz: WebGlUniformLocation);
    accesser!(None as unif_model: WebGlUniformLocation);
    accesser!(None as unif_object_type: WebGlUniformLocation);
    accesser!(None as unif_point_size: WebGlUniformLocation);
    accesser!(None as unif_shade_intensity: WebGlUniformLocation);
    accesser!(None as unif_shadowmap_px: WebGlUniformLocation);
    accesser!(None as unif_shadowmap_py: WebGlUniformLocation);
    accesser!(None as unif_shadowmap_pz: WebGlUniformLocation);
    accesser!(None as unif_shadowmap_nx: WebGlUniformLocation);
    accesser!(None as unif_shadowmap_ny: WebGlUniformLocation);
    accesser!(None as unif_shadowmap_nz: WebGlUniformLocation);
    accesser!(None as unif_screen_size: WebGlUniformLocation);
    accesser!(None as unif_text_color_1: WebGlUniformLocation);
    accesser!(None as unif_text_color_2: WebGlUniformLocation);
    accesser!(None as unif_texture: WebGlUniformLocation);
    accesser!(None as unif_texture_1: WebGlUniformLocation);
    accesser!(None as unif_texture_2: WebGlUniformLocation);
    accesser!(None as unif_texture_2_is_available: WebGlUniformLocation);
    accesser!(None as unif_translate: WebGlUniformLocation);
    accesser!(None as unif_use_texture_as_mask: WebGlUniformLocation);
    accesser!(None as unif_vp: WebGlUniformLocation);
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
        let info_log = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));
        crate::debug::log_1(&info_log);
        Err(info_log)
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
        let info_log = context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"));
        crate::debug::log_1(&info_log);
        Err(info_log)
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

/*----------AreaProgram----------*/

pub struct AreaProgram {
    program: web_sys::WebGlProgram,
    pub a_vertex_location: WebGlAttributeLocation,
    pub a_texture_coord_location: WebGlAttributeLocation,
    pub u_translate_location: WebGlUniformLocation,
    pub u_bg_color_1_location: WebGlUniformLocation,
    pub u_bg_color_2_location: WebGlUniformLocation,
    pub u_area_size_location: WebGlUniformLocation,
    pub u_flag_round_location: WebGlUniformLocation,
}

impl AreaProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/area.vert");
        let frag = include_str!("./shader/area.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_bg_color_1_location = gl.get_uniform_location(&program, "u_bgColor1").unwrap();
        let u_bg_color_2_location = gl.get_uniform_location(&program, "u_bgColor2").unwrap();
        let u_area_size_location = gl.get_uniform_location(&program, "u_areaSize").unwrap();
        let u_flag_round_location = gl.get_uniform_location(&program, "u_flagRound").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_bg_color_1_location,
            u_bg_color_2_location,
            u_area_size_location,
            u_flag_round_location,
        }
    }
}

impl Program for AreaProgram {
    accesser!(program);
    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_area_size_location as unif_area_size: WebGlUniformLocation);
    accesser!(u_bg_color_1_location as unif_bg_color_1: WebGlUniformLocation);
    accesser!(u_bg_color_2_location as unif_bg_color_2: WebGlUniformLocation);
    accesser!(u_flag_round_location as unif_flag_round: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
}

/*----------BoxblockProgram----------*/

pub struct BoxblockProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_normal_location: WebGlAttributeLocation,
    u_model_location: WebGlUniformLocation,
    u_vp_location: WebGlUniformLocation,
    u_inv_model_location: WebGlUniformLocation,
    u_light_location: WebGlUniformLocation,
    u_light_color_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
    u_shade_intensity_location: WebGlUniformLocation,
    u_light_intensity_location: WebGlUniformLocation,
    u_is_shadowmap_location: WebGlUniformLocation,
    u_light_vp_px_location: WebGlUniformLocation,
    u_light_vp_py_location: WebGlUniformLocation,
    u_light_vp_pz_location: WebGlUniformLocation,
    u_light_vp_nx_location: WebGlUniformLocation,
    u_light_vp_ny_location: WebGlUniformLocation,
    u_light_vp_nz_location: WebGlUniformLocation,
    u_shadowmap_px_location: WebGlUniformLocation,
    u_shadowmap_py_location: WebGlUniformLocation,
    u_shadowmap_pz_location: WebGlUniformLocation,
    u_shadowmap_nx_location: WebGlUniformLocation,
    u_shadowmap_ny_location: WebGlUniformLocation,
    u_shadowmap_nz_location: WebGlUniformLocation,
    u_attenation_location: WebGlUniformLocation,
}

impl BoxblockProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/boxblock.vert");
        let frag = include_str!("./shader/boxblock.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_normal_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_normal") as u32);

        let u_model_location = gl.get_uniform_location(&program, "u_model").unwrap();
        let u_vp_location = gl.get_uniform_location(&program, "u_vp").unwrap();
        let u_inv_model_location = gl.get_uniform_location(&program, "u_invModel").unwrap();
        let u_light_location = gl.get_uniform_location(&program, "u_light").unwrap();
        let u_light_color_location = gl.get_uniform_location(&program, "u_lightColor").unwrap();
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();
        let u_shade_intensity_location = gl
            .get_uniform_location(&program, "u_shadeIntensity")
            .unwrap();
        let u_light_intensity_location = gl
            .get_uniform_location(&program, "u_lightIntensity")
            .unwrap();
        let u_is_shadowmap_location = gl.get_uniform_location(&program, "u_isShadowmap").unwrap();
        let u_light_vp_px_location = gl.get_uniform_location(&program, "u_lightVpPx").unwrap();
        let u_light_vp_py_location = gl.get_uniform_location(&program, "u_lightVpPy").unwrap();
        let u_light_vp_pz_location = gl.get_uniform_location(&program, "u_lightVpPz").unwrap();
        let u_light_vp_nx_location = gl.get_uniform_location(&program, "u_lightVpNx").unwrap();
        let u_light_vp_ny_location = gl.get_uniform_location(&program, "u_lightVpNy").unwrap();
        let u_light_vp_nz_location = gl.get_uniform_location(&program, "u_lightVpNz").unwrap();
        let u_shadowmap_px_location = gl.get_uniform_location(&program, "u_shadowmapPx").unwrap();
        let u_shadowmap_py_location = gl.get_uniform_location(&program, "u_shadowmapPy").unwrap();
        let u_shadowmap_pz_location = gl.get_uniform_location(&program, "u_shadowmapPz").unwrap();
        let u_shadowmap_nx_location = gl.get_uniform_location(&program, "u_shadowmapNx").unwrap();
        let u_shadowmap_ny_location = gl.get_uniform_location(&program, "u_shadowmapNy").unwrap();
        let u_shadowmap_nz_location = gl.get_uniform_location(&program, "u_shadowmapNz").unwrap();

        let u_attenation_location = gl.get_uniform_location(&program, "u_attenation").unwrap();

        Self {
            program,
            a_vertex_location,
            a_normal_location,
            u_model_location,
            u_vp_location,
            u_inv_model_location,
            u_light_location,
            u_light_color_location,
            u_bg_color_location,
            u_shade_intensity_location,
            u_light_intensity_location,
            u_is_shadowmap_location,
            u_light_vp_px_location,
            u_light_vp_py_location,
            u_light_vp_pz_location,
            u_light_vp_nx_location,
            u_light_vp_ny_location,
            u_light_vp_nz_location,
            u_shadowmap_px_location,
            u_shadowmap_py_location,
            u_shadowmap_pz_location,
            u_shadowmap_nx_location,
            u_shadowmap_ny_location,
            u_shadowmap_nz_location,
            u_attenation_location,
        }
    }
}

impl Program for BoxblockProgram {
    accesser!(program);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(a_normal_location as attr_normal: WebGlAttributeLocation);
    accesser!(u_model_location as unif_model: WebGlUniformLocation);
    accesser!(u_vp_location as unif_vp: WebGlUniformLocation);
    accesser!(u_inv_model_location as unif_inv_model: WebGlUniformLocation);
    accesser!(u_light_location as unif_light: WebGlUniformLocation);
    accesser!(u_light_color_location as unif_light_color: WebGlUniformLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_shade_intensity_location as unif_shade_intensity: WebGlUniformLocation);
    accesser!(u_light_intensity_location as unif_light_intensity: WebGlUniformLocation);
    accesser!(u_is_shadowmap_location as unif_is_shadowmap: WebGlUniformLocation);
    accesser!(u_light_vp_px_location as unif_light_vp_px: WebGlUniformLocation);
    accesser!(u_light_vp_py_location as unif_light_vp_py: WebGlUniformLocation);
    accesser!(u_light_vp_pz_location as unif_light_vp_pz: WebGlUniformLocation);
    accesser!(u_light_vp_nx_location as unif_light_vp_nx: WebGlUniformLocation);
    accesser!(u_light_vp_ny_location as unif_light_vp_ny: WebGlUniformLocation);
    accesser!(u_light_vp_nz_location as unif_light_vp_nz: WebGlUniformLocation);
    accesser!(u_shadowmap_px_location as unif_shadowmap_px: WebGlUniformLocation);
    accesser!(u_shadowmap_py_location as unif_shadowmap_py: WebGlUniformLocation);
    accesser!(u_shadowmap_pz_location as unif_shadowmap_pz: WebGlUniformLocation);
    accesser!(u_shadowmap_nx_location as unif_shadowmap_nx: WebGlUniformLocation);
    accesser!(u_shadowmap_ny_location as unif_shadowmap_ny: WebGlUniformLocation);
    accesser!(u_shadowmap_nz_location as unif_shadowmap_nz: WebGlUniformLocation);
    accesser!(u_attenation_location as unif_attenation: WebGlUniformLocation);
}

/*----------CharacterProgram----------*/

pub struct CharacterProgram {
    program: WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_text_color_1_location: WebGlUniformLocation,
    u_text_color_2_location: WebGlUniformLocation,
    u_translate_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
    u_texture_location: WebGlUniformLocation,
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
        let u_text_color_1_location = gl.get_uniform_location(&program, "u_textColor1").unwrap();
        let u_text_color_2_location = gl.get_uniform_location(&program, "u_textColor2").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_bg_color_location,
            u_texture_location,
            u_text_color_1_location,
            u_text_color_2_location,
        }
    }
}

impl Program for CharacterProgram {
    accesser!(program);
    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_texture_location as unif_texture: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
    accesser!(u_text_color_1_location as unif_text_color_1: WebGlUniformLocation);
    accesser!(u_text_color_2_location as unif_text_color_2: WebGlUniformLocation);
}

/*----------DefaultProgram----------*/

pub struct DefaultProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
}

impl DefaultProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/default.vert");
        let frag = include_str!("./shader/default.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();

        Self {
            program,
            a_vertex_location,
            u_translate_location,
            u_bg_color_location,
        }
    }
}

impl Program for DefaultProgram {
    accesser!(program);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
}

/*----------NamePlateProgram----------*/

pub struct NamePlateProgram {
    program: WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_text_color_1_location: WebGlUniformLocation,
    u_text_color_2_location: WebGlUniformLocation,
    u_translate_location: WebGlUniformLocation,
    u_texture_location: WebGlUniformLocation,
}

impl NamePlateProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/default.vert");
        let frag = include_str!("./shader/name_plate.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_texture_location = gl.get_uniform_location(&program, "u_texture").unwrap();
        let u_text_color_1_location = gl.get_uniform_location(&program, "u_textColor1").unwrap();
        let u_text_color_2_location = gl.get_uniform_location(&program, "u_textColor2").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_texture_location,
            u_text_color_1_location,
            u_text_color_2_location,
        }
    }
}

impl Program for NamePlateProgram {
    accesser!(program);
    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_texture_location as unif_texture: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
    accesser!(u_text_color_1_location as unif_text_color_1: WebGlUniformLocation);
    accesser!(u_text_color_2_location as unif_text_color_2: WebGlUniformLocation);
}

/*----------OffscreenProgram----------*/

pub struct OffscreenProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
    u_flag_round_location: WebGlUniformLocation,
}

impl OffscreenProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/offscreen.vert");
        let frag = include_str!("./shader/offscreen.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();
        let u_flag_round_location = gl.get_uniform_location(&program, "u_flagRound").unwrap();

        Self {
            program,
            a_texture_coord_location,
            a_vertex_location,
            u_bg_color_location,
            u_flag_round_location,
            u_translate_location,
        }
    }
}

impl Program for OffscreenProgram {
    accesser!(program);
    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_flag_round_location as unif_flag_round: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
}

/*----------ScreenProgram----------*/

pub struct ScreenProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_texture_location: WebGlUniformLocation,
    u_screen_size_location: WebGlUniformLocation,
}

impl ScreenProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/screen.vert");
        let frag = include_str!("./shader/screen.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_screen_size_location = gl.get_uniform_location(&program, "u_screenSize").unwrap();
        let u_texture_location = gl.get_uniform_location(&program, "u_texture").unwrap();

        Self {
            program,
            a_texture_coord_location,
            a_vertex_location,
            u_texture_location,
            u_screen_size_location,
        }
    }
}

impl Program for ScreenProgram {
    accesser!(program);
    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_screen_size_location as unif_screen_size: WebGlUniformLocation);
    accesser!(u_texture_location as unif_texture: WebGlUniformLocation);
}

/*----------ShadowmapProgram----------*/

pub struct ShadowmapProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
}

impl ShadowmapProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/shadowmap.vert");
        let frag = include_str!("./shader/shadowmap.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();

        Self {
            program,
            a_vertex_location,
            u_translate_location,
        }
    }
}

impl Program for ShadowmapProgram {
    accesser!(program);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
}

/*----------TablegridProgram----------*/

pub struct TablegridProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
    u_point_size_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
}

impl TablegridProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/tablegrid.vert");
        let frag = include_str!("./shader/tablegrid.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_point_size_location = gl.get_uniform_location(&program, "u_pointSize").unwrap();
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();

        Self {
            program,
            a_vertex_location,
            u_bg_color_location,
            u_point_size_location,
            u_translate_location,
        }
    }
}

impl Program for TablegridProgram {
    accesser!(program);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_point_size_location as unif_point_size: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
}

/*----------TablemaskProgram----------*/

pub struct TablemaskProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
    u_flag_round_location: WebGlUniformLocation,
}

impl TablemaskProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/tablemask.vert");
        let frag = include_str!("./shader/tablemask.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();
        let u_flag_round_location = gl.get_uniform_location(&program, "u_flagRound").unwrap();

        Self {
            program,
            a_vertex_location,
            a_texture_coord_location,
            u_translate_location,
            u_bg_color_location,
            u_flag_round_location,
        }
    }
}

impl Program for TablemaskProgram {
    accesser!(program);

    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
    accesser!(u_flag_round_location as unif_flag_round: WebGlUniformLocation);
}

/*----------TabletextureProgram----------*/

pub struct TabletextureProgram {
    program: web_sys::WebGlProgram,
    a_vertex_location: WebGlAttributeLocation,
    a_texture_coord_location: WebGlAttributeLocation,
    u_translate_location: WebGlUniformLocation,
    u_bg_color_location: WebGlUniformLocation,
    u_texture_location: WebGlUniformLocation,
    u_texture_1_location: WebGlUniformLocation,
    u_texture_2_location: WebGlUniformLocation,
    u_texture_2_is_available_location: WebGlUniformLocation,
    u_use_texture_as_mask_location: WebGlUniformLocation,
}

impl TabletextureProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/tabletexture.vert");
        let frag = include_str!("./shader/tabletexture.frag");
        let program = create_program(gl, vert, frag);

        let a_vertex_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_vertex") as u32);
        let a_texture_coord_location =
            WebGlAttributeLocation(gl.get_attrib_location(&program, "a_textureCoord") as u32);
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        let u_bg_color_location = gl.get_uniform_location(&program, "u_bgColor").unwrap();
        let u_texture_location = gl.get_uniform_location(&program, "u_texture").unwrap();
        let u_texture_1_location = gl.get_uniform_location(&program, "u_texture1").unwrap();
        let u_texture_2_location = gl.get_uniform_location(&program, "u_texture2").unwrap();
        let u_texture_2_is_available_location = gl
            .get_uniform_location(&program, "u_texture2IsAvailable")
            .unwrap();
        let u_use_texture_as_mask_location = gl
            .get_uniform_location(&program, "u_useTextureAsMask")
            .unwrap();

        Self {
            program,
            a_texture_coord_location,
            a_vertex_location,
            u_bg_color_location,
            u_texture_location,
            u_texture_2_location,
            u_texture_1_location,
            u_texture_2_is_available_location,
            u_translate_location,
            u_use_texture_as_mask_location,
        }
    }
}

impl Program for TabletextureProgram {
    accesser!(program);

    accesser!(a_texture_coord_location as attr_tex_coord: WebGlAttributeLocation);
    accesser!(a_vertex_location as attr_vertex: WebGlAttributeLocation);
    accesser!(u_bg_color_location as unif_bg_color: WebGlUniformLocation);
    accesser!(u_texture_location as unif_texture: WebGlUniformLocation);
    accesser!(u_texture_1_location as unif_texture_1: WebGlUniformLocation);
    accesser!(u_texture_2_location as unif_texture_2: WebGlUniformLocation);
    accesser!(
        u_texture_2_is_available_location as unif_texture_2_is_available: WebGlUniformLocation
    );
    accesser!(u_translate_location as unif_translate: WebGlUniformLocation);
    accesser!(u_use_texture_as_mask_location as unif_use_texture_as_mask: WebGlUniformLocation);
}
