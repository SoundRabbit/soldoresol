use kagura::prelude::*;

use super::checkbox::checkbox;
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
        &state.form_state,
        move || {
            let messenger = messenger();
            Box::new(|msg| messenger(Msg::FormMsg(msg)))
        },
        Attributes::new().id("measure_tool"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("form-header"),
                Events::new(),
                vec![Html::text("計測オプション")],
            ),
            Html::div(
                Attributes::new().class("form-body"),
                Events::new(),
                vec![
                    checkbox(Attributes::new(), Events::new(), "テーブルに円を表示", true),
                    checkbox(Attributes::new(), Events::new(), "グリッドにスナップ", true),
                    checkbox(Attributes::new(), Events::new(), "測定内容を共有", true),
                ],
            ),
            Html::div(
                Attributes::new().class("form-footer"),
                Events::new(),
                vec![],
            ),
        ],
    )
}
