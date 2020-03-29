use kagura::prelude::*;

use super::checkbox::checkbox;
use super::form;

pub struct State {
    form_state: form::State,
    pub show_circle_on_table: bool,
    pub bind_to_grid: bool,
}

pub enum Msg {
    FormMsg(form::Msg),
    SetShowCircleOnTableFlag(bool),
    SetBindToGridFlag(bool),
}

pub fn init() -> State {
    State {
        form_state: form::init(),
        show_circle_on_table: true,
        bind_to_grid: true,
    }
}

pub fn open(state: &mut State) {
    form::open(&mut state.form_state);
}

pub fn close(state: &mut State) {
    form::close(&mut state.form_state);
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::FormMsg(m) => form::update(&mut state.form_state, m),
        Msg::SetShowCircleOnTableFlag(s) => {
            state.show_circle_on_table = s;
        }
        Msg::SetBindToGridFlag(s) => {
            state.bind_to_grid = s;
        }
    }
}

pub fn render<M: 'static>(
    state: &State,
    messenger: impl Fn() -> Box<dyn FnOnce(Msg) -> M + 'static> + 'static,
) -> Html<M> {
    let m_1 = messenger();
    let m_2 = messenger();
    form::render(
        false,
        false,
        &state.form_state,
        move || {
            let messenger = messenger();
            Box::new(|msg| messenger(Msg::FormMsg(msg)))
        },
        Attributes::new().id("measure_tool"),
        Events::new(),
        "測定オプション",
        vec![
            checkbox(
                Attributes::new(),
                Events::new().on_click({
                    let f = state.show_circle_on_table;
                    move |_| m_1(Msg::SetShowCircleOnTableFlag(!f))
                }),
                "テーブルに円を表示",
                state.show_circle_on_table,
            ),
            checkbox(
                Attributes::new(),
                Events::new().on_click({
                    let f = state.bind_to_grid;
                    move |_| m_2(Msg::SetBindToGridFlag(!f))
                }),
                "グリッドにスナップ",
                state.bind_to_grid,
            ),
            checkbox(Attributes::new(), Events::new(), "測定内容を共有", true),
        ],
    )
}
