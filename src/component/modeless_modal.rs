use super::btn;
use super::modal;
use crate::model::Color;
use crate::random_id;
use kagura::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct Props {
    pub origin: [i32; 2],
    pub corner: [i32; 2],
}

pub struct State {
    context: Option<web_sys::CanvasRenderingContext2d>,
    canvas_size: [f64; 2],
    canvas_id: String,
    origin: [i32; 2],
    corner: [i32; 2],
}

impl State {
    pub fn new(props: &Props) -> Self {
        Self {
            context: None,
            canvas_size: [0.0, 0.0],
            canvas_id: random_id::base64url(),
            origin: props.origin.clone(),
            corner: [props.corner[0] - 1, props.corner[1] - 1],
        }
    }

    fn to_props(&self) -> Props {
        Props {
            origin: self.origin.clone(),
            corner: [self.corner[0] + 1, self.corner[1] + 1],
        }
    }
}

pub struct StateWrapper {
    state: Rc<RefCell<State>>,
}

pub enum Msg {
    NoOp,
    SetCanvasContext,
    Close,
    Reflect,
    ReflectToClose,
    SetOrigin([i32; 2]),
    SetCorner([i32; 2]),
    RefineCoord,
}

pub enum Sub {
    Close,
    Reflect(Props),
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
    let height = width * 14.0 / 24.0;
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
        Msg::Close => Cmd::sub(Sub::Close),
        Msg::Reflect => Cmd::sub(Sub::Reflect(state.to_props())),
        Msg::ReflectToClose => Cmd::sub(Sub::ReflectToClose(state.to_props())),
        Msg::SetOrigin(origin) => {
            let w = state.corner[0] - state.origin[0];
            let h = state.corner[1] - state.origin[1];

            state.corner = [origin[0] + w, origin[1] + h];
            state.origin = origin;
            render_canvas(&state);
            Cmd::none()
        }
        Msg::SetCorner(corner) => {
            if (state.origin[0] - corner[0]).abs() != 0 && (state.origin[1] - corner[1]).abs() != 0
            {
                state.corner = corner;
                render_canvas(&state);
            }
            Cmd::none()
        }
        Msg::RefineCoord => {
            let xa = state.origin[0].min(state.corner[0]);
            let xb = state.origin[0].max(state.corner[0]);
            let ya = state.origin[1].min(state.corner[1]);
            let yb = state.origin[1].max(state.corner[1]);

            state.origin = [xa, ya];
            state.corner = [xb, yb];

            render_canvas(&state);

            Cmd::none()
        }
    }
}

fn render(wrapper: &StateWrapper) -> Html<Msg> {
    let state = wrapper.state.borrow();

    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                modal::header(
                    Attributes::new()
                        .style("display", "grid")
                        .style("grid-template-columns", "1fr max-content"),
                    Events::new(),
                    vec![
                        Html::div(Attributes::new(), Events::new(), vec![]),
                        Html::div(
                            Attributes::new().class("linear-h"),
                            Events::new(),
                            vec![btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::Close),
                            )],
                        ),
                    ],
                ),
                modal::body(
                    Attributes::new().class("grid"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new()
                                .class("grid-w-4 container-a")
                                .style("overflow-y", "scroll"),
                            Events::new(),
                            vec![],
                        ),
                        Html::div(
                            Attributes::new()
                                .class("grid-w-20 container-a")
                                .style("overflow-y", "scroll"),
                            Events::new(),
                            vec![
                                Html::h5(
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text("表示位置")],
                                ),
                                Html::div(
                                    Attributes::new().class("container-a").draggable(false),
                                    Events::new(),
                                    vec![Html::canvas(
                                        Attributes::new()
                                            .id(&state.canvas_id)
                                            .style("width", "100%"),
                                        Events::new()
                                            .on_mousedown({
                                                let width = state.canvas_size[0];
                                                let height = state.canvas_size[1];
                                                let x_step = width / 24.0;
                                                let y_step = height / 14.0;
                                                move |e| {
                                                    let dpr = get_device_pixel_ratio();
                                                    let col = (e.offset_x() as f64 * dpr / x_step)
                                                        .ceil()
                                                        as i32;
                                                    let row = (e.offset_y() as f64 * dpr / y_step)
                                                        .ceil()
                                                        as i32;
                                                    Msg::SetOrigin([col, row])
                                                }
                                            })
                                            .on_mouseup(|_| Msg::RefineCoord)
                                            .on_mousemove({
                                                let width = state.canvas_size[0];
                                                let height = state.canvas_size[1];
                                                let x_step = width / 24.0;
                                                let y_step = height / 14.0;
                                                move |e| {
                                                    e.stop_propagation();
                                                    if e.buttons() & 1 != 0 {
                                                        let dpr = get_device_pixel_ratio();
                                                        let col = (e.offset_x() as f64 * dpr
                                                            / x_step)
                                                            .ceil()
                                                            as i32;
                                                        let row = (e.offset_y() as f64 * dpr
                                                            / y_step)
                                                            .ceil()
                                                            as i32;
                                                        Msg::SetCorner([col, row])
                                                    } else {
                                                        Msg::NoOp
                                                    }
                                                }
                                            }),
                                        vec![],
                                    )],
                                ),
                            ],
                        ),
                    ],
                ),
                modal::header(
                    Attributes::new().class("justify-r"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("linear-h"),
                        Events::new(),
                        vec![
                            btn::success(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::Reflect),
                                vec![Html::text("適用")],
                            ),
                            btn::primary(
                                Attributes::new(),
                                Events::new().on_click(|_| Msg::ReflectToClose),
                                vec![Html::text("OK")],
                            ),
                        ],
                    )],
                ),
            ],
        )],
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

        context.set_fill_style(&Color::from(0xff6a737d).to_jsvalue());

        for row in 0..14 {
            for col in 0..24 {
                if !(xa <= col + 1 && col + 1 <= xb && ya <= row + 1 && row + 1 <= yb) {
                    let x = col as f64 * x_step;
                    let y = row as f64 * y_step;
                    context.fill_rect(x, y, x_step, y_step);
                }
            }
        }

        context.set_fill_style(&Color::from(0xffd73a49).to_jsvalue());

        for row in 0..14 {
            for col in 0..24 {
                if xa <= col + 1 && col + 1 <= xb && ya <= row + 1 && row + 1 <= yb {
                    let x = col as f64 * x_step;
                    let y = row as f64 * y_step;
                    context.fill_rect(x, y, x_step, y_step);
                }
            }
        }

        context.set_line_width(2.0);
        context.set_stroke_style(&Color::from(0xff2f363d).to_jsvalue());

        for row in 0..14 {
            for col in 0..24 {
                let x = col as f64 * x_step;
                let y = row as f64 * y_step;
                context.stroke_rect(x, y, x_step, y_step);
            }
        }
    }
}
