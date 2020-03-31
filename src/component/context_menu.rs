use super::MessengerGen;
use kagura::prelude::*;
use wasm_bindgen::prelude::*;

pub struct State {
    showing: bool,
    loc: [f32; 2],
}

pub enum Msg {
    SetShowingState(bool),
}

pub fn init() -> State {
    State {
        showing: false,
        loc: [0.0, 0.0],
    }
}

pub fn open(state: &mut State, loc: [f32; 2]) {
    state.loc = loc;
    update(state, Msg::SetShowingState(true));
}

pub fn close(state: &mut State) {
    update(state, Msg::SetShowingState(false));
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::SetShowingState(s) => {
            state.showing = s;
        }
    }
}

pub fn render<M: 'static>(
    centering: bool,
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
    attributes: Attributes,
    events: Events<M>,
    children: Vec<Html<M>>,
) -> Html<M> {
    if !state.showing {
        return Html::none();
    }
    Html::div(
        Attributes::new().class("context_menu-bg"),
        Events::new().on_click({
            let m = messenger_gen()();
            |_| m(Msg::SetShowingState(false))
        }),
        vec![Html::div(
            attributes
                .style("left", state.loc[0].to_string() + "px")
                .style("top", state.loc[1].to_string() + "px")
                .string("data-context_menu-centering", centering.to_string())
                .class("context_menu"),
            events,
            children,
        )],
    )
}
