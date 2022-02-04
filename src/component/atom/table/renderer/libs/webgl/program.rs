use super::WebGlAttributeLocation;
use super::WebGlRenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation;

pub const PERSPECTIVE_NORMAL: i32 = 0x00000000;
pub const PERSPECTIVE_PROJECTION: i32 = 0x00000001;

pub const V_COLOR_MASK_NONE: i32 = 0x00000000;
pub const V_COLOR_MASK_SOME: i32 = 0x00000001;

pub const COLOR_NONE: i32 = 0x00000000;
pub const COLOR_SOME: i32 = 0x00000001;

pub const ID_NONE: i32 = 0x00000000;
pub const ID_U_READ: i32 = 0x01000001;
pub const ID_U_WRITE: i32 = 0x01000002;
pub const ID_V_READ: i32 = 0x02000001;
pub const ID_V_WRITE: i32 = 0x02000002;

pub const TEXTURE_NONE: i32 = 0x00000000;
pub const TEXTURE_NORMAL: i32 = 0x00000001;
pub const TEXTURE_MASK: i32 = 0x00000002;
pub const TEXTURE_TEXT: i32 = 0x00000003;

pub const LIGHT_NONE: i32 = 0x00000000;
pub const LIGHT_AMBIENT: i32 = 0x00000001;
pub const LIGHT_POINT_WITH_ID: i32 = 0x01000001;

pub const SHAPE_2D_BOX: i32 = 0x02000000;
pub const SHAPE_2D_CIRCLE: i32 = 0x02000001;
pub const SHAPE_2D_GRID: i32 = 0x02000002;
pub const SHAPE_2D_RING: i32 = 0x02000003;
pub const SHAPE_3D_BOX: i32 = 0x03000000;
pub const SHAPE_3D_SPHERE: i32 = 0x03000001;
pub const SHAPE_3D_CYLINDER: i32 = 0x03000002;

macro_rules! import_shader {
    (frag $s:literal) => {{
        import_shader!("frag", $s)
    }};

    (vert $s:literal) => {{
        import_shader!("vert", $s)
    }};

    ($e:literal, $s:literal) => {{
        concat!(
            include_str!(concat!("./shader/default/", $e, "/head-prelude.", $e)),
            "\n",
            include_str!(concat!("./shader/", $s, "/", $e, "/head-prelude.", $e)),
            "\n",
            include_str!(concat!("./shader/default/", $e, "/head.", $e)),
            "\n",
            include_str!(concat!("./shader/", $s, "/", $e, "/head.", $e)),
            "\n",
            include_str!(concat!("./shader/default/", $e, "/main-prelude.", $e)),
            "\n",
            include_str!(concat!("./shader/", $s, "/", $e, "/main-prelude.", $e)),
            "\n",
            include_str!(concat!("./shader/default/", $e, "/main.", $e)),
            "\n",
            include_str!(concat!("./shader/", $s, "/", $e, "/main.", $e))
        )
    }};
}

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

    ($a:ident : $t:ty as None) => {
        fn $a(&self) -> Option<&$t> {
            None
        }
    };

    ($na:ident : $t:ty) => {
        fn $na(&self) -> Option<&$t> {
            self.$na.as_ref()
        }
    };
}

pub trait Program {
    fn as_program(&self) -> &WebGlProgram;

    // vertシェーダー
    // attr変数
    accesser!(a_id: WebGlAttributeLocation as None);
    accesser!(a_normal: WebGlAttributeLocation as None);
    accesser!(a_texture_coord: WebGlAttributeLocation as None);
    accesser!(a_v_color: WebGlAttributeLocation as None);
    accesser!(a_vertex: WebGlAttributeLocation as None);

    // unif変数
    accesser!(u_translate: WebGlUniformLocation as None);
    accesser!(u_expand: WebGlUniformLocation as None);

    // 頂点色の編集
    accesser!(u_v_color_mask: WebGlUniformLocation as None);
    accesser!(u_v_color_mask_fill_color: WebGlUniformLocation as None);
    accesser!(u_v_color_mask_stroke_color: WebGlUniformLocation as None);

    // fragシェーダー
    // unif変数
    // その他uniform変数
    accesser!(u_camera_position: WebGlUniformLocation as None);
    accesser!(u_inv_model_matrix: WebGlUniformLocation as None);
    accesser!(u_model_matrix: WebGlUniformLocation as None);
    accesser!(u_vp_matrix: WebGlUniformLocation as None);
    accesser!(u_perspective: WebGlUniformLocation as None);

    //形状
    accesser!(u_shape: WebGlUniformLocation as None);
    accesser!(u_shape_line_width: WebGlUniformLocation as None);
    accesser!(u_shape_scale: WebGlUniformLocation as None);

    // 背景色
    accesser!(u_bg_color_1: WebGlUniformLocation as None);
    accesser!(u_bg_color_2: WebGlUniformLocation as None);
    accesser!(u_bg_color_1_value: WebGlUniformLocation as None);
    accesser!(u_bg_color_2_value: WebGlUniformLocation as None);

    // ID
    accesser!(u_id: WebGlUniformLocation as None);
    accesser!(u_id_value: WebGlUniformLocation as None);

    // テクスチャ
    accesser!(u_texture_0: WebGlUniformLocation as None);
    accesser!(u_texture_1: WebGlUniformLocation as None);
    accesser!(u_texture_2: WebGlUniformLocation as None);
    accesser!(u_texture_0_sampler: WebGlUniformLocation as None);
    accesser!(u_texture_1_sampler: WebGlUniformLocation as None);
    accesser!(u_texture_2_sampler: WebGlUniformLocation as None);
    accesser!(u_texture_0_text_fill_color: WebGlUniformLocation as None);
    accesser!(u_texture_1_text_fill_color: WebGlUniformLocation as None);
    accesser!(u_texture_2_text_fill_color: WebGlUniformLocation as None);
    accesser!(u_texture_0_text_stroke_color: WebGlUniformLocation as None);
    accesser!(u_texture_1_text_stroke_color: WebGlUniformLocation as None);
    accesser!(u_texture_2_text_stroke_color: WebGlUniformLocation as None);

    // ライティング／シェ―ディング
    accesser!(u_light: WebGlUniformLocation as None);
    accesser!(u_light_attenation: WebGlUniformLocation as None);
    accesser!(u_light_color: WebGlUniformLocation as None);
    accesser!(u_light_intensity: WebGlUniformLocation as None);
    accesser!(u_light_map_nx: WebGlUniformLocation as None);
    accesser!(u_light_map_ny: WebGlUniformLocation as None);
    accesser!(u_light_map_nz: WebGlUniformLocation as None);
    accesser!(u_light_map_px: WebGlUniformLocation as None);
    accesser!(u_light_map_py: WebGlUniformLocation as None);
    accesser!(u_light_map_pz: WebGlUniformLocation as None);
    accesser!(u_light_position: WebGlUniformLocation as None);
    accesser!(u_light_vp_nx: WebGlUniformLocation as None);
    accesser!(u_light_vp_ny: WebGlUniformLocation as None);
    accesser!(u_light_vp_nz: WebGlUniformLocation as None);
    accesser!(u_light_vp_px: WebGlUniformLocation as None);
    accesser!(u_light_vp_py: WebGlUniformLocation as None);
    accesser!(u_light_vp_pz: WebGlUniformLocation as None);
    accesser!(u_shade_intensity: WebGlUniformLocation as None);

    // 特殊
    accesser!(u_screen_size: WebGlUniformLocation as None);
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
        crate::debug::log_1(shader_source);
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

fn attr(
    name: &str,
    gl: &WebGlRenderingContext,
    program: &web_sys::WebGlProgram,
) -> Option<WebGlAttributeLocation> {
    Some(WebGlAttributeLocation(
        gl.get_attrib_location(&program, name) as u32,
    ))
}

fn unif(
    name: &str,
    gl: &WebGlRenderingContext,
    program: &web_sys::WebGlProgram,
) -> Option<WebGlUniformLocation> {
    gl.get_uniform_location(&program, name)
}

/*----------ShapedProgram----------*/

pub struct ShapedProgram {
    a_id: Option<WebGlAttributeLocation>,
    a_normal: Option<WebGlAttributeLocation>,
    a_texture_coord: Option<WebGlAttributeLocation>,
    a_v_color: Option<WebGlAttributeLocation>,
    a_vertex: Option<WebGlAttributeLocation>,

    u_translate: Option<WebGlUniformLocation>,
    u_expand: Option<WebGlUniformLocation>,
    u_v_color_mask: Option<WebGlUniformLocation>,
    u_v_color_mask_fill_color: Option<WebGlUniformLocation>,
    u_v_color_mask_stroke_color: Option<WebGlUniformLocation>,

    u_camera_position: Option<WebGlUniformLocation>,
    u_inv_model_matrix: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_vp_matrix: Option<WebGlUniformLocation>,
    u_perspective: Option<WebGlUniformLocation>,
    u_shape: Option<WebGlUniformLocation>,
    u_shape_line_width: Option<WebGlUniformLocation>,
    u_shape_scale: Option<WebGlUniformLocation>,
    u_bg_color_1: Option<WebGlUniformLocation>,
    u_bg_color_2: Option<WebGlUniformLocation>,
    u_bg_color_1_value: Option<WebGlUniformLocation>,
    u_bg_color_2_value: Option<WebGlUniformLocation>,
    u_id: Option<WebGlUniformLocation>,
    u_id_value: Option<WebGlUniformLocation>,
    u_texture_0: Option<WebGlUniformLocation>,
    u_texture_1: Option<WebGlUniformLocation>,
    u_texture_2: Option<WebGlUniformLocation>,
    u_texture_0_sampler: Option<WebGlUniformLocation>,
    u_texture_1_sampler: Option<WebGlUniformLocation>,
    u_texture_2_sampler: Option<WebGlUniformLocation>,
    u_texture_0_text_fill_color: Option<WebGlUniformLocation>,
    u_texture_1_text_fill_color: Option<WebGlUniformLocation>,
    u_texture_2_text_fill_color: Option<WebGlUniformLocation>,
    u_texture_0_text_stroke_color: Option<WebGlUniformLocation>,
    u_texture_1_text_stroke_color: Option<WebGlUniformLocation>,
    u_texture_2_text_stroke_color: Option<WebGlUniformLocation>,
    u_light: Option<WebGlUniformLocation>,
    u_light_attenation: Option<WebGlUniformLocation>,
    u_light_color: Option<WebGlUniformLocation>,
    u_light_intensity: Option<WebGlUniformLocation>,
    u_light_map_nx: Option<WebGlUniformLocation>,
    u_light_map_ny: Option<WebGlUniformLocation>,
    u_light_map_nz: Option<WebGlUniformLocation>,
    u_light_map_px: Option<WebGlUniformLocation>,
    u_light_map_py: Option<WebGlUniformLocation>,
    u_light_map_pz: Option<WebGlUniformLocation>,
    u_light_position: Option<WebGlUniformLocation>,
    u_light_vp_nx: Option<WebGlUniformLocation>,
    u_light_vp_ny: Option<WebGlUniformLocation>,
    u_light_vp_nz: Option<WebGlUniformLocation>,
    u_light_vp_px: Option<WebGlUniformLocation>,
    u_light_vp_py: Option<WebGlUniformLocation>,
    u_light_vp_pz: Option<WebGlUniformLocation>,
    u_shade_intensity: Option<WebGlUniformLocation>,

    program: web_sys::WebGlProgram,
}

impl ShapedProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = import_shader!(vert "shaped");
        let frag = import_shader!(frag "shaped");
        let program = create_program(gl, vert, frag);

        Self {
            a_id: attr("a_id", gl, &program),
            a_normal: attr("a_normal", gl, &program),
            a_texture_coord: attr("a_textureCoord", gl, &program),
            a_v_color: attr("a_vColor", gl, &program),
            a_vertex: attr("a_vertex", gl, &program),
            u_translate: unif("u_translate", gl, &program),
            u_expand: unif("u_expand", gl, &program),
            u_v_color_mask: unif("u_vColorMask", gl, &program),
            u_v_color_mask_fill_color: unif("u_vColorMaskFillColor", gl, &program),
            u_v_color_mask_stroke_color: unif("u_vColorMaskStrokeColor", gl, &program),
            u_camera_position: unif("u_cameraPosition", gl, &program),
            u_inv_model_matrix: unif("u_invModelMatrix", gl, &program),
            u_model_matrix: unif("u_modelMatrix", gl, &program),
            u_vp_matrix: unif("u_vpMatrix", gl, &program),
            u_perspective: unif("u_perspective", gl, &program),
            u_shape: unif("u_shape", gl, &program),
            u_shape_line_width: unif("u_shapeLineWidth", gl, &program),
            u_shape_scale: unif("u_shapeScale", gl, &program),
            u_bg_color_1: unif("u_bgColor1", gl, &program),
            u_bg_color_2: unif("u_bgColor2", gl, &program),
            u_bg_color_1_value: unif("u_bgColor1Value", gl, &program),
            u_bg_color_2_value: unif("u_bgColor2Value", gl, &program),
            u_id: unif("u_id", gl, &program),
            u_id_value: unif("u_idValue", gl, &program),
            u_texture_0: unif("u_texture0", gl, &program),
            u_texture_1: unif("u_texture1", gl, &program),
            u_texture_2: unif("u_texture2", gl, &program),
            u_texture_0_sampler: unif("u_texture0Sampler", gl, &program),
            u_texture_1_sampler: unif("u_texture1Sampler", gl, &program),
            u_texture_2_sampler: unif("u_texture2Sampler", gl, &program),
            u_texture_0_text_fill_color: unif("u_texture0TextFillColor", gl, &program),
            u_texture_1_text_fill_color: unif("u_texture1TextFillColor", gl, &program),
            u_texture_2_text_fill_color: unif("u_texture2TextFillColor", gl, &program),
            u_texture_0_text_stroke_color: unif("u_texture0TextStrokeColor", gl, &program),
            u_texture_1_text_stroke_color: unif("u_texture1TextStrokeColor", gl, &program),
            u_texture_2_text_stroke_color: unif("u_texture2TextStrokeColor", gl, &program),
            u_light: unif("u_light", gl, &program),
            u_light_attenation: unif("u_lightAttenation", gl, &program),
            u_light_color: unif("u_lightColor", gl, &program),
            u_light_intensity: unif("u_lightIntensity", gl, &program),
            u_light_map_nx: unif("u_lightMapNx", gl, &program),
            u_light_map_ny: unif("u_lightMapNy", gl, &program),
            u_light_map_nz: unif("u_lightMapNz", gl, &program),
            u_light_map_px: unif("u_lightMapPx", gl, &program),
            u_light_map_py: unif("u_lightMapPy", gl, &program),
            u_light_map_pz: unif("u_lightMapPz", gl, &program),
            u_light_position: unif("u_lightPosition", gl, &program),
            u_light_vp_nx: unif("u_lightVpNx", gl, &program),
            u_light_vp_ny: unif("u_lightVpNy", gl, &program),
            u_light_vp_nz: unif("u_lightVpNz", gl, &program),
            u_light_vp_px: unif("u_lightVpPx", gl, &program),
            u_light_vp_py: unif("u_lightVpPy", gl, &program),
            u_light_vp_pz: unif("u_lightVpPz", gl, &program),
            u_shade_intensity: unif("u_shadeIntensity", gl, &program),
            program,
        }
    }
}

impl Program for ShapedProgram {
    accesser!(program);
    // vertシェーダー
    // attr変数
    accesser!(a_id: WebGlAttributeLocation);
    accesser!(a_normal: WebGlAttributeLocation);
    accesser!(a_texture_coord: WebGlAttributeLocation);
    accesser!(a_v_color: WebGlAttributeLocation);
    accesser!(a_vertex: WebGlAttributeLocation);

    // unif変数
    accesser!(u_translate: WebGlUniformLocation);
    accesser!(u_expand: WebGlUniformLocation);

    // 頂点色の編集
    accesser!(u_v_color_mask: WebGlUniformLocation);
    accesser!(u_v_color_mask_fill_color: WebGlUniformLocation);
    accesser!(u_v_color_mask_stroke_color: WebGlUniformLocation);

    // fragシェーダー
    // unif変数
    // その他uniform変数
    accesser!(u_camera_position: WebGlUniformLocation);
    accesser!(u_inv_model_matrix: WebGlUniformLocation);
    accesser!(u_model_matrix: WebGlUniformLocation);
    accesser!(u_vp_matrix: WebGlUniformLocation);
    accesser!(u_perspective: WebGlUniformLocation);

    //形状
    accesser!(u_shape: WebGlUniformLocation);
    accesser!(u_shape_line_width: WebGlUniformLocation);
    accesser!(u_shape_scale: WebGlUniformLocation);

    // 背景色
    accesser!(u_bg_color_1: WebGlUniformLocation);
    accesser!(u_bg_color_2: WebGlUniformLocation);
    accesser!(u_bg_color_1_value: WebGlUniformLocation);
    accesser!(u_bg_color_2_value: WebGlUniformLocation);

    // ID
    accesser!(u_id: WebGlUniformLocation);
    accesser!(u_id_value: WebGlUniformLocation);

    // テクスチャ
    accesser!(u_texture_0: WebGlUniformLocation);
    accesser!(u_texture_1: WebGlUniformLocation);
    accesser!(u_texture_2: WebGlUniformLocation);
    accesser!(u_texture_0_sampler: WebGlUniformLocation);
    accesser!(u_texture_1_sampler: WebGlUniformLocation);
    accesser!(u_texture_2_sampler: WebGlUniformLocation);
    accesser!(u_texture_0_text_fill_color: WebGlUniformLocation);
    accesser!(u_texture_1_text_fill_color: WebGlUniformLocation);
    accesser!(u_texture_2_text_fill_color: WebGlUniformLocation);
    accesser!(u_texture_0_text_stroke_color: WebGlUniformLocation);
    accesser!(u_texture_1_text_stroke_color: WebGlUniformLocation);
    accesser!(u_texture_2_text_stroke_color: WebGlUniformLocation);

    // ライティング／シェ―ディング
    accesser!(u_light: WebGlUniformLocation);
    accesser!(u_light_attenation: WebGlUniformLocation);
    accesser!(u_light_color: WebGlUniformLocation);
    accesser!(u_light_intensity: WebGlUniformLocation);
    accesser!(u_light_map_nx: WebGlUniformLocation);
    accesser!(u_light_map_ny: WebGlUniformLocation);
    accesser!(u_light_map_nz: WebGlUniformLocation);
    accesser!(u_light_map_px: WebGlUniformLocation);
    accesser!(u_light_map_py: WebGlUniformLocation);
    accesser!(u_light_map_pz: WebGlUniformLocation);
    accesser!(u_light_position: WebGlUniformLocation);
    accesser!(u_light_vp_nx: WebGlUniformLocation);
    accesser!(u_light_vp_ny: WebGlUniformLocation);
    accesser!(u_light_vp_nz: WebGlUniformLocation);
    accesser!(u_light_vp_px: WebGlUniformLocation);
    accesser!(u_light_vp_py: WebGlUniformLocation);
    accesser!(u_light_vp_pz: WebGlUniformLocation);
    accesser!(u_shade_intensity: WebGlUniformLocation);
}

/*----------UnshapedProgram----------*/

pub struct UnshapedProgram {
    a_id: Option<WebGlAttributeLocation>,
    a_normal: Option<WebGlAttributeLocation>,
    a_texture_coord: Option<WebGlAttributeLocation>,
    a_v_color: Option<WebGlAttributeLocation>,
    a_vertex: Option<WebGlAttributeLocation>,

    u_translate: Option<WebGlUniformLocation>,
    u_expand: Option<WebGlUniformLocation>,
    u_v_color_mask: Option<WebGlUniformLocation>,
    u_v_color_mask_fill_color: Option<WebGlUniformLocation>,
    u_v_color_mask_stroke_color: Option<WebGlUniformLocation>,

    u_camera_position: Option<WebGlUniformLocation>,
    u_inv_model_matrix: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_vp_matrix: Option<WebGlUniformLocation>,
    u_perspective: Option<WebGlUniformLocation>,
    u_shape: Option<WebGlUniformLocation>,
    u_shape_line_width: Option<WebGlUniformLocation>,
    u_shape_scale: Option<WebGlUniformLocation>,
    u_bg_color_1: Option<WebGlUniformLocation>,
    u_bg_color_2: Option<WebGlUniformLocation>,
    u_bg_color_1_value: Option<WebGlUniformLocation>,
    u_bg_color_2_value: Option<WebGlUniformLocation>,
    u_id: Option<WebGlUniformLocation>,
    u_id_value: Option<WebGlUniformLocation>,
    u_texture_0: Option<WebGlUniformLocation>,
    u_texture_1: Option<WebGlUniformLocation>,
    u_texture_2: Option<WebGlUniformLocation>,
    u_texture_0_sampler: Option<WebGlUniformLocation>,
    u_texture_1_sampler: Option<WebGlUniformLocation>,
    u_texture_2_sampler: Option<WebGlUniformLocation>,
    u_texture_0_text_fill_color: Option<WebGlUniformLocation>,
    u_texture_1_text_fill_color: Option<WebGlUniformLocation>,
    u_texture_2_text_fill_color: Option<WebGlUniformLocation>,
    u_texture_0_text_stroke_color: Option<WebGlUniformLocation>,
    u_texture_1_text_stroke_color: Option<WebGlUniformLocation>,
    u_texture_2_text_stroke_color: Option<WebGlUniformLocation>,
    u_light: Option<WebGlUniformLocation>,
    u_light_attenation: Option<WebGlUniformLocation>,
    u_light_color: Option<WebGlUniformLocation>,
    u_light_intensity: Option<WebGlUniformLocation>,
    u_light_map_nx: Option<WebGlUniformLocation>,
    u_light_map_ny: Option<WebGlUniformLocation>,
    u_light_map_nz: Option<WebGlUniformLocation>,
    u_light_map_px: Option<WebGlUniformLocation>,
    u_light_map_py: Option<WebGlUniformLocation>,
    u_light_map_pz: Option<WebGlUniformLocation>,
    u_light_position: Option<WebGlUniformLocation>,
    u_light_vp_nx: Option<WebGlUniformLocation>,
    u_light_vp_ny: Option<WebGlUniformLocation>,
    u_light_vp_nz: Option<WebGlUniformLocation>,
    u_light_vp_px: Option<WebGlUniformLocation>,
    u_light_vp_py: Option<WebGlUniformLocation>,
    u_light_vp_pz: Option<WebGlUniformLocation>,
    u_shade_intensity: Option<WebGlUniformLocation>,

    program: web_sys::WebGlProgram,
}

impl UnshapedProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = import_shader!(vert "unshaped");
        let frag = import_shader!(frag "unshaped");
        let program = create_program(gl, vert, frag);

        Self {
            a_id: attr("a_id", gl, &program),
            a_normal: attr("a_normal", gl, &program),
            a_texture_coord: attr("a_textureCoord", gl, &program),
            a_v_color: attr("a_vColor", gl, &program),
            a_vertex: attr("a_vertex", gl, &program),
            u_translate: unif("u_translate", gl, &program),
            u_expand: unif("u_expand", gl, &program),
            u_v_color_mask: unif("u_vColorMask", gl, &program),
            u_v_color_mask_fill_color: unif("u_vColorMaskFillColor", gl, &program),
            u_v_color_mask_stroke_color: unif("u_vColorMaskStrokeColor", gl, &program),
            u_camera_position: unif("u_cameraPosition", gl, &program),
            u_inv_model_matrix: unif("u_invModelMatrix", gl, &program),
            u_perspective: unif("u_perspective", gl, &program),
            u_model_matrix: unif("u_modelMatrix", gl, &program),
            u_shape: unif("u_shape", gl, &program),
            u_shape_line_width: unif("u_shapeLineWidth", gl, &program),
            u_shape_scale: unif("u_shapeScale", gl, &program),
            u_vp_matrix: unif("u_vpMatrix", gl, &program),
            u_bg_color_1: unif("u_bgColor1", gl, &program),
            u_bg_color_2: unif("u_bgColor2", gl, &program),
            u_bg_color_1_value: unif("u_bgColor1Value", gl, &program),
            u_bg_color_2_value: unif("u_bgColor2Value", gl, &program),
            u_id: unif("u_id", gl, &program),
            u_id_value: unif("u_idValue", gl, &program),
            u_texture_0: unif("u_texture0", gl, &program),
            u_texture_1: unif("u_texture1", gl, &program),
            u_texture_2: unif("u_texture2", gl, &program),
            u_texture_0_sampler: unif("u_texture0Sampler", gl, &program),
            u_texture_1_sampler: unif("u_texture1Sampler", gl, &program),
            u_texture_2_sampler: unif("u_texture2Sampler", gl, &program),
            u_texture_0_text_fill_color: unif("u_texture0TextFillColor", gl, &program),
            u_texture_1_text_fill_color: unif("u_texture1TextFillColor", gl, &program),
            u_texture_2_text_fill_color: unif("u_texture2TextFillColor", gl, &program),
            u_texture_0_text_stroke_color: unif("u_texture0TextStrokeColor", gl, &program),
            u_texture_1_text_stroke_color: unif("u_texture1TextStrokeColor", gl, &program),
            u_texture_2_text_stroke_color: unif("u_texture2TextStrokeColor", gl, &program),
            u_light: unif("u_light", gl, &program),
            u_light_attenation: unif("u_lightAttenation", gl, &program),
            u_light_color: unif("u_lightColor", gl, &program),
            u_light_intensity: unif("u_lightIntensity", gl, &program),
            u_light_map_nx: unif("u_lightMapNx", gl, &program),
            u_light_map_ny: unif("u_lightMapNy", gl, &program),
            u_light_map_nz: unif("u_lightMapNz", gl, &program),
            u_light_map_px: unif("u_lightMapPx", gl, &program),
            u_light_map_py: unif("u_lightMapPy", gl, &program),
            u_light_map_pz: unif("u_lightMapPz", gl, &program),
            u_light_position: unif("u_lightPosition", gl, &program),
            u_light_vp_nx: unif("u_lightVpNx", gl, &program),
            u_light_vp_ny: unif("u_lightVpNy", gl, &program),
            u_light_vp_nz: unif("u_lightVpNz", gl, &program),
            u_light_vp_px: unif("u_lightVpPx", gl, &program),
            u_light_vp_py: unif("u_lightVpPy", gl, &program),
            u_light_vp_pz: unif("u_lightVpPz", gl, &program),
            u_shade_intensity: unif("u_shadeIntensity", gl, &program),
            program,
        }
    }
}

impl Program for UnshapedProgram {
    accesser!(program);
    // vertシェーダー
    // attr変数
    accesser!(a_id: WebGlAttributeLocation);
    accesser!(a_normal: WebGlAttributeLocation);
    accesser!(a_texture_coord: WebGlAttributeLocation);
    accesser!(a_v_color: WebGlAttributeLocation);
    accesser!(a_vertex: WebGlAttributeLocation);

    // unif変数
    accesser!(u_translate: WebGlUniformLocation);
    accesser!(u_expand: WebGlUniformLocation);

    // 頂点色の編集
    accesser!(u_v_color_mask: WebGlUniformLocation);
    accesser!(u_v_color_mask_fill_color: WebGlUniformLocation);
    accesser!(u_v_color_mask_stroke_color: WebGlUniformLocation);

    // fragシェーダー
    // unif変数
    // その他uniform変数
    accesser!(u_camera_position: WebGlUniformLocation);
    accesser!(u_inv_model_matrix: WebGlUniformLocation);
    accesser!(u_model_matrix: WebGlUniformLocation);
    accesser!(u_vp_matrix: WebGlUniformLocation);
    accesser!(u_perspective: WebGlUniformLocation);

    //形状
    accesser!(u_shape: WebGlUniformLocation);
    accesser!(u_shape_line_width: WebGlUniformLocation);
    accesser!(u_shape_scale: WebGlUniformLocation);

    // 背景色
    accesser!(u_bg_color_1: WebGlUniformLocation);
    accesser!(u_bg_color_2: WebGlUniformLocation);
    accesser!(u_bg_color_1_value: WebGlUniformLocation);
    accesser!(u_bg_color_2_value: WebGlUniformLocation);

    // ID
    accesser!(u_id: WebGlUniformLocation);
    accesser!(u_id_value: WebGlUniformLocation);

    // テクスチャ
    accesser!(u_texture_0: WebGlUniformLocation);
    accesser!(u_texture_1: WebGlUniformLocation);
    accesser!(u_texture_2: WebGlUniformLocation);
    accesser!(u_texture_0_sampler: WebGlUniformLocation);
    accesser!(u_texture_1_sampler: WebGlUniformLocation);
    accesser!(u_texture_2_sampler: WebGlUniformLocation);
    accesser!(u_texture_0_text_fill_color: WebGlUniformLocation);
    accesser!(u_texture_1_text_fill_color: WebGlUniformLocation);
    accesser!(u_texture_2_text_fill_color: WebGlUniformLocation);
    accesser!(u_texture_0_text_stroke_color: WebGlUniformLocation);
    accesser!(u_texture_1_text_stroke_color: WebGlUniformLocation);
    accesser!(u_texture_2_text_stroke_color: WebGlUniformLocation);

    // ライティング／シェ―ディング
    accesser!(u_light: WebGlUniformLocation);
    accesser!(u_light_attenation: WebGlUniformLocation);
    accesser!(u_light_color: WebGlUniformLocation);
    accesser!(u_light_intensity: WebGlUniformLocation);
    accesser!(u_light_map_nx: WebGlUniformLocation);
    accesser!(u_light_map_ny: WebGlUniformLocation);
    accesser!(u_light_map_nz: WebGlUniformLocation);
    accesser!(u_light_map_px: WebGlUniformLocation);
    accesser!(u_light_map_py: WebGlUniformLocation);
    accesser!(u_light_map_pz: WebGlUniformLocation);
    accesser!(u_light_position: WebGlUniformLocation);
    accesser!(u_light_vp_nx: WebGlUniformLocation);
    accesser!(u_light_vp_ny: WebGlUniformLocation);
    accesser!(u_light_vp_nz: WebGlUniformLocation);
    accesser!(u_light_vp_px: WebGlUniformLocation);
    accesser!(u_light_vp_py: WebGlUniformLocation);
    accesser!(u_light_vp_pz: WebGlUniformLocation);
    accesser!(u_shade_intensity: WebGlUniformLocation);
}

/*----------ScreenProgram----------*/

pub struct ScreenProgram {
    a_texture_coord: Option<WebGlAttributeLocation>,
    a_vertex: Option<WebGlAttributeLocation>,
    u_texture_0_sampler: Option<WebGlUniformLocation>,
    u_screen_size: Option<WebGlUniformLocation>,

    program: web_sys::WebGlProgram,
}

impl ScreenProgram {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vert = include_str!("./shader/screen.vert");
        let frag = include_str!("./shader/screen.frag");
        let program = create_program(gl, vert, frag);

        Self {
            a_texture_coord: attr("a_textureCoord", gl, &program),
            a_vertex: attr("a_vertex", gl, &program),
            u_texture_0_sampler: unif("u_texture0Sampler", gl, &program),
            u_screen_size: unif("u_screenSize", gl, &program),
            program,
        }
    }
}

impl Program for ScreenProgram {
    accesser!(program);
    // vertシェーダー
    // attr変数
    accesser!(a_texture_coord: WebGlAttributeLocation);
    accesser!(a_vertex: WebGlAttributeLocation);

    // unif変数
    accesser!(u_texture_0_sampler: WebGlUniformLocation);
    accesser!(u_screen_size: WebGlUniformLocation);
}
