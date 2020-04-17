use super::btn;
use super::control;
use super::dialog;
use super::form;
use super::icon;
use super::icon::Icon;
use super::MessengerGen;
use crate::random_id;
use kagura::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type TabId = u128;

struct Message {
    sender: String,
    timestamp: js_sys::Date,
    text: String,
}

struct Tab {
    name: String,
    messages: Vec<Message>,
}

struct User {
    name: String,
    icon: Option<String>,
    selected: bool,
    chatpallet: Vec<String>,
}

pub struct State {
    form_state: form::State,
    create_tab_dialog_state: dialog::State,
    tabs: HashMap<TabId, Tab>,
    tab_index: Vec<TabId>,
    selected_tab_id: TabId,
    selected_sender_idx: usize,
    selected_destination_idx: HashSet<usize>,
    inputing_chat_text: String,
    senders: Vec<User>,
    destinations: Vec<User>,
    inputing_tag_name: String,
}

pub enum Msg {
    FormMsg(form::Msg),
    CreateChatDialogMsg(dialog::Msg),
    InputChatText(String),
    InputTagName(String),
    SendInputingMessage(),
    SendMessage(String),
    AddTab(),
    ShowCreateTabDialog(),
    SetSelectedTab(TabId),
}

pub fn init() -> State {
    let initial_tab_id = random_id::u128val();
    let mut tabs = HashMap::new();
    tabs.insert(
        initial_tab_id,
        Tab {
            name: String::from("メイン"),
            messages: vec![],
        },
    );
    let mut selected_destination_idx = HashSet::new();
    selected_destination_idx.insert(0);
    State {
        form_state: form::init(),
        create_tab_dialog_state: dialog::init(),
        tabs: tabs,
        tab_index: vec![initial_tab_id],
        selected_tab_id: initial_tab_id,
        selected_sender_idx: 0,
        selected_destination_idx,
        inputing_chat_text: String::new(),
        senders: vec![User {
            name: String::from("自分"),
            icon: None,
            selected: true,
            chatpallet: vec![
                String::from(r#"#チャットパレット入力例："#),
                String::from("2d6+1 ダイスロール"),
                String::from("1d20+敏捷+格闘 {name}の格闘！"),
                String::from("敏捷=10+{敏捷A}"),
                String::from("敏捷A=10"),
                String::from("格闘=0"),
                String::from("器用度=20"),
            ],
        }],
        destinations: vec![User {
            name: String::from("全体"),
            icon: None,
            selected: true,
            chatpallet: vec![],
        }],
        inputing_tag_name: String::new(),
    }
}

#[allow(dead_code)]
pub fn open(state: &mut State) {
    form::open(&mut state.form_state);
}

#[allow(dead_code)]
pub fn close(state: &mut State) {
    form::close(&mut state.form_state);
}

#[allow(dead_code)]
pub fn toggle_open_close(state: &mut State) {
    form::toggle_open_close(&mut state.form_state);
}

#[allow(dead_code)]
pub fn is_moving(state: &State) -> bool {
    form::is_moving(&state.form_state)
}

#[allow(dead_code)]
pub fn window_resized(state: &mut State) {
    form::window_resized(&mut state.form_state);
}

pub fn update(state: &mut State, msg: Msg) {
    match msg {
        Msg::FormMsg(m) => form::update(&mut state.form_state, m),
        Msg::CreateChatDialogMsg(m) => dialog::update(&mut state.create_tab_dialog_state, m),
        Msg::InputChatText(text) => state.inputing_chat_text = text,
        Msg::InputTagName(name) => state.inputing_tag_name = name,
        Msg::ShowCreateTabDialog() => dialog::open(&mut state.create_tab_dialog_state),
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
        Msg::AddTab() => {
            let tab_id = random_id::u128val();
            let mut name = String::new();
            std::mem::swap(&mut state.inputing_tag_name, &mut name);
            state.tabs.insert(
                tab_id,
                Tab {
                    name: name,
                    messages: vec![],
                },
            );
            state.tab_index.push(tab_id);
            dialog::close(&mut state.create_tab_dialog_state);
        }
        Msg::SetSelectedTab(tab_id) => {
            state.selected_tab_id = tab_id;
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
            render_controller(
                &state.inputing_chat_text,
                state.selected_sender_idx,
                &state.senders,
                &state.destinations,
                || {
                    let messenger = messenger_gen();
                    Box::new(move || messenger())
                },
            ),
            render_gap(),
            render_tabs(
                &state.tabs,
                &state.tab_index,
                &state.selected_tab_id,
                || {
                    let messenger = messenger_gen();
                    Box::new(move || messenger())
                },
            ),
            dialog::render(
                true,
                "チャットタブを追加",
                &state.create_tab_dialog_state,
                || {
                    let messenger = messenger_gen();
                    Box::new(move || {
                        let m = messenger();
                        Box::new(|msg| m(Msg::CreateChatDialogMsg(msg)))
                    })
                },
                Attributes::new().class("chat-create_tab_dialog"),
                Events::new(),
                vec![control::input(
                    Attributes::new().placeholder("新規タブ名"),
                    Events::new().on_input({
                        let m = messenger_gen()();
                        |v| m(Msg::InputTagName(v))
                    }),
                )],
                vec![btn::success(
                    Attributes::new(),
                    Events::new().on_click({
                        let m = messenger_gen()();
                        |_| m(Msg::AddTab())
                    }),
                    vec![Html::text("追加")],
                )],
            ),
        ],
    )
}

fn render_controller<M: 'static>(
    inputing_chat_text: &String,
    selected_sender_idx: usize,
    senders: &Vec<User>,
    destinations: &Vec<User>,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
) -> Html<M> {
    let mut sender_list = senders
        .iter()
        .map(|user| render_controller_select_user_btn(&user.name, &user.icon, user.selected))
        .collect::<Vec<Html<M>>>();
    sender_list.push(btn::add(Attributes::new(), Events::new()));
    let mut destination_list = destinations
        .iter()
        .map(|user| render_controller_select_user_btn(&user.name, &user.icon, user.selected))
        .collect::<Vec<Html<M>>>();
    destination_list.push(btn::add(Attributes::new(), Events::new()));
    let selected_sender = &senders[selected_sender_idx];
    Html::div(
        Attributes::new().class("chat-controller"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("chat-controller-chat_pallet"),
                Events::new(),
                selected_sender
                    .chatpallet
                    .iter()
                    .enumerate()
                    .map(|(_, row)| {
                        btn::dark(
                            Attributes::new(),
                            Events::new()
                                .on_click({
                                    let m = messenger_gen()();
                                    let text = row.clone();
                                    |_| m(Msg::InputChatText(text))
                                })
                                .on_dblclick({
                                    let m = messenger_gen()();
                                    |_| m(Msg::SendInputingMessage())
                                }),
                            vec![Html::text(row)],
                        )
                    })
                    .collect(),
            ),
            Html::div(
                Attributes::new().class("chat-controller-sender_option"),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class("chat-controller-sender_option-row"),
                        Events::new(),
                        vec![
                            btn::primary(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text("ダイスボットを選択")],
                            ),
                            btn::primary(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text("チャットパレットを編集")],
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class("chat-controller-sender_option-row"),
                        Events::new(),
                        vec![btn::primary(
                            Attributes::new(),
                            Events::new(),
                            vec![Html::text("文字色を変更")],
                        )],
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
                        sender_list,
                    ),
                    Html::div(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("送り先:")],
                    ),
                    Html::div(
                        Attributes::new().class("chat-controller-sending_option-list"),
                        Events::new(),
                        destination_list,
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
                            let m = messenger_gen()();
                            |text| m(Msg::InputChatText(text))
                        }),
                        vec![],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click({
                            let m = messenger_gen()();
                            |_| m(Msg::SendInputingMessage())
                        }),
                        vec![Html::text("送信")],
                    ),
                ],
            ),
        ],
    )
}

fn render_controller_select_user_btn<M: 'static>(
    name: &String,
    icon: &Option<String>,
    selected: bool,
) -> Html<M> {
    Html::div(
        Attributes::new()
            .class("chat-controller-select_user_btn")
            .string(
                "data-chat-controller-select_user_btn-selected",
                selected.to_string(),
            ),
        Events::new(),
        vec![
            icon::small(
                Attributes::new().class("chat-tabs-log-column-content-message-icon"),
                Events::new(),
                Icon::MaterialIcon("person"),
            ),
            Html::span(Attributes::new(), Events::new(), vec![Html::text(name)]),
        ],
    )
}

fn render_gap<M>() -> Html<M> {
    Html::div(Attributes::new().class("chat-gap"), Events::new(), vec![])
}

fn render_tabs<M: 'static>(
    tabs: &HashMap<TabId, Tab>,
    tab_index: &Vec<TabId>,
    selected_tab_id: &TabId,
    messenger_gen: impl Fn() -> MessengerGen<Msg, M>,
) -> Html<M> {
    let mut chat_tabs_list = tab_index
        .iter()
        .map(|tab_id| match tabs.get(tab_id) {
            None => Html::none(),
            Some(tab) => btn::tab(
                tab_id == selected_tab_id,
                true,
                Attributes::new().href(String::from("#chat-tabs-") + &tab_id.to_string()),
                Events::new().on_click({
                    let m = messenger_gen()();
                    let tab_id = tab_id.clone();
                    move |_| m(Msg::SetSelectedTab(tab_id))
                }),
                &tab.name,
            ),
        })
        .collect::<Vec<Html<M>>>();
    chat_tabs_list.push(btn::add(
        Attributes::new().string("data-btn_add-tab", "true"),
        Events::new().on_click({
            let m = messenger_gen()();
            |_| m(Msg::ShowCreateTabDialog())
        }),
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

fn render_tabs_log_column<M>(tab: &Tab, tab_id: &TabId, selected_tab_id: &TabId) -> Html<M> {
    Html::div(
        Attributes::new()
            .class("chat-tabs-log-column")
            .string(
                "data-chat-selected",
                (tab_id == selected_tab_id).to_string(),
            )
            .id(String::from("chat-tabs-") + &tab_id.to_string()),
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
                Icon::Character(
                    if let Some(initial) = message.sender.as_str().chars().next() {
                        initial
                    } else {
                        'あ'
                    },
                ),
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
