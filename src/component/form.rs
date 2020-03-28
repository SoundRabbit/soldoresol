use kagura::prelude::*;

pub struct State {
    loc: [f32; 2],
    drag_position: [f32; 2],
    dragged: bool,
    moving: bool,
}

pub enum Msg {
    MoveForm([f32; 2]),
    SetDragged(bool, [f32; 2]),
}

pub fn init() -> State {
    State {
        loc: [0.0, 0.0],
        drag_position: [0.0, 0.0],
        dragged: false,
        moving: false,
    }
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
    }
}

pub fn render<M: 'static>(
    state: &State,
    messenger: impl Fn() -> Box<dyn FnOnce(Msg) -> M + 'static> + 'static,
    attributes: Attributes,
    events: Events<M>,
    children: Vec<Html<M>>,
) -> Html<M> {
    Html::div(
        attributes
            .class("form")
            .style("left", state.loc[0].to_string() + "px")
            .style("top", state.loc[1].to_string() + "px")
            .string("data-form-moving", state.moving.to_string()),
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
            .on_mousemove({
                let m = messenger();
                |e| m(Msg::MoveForm([e.client_x() as f32, e.client_y() as f32]))
            }),
        children,
    )
}
