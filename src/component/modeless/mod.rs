use super::MessengerGen;
use kagura::prelude::*;

pub mod object_editor;

struct State {}

enum Msg {}

fn init() -> State {
    State {}
}

fn update(_: &mut State, _: Msg) {}

fn render<M: 'static>(
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
    attributes: Attributes,
    events: Events<M>,
    header: Vec<Html<M>>,
    body: Vec<Html<M>>,
    footer: Vec<Html<M>>,
) -> Html<M> {
    Html::div(
        attributes.class("frame").style("position", "fixed"),
        events,
        vec![
            Html::div(
                Attributes::new().class("frame-header"),
                Events::new(),
                header,
            ),
            Html::div(Attributes::new().class("frame-body"), Events::new(), body),
            Html::div(
                Attributes::new().class("frame-footer"),
                Events::new(),
                footer,
            ),
        ],
    )
}
