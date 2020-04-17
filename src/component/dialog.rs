use super::btn;
use super::MessengerGen;
use kagura::prelude::*;

pub struct State {
    showing: bool,
}

pub enum Msg {
    NoOp,
    SetShowingState(bool),
}

pub fn init() -> State {
    State { showing: false }
}

#[allow(dead_code)]
pub fn open(state: &mut State) {
    update(state, Msg::SetShowingState(true));
}

#[allow(dead_code)]
pub fn close(state: &mut State) {
    update(state, Msg::SetShowingState(false));
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::NoOp => {}
        Msg::SetShowingState(s) => {
            state.showing = s;
        }
    }
}

pub fn render<M: 'static>(
    closable: bool,
    title: impl Into<String>,
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
    attributes: Attributes,
    events: Events<M>,
    body: Vec<Html<M>>,
    footer: Vec<Html<M>>,
) -> Html<M> {
    if !state.showing {
        return Html::none();
    }
    Html::div(
        Attributes::new().class("dialog-bg"),
        Events::new().on_mousedown({
            let m = messenger_gen()();
            |e| {
                e.stop_propagation();
                m(Msg::NoOp)
            }
        }),
        vec![Html::div(
            attributes.class("dialog"),
            events,
            vec![
                Html::div(
                    Attributes::new().class("dialog-header"),
                    Events::new(),
                    vec![
                        Html::text(title),
                        if closable {
                            btn::close(
                                Attributes::new(),
                                Events::new().on_click({
                                    let m = messenger_gen()();
                                    |_| m(Msg::SetShowingState(false))
                                }),
                            )
                        } else {
                            Html::none()
                        },
                    ],
                ),
                Html::div(Attributes::new().class("dialog-body"), Events::new(), body),
                Html::div(
                    Attributes::new().class("dialog-footer"),
                    Events::new(),
                    footer,
                ),
            ],
        )],
    )
}
