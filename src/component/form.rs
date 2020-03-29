use super::btn;
use kagura::prelude::*;

pub struct State {
    loc: [f32; 2],
    drag_position: [f32; 2],
    dragged: bool,
    moving: bool,
    showing: bool,
}

pub enum Msg {
    MoveForm([f32; 2]),
    SetDragged(bool, [f32; 2]),
    SetShowingState(bool),
}

pub fn init() -> State {
    State {
        loc: [0.0, 0.0],
        drag_position: [0.0, 0.0],
        dragged: false,
        moving: false,
        showing: false,
    }
}

pub fn open(state: &mut State) {
    update(state, Msg::SetShowingState(true));
}

pub fn close(state: &mut State) {
    update(state, Msg::SetShowingState(false));
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
            if state.dragged {
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
    Html::div(
        attributes
            .class("form")
            .style("left", state.loc[0].to_string() + "px")
            .style("top", state.loc[1].to_string() + "px")
            .string("data-form-moving", state.moving.to_string()),
        events,
        vec![
            Html::div(
                Attributes::new().class("form-header"),
                Events::new()
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
                    .on_mousemove({
                        let m = messenger();
                        |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
                    }),
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
                Attributes::new().class("form-footer"),
                Events::new(),
                vec![],
            ),
        ],
    )
}
