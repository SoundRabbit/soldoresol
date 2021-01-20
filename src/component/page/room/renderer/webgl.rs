use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

mod program;

use program::Program;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum ProgramType {
    CharacterProgram,
}

pub struct WebGlF32Vbo(web_sys::WebGlBuffer);
pub struct WebGlI16Ibo(web_sys::WebGlBuffer);
pub struct WebGlAttributeLocation(u32);
pub struct WebGlRenderingContext {
    gl: Rc<web_sys::WebGlRenderingContext>,
    using_program: Option<ProgramType>,
    program_table: HashMap<ProgramType, Box<dyn Program>>,
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

impl WebGlRenderingContext {
    fn program(&self) -> Option<&Box<dyn Program>> {
        if let Some(using_program) = &self.using_program {
            self.program_table.get(using_program)
        } else {
            None
        }
    }

    pub fn new(gl: web_sys::WebGlRenderingContext) -> Self {
        Self {
            gl: Rc::new(gl),
            using_program: None,
            program_table: HashMap::new(),
        }
    }

    pub fn gl(&self) -> Rc<web_sys::WebGlRenderingContext> {
        Rc::clone(&self.gl)
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
                ProgramType::CharacterProgram => program::CharacterProgram::new(&self),
            };
            self.program_table
                .insert(program_type.clone(), Box::new(program) as Box<dyn Program>);
        }

        let program = self
            .program_table
            .get(&program_type)
            .map(|p| p.as_program());
        self.gl.use_program(program);

        self.using_program = Some(program_type);
    }

    pub fn set_attr_vertex(&self, vertex_buffer: &WebGlF32Vbo, size: i32, stride: i32) {
        if let Some(attr_loc) = self.program().and_then(|p| p.attr_vertex()) {
            self.set_attr_f32vbo(vertex_buffer, attr_loc, size, stride);
        }
    }

    pub fn set_attr_tex_coord(&self, tex_coord_buffer: &WebGlF32Vbo, size: i32, stride: i32) {
        if let Some(attr_loc) = self.program().and_then(|p| p.attr_tex_coord()) {
            self.set_attr_f32vbo(tex_coord_buffer, attr_loc, size, stride);
        }
    }
}
