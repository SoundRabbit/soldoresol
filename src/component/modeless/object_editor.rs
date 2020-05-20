use super::MessengerGen;
use kagura::prelude::*;

pub struct State {
    parent_state: super::State,
}

pub enum Msg {
    Transport(super::Msg),
}

pub fn init() -> State {
    State {
        parent_state: super::init(),
    }
}

pub fn update(_: &mut State, _: Msg) {}

pub fn render<M: 'static>(
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
) -> Html<M> {
    super::render(
        &state.parent_state,
        || {
            let messenger_gen = messenger_gen();
            Box::new(move || {
                let messenger = messenger_gen();
                Box::new(move |msg| messenger(Msg::Transport(msg)))
            })
        },
        Attributes::new(),
        Events::new(),
        vec![],
        vec![],
        vec![],
    )
}
