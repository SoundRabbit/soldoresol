use kagura::prelude::*;

pub struct State;

pub struct Msg;

pub struct Sub;

pub fn new() -> Component<Msg, State, Sub> {
    Component::new(init, update, render)
}

fn init() -> (State, Cmd<Msg, Sub>) {
    (State, Cmd::none())
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(_: &State) -> Html<Msg> {
    Html::div(
        Attributes::new().id("app"),
        Events::new(),
        vec![render_side_menu()],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::menu(Attributes::new().id("app-side-menu"), Events::new(), vec![])
}
