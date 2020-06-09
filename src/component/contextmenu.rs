use super::MessengerGen;
use kagura::prelude::*;

pub struct State {
    showing: bool,
    loc: [f64; 2],
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

#[allow(dead_code)]
pub fn open(state: &mut State, loc: [f64; 2]) {
    state.loc = loc;
    update(state, Msg::SetShowingState(true));
}

#[allow(dead_code)]
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
    _centering: bool,
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
        Attributes::new().class("fullscreen").style("z-index", "0"),
        Events::new()
            .on_click({
                let m = messenger_gen()();
                |_| m(Msg::SetShowingState(false))
            })
            .on_contextmenu({
                let m = messenger_gen()();
                |e| {
                    e.prevent_default();
                    m(Msg::SetShowingState(false))
                }
            }),
        vec![Html::div(
            attributes
                .class("pure-menu")
                .style("position", "absolute")
                .style("left", state.loc[0].to_string() + "px")
                .style("top", state.loc[1].to_string() + "px"),
            events,
            children,
        )],
    )
}
