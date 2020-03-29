use kagura::prelude::*;

use super::btn;
use super::form;
use super::radio;

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

pub fn open(state: &mut State) {
    form::open(&mut state.form_state);
}

pub fn close(state: &mut State) {
    form::close(&mut state.form_state);
}

pub fn is_moving(state: &State) -> bool {
    form::is_moving(&state.form_state)
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
        Attributes::new().class("chat"),
        Events::new(),
        "チャット",
        vec![render_controller(), render_gap(), render_tabs()],
    )
}

fn render_controller<M>() -> Html<M> {
    Html::div(
        Attributes::new().class("chat-controller"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("chat-controller-chat_pallet"),
                Events::new(),
                vec![],
            ),
            Html::div(
                Attributes::new().class("chat-controller-sender_option"),
                Events::new(),
                vec![
                    btn::primary(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("文字色を変更")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("チャットパレットを編集")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("ダイスボットを選択")],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("chat-controller-sending_option"),
                Events::new(),
                vec![
                    Html::div(Attributes::new(), Events::new(), vec![Html::text("from:")]),
                    Html::div(
                        Attributes::new().class("chat-controller-sending_option-list"),
                        Events::new(),
                        vec![btn::success(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("追加")],
                        )],
                    ),
                    Html::div(Attributes::new(), Events::new(), vec![Html::text("to:")]),
                    Html::div(
                        Attributes::new().class("chat-controller-sending_option-list"),
                        Events::new(),
                        vec![btn::success(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("追加")],
                        )],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("chat-controller-content"),
                Events::new(),
                vec![
                    Html::textarea(Attributes::new().string("rows", "3"), Events::new(), vec![]),
                    btn::primary(Attributes::new(), Events::new(), vec![Html::text("送信")]),
                ],
            ),
        ],
    )
}

fn render_gap<M>() -> Html<M> {
    Html::div(Attributes::new().class("chat-gap"), Events::new(), vec![])
}

fn render_tabs<M>() -> Html<M> {
    Html::div(
        Attributes::new().class("chat-tabs"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("chat-tabs-list"),
                Events::new(),
                vec![btn::tab(
                    false,
                    false,
                    Attributes::new(),
                    Events::new(),
                    "追加",
                )],
            ),
            Html::div(
                Attributes::new().class("chat-tabs-log"),
                Events::new(),
                vec![],
            ),
        ],
    )
}
