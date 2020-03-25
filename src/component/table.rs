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
                }
            }
            let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
            let buffer = context.create_buffer().unwrap();
            context.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
            unsafe {
                let vert_array = js_sys::Float32Array::view(&vertices);

                context.buffer_data_with_array_buffer_view(
                    web_sys::WebGlRenderingContext::ARRAY_BUFFER,
                    &vert_array,
                    web_sys::WebGlRenderingContext::STATIC_DRAW,
                );
            }
            context.vertex_attrib_pointer_with_i32(
                0,
                3,
                web_sys::WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );
            context.enable_vertex_attrib_array(0);

            context.clear_color(0.0, 0.0, 0.0, 1.0);
            context.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);

            context.draw_arrays(
                web_sys::WebGlRenderingContext::TRIANGLES,
                0,
                (vertices.len() / 3) as i32,
            );
            state.context = Some(context);
            Cmd::none()
        }
    }
}

fn render(_: &State) -> Html<Msg> {
    Html::canvas(Attributes::new().id("table"), Events::new(), vec![])
}
