use crate::shader;

pub struct Table {
    camera_matrix: [[f32; 4]; 4],
    context: Option<web_sys::WebGlRenderingContext>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            camera_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            context: None,
        }
    }

    pub fn set_context(&mut self, context: web_sys::WebGlRenderingContext) {
        let vertex_shader = shader::compile_shader(&context, &shader::default::vertex_shader());
        let fragment_shader = shader::compile_shader(&context, &shader::default::fragment_shader());
        if let (Ok(vertex_shader), Ok(fragment_shader)) = (vertex_shader, fragment_shader) {
            if let Ok(program) = shader::link_program(&context, &vertex_shader, &fragment_shader) {
                context.use_program(Some(&program));

                let vbo = create_vbo(
                    &context,
                    &[
                        -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5,
                    ],
                );
                let ibo = create_ibo(&context, &[0, 1, 2, 2, 3, 0]);

                let attrib_location = context.get_attrib_location(&program, "position") as u32;

                context.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&vbo));
                context.enable_vertex_attrib_array(attrib_location);
                context.vertex_attrib_pointer_with_i32(
                    attrib_location,
                    3,
                    web_sys::WebGlRenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );

                context.bind_buffer(
                    web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    Some(&ibo),
                );
            }
        }
        self.context = Some(context);
    }
}

fn create_vbo(context: &web_sys::WebGlRenderingContext, vertices: &[f32]) -> web_sys::WebGlBuffer {
    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(vertices);

        context.buffer_data_with_array_buffer_view(
            web_sys::WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            web_sys::WebGlRenderingContext::STATIC_DRAW,
        );
    }
    context.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, None);
    buffer
}

fn create_ibo(context: &web_sys::WebGlRenderingContext, vertices: &[i16]) -> web_sys::WebGlBuffer {
    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(
        web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&buffer),
    );
    unsafe {
        let vert_array = js_sys::Int16Array::view(vertices);

        context.buffer_data_with_array_buffer_view(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &vert_array,
            web_sys::WebGlRenderingContext::STATIC_DRAW,
        );
    }
    context.bind_buffer(web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
    buffer
}
