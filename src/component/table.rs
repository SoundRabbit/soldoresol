use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::shader;
use crate::shader::ShaderSource;

pub struct State {
    context: Option<web_sys::WebGlRenderingContext>,
}

pub enum Msg {
    NoOp,
    InitializeContext(web_sys::HtmlCanvasElement),
}

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(
        || {
            (
                State { context: None },
                Cmd::task(|resolver| {
                    let canvas = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("table")
                        .unwrap()
                        .dyn_into::<web_sys::HtmlCanvasElement>()
                        .unwrap();
                    resolver(Msg::InitializeContext(canvas));
                }),
            )
        },
        update,
        render,
    )
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match (msg) {
        Msg::NoOp => Cmd::none(),
        Msg::InitializeContext(canvas) => {
            let height = canvas.client_height();
            let width = canvas.client_width();
            canvas.set_attribute("height", &height.to_string());
            canvas.set_attribute("width", &width.to_string());
            let context = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            let vertex_shader = shader::compile_shader(&context, &shader::default::vertex_shader());
            let fragment_shader =
                shader::compile_shader(&context, &shader::default::fragment_shader());
            if let (Ok(vertex_shader), Ok(fragment_shader)) = (vertex_shader, fragment_shader) {
                if let Ok(program) =
                    shader::link_program(&context, &vertex_shader, &fragment_shader)
                {
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

            context.clear_color(0.0, 0.0, 0.0, 1.0);
            context.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);

            context.draw_elements_with_i32(
                web_sys::WebGlRenderingContext::TRIANGLES,
                6,
                web_sys::WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
            state.context = Some(context);
            Cmd::none()
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

fn render(_: &State) -> Html<Msg> {
    Html::canvas(Attributes::new().id("table"), Events::new(), vec![])
}
