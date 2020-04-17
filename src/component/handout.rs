use kagura::prelude::*;

use super::form;
use super::MessengerGen;

pub struct State {
    form_state: form::State,
}

pub enum Msg {
    FormMsg(form::Msg),
}

pub fn init() -> State {
    State {
        form_state: form::init(),
    }
}

#[allow(dead_code)]
pub fn open(state: &mut State) {
    form::open(&mut state.form_state);
}

#[allow(dead_code)]
pub fn close(state: &mut State) {
    form::close(&mut state.form_state);
}

#[allow(dead_code)]
pub fn toggle_open_close(state: &mut State) {
    form::toggle_open_close(&mut state.form_state);
}

#[allow(dead_code)]
pub fn is_moving(state: &State) -> bool {
    form::is_moving(&state.form_state)
}

#[allow(dead_code)]
pub fn window_resized(state: &mut State) {
    form::window_resized(&mut state.form_state);
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::FormMsg(m) => form::update(&mut state.form_state, m),
    }
}

pub fn render<M: 'static>(
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
    attributes: Attributes,
    events: Events<M>,
) -> Html<M> {
    form::render(
        true,
        true,
        &state.form_state,
        || {
            let messenger = messenger_gen();
            Box::new(move || {
                let m = messenger();
                Box::new(|msg| m(Msg::FormMsg(msg)))
            })
        },
        attributes.class("handout"),
        events,
        "資料",
        vec![],
    )
}
