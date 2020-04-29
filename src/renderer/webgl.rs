use std::ops::Deref;

pub struct WebGlF32Vbo(web_sys::WebGlBuffer);
pub struct WebGlI16Ibo(web_sys::WebGlBuffer);
pub struct WebGlAttributeLocation(pub u32);
pub struct WebGlRenderingContext(pub web_sys::WebGlRenderingContext);

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
        &self.0
    }
}

impl WebGlRenderingContext {
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

    pub fn set_attribute(
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
}
