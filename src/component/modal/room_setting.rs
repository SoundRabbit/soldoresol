use kagura::prelude::*;

pub type RoomSetting = Component<Props, Sub>;

pub struct Props {}

struct State {}

enum Msg {}

pub enum Sub {}

pub fn new() -> RoomSetting {
    Component::new(init, update, render)
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {}

fn render(state: &State, children: Vec<Html>) -> Html {}
