// use super::btn;
// use super::chat;
// use super::context_menu;
// use super::handout;
// use super::measure_length::measure_length;
// use super::radio::radio;
use crate::model::Camera;
use crate::model::Table;
use crate::model::World;
use crate::random_id;
use crate::renderer::Renderer;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

enum FormKind {
    Chat,
    Handout,
}

// struct FormState {
//     chat: chat::State,
//     handout: handout::State,
// }

enum TableTool {
    Selecter,
    Pen,
    Eracer,
    Measure,
}

struct TableState {
    selecting_tool: TableTool,
    ctr_key_is_downed: bool,
}

pub struct State {
    room_name: String,
    world: World,
    camera: Camera,
    renderer: Option<Renderer>,
    canvas_size: [f64; 2],
    selecting_table_tool: TableTool,
    // context_menu_state: context_menu::State,
    // form_state: FormState,
}

// enum FormMsg {
//     ChatMsg(chat::Msg),
//     HandoutMsg(handout::Msg),
// }

pub enum Msg {
    NoOp,
    SetTableContext(web_sys::HtmlCanvasElement),
    WindowResized,
}

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render).batch(|mut handler| {
        let a = Closure::wrap(Box::new(move || {
            handler(Msg::WindowResized);
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .set_onresize(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

fn init() -> (State, Cmd<Msg, Sub>) {
    let state = State {
        room_name: String::from("無名の部屋@") + &random_id::hex(16),
        world: World::new([20.0, 20.0]),
        camera: Camera::new(),
        renderer: None,
        canvas_size: [0.0, 0.0],
        selecting_table_tool: TableTool::Selecter,
        // context_menu_state: context_menu::init(),
        // form_state: FormState {
        //     chat: chat::init(),
        //     handout: handout::init(),
        // },
    };
    let task = Cmd::task(|handler| {
        handler(Msg::SetTableContext(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("table")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap(),
        ));
    });
    (state, task)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetTableContext(canvas) => {
            let canvas_size = [canvas.client_width() as f64, canvas.client_height() as f64];
            canvas.set_height(canvas.client_height() as u32);
            canvas.set_width(canvas.client_width() as u32);
            let gl = canvas
                .get_context("webgl")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .unwrap();
            let renderer = Renderer::new(gl);
            renderer.render(&mut state.world, &state.camera);
            state.renderer = Some(renderer);
            Cmd::none()
        }
        Msg::WindowResized => {
            // chat::window_resized(&mut state.form_state.chat);
            // handout::window_resized(&mut state.form_state.handout);
            if let Some(renderer) = &state.renderer {
                renderer.render(&mut state.world, &state.camera);
            }
            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().class("app"),
        Events::new(),
        vec![render_canvas()],
    )
}

fn render_canvas() -> Html<Msg> {
    Html::canvas(
        Attributes::new().class("app__table").id("table"),
        Events::new(),
        vec![],
    )
}
