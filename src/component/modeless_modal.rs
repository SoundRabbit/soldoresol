use super::{btn, modal};
use crate::{model::Color, random_id};
use kagura::prelude::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;

pub struct Props {
    pub origin: [i32; 2],
    pub corner: [i32; 2],
    pub resizable: [bool; 4],
}

pub struct State {
    context: Option<web_sys::CanvasRenderingContext2d>,
    canvas_size: [f64; 2],
    canvas_id: String,
    origin: [i32; 2],
    corner: [i32; 2],
    mouse: Option<[i32; 4]>,
    resizable: [bool; 4],
}

impl State {
    pub fn new(props: &Props) -> Self {
        Self {
            context: None,
            canvas_size: [0.0, 0.0],
            canvas_id: random_id::base64url(),
            origin: [props.origin[0] - 1, props.origin[1] - 1],
            corner: [props.corner[0] - 1, props.corner[1] - 1],
            mouse: None,
            resizable: props.resizable.clone(),
        }
    }

    fn as_props(&self) -> Props {
        Props {
            origin: [self.origin[0] + 1, self.origin[1] + 1],
            corner: [self.corner[0] + 1, self.corner[1] + 1],
            resizable: self.resizable.clone(),
        }
    }
}

pub struct StateWrapper {
    state: Rc<RefCell<State>>,
}

pub enum Msg {
    NoOp,
    SetCanvasContext,
    ReflectToClose,
    SetOrigin([i32; 2]),
}

pub enum Sub {
    ReflectToClose(Props),
}

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

fn get_device_pixel_ratio() -> f64 {
    web_sys::window().unwrap().device_pixel_ratio()
}

fn set_canvas_size(canvas: &web_sys::HtmlCanvasElement) -> [f64; 2] {
    let dpr = get_device_pixel_ratio();
    let width = canvas.client_width() as f64 * dpr;
    let height = canvas.client_height() as f64 * dpr;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    [width, height]
}

pub fn new(state: Rc<RefCell<State>>) -> Component<Msg, StateWrapper, Sub> {
    let init = move || {
        let state = StateWrapper { state: state };
        let task = Cmd::task(|handler| {
            handler(Msg::SetCanvasContext);
        });
        (state, task)
    };
    Component::new(init, update, render)
}

fn update(wrapper: &mut StateWrapper, msg: Msg) -> Cmd<Msg, Sub> {
    let mut state = wrapper.state.borrow_mut();
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetCanvasContext => {
            let canvas = get_canvas_element(&state.canvas_id);
            state.canvas_size = set_canvas_size(&canvas);

            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            state.context = Some(context);

            render_canvas(&state);

            Cmd::none()
        }
        Msg::ReflectToClose => Cmd::sub(Sub::ReflectToClose(state.as_props())),
        Msg::SetOrigin(origin) => {
            if let Some(mouse) = state.mouse {
                if state.resizable[0] {
                    state.origin[1] = origin[1] + mouse[0];
                }
                if state.resizable[1] {
                    state.origin[0] = origin[0] + mouse[1];
                }
                if state.resizable[2] {
                    state.corner[1] = origin[1] + mouse[2];
                }
                if state.resizable[3] {
                    state.corner[0] = origin[0] + mouse[3];
                }
            } else {
                let dt = state.origin[1] - origin[1];
                let dl = state.origin[0] - origin[0];
                let db = state.corner[1] - origin[1];
                let dr = state.corner[0] - origin[0];
                state.mouse = Some([dt, dl, db, dr]);
            }

            render_canvas(&state);

            Cmd::none()
        }
    }
}

fn render(wrapper: &StateWrapper) -> Html<Msg> {
    let state = wrapper.state.borrow();

    Html::canvas(
        Attributes::new()
            .id(&state.canvas_id)
            .class("cover cover-a"),
        Events::new()
            .on_mouseup(|_| Msg::ReflectToClose)
            .on_mousemove({
                let width = state.canvas_size[0];
                let height = state.canvas_size[1];
                let x_step = width / 24.0;
                let y_step = height / 14.0;

                move |e| {
                    e.stop_propagation();
                    if e.buttons() & 1 != 0 {
                        let dpr = get_device_pixel_ratio();
                        let col = (e.offset_x() as f64 * dpr / x_step).floor() as i32;
                        let row = (e.offset_y() as f64 * dpr / y_step).floor() as i32;
                        Msg::SetOrigin([col, row])
                    } else {
                        Msg::NoOp
                    }
                }
            }),
        vec![],
    )
}

fn render_canvas(state: &State) {
    if let Some(context) = &state.context {
        let width = state.canvas_size[0];
        let height = state.canvas_size[1];
        let x_step = width / 24.0;
        let y_step = height / 14.0;
        let xa = state.origin[0].min(state.corner[0]);
        let xb = state.origin[0].max(state.corner[0]);
        let ya = state.origin[1].min(state.corner[1]);
        let yb = state.origin[1].max(state.corner[1]);

        context.clear_rect(0.0, 0.0, width, height);

        context.set_fill_style(&Color::from(0x7f0366d6).to_jsvalue());

        for row in 0..14 {
            for col in 0..24 {
                if xa <= col && col < xb && ya <= row && row < yb {
                    let x = col as f64 * x_step;
                    let y = row as f64 * y_step;
                    context.fill_rect(x, y, x_step, y_step);
                }
            }
        }
    }
}
