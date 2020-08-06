use super::super::super::super::{awesome, btn, modeless, text};
use super::super::super::state::{chat, dicebot, Modal, Modeless};
use super::Msg;
use crate::{
    block::{self, chat::item::Sender, BlockId},
    model::{self},
    Color, Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

mod common {
    pub use super::super::common::*;
}

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    modeless_id: model::modeless::ModelessId,
    modeless: &model::Modeless<Modeless>,
    grubbed: Option<model::modeless::ModelessId>,
    chat_state: &chat::State,
    dicebot_state: &dicebot::State,
    chat_data: &block::Chat,
    personal_data: &model::PersonalData,
    selecting_tab_id: &BlockId,
    selecting_tab: &block::chat::Tab,
) -> Html {
    let take_num = chat_state.take_num();
    let is_grubbed = grubbed.is_some();

    super::frame(
        modeless_id,
        modeless,
        Attributes::new(),
        Events::new(),
        vec![
            super::header(
                modeless_id,
                grubbed,
                Attributes::new().class("frame-header-tab"),
                Events::new(),
                chat_tab_list(block_field, chat_data, selecting_tab_id),
            ),
            modeless::body(
                Attributes::new()
                    .class("linear-v")
                    .style("grid-template-rows", "1fr"),
                Events::new().on_mousemove(move |e| {
                    if !is_grubbed {
                        e.stop_propagation();
                    }
                    Msg::NoOp
                }),
                vec![
                    Html::div(
                        Attributes::new()
                            .class("container-a linear-v")
                            .style("align-self", "stretch")
                            .style("grid-template-rows", "max-content max-content 1fr"),
                        Events::new(),
                        vec![
                            if selecting_tab.len() > take_num {
                                btn::secondary(
                                    Attributes::new(),
                                    Events::new().on_click({
                                        let selecting_tab_id = selecting_tab_id.clone();
                                        move |_| Msg::OpenModal(Modal::ChatLog(selecting_tab_id))
                                    }),
                                    vec![Html::text("全履歴を表示")],
                                )
                            } else {
                                Html::div(Attributes::new(), Events::new(), vec![])
                            },
                            chat_item_list(block_field, resource, selecting_tab, take_num),
                        ],
                    ),
                    Html::div(
                        Attributes::new()
                            .class("pure-form linear-v")
                            .style("grid-template-rows", "1fr"),
                        Events::new(),
                        vec![
                            Html::div(Attributes::new(), Events::new(), vec![]),
                            Html::div(
                                Attributes::new().class("keyvalueoption"),
                                Events::new(),
                                dicebot_menu(dicebot_state),
                            ),
                            Html::div(
                                Attributes::new()
                                    .class("keyvalueoption")
                                    .class("keyvalueoption-align-stretch"),
                                Events::new(),
                                sender_list(block_field, resource, personal_data, chat_state),
                            ),
                            Html::textarea(
                                Attributes::new()
                                    .style("resize", "none")
                                    .class("text-wrap")
                                    .value(chat_state.inputing_message()),
                                Events::new()
                                    .on_input(Msg::SetInputingChatMessage)
                                    .on_keydown(|e| {
                                        if e.key_code() == 13
                                            && !e.shift_key()
                                            && !e.ctrl_key()
                                            && !e.alt_key()
                                        {
                                            e.prevent_default();
                                            Msg::SendInputingChatMessage
                                        } else {
                                            Msg::NoOp
                                        }
                                    }),
                                vec![],
                            ),
                            Html::div(
                                Attributes::new().class("justify-r"),
                                Events::new(),
                                vec![Html::div(
                                    Attributes::new().class("linear-h"),
                                    Events::new(),
                                    vec![btn::info(
                                        Attributes::new(),
                                        Events::new().on_click(|_| Msg::SendInputingChatMessage),
                                        vec![awesome::i("fa-paper-plane"), Html::text(" 送信")],
                                    )],
                                )],
                            ),
                        ],
                    ),
                ],
            ),
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn chat_item_list(
    block_field: &block::Field,
    resource: &Resource,
    selecting_tab: &block::chat::Tab,
    take_num: usize,
) -> Html {
    Html::div(
        Attributes::new()
            .style("align-self", "stretch")
            .class("scroll-v")
            .id("chat-area"),
        Events::new(),
        selecting_tab
            .iter()
            .rev()
            .take(take_num)
            .rev()
            .filter_map(|(timestamp, item_id)| {
                block_field
                    .get::<block::chat::Item>(item_id)
                    .map(|item| (timestamp, item))
            })
            .map(|(timestamp, item)| chat_item(resource, timestamp, item))
            .collect(),
    )
}

fn chat_item(resource: &Resource, timestamp: f64, item: &block::chat::Item) -> Html {
    Html::div(
        Attributes::new().class("pure-form chat-item"),
        Events::new(),
        vec![
            common::chat_icon(
                Attributes::new().class("icon-medium").class("chat-icon"),
                item.icon(),
                item.display_name(),
                resource,
            ),
            Html::div(
                Attributes::new().class("chat-args").class("linear-h"),
                Events::new(),
                vec![
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text(item.display_name())],
                    ),
                    Html::span(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text(
                            js_sys::Date::new(&wasm_bindgen::JsValue::from(timestamp))
                                .to_locale_string("ja-JP", object! {}.as_ref())
                                .as_string()
                                .unwrap_or(String::from("")),
                        )],
                    ),
                    Html::span(
                        Attributes::new().class("aside"),
                        Events::new(),
                        vec![Html::text(String::from("@") + item.peer_id())],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("chat-payload"),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class("text-wrap"),
                        Events::new(),
                        vec![Html::text(item.text())],
                    ),
                    if let Some(reply) = item.reply() {
                        Html::div(
                            Attributes::new().class("text-wrap"),
                            Events::new(),
                            vec![Html::text(reply)],
                        )
                    } else {
                        Html::none()
                    },
                ],
            ),
        ],
    )
}

fn chat_tab_list(
    block_field: &block::Field,
    chat_data: &block::Chat,
    selecting_tab_id: &BlockId,
) -> Html {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            chat_data
                .tabs()
                .iter()
                .enumerate()
                .filter_map(|(tab_idx, tab_id)| {
                    block_field
                        .get::<block::chat::Tab>(tab_id)
                        .map(|tab| (tab_idx, tab_id, tab))
                })
                .map(|(tab_idx, tab_id, tab)| {
                    btn::frame_tab(
                        *tab_id == *selecting_tab_id,
                        false,
                        Events::new().on_click(move |_| Msg::SetSelectingChatTabIdx(tab_idx)),
                        tab.name(),
                    )
                })
                .collect(),
            vec![btn::transparent(
                Attributes::new(),
                Events::new().on_click(|_| Msg::OpenModal(Modal::ChatTabEditor)),
                vec![awesome::i("fa-plus")],
            )],
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn dicebot_menu(dicebot_state: &dicebot::State) -> Vec<Html> {
    vec![
        Html::div(
            Attributes::new().style("justify-self", "right"),
            Events::new(),
            vec![Html::text("ダイスボット")],
        ),
        text::div(
            dicebot_state
                .bcdice()
                .system_info()
                .map(|system_info| system_info.name().to_string())
                .unwrap_or("［未選択］".to_string()),
        ),
        btn::secondary(
            Attributes::new(),
            Events::new().on_click(|_| Msg::OpenModal(Modal::DicebotSelecter)),
            vec![Html::text("編集")],
        ),
    ]
}

fn sender_list(
    block_field: &block::Field,
    resource: &Resource,
    personal_data: &model::PersonalData,
    chat_state: &chat::State,
) -> Vec<Html> {
    vec![
        Html::div(
            Attributes::new().style("justify-self", "right"),
            Events::new(),
            vec![Html::text("送信元")],
        ),
        Html::div(
            Attributes::new()
                .class("flex-h")
                .class("flex-padding")
                .class("centering-v-i"),
            Events::new(),
            chat_state
                .senders()
                .iter()
                .enumerate()
                .map(|(idx, sender)| {
                    sender_item(
                        block_field,
                        resource,
                        personal_data,
                        idx,
                        sender,
                        idx == chat_state.selecting_sender_idx(),
                    )
                })
                .collect(),
        ),
        btn::secondary(
            Attributes::new(),
            Events::new().on_click(|_| Msg::OpenModal(Modal::SenderCharacterSelecter)),
            vec![Html::text("編集")],
        ),
    ]
}

fn sender_item(
    block_field: &block::Field,
    resource: &Resource,
    personal_data: &model::PersonalData,
    sender_idx: usize,
    sender: &Sender,
    is_selected: bool,
) -> Html {
    use block::chat::item::Icon;

    let attrs = if is_selected {
        Attributes::new().class("icon-selected")
    } else {
        Attributes::new()
    };
    if let Some((icon, name)) = match sender {
        Sender::User => {
            let icon = personal_data
                .icon()
                .map(|icon_id| Icon::Resource(icon_id.clone()))
                .unwrap_or(Icon::DefaultUser);
            Some((icon, personal_data.name()))
        }
        Sender::Character(character_id) => {
            if let Some(character) = block_field.get::<block::Character>(character_id) {
                let icon = character
                    .texture_id()
                    .map(|r_id| Icon::Resource(r_id.clone()))
                    .unwrap_or(Icon::DefaultUser);
                Some((icon, character.name()))
            } else {
                None
            }
        }
        _ => None,
    } {
        Html::div(
            Attributes::new()
                .class("chat-sender")
                .string("data-sender-idx", sender_idx.to_string()),
            Events::new().on_click(move |_| Msg::SetSelectingChatSenderIdx(sender_idx)),
            vec![
                common::chat_icon(
                    attrs.class("clickable").class("icon-small").title(name),
                    &icon,
                    name,
                    resource,
                ),
                Html::span(
                    Attributes::new().class("chat-sender-text"),
                    Events::new(),
                    vec![Html::text(name)],
                ),
            ],
        )
    } else {
        Html::none()
    }
}
