use crate::shader;
use ndarray::{arr1, arr2, Array1, Array2};
use std::ops::Mul;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Context {
    gl: web_sys::WebGlRenderingContext,
    u_translate_location: web_sys::WebGlUniformLocation,
    u_texture_location: web_sys::WebGlUniformLocation,
}

pub struct Table {
    camera: Array2<f32>,
    context: Option<Context>,
    row_num: u32,
    column_num: u32,
    canvas: web_sys::HtmlCanvasElement,
    grid_size: f64,
}

impl Table {
    pub fn new() -> Self {
        Table {
            camera: arr2(&[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            context: None,
            row_num: 20,
            column_num: 20,
            canvas: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap(),
            grid_size: 64.0,
        }
    }

    pub fn set_context(&mut self, gl: web_sys::WebGlRenderingContext) {
        web_sys::console::log_1(&JsValue::from("shader::compile_shader"));
        let vertex_shader = shader::compile_shader(&gl, &shader::default::vertex_shader()).unwrap();
        web_sys::console::log_1(&JsValue::from("shader::compile_shader"));
        let fragment_shader =
            shader::compile_shader(&gl, &shader::default::fragment_shader()).unwrap();
        web_sys::console::log_1(&JsValue::from("shader::link_program"));
        let program = shader::link_program(&gl, &vertex_shader, &fragment_shader).unwrap();
        gl.use_program(Some(&program));

        let table_height = self.row_num as f32;
        let table_width = self.column_num as f32;

        let v_position = create_vbo(
            &gl,
            &create_squre(
                &[0.0, 0.0, 0.0],
                &[1.0, 0.0, 0.0],
                &[0.0, 1.0, 0.0],
                &[
                    [table_width / 2.0, table_height / 2.0],
                    [-table_width / 2.0, table_height / 2.0],
                    [table_width / 2.0, -table_height / 2.0],
                    [-table_width / 2.0, -table_height / 2.0],
                ],
            ),
        );
        let v_color = create_vbo(
            &gl,
            &[
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            ]
            .concat(),
        );
        let v_texture_coord = create_vbo(
            &gl,
            &[[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]].concat(),
        );
        let i_index = create_ibo(&gl, &[0, 1, 2, 1, 2, 3]);

        let a_position_location = gl.get_attrib_location(&program, "a_position") as u32;
        let a_color_location = gl.get_attrib_location(&program, "a_color") as u32;
        let a_texture_coord_location = gl.get_attrib_location(&program, "a_textureCoord") as u32;

        web_sys::console::log_1(&JsValue::from("get_uniform_location"));
        let u_translate_location = gl.get_uniform_location(&program, "u_translate").unwrap();
        web_sys::console::log_1(&JsValue::from("get_uniform_location"));
        let u_texture_location = gl.get_uniform_location(&program, "u_texture").unwrap();

        set_attribute(&gl, &v_position, a_position_location, 3, 0);
        set_attribute(&gl, &v_color, a_color_location, 4, 0);
        set_attribute(&gl, &v_texture_coord, a_texture_coord_location, 2, 0);

        gl.bind_buffer(
            web_sys::WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&i_index),
        );

        let texture = gl.create_texture().unwrap();
        gl.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        gl.pixel_storei(web_sys::WebGlRenderingContext::UNPACK_ALIGNMENT, 1);
        {
            self.canvas.set_height(self.grid_size as u32 * self.row_num);
            self.canvas
                .set_width(self.grid_size as u32 * self.column_num);
            let texture = self
                .canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            texture.set_fill_style(&JsValue::from("#fff"));
            texture.fill_rect(
                0.0,
                0.0,
                self.canvas.width() as f64,
                self.canvas.height() as f64,
            );
            texture.set_line_width(8.0);
            texture.set_stroke_style(&JsValue::from("#000"));
            texture.stroke_rect(
                0.0,
                0.0,
                self.canvas.width() as f64,
                self.canvas.height() as f64,
            );
            texture.set_line_width(1.0);
            for x in 1..self.column_num {
                texture.move_to(x as f64 * self.grid_size, 0.0);
                texture.line_to(x as f64 * self.grid_size, self.canvas.height() as f64)
            }
            for y in 1..self.row_num {
                texture.move_to(0.0, y as f64 * self.grid_size);
                texture.line_to(self.canvas.width() as f64, y as f64 * self.grid_size)
            }
            texture.stroke();
        }

        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        self.context = Some(Context {
            gl,
            u_translate_location,
            u_texture_location,
        });
    }

    fn get_perspective(&self) -> Array2<f32> {
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
        self.camera = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn set_x_axis_rotation(&mut self, r: f32) {
        let (s, c) = r.sin_cos();
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, s, 0.0],
            [0.0, -s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.camera = t.dot(&self.camera);
    }

    pub fn set_z_axis_rotation(&mut self, r: f32) {
        let (s, c) = r.sin_cos();
        let t = arr2(&[
            [c, s, 0.0, 0.0],
            [-s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        self.camera = t.dot(&self.camera);
    }

    pub fn set_movement(&mut self, m: &[f32; 3]) {
        let t = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [m[0], m[1], m[2], 1.0],
        ]);
        self.camera = t.dot(&self.camera);
    }

    fn get_inv_perspective(&self) -> Array2<f32> {
        let p = self.get_perspective();
        arr2(&[
            [1.0 / p.row(0)[0], 0.0, 0.0, 0.0],
            [0.0, 1.0 / p.row(1)[1], 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0 / p.row(3)[2]],
            [0.0, 0.0, -1.0, p.row(2)[2] / p.row(3)[2]],
        ])
    }

    fn get_inv_camera(&self) -> Array2<f32> {
        let t = &self.camera;
        arr2(&[
            [t.row(0)[0], t.row(1)[0], t.row(2)[0], 0.0],
            [t.row(0)[1], t.row(1)[1], t.row(2)[1], 0.0],
            [t.row(0)[2], t.row(1)[2], t.row(2)[2], 0.0],
            [
                -t.row(0).dot(&t.row(3)),
                -t.row(1).dot(&t.row(3)),
                -t.row(2).dot(&t.row(3)),
                1.0,
            ],
        ])
    }

    fn get_table_location_from_screen(&self, p: &[f32; 2]) -> Array1<f32> {
        let inv_c = self.get_inv_camera();
        let inv_p = self.get_inv_perspective();
        let inv = inv_p.dot(&inv_c);
        let inv = inv.t();

        // inv * [p[0] * w, p[1] * w, z, w] = [x, y, 0, 1] を解く
        //
        // inv[2][2] * z + ( inv[2][3] + inv[2][0] * p[0] + inv[2][1] * p[1] ) * w = 0
        // inv[3][2] * z + ( inv[3][3] + inv[3][0] * p[0] + inv[3][1] * p[1] ) * w = 1

        let a = inv.row(2)[2];
        let b = inv.row(2)[3] + inv.row(2)[0] * p[0] + inv.row(2)[1] * p[1];
        let c = inv.row(3)[2];
        let d = inv.row(3)[3] + inv.row(3)[0] * p[0] + inv.row(3)[1] * p[1];
        let aa = 0.0;
        let bb = 1.0;

        let z = (d * aa - b * bb) / (a * d - b * c);
        let w = (a * bb - c * aa) / (a * d - b * c);

        inv.dot(&arr1(&[p[0] * w, p[1] * w, z, w]))
    }

    pub fn draw_line(&self, b: &[f32; 2], e: &[f32; 2]) {
        if let Some(context) = &self.context {
            let gl = &context.gl;
            let canvas = gl
                .canvas()
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let height = canvas.client_height() as f32;
            let width = canvas.client_width() as f32;

            let b = self.get_table_location_from_screen(&[
                b[0] / width * 2.0 - 1.0,
                1.0 - 2.0 * b[1] / height,
            ]);

            let e = self.get_table_location_from_screen(&[
                e[0] / width * 2.0 - 1.0,
                1.0 - 2.0 * e[1] / height,
            ]);

            let texture = self
                .canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            texture.begin_path();
            texture.set_line_width(8.0);
            texture.set_stroke_style(&JsValue::from("#0366d6"));
            texture.move_to(
                (b[0] + 10.0) as f64 * self.grid_size,
                (b[1] + 10.0) as f64 * self.grid_size,
            );
            texture.line_to(
                (e[0] + 10.0) as f64 * self.grid_size,
                (e[1] + 10.0) as f64 * self.grid_size,
            );
            texture.stroke();
        }
    }

    pub fn render(&self) {
        if let Some(context) = &self.context {
            let gl = &context.gl;
            let t = (&self.camera).dot(&self.get_perspective());
            gl.uniform1i(Some(&context.u_texture_location), 0);
            gl.uniform_matrix4fv_with_f32_array(
                Some(&context.u_translate_location),
                false,
                &[
                    t.row(0).to_vec(),
                    t.row(1).to_vec(),
                    t.row(2).to_vec(),
                    t.row(3).to_vec(),
                ]
                .concat(),
            );
            gl.tex_image_2d_with_u32_and_u32_and_canvas(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                &self.canvas,
            )
            .expect("");
            gl.clear_color(1.0, 1.0, 1.0, 1.0);
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

fn set_attribute(
    gl: &web_sys::WebGlRenderingContext,
    buffer: &web_sys::WebGlBuffer,
    position: u32,
    size: i32,
    stride: i32,
) {
    gl.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.enable_vertex_attrib_array(position);
    gl.vertex_attrib_pointer_with_i32(
        position,
        size,
        web_sys::WebGlRenderingContext::FLOAT,
        false,
        stride,
        0,
    );
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
