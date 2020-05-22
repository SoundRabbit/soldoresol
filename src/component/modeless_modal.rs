use super::modal;
use crate::random_id;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub struct State {
    context: Option<web_sys::CanvasRenderingContext2d>,
    canvas_size: [f64; 2],
    canvas_id: String,
}

pub enum Msg {
    SetCanvasContext,
}

pub enum Sub {}

fn get_canvas_element(id: &str) -> web_sys::HtmlCanvasElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

pub fn new() -> Component<Msg, State, Sub> {
    let init = || {
        let state = State {
            context: None,
            canvas_size: [0.0, 0.0],
            canvas_id: random_id::base64url(),
        };
        let task = Cmd::task(|handler| {
            handler(Msg::SetCanvasContext);
        });
        (state, task)
    };
    Component::new(init, update, render)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::SetCanvasContext => {
            let canvas = get_canvas_element(&state.canvas_id);
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            state.context = Some(context);
            Cmd::none()
        }
    }
}

fn render(state: &State) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                modal::header(Attributes::new(), Events::new(), vec![]),
                modal::body(Attributes::new(), Events::new(), vec![]),
                modal::header(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
