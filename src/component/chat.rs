use super::btn;
use super::form;
use super::icon;
use super::MessengerGen;
use crate::random_id;
use kagura::prelude::*;
use std::collections::HashMap;

struct Message {
    sender: String,
    timestamp: js_sys::Date,
    text: String,
}

struct Tab {
    name: String,
    messages: Vec<Message>,
}

pub struct State {
    form_state: form::State,
    tabs: HashMap<String, Tab>,
    tab_index: Vec<String>,
    selected_tab_id: String,
    inputing_chat_text: String,
}

pub enum Msg {
    FormMsg(form::Msg),
    InputChatText(String),
    SendInputingMessage(),
    SendMessage(String),
}

pub fn init() -> State {
    let initial_tab_id = random_id::hex(6);
    let mut tabs = HashMap::new();
    tabs.insert(
        initial_tab_id.clone(),
        Tab {
            name: String::from("メイン"),
            messages: vec![],
        },
    );
    State {
        form_state: form::init(),
        tabs: tabs,
        tab_index: vec![initial_tab_id.clone()],
        selected_tab_id: initial_tab_id,
        inputing_chat_text: String::new(),
    }
}

pub fn open(state: &mut State) {
    form::open(&mut state.form_state);
}

pub fn close(state: &mut State) {
    form::close(&mut state.form_state);
}

pub fn toggle_open_close(state: &mut State) {
    form::toggle_open_close(&mut state.form_state);
}

pub fn is_moving(state: &State) -> bool {
    form::is_moving(&state.form_state)
}

pub fn window_resized(state: &mut State) {
    form::window_resized(&mut state.form_state);
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::FormMsg(m) => form::update(&mut state.form_state, m),
        Msg::InputChatText(text) => state.inputing_chat_text = text,
        Msg::SendInputingMessage() => {
            let mut text = String::new();
            std::mem::swap(&mut state.inputing_chat_text, &mut text);
            update(state, Msg::SendMessage(text));
        }
        Msg::SendMessage(text) => {
            if let Some(tab) = state.tabs.get_mut(&state.selected_tab_id) {
                tab.messages.push(Message {
                    sender: String::from("test"),
                    timestamp: js_sys::Date::new_0(),
                    text: text,
                });
            }
        }
    }
}

pub fn render<M: 'static>(
    state: &State,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
    attributes: Attributes,
    events: Events<M>,
) -> Html<M> {
    form::render(
        true,
        true,
        &state.form_state,
        || {
            let messenger = messenger_gen();
            Box::new(move || {
                let m = messenger();
                Box::new(|msg| m(Msg::FormMsg(msg)))
            })
        },
        attributes.class("chat"),
        events,
        "チャット",
        vec![
            render_controller(&state.inputing_chat_text, || {
                let messenger = messenger_gen();
                Box::new(move || messenger())
            }),
            render_gap(),
            render_tabs(&state.tabs, &state.tab_index, &state.selected_tab_id),
        ],
    )
}

fn render_controller<M: 'static>(
    inputing_chat_text: &String,
    messange_gen: impl Fn() -> MessengerGen<Msg, M>,
) -> Html<M> {
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
                    Html::div(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("送り主:")],
                    ),
                    Html::div(
                        Attributes::new().class("chat-controller-sending_option-list"),
                        Events::new(),
                        vec![btn::add(Attributes::new(), Events::new())],
                    ),
                    Html::div(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("送り先:")],
                    ),
                    Html::div(
                        Attributes::new().class("chat-controller-sending_option-list"),
                        Events::new(),
                        vec![btn::add(Attributes::new(), Events::new())],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("chat-controller-content"),
                Events::new(),
                vec![
                    Html::textarea(
                        Attributes::new()
                            .string("rows", "3")
                            .value(inputing_chat_text),
                        Events::new().on_input({
                            let m = messange_gen()();
                            |text| m(Msg::InputChatText(text))
                        }),
                        vec![],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click({
                            let m = messange_gen()();
                            |_| m(Msg::SendInputingMessage())
                        }),
                        vec![Html::text("送信")],
                    ),
                ],
            ),
        ],
    )
}

fn render_gap<M>() -> Html<M> {
    Html::div(Attributes::new().class("chat-gap"), Events::new(), vec![])
}

fn render_tabs<M>(
    tabs: &HashMap<String, Tab>,
    tab_index: &Vec<String>,
    selected_tab_id: &String,
) -> Html<M> {
    let mut chat_tabs_list = tab_index
        .iter()
        .map(|tab_id| match tabs.get(tab_id) {
            None => Html::none(),
            Some(tab) => btn::tab(
                tab_id == selected_tab_id,
                true,
                Attributes::new(),
                Events::new(),
                &tab.name,
            ),
        })
        .collect::<Vec<Html<M>>>();
    chat_tabs_list.push(btn::add(
        Attributes::new().string("data-btn_add-tab", "true"),
        Events::new(),
    ));
    Html::div(
        Attributes::new().class("chat-tabs"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("chat-tabs-list"),
                Events::new(),
                chat_tabs_list,
            ),
            Html::div(
                Attributes::new().class("chat-tabs-log"),
                Events::new(),
                tab_index
                    .iter()
                    .map(|tab_id| match tabs.get(tab_id) {
                        None => Html::none(),
                        Some(tab) => render_tabs_log_column(&tab, &tab_id, &selected_tab_id),
                    })
                    .collect(),
            ),
        ],
    )
}

fn render_tabs_log_column<M>(tab: &Tab, tab_id: &String, selected_tab_id: &String) -> Html<M> {
    Html::div(
        Attributes::new().class("chat-tabs-log-column").string(
            "data-chat-selected",
            (tab_id == selected_tab_id).to_string(),
        ),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("chat-tabs-log-column-heading"),
                Events::new(),
                vec![Html::text(&tab.name)],
            ),
            Html::div(
                Attributes::new().class("chat-tabs-log-column-content"),
                Events::new(),
                tab.messages
                    .iter()
                    .map(|message| render_tabs_log_column_content_message(&message))
                    .collect(),
            ),
        ],
    )
}

fn render_tabs_log_column_content_message<M>(message: &Message) -> Html<M> {
    Html::div(
        Attributes::new().class("chat-tabs-log-column-content-message"),
        Events::new(),
        vec![
            icon::medium(
                Attributes::new().class("chat-tabs-log-column-content-message-icon"),
                Events::new(),
            ),
            Html::div(
                Attributes::new().class("chat-tabs-log-column-content-message-sender"),
                Events::new(),
                vec![Html::text(&message.sender)],
            ),
            Html::div(
                Attributes::new().class("chat-tabs-log-column-content-message-timestamp"),
                Events::new(),
                vec![Html::text(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02} + {:03}ms",
                    message.timestamp.get_full_year(),
                    message.timestamp.get_month(),
                    message.timestamp.get_date(),
                    message.timestamp.get_hours(),
                    message.timestamp.get_minutes(),
                    message.timestamp.get_seconds(),
                    message.timestamp.get_milliseconds()
                ))],
            ),
            Html::div(
                Attributes::new().class("chat-tabs-log-column-content-message-text"),
                Events::new(),
                vec![Html::text(&message.text)],
            ),
        ],
    )
}
