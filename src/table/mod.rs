use crate::shader;
use ndarray::{arr1, arr2, Array2};
use std::ops::Mul;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Context {
    gl: web_sys::WebGlRenderingContext,
    uniform_translate_location: web_sys::WebGlUniformLocation,
}

pub struct Table {
    translate: Array2<f32>,
    context: Option<Context>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            translate: arr2(&[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            context: None,
        }
    }

    pub fn set_context(&mut self, gl: web_sys::WebGlRenderingContext) {
        let vertex_shader = shader::compile_shader(&gl, &shader::default::vertex_shader()).unwrap();
        let fragment_shader =
            shader::compile_shader(&gl, &shader::default::fragment_shader()).unwrap();
        let program = shader::link_program(&gl, &vertex_shader, &fragment_shader).unwrap();
        gl.use_program(Some(&program));

        let vbo = create_vbo(
            &gl,
            &create_squre(
                &[0.0, 0.0, 0.0],
                &[1.0, 0.0, 0.0],
                &[0.0, 1.0, 0.0],
                &[[20.0, 20.0], [-20.0, 20.0], [-20.0, -20.0], [20.0, -20.0]],
            ),
        );
        let ibo = create_ibo(&gl, &[0, 1, 2, 2, 3, 0]);

        let attrib_location = gl.get_attrib_location(&program, "position") as u32;
        let uniform_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();

        gl.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&vbo));
        gl.enable_vertex_attrib_array(attrib_location);
        gl.vertex_attrib_pointer_with_i32(
            attrib_location,
            3,
            web_sys::WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&ibo),
        );

        self.context = Some(Context {
            gl,
            uniform_translate_location,
        });
    }

    fn perspective(&self) -> Array2<f32> {
        if let Some(context) = &self.context {
            let gl = &context.gl;
            let h = gl.drawing_buffer_height() as f32;
            let w = gl.drawing_buffer_width() as f32;
            let aspect = w / h;
            let field_of_view = 30.0;
            let near = 1.0;
            let far = 200.0;
            let f = (std::f32::consts::PI * 0.5 - field_of_view * 0.5).tan();
            let range_inv = 1.0 / (near - far);

            arr2(&[
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (near + far) * range_inv, -1.0],
                [0.0, 0.0, near * far * range_inv * 2.0, 0.0],
            ])
        } else {
            arr2(&[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        }
    }

    pub fn reset_translate(&mut self) {
        self.translate = self.perspective();
    }

    pub fn set_x_axis_rotation(&mut self, r: f32) {
        let (s, c) = r.sin_cos();
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, s, 0.0],
            [0.0, -s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.translate = t.dot(&self.translate);
    }

    pub fn set_z_axis_rotation(&mut self, r: f32) {
        let (s, c) = r.sin_cos();
        let t = arr2(&[
            [c, s, 0.0, 0.0],
            [-s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.translate = t.dot(&self.translate);
    }

    pub fn set_movement(&mut self, m: &[f32; 3]) {
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [m[0], m[1], m[2], 1.0],
        ]);
        self.translate = t.dot(&self.translate);
    }

    pub fn render(&self) {
        if let Some(context) = &self.context {
            let gl = &context.gl;
            let t = &self.translate;
            gl.uniform_matrix4fv_with_f32_array(
                Some(&context.uniform_translate_location),
                false,
                &[
                    t.row(0).to_vec(),
                    t.row(1).to_vec(),
                    t.row(2).to_vec(),
                    t.row(3).to_vec(),
                ]
                .concat(),
            );
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);
            gl.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }

    pub fn resize(&self) {
        if let Some(context) = &self.context {
            let gl = &context.gl;
            let canvas = gl
                .canvas()
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let height = canvas.client_height();
            let width = canvas.client_width();

            gl.viewport(0, 0, width, height);
        }
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

fn create_squre(
    o: &[f32; 3],
    u_axis: &[f32; 3],
    v_axis: &[f32; 3],
    vertices: &[[f32; 2]; 4],
) -> Vec<f32> {
    let u_axis = arr1(u_axis);
    let v_axis = arr1(v_axis);
    let verteices = vertices
        .iter()
        .map(|vertex| {
            let u_loc = arr1(&[vertex[0], vertex[0], vertex[0]]);
            let v_loc = arr1(&[vertex[1], vertex[1], vertex[1]]);
            let o = arr1(o);
            (o + u_loc.mul(&u_axis) + v_loc.mul(&v_axis)).to_vec()
        })
        .collect::<Vec<Vec<f32>>>()
        .concat();
    verteices
}
