use kagura::prelude::*;

use super::btn;
use super::form;

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

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::FormMsg(m) => form::update(&mut state.form_state, m),
    }
}

pub fn render<M: 'static>(
    state: &State,
    messenger: impl Fn() -> Box<dyn FnOnce(Msg) -> M + 'static> + 'static,
) -> Html<M> {
    form::render(
        true,
        true,
        &state.form_state,
        move || {
            let messenger = messenger();
            Box::new(|msg| messenger(Msg::FormMsg(msg)))
        },
        Attributes::new().class("handout"),
        Events::new(),
        "資料",
        vec![],
    )
}
