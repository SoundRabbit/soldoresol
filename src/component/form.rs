use super::btn;
use kagura::prelude::*;

pub struct State {
    loc: [f32; 2],
    size: [f32; 2],
    drag_position: [f32; 2],
    dragged: bool,
    moving: bool,
    resizing: bool,
    showing: bool,
}

pub enum Msg {
    MoveForm([f32; 2]),
    SetDragged(bool, [f32; 2]),
    SetShowingState(bool),
    ResizeL(f32),
    ResizeR(f32),
    ResizeB(f32),
    ResizeLB([f32; 2]),
    ResizeRB([f32; 2]),
}

pub fn init() -> State {
    let window = web_sys::window().unwrap();
    let w = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let h = window.inner_height().unwrap().as_f64().unwrap() as f32;
    State {
        loc: [0.0, 0.0],
        size: [w * 0.6, h * 0.6],
        drag_position: [0.0, 0.0],
        dragged: false,
        moving: false,
        resizing: false,
        showing: false,
    }
}

pub fn open(state: &mut State) {
    let document = web_sys::window().unwrap().document().unwrap();
    let header = document.get_element_by_id("app-header").unwrap();
    let sidemenu = document.get_element_by_id("app-sidemenu").unwrap();
    state.loc = [
        sidemenu.client_width() as f32,
        header.client_height() as f32,
    ];
    update(state, Msg::SetShowingState(true));
}

pub fn close(state: &mut State) {
    update(state, Msg::SetShowingState(false));
}

pub fn is_moving(state: &State) -> bool {
    state.moving || state.resizing
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::SetDragged(dragged, p) => {
            if dragged {
                state.drag_position = [state.loc[0] - p[0], state.loc[1] - p[1]];
            } else {
                state.moving = false;
            }
            state.dragged = dragged;
        }
        Msg::MoveForm(p) => {
            if state.dragged && !state.resizing {
                state.loc[0] = p[0] + state.drag_position[0];
                state.loc[1] = p[1] + state.drag_position[1];
                state.moving = true;
            } else {
                state.moving = false;
            }
        }
        Msg::SetShowingState(s) => {
            state.showing = s;
        }
        Msg::ResizeL(x) => {
            if state.dragged && !state.moving {
                let x = x + state.drag_position[0];
                state.size[0] += state.loc[0] - x;
                state.loc[0] = x;
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeR(x) => {
            if state.dragged && !state.moving {
                state.size[0] = x - state.loc[0];
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeB(y) => {
            if state.dragged && !state.moving {
                state.size[1] = y - state.loc[1];
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeLB(p) => {
            update(state, Msg::ResizeL(p[0]));
            update(state, Msg::ResizeB(p[1]));
        }
        Msg::ResizeRB(p) => {
            update(state, Msg::ResizeR(p[0]));
            update(state, Msg::ResizeB(p[1]));
        }
    }
}

pub fn render<M: 'static>(
    closable: bool,
    resizable: bool,
    state: &State,
    messenger: impl Fn() -> Box<dyn FnOnce(Msg) -> M + 'static> + 'static,
    attributes: Attributes,
    events: Events<M>,
    title: impl Into<String>,
    children: Vec<Html<M>>,
) -> Html<M> {
    if !state.showing {
        return Html::none();
    }
    let attributes = attributes
        .class("form")
        .style("left", state.loc[0].to_string() + "px")
        .style("top", state.loc[1].to_string() + "px")
        .string("data-form-resizable", resizable.to_string());
    let attributes = if resizable {
        attributes
            .style("min-width", state.size[0].to_string() + "px")
            .style("min-height", state.size[1].to_string() + "px")
            .style("max-width", state.size[0].to_string() + "px")
            .style("max-height", state.size[1].to_string() + "px")
    } else {
        attributes
    };
    Html::div(
        attributes,
        events
            .on_mousedown({
                let m = messenger();
                |e| {
                    m(Msg::SetDragged(
                        true,
                        [e.client_x() as f32, e.client_y() as f32],
                    ))
                }
            })
            .on_mouseup({
                let m = messenger();
                |e| {
                    m(Msg::SetDragged(
                        false,
                        [e.client_x() as f32, e.client_y() as f32],
                    ))
                }
            })
            .on_mouseleave({
                let m = messenger();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            })
            .on_mouseenter({
                let m = messenger();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            })
            .on_mousemove({
                let m = messenger();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            }),
        vec![
            Html::div(
                Attributes::new().class("form-header"),
                Events::new(),
                vec![
                    Html::text(title),
                    if resizable {
                        Html::none()
                    } else {
                        Html::none()
                    },
                    if closable {
                        btn::close(
                            Attributes::new(),
                            Events::new().on_click({
                                let m = messenger();
                                |e| m(Msg::SetShowingState(false))
                            }),
                        )
                    } else {
                        Html::none()
                    },
                ],
            ),
            Html::div(
                Attributes::new().class("form-body"),
                Events::new(),
                children,
            ),
            Html::div(
                Attributes::new().class("form-lefter"),
                Events::new()
                    .on_mouseleave({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeL(e.client_x() as f32))
                        }
                    })
                    .on_mouseenter({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeL(e.client_x() as f32))
                        }
                    })
                    .on_mousemove({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeL(e.client_x() as f32))
                        }
                    }),
                vec![],
            ),
            Html::div(
                Attributes::new().class("form-righter"),
                Events::new()
                    .on_mouseleave({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeR(e.client_x() as f32))
                        }
                    })
                    .on_mouseenter({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeR(e.client_x() as f32))
                        }
                    })
                    .on_mousemove({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeR(e.client_x() as f32))
                        }
                    }),
                vec![],
            ),
            Html::div(
                Attributes::new().class("form-bottomer"),
                Events::new()
                    .on_mouseleave({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeB(e.client_y() as f32))
                        }
                    })
                    .on_mouseenter({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeB(e.client_y() as f32))
                        }
                    })
                    .on_mousemove({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeB(e.client_y() as f32))
                        }
                    }),
                vec![],
            ),
            Html::div(
                Attributes::new().class("form-l_bottomer"),
                Events::new()
                    .on_mouseleave({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeLB([e.client_x() as f32, e.client_y() as f32]))
                        }
                    })
                    .on_mouseenter({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeLB([e.client_x() as f32, e.client_y() as f32]))
                        }
                    })
                    .on_mousemove({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeLB([e.client_x() as f32, e.client_y() as f32]))
                        }
                    }),
                vec![],
            ),
            Html::div(
                Attributes::new().class("form-r_bottomer"),
                Events::new()
                    .on_mouseleave({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeRB([e.client_x() as f32, e.client_y() as f32]))
                        }
                    })
                    .on_mouseenter({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeRB([e.client_x() as f32, e.client_y() as f32]))
                        }
                    })
                    .on_mousemove({
                        let m = messenger();
                        |e| {
                            e.stop_propagation();
                            m(Msg::ResizeRB([e.client_x() as f32, e.client_y() as f32]))
                        }
                    }),
                vec![],
            ),
        ],
    )
}
