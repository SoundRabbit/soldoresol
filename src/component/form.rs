use super::btn;
use super::context_menu;
use super::MessengerGen;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub struct State {
    loc: [f32; 2],
    size: [f32; 2],
    drag_position: [f32; 2],
    dragged: bool,
    moving: bool,
    resizing: bool,
    showing: bool,
    bind_top: bool,
    bind_left: bool,
    bind_right: bool,
    bind_bottom: bool,
    context_menu_state: context_menu::State,
}

pub enum Msg {
    ShowContextMenu([f32; 2]),
    ContextMenuMsg(context_menu::Msg),
    MoveForm([f32; 2]),
    SetDragged(bool, [f32; 2]),
    SetShowingState(bool),
    ResizeL(f32),
    ResizeR(f32),
    ResizeT(f32),
    ResizeB(f32),
    ResizeLT([f32; 2]),
    ResizeRT([f32; 2]),
    ResizeLB([f32; 2]),
    ResizeRB([f32; 2]),
    SetBindTop(bool),
    SetBindLeft(bool),
    SetBindRight(bool),
    SetBindBottom(bool),
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
        bind_top: false,
        bind_left: false,
        bind_right: false,
        bind_bottom: false,
        context_menu_state: context_menu::init(),
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

pub fn toggle_open_close(state: &mut State) {
    if state.showing {
        close(state);
    } else {
        open(state);
    }
}

pub fn window_resized(state: &mut State) {
    update(state, Msg::SetBindTop(state.bind_top));
    update(state, Msg::SetBindLeft(state.bind_left));
    update(state, Msg::SetBindRight(state.bind_right));
    update(state, Msg::SetBindBottom(state.bind_bottom));
}

pub fn is_moving(state: &State) -> bool {
    state.moving || state.resizing
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::ShowContextMenu(p) => {
            context_menu::open(&mut state.context_menu_state, p);
        }
        Msg::ContextMenuMsg(m) => {
            context_menu::update(&mut state.context_menu_state, m);
        }
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
                let x = p[0] + state.drag_position[0];
                let y = p[1] + state.drag_position[1];
                if (!state.bind_left && !state.bind_right) || 20.0 < (state.loc[0] - x).abs() {
                    state.loc[0] = x;
                    state.bind_left = false;
                    state.bind_right = false;
                }
                if (!state.bind_top && !state.bind_bottom) || 20.0 < (state.loc[1] - y).abs() {
                    state.loc[1] = y;
                    state.bind_top = false;
                    state.bind_bottom = false;
                }
                state.moving = true;
            } else {
                state.moving = false;
            }
        }
        Msg::SetShowingState(s) => {
            state.showing = s;
        }
        Msg::ResizeL(x) => {
            if state.dragged && !state.moving && !state.bind_left {
                let x = x + state.drag_position[0];
                state.size[0] += state.loc[0] - x;
                state.loc[0] = x;
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeR(x) => {
            if state.dragged && !state.moving && !state.bind_right {
                state.size[0] = x - state.loc[0];
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeT(y) => {
            if state.dragged && !state.moving && !state.bind_top {
                let y = y + state.drag_position[1];
                state.size[1] += state.loc[1] - y;
                state.loc[1] = y;
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeB(y) => {
            if state.dragged && !state.moving && !state.bind_bottom {
                state.size[1] = y - state.loc[1];
                state.resizing = true;
            } else {
                state.resizing = false;
            }
        }
        Msg::ResizeLT(p) => {
            update(state, Msg::ResizeL(p[0]));
            update(state, Msg::ResizeT(p[1]));
        }
        Msg::ResizeRT(p) => {
            update(state, Msg::ResizeR(p[0]));
            update(state, Msg::ResizeT(p[1]));
        }
        Msg::ResizeLB(p) => {
            update(state, Msg::ResizeL(p[0]));
            update(state, Msg::ResizeB(p[1]));
        }
        Msg::ResizeRB(p) => {
            update(state, Msg::ResizeR(p[0]));
            update(state, Msg::ResizeB(p[1]));
        }
        Msg::SetBindTop(s) => {
            state.bind_top = s;
            if s {
                let document = web_sys::window().unwrap().document().unwrap();
                let header = document
                    .get_element_by_id("app-header")
                    .expect("there is no element whose id is \"app-header\"");
                let header_height = header.client_height() as f32;
                if state.bind_bottom {
                    let window = web_sys::window().unwrap();
                    let h = window.inner_height().unwrap().as_f64().unwrap() as f32;
                    state.size[1] = h - header_height;
                    state.loc[1] = header_height;
                } else {
                    state.loc[1] = header_height;
                }
            }
        }
        Msg::SetBindLeft(s) => {
            state.bind_left = s;
            if s {
                let document = web_sys::window().unwrap().document().unwrap();
                let menu = document
                    .get_element_by_id("app-sidemenu")
                    .expect("there is no element whose id is \"app-sidemenu\"");
                let menu_width = menu.client_width() as f32;
                if state.bind_right {
                    let window = web_sys::window().unwrap();
                    let w = window.inner_width().unwrap().as_f64().unwrap() as f32;
                    state.size[0] = w - menu_width;
                    state.loc[0] = menu_width;
                } else {
                    state.loc[0] = menu_width;
                }
            }
        }
        Msg::SetBindRight(s) => {
            state.bind_right = s;
            if s {
                let window = web_sys::window().unwrap();
                let w = window.inner_width().unwrap().as_f64().unwrap() as f32;
                if state.bind_left {
                    state.size[0] = w - state.loc[0];
                } else {
                    state.loc[0] = w - state.size[0];
                }
            }
        }
        Msg::SetBindBottom(s) => {
            state.bind_bottom = s;
            if s {
                let window = web_sys::window().unwrap();
                let h = window.inner_height().unwrap().as_f64().unwrap() as f32;
                if state.bind_top {
                    state.size[1] = h - state.loc[1];
                } else {
                    state.loc[1] = h - state.size[1];
                }
            }
        }
    }
}

pub fn render<M: 'static>(
    closable: bool,
    resizable: bool,
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
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
                let m = messenger_gen()();
                |e| {
                    m(Msg::SetDragged(
                        true,
                        [e.client_x() as f32, e.client_y() as f32],
                    ))
                }
            })
            .on_mouseup({
                let m = messenger_gen()();
                |e| {
                    m(Msg::SetDragged(
                        false,
                        [e.client_x() as f32, e.client_y() as f32],
                    ))
                }
            })
            .on_mouseleave({
                let m = messenger_gen()();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            })
            .on_mouseenter({
                let m = messenger_gen()();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            })
            .on_mousemove({
                let m = messenger_gen()();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            }),
        vec![
            Html::div(
                Attributes::new().class("form-header"),
                Events::new().on_contextmenu({
                    let m = messenger_gen()();
                    |e| {
                        e.prevent_default();
                        m(Msg::ShowContextMenu([
                            e.client_x() as f32,
                            e.client_y() as f32,
                        ]))
                    }
                }),
                vec![
                    Html::text(title),
                    if resizable {
                        context_menu::render(
                            &state.context_menu_state,
                            || {
                                let messenger = messenger_gen();
                                Box::new(move || {
                                    let m = messenger();
                                    Box::new(|msg| m(Msg::ContextMenuMsg(msg)))
                                })
                            },
                            Attributes::new().class("form-header-context_menu"),
                            Events::new(),
                            vec![
                                btn::context_menu(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let m = messenger_gen()();
                                        |e| m(Msg::SetBindTop(true))
                                    }),
                                    "上にバインド",
                                ),
                                btn::context_menu(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let m = messenger_gen()();
                                        |e| m(Msg::SetBindLeft(true))
                                    }),
                                    "左にバインド",
                                ),
                                btn::context_menu(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let m = messenger_gen()();
                                        |e| m(Msg::SetBindRight(true))
                                    }),
                                    "右にバインド",
                                ),
                                btn::context_menu(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let m = messenger_gen()();
                                        |e| m(Msg::SetBindBottom(true))
                                    }),
                                    "下にバインド",
                                ),
                            ],
                        )
                    } else {
                        Html::none()
                    },
                    if closable {
                        btn::close(
                            Attributes::new(),
                            Events::new().on_click({
                                let m = messenger_gen()();
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
            render_resizer("form-lefter", state.bind_left, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeL(e.client_x() as f32)))
                }
            }),
            render_resizer("form-righter", state.bind_right, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeR(e.client_x() as f32)))
                }
            }),
            render_resizer("form-topper", state.bind_top, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeT(e.client_y() as f32)))
                }
            }),
            render_resizer("form-bottomer", state.bind_bottom, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeB(e.client_y() as f32)))
                }
            }),
            render_resizer("form-l_topper", state.bind_left || state.bind_top, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeLT([e.client_x() as f32, e.client_y() as f32])))
                }
            }),
            render_resizer("form-r_topper", state.bind_right || state.bind_top, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeRT([e.client_x() as f32, e.client_y() as f32])))
                }
            }),
            render_resizer("form-l_bottomer", state.bind_left || state.bind_bottom, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeLB([e.client_x() as f32, e.client_y() as f32])))
                }
            }),
            render_resizer("form-r_bottomer", state.bind_right || state.bind_bottom, {
                let messenger = messenger_gen();
                move || {
                    let m = messenger();
                    Box::new(|e| m(Msg::ResizeRB([e.client_x() as f32, e.client_y() as f32])))
                }
            }),
        ],
    )
}

fn render_resizer<M: 'static>(
    class_name: impl Into<String>,
    binded: bool,
    messenger: impl Fn() -> Box<dyn FnOnce(web_sys::MouseEvent) -> M + 'static>,
) -> Html<M> {
    Html::div(
        Attributes::new()
            .class(class_name)
            .class("form-edge")
            .string("data-form-binded", binded.to_string()),
        Events::new()
            .on("mouseleave", {
                let m = messenger();
                |e| {
                    e.stop_propagation();
                    m(e.dyn_into::<web_sys::MouseEvent>().unwrap())
                }
            })
            .on("mouseenter", {
                let m = messenger();
                |e| {
                    e.stop_propagation();
                    m(e.dyn_into::<web_sys::MouseEvent>().unwrap())
                }
            })
            .on("mousemove", {
                let m = messenger();
                |e| {
                    e.stop_propagation();
                    m(e.dyn_into::<web_sys::MouseEvent>().unwrap())
                }
            }),
        vec![],
    )
}
