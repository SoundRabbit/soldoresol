use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub mod program;

use program::Program;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum ProgramType {
    ShapedProgram,
    UnshapedProgram,
    ScreenProgram,
}

pub struct WebGlF32Vbo(web_sys::WebGlBuffer);
pub struct WebGlI16Ibo(web_sys::WebGlBuffer);
pub struct WebGlAttributeLocation(u32);
pub struct WebGlRenderingContext {
    gl: Rc<web_sys::WebGlRenderingContext>,
    using_program: Option<ProgramType>,
    program_table: HashMap<ProgramType, Box<dyn Program>>,
    depth_func: u32,
}

impl Deref for WebGlF32Vbo {
    type Target = web_sys::WebGlBuffer;

    fn deref(&self) -> &web_sys::WebGlBuffer {
        &self.0
    }
}

impl Deref for WebGlI16Ibo {
    type Target = web_sys::WebGlBuffer;

    fn deref(&self) -> &web_sys::WebGlBuffer {
        &self.0
    }
}

impl Deref for WebGlAttributeLocation {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl Deref for WebGlRenderingContext {
    type Target = web_sys::WebGlRenderingContext;

    fn deref(&self) -> &web_sys::WebGlRenderingContext {
        &self.gl
    }
}

macro_rules! setter {
    (attr $n:ident: WebGlF32Vbo as $a:ident) => {
        pub fn $a(&self, vertex_buffer: &WebGlF32Vbo, size: i32, stride: i32) {
            if let Some(attr_loc) = self.program().and_then(|p| p.$n()) {
                self.set_attr_f32vbo(vertex_buffer, attr_loc, size, stride);
            }
        }
    };

    (unif $n:ident: 1i as $a:ident) => {
        pub fn $a(&self, data: i32) {
            if let Some(unif_loc) = self.program().and_then(|p| p.$n()) {
                self.uniform1i(Some(unif_loc), data);
            }
        }
    };

    (unif $n:ident: 1f as $a:ident) => {
        pub fn $a(&self, data: f32) {
            if let Some(unif_loc) = self.program().and_then(|p| p.$n()) {
                self.uniform1f(Some(unif_loc), data);
            }
        }
    };

    (unif $n:ident: 2fv as $a:ident) => {
        pub fn $a(&self, data: &[f32]) {
            if let Some(unif_loc) = self.program().and_then(|p| p.$n()) {
                self.uniform2fv_with_f32_array(Some(unif_loc), data);
            }
        }
    };

    (unif $n:ident: 3fv as $a:ident) => {
        pub fn $a(&self, data: &[f32]) {
            if let Some(unif_loc) = self.program().and_then(|p| p.$n()) {
                self.uniform3fv_with_f32_array(Some(unif_loc), data);
            }
        }
    };

    (unif $n:ident: 4fv as $a:ident) => {
        pub fn $a(&self, data: &[f32]) {
            if let Some(unif_loc) = self.program().and_then(|p| p.$n()) {
                self.uniform4fv_with_f32_array(Some(unif_loc), data);
            }
        }
    };

    (unif $n:ident: matrix4fv as $a:ident) => {
        pub fn $a(&self, data: ndarray::Array2<f32>) {
            if let Some(unif_loc) = self.program().and_then(|p| p.$n()) {
                self.uniform_matrix4fv_with_f32_array(
                    Some(unif_loc),
                    false,
                    &[
                        data.row(0).to_vec(),
                        data.row(1).to_vec(),
                        data.row(2).to_vec(),
                        data.row(3).to_vec(),
                    ]
                    .concat(),
                );
            }
        }
    };
}

impl WebGlRenderingContext {
    fn program(&self) -> Option<&Box<dyn Program>> {
        if let Some(using_program) = &self.using_program {
            self.program_table.get(using_program)
        } else {
            None
        }
    }

    pub fn new(gl: web_sys::WebGlRenderingContext) -> Self {
        gl.depth_func(web_sys::WebGlRenderingContext::ALWAYS);

        Self {
            gl: Rc::new(gl),
            using_program: None,
            program_table: HashMap::new(),
            depth_func: web_sys::WebGlRenderingContext::ALWAYS,
        }
    }

    pub fn gl(&self) -> Rc<web_sys::WebGlRenderingContext> {
        Rc::clone(&self.gl)
    }

    pub fn depth_func(&mut self, func: u32) {
        if self.depth_func != func {
            self.gl.depth_func(func);
            self.depth_func = func;
        }
    }

    pub fn create_vbo_with_f32array(&self, data: &[f32]) -> WebGlF32Vbo {
        let buffer = self.create_buffer().unwrap();
        self.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let data = js_sys::Float32Array::view(data);

            self.buffer_data_with_array_buffer_view(
                web_sys::WebGlRenderingContext::ARRAY_BUFFER,
                &data,
                web_sys::WebGlRenderingContext::STATIC_DRAW,
            );
        }
        WebGlF32Vbo(buffer)
    }

    pub fn create_ibo_with_i16array(&self, data: &[i16]) -> WebGlI16Ibo {
        let buffer = self.create_buffer().unwrap();
        self.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&buffer),
        );
        unsafe {
            let data = js_sys::Int16Array::view(data);

            self.buffer_data_with_array_buffer_view(
                web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &data,
                web_sys::WebGlRenderingContext::STATIC_DRAW,
            );
        }
        self.bind_buffer(web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
        WebGlI16Ibo(buffer)
    }

    pub fn set_attr_f32vbo(
        &self,
        buffer: &WebGlF32Vbo,
        position: &WebGlAttributeLocation,
        size: i32,
        stride: i32,
    ) {
        let position = (position as &u32).clone();
        self.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        self.enable_vertex_attrib_array(position);
        self.vertex_attrib_pointer_with_i32(
            position,
            size,
            web_sys::WebGlRenderingContext::FLOAT,
            false,
            stride,
            0,
        );
    }

    pub fn use_program(&mut self, program_type: ProgramType) {
        if !self.program_table.contains_key(&program_type) {
            let program = match &program_type {
                ProgramType::ShapedProgram => {
                    Box::new(program::ShapedProgram::new(&self)) as Box<dyn Program>
                }
                ProgramType::UnshapedProgram => {
                    Box::new(program::UnshapedProgram::new(&self)) as Box<dyn Program>
                }
                ProgramType::ScreenProgram => {
                    Box::new(program::ScreenProgram::new(&self)) as Box<dyn Program>
                }
            };
            self.program_table.insert(program_type.clone(), program);
        }

        if self
            .using_program
            .as_ref()
            .map(|using_program| *using_program != program_type)
            .unwrap_or(true)
        {
            let program = self
                .program_table
                .get(&program_type)
                .map(|p| p.as_program());
            self.gl.use_program(program);
        }

        self.using_program = Some(program_type);
    }

    setter!(attr a_id: WebGlF32Vbo as set_a_id);
    setter!(attr a_normal: WebGlF32Vbo as set_a_normal);
    setter!(attr a_texture_coord: WebGlF32Vbo as set_a_texture_coord);
    setter!(attr a_v_color: WebGlF32Vbo as set_a_v_color);
    setter!(attr a_vertex: WebGlF32Vbo as set_a_vertex);

    setter!(unif u_translate: matrix4fv as set_u_translate);
    setter!(unif u_expand: 1f as set_u_expand);

    setter!(unif u_camera_position: 3fv as set_u_camera_position);
    setter!(unif u_inv_model_matrix: matrix4fv as set_u_inv_model_matrix);
    setter!(unif u_model_matrix: matrix4fv as set_u_model_matrix);
    setter!(unif u_vp_matrix: matrix4fv as set_u_vp_matrix);
    setter!(unif u_perspective: 1i as set_u_perspective);

    setter!(unif u_shape: 1i as set_u_shape);
    setter!(unif u_shape_line_width: 1f as set_u_shape_line_width);
    setter!(unif u_shape_scale: 3fv as set_u_shape_scale);

    setter!(unif u_bg_color_1: 1i as set_u_bg_color_1);
    setter!(unif u_bg_color_2: 1i as set_u_bg_color_2);
    setter!(unif u_bg_color_1_value: 4fv as set_u_bg_color_1_value);
    setter!(unif u_bg_color_2_value: 4fv as set_u_bg_color_2_value);

    setter!(unif u_id: 1i as set_u_id);
    setter!(unif u_id_value: 1i as set_u_id_value);

    setter!(unif u_texture_0: 1i as set_u_texture_0);
    setter!(unif u_texture_1: 1i as set_u_texture_1);
    setter!(unif u_texture_2: 1i as set_u_texture_2);
    setter!(unif u_texture_0_sampler: 1i as set_u_texture_0_sampler);
    setter!(unif u_texture_1_sampler: 1i as set_u_texture_1_sampler);
    setter!(unif u_texture_2_sampler: 1i as set_u_texture_2_sampler);
    setter!(unif u_texture_0_text_fill_color: 4fv as set_u_texture_0_text_fill_color);
    setter!(unif u_texture_1_text_fill_color: 4fv as set_u_texture_1_text_fill_color);
    setter!(unif u_texture_2_text_fill_color: 4fv as set_u_texture_2_text_fill_color);
    setter!(unif u_texture_0_text_stroke_color: 4fv as set_u_texture_0_text_stroke_color);
    setter!(unif u_texture_1_text_stroke_color: 4fv as set_u_texture_1_text_stroke_color);
    setter!(unif u_texture_2_text_stroke_color: 4fv as set_u_texture_2_text_stroke_color);

    setter!(unif u_light: 1i as set_u_light);
    setter!(unif u_light_attenation: 1f as set_u_light_attenation);
    setter!(unif u_light_color: 4fv as set_u_light_color);
    setter!(unif u_light_intensity: 1f as set_u_light_intensity);
    setter!(unif u_light_map_nx: 1i as set_u_light_map_nx);
    setter!(unif u_light_map_ny: 1i as set_u_light_map_ny);
    setter!(unif u_light_map_nz: 1i as set_u_light_map_nz);
    setter!(unif u_light_map_px: 1i as set_u_light_map_px);
    setter!(unif u_light_map_py: 1i as set_u_light_map_py);
    setter!(unif u_light_map_pz: 1i as set_u_light_map_pz);
    setter!(unif u_light_position: 3fv as set_u_light_position);
    setter!(unif u_light_vp_nx: matrix4fv as set_u_light_vp_nx);
    setter!(unif u_light_vp_ny: matrix4fv as set_u_light_vp_ny);
    setter!(unif u_light_vp_nz: matrix4fv as set_u_light_vp_nz);
    setter!(unif u_light_vp_px: matrix4fv as set_u_light_vp_px);
    setter!(unif u_light_vp_py: matrix4fv as set_u_light_vp_py);
    setter!(unif u_light_vp_pz: matrix4fv as set_u_light_vp_pz);
    setter!(unif u_shade_intensity: 1f as set_u_shade_intensity);

    setter!(unif u_screen_size: 2fv as set_u_screen_size);
}
