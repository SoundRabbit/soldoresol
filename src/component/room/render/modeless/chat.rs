use super::super::super::super::{awesome, btn, modeless, text};
use super::super::super::state::{chat, Modal, Modeless};
use super::Msg;
use crate::{
    block::{self, BlockId},
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
    chat_data: &block::Chat,
    personal_data: &model::PersonalData,
    selecting_tab_id: &BlockId,
    selecting_tab: &block::chat::Tab,
) -> Html<Msg> {
    let take_num = chat_state.take_num();

    super::frame(
        modeless_id,
        modeless,
        Attributes::new(),
        Events::new(),
        vec![
            super::header(
                modeless_id,
                grubbed,
                Attributes::new(),
                Events::new(),
                text::div("チャット"),
            ),
            modeless::body(
                Attributes::new()
                    .class("linear-v")
                    .style("grid-template-rows", "1fr fit-content(40%)")
                    .style("row-gap", "0"),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new()
                            .class("container-a linear-v")
                            .style("align-self", "stretch")
                            .style("grid-template-rows", "max-content 1fr"),
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
                            chat_tab_list(block_field, chat_data, selecting_tab_id),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class("pure-form linear-v"),
                        Events::new(),
                        vec![
                            sender_list(block_field, resource, personal_data, chat_state),
                            Html::textarea(
                                Attributes::new()
                                    .style("resize", "none")
                                    .class("text-wrap")
                                    .value(chat_state.inputing_message()),
                                Events::new().on_input(|m| Msg::NoOp).on_keydown(|e| {
                                    if e.key_code() == 13
                                        && !e.shift_key()
                                        && !e.ctrl_key()
                                        && !e.alt_key()
                                    {
                                        e.prevent_default();
                                        Msg::NoOp
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
                                        Events::new().on_click(|_| Msg::NoOp),
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
) -> Html<Msg> {
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
            .filter_map(|item_id| block_field.get::<block::chat::Item>(item_id))
            .map(|item| chat_item(resource, item))
            .collect(),
    )
}

fn chat_item(resource: &Resource, item: &block::chat::Item) -> Html<Msg> {
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
                Attributes::new().class("chat-args"),
                Events::new(),
                vec![Html::text(
                    String::from("") + item.display_name() + "@" + item.peer_id(),
                )],
            ),
            Html::div(
                Attributes::new().class("chat-payload"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("text-wrap"),
                    Events::new(),
                    vec![Html::text(item.payload())],
                )],
            ),
        ],
    )
}

fn chat_tab_list(
    block_field: &block::Field,
    chat_data: &block::Chat,
    selecting_tab_id: &BlockId,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("keyvalue").class("aside"),
        Events::new(),
        vec![
            btn::info(
                Attributes::new(),
                Events::new().on_click(|_| Msg::OpenModal(Modal::ChatTabEditor)),
                vec![Html::text("設定")],
            ),
            Html::div(
                Attributes::new(),
                Events::new(),
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
                        btn::selectable(
                            *tab_id == *selecting_tab_id,
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::NoOp),
                            vec![Html::text(tab.name())],
                        )
                    })
                    .collect(),
            ),
        ],
    )
}

fn sender_list(
    block_field: &block::Field,
    resource: &Resource,
    personal_data: &model::PersonalData,
    chat_state: &chat::State,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("keyvalue"),
        Events::new(),
        vec![
            btn::info(
                Attributes::new().class("aside"),
                Events::new().on_click(|_| Msg::OpenModal(Modal::SenderCharacterSelecter)),
                vec![Html::text("送信元")],
            ),
            Html::div(
                Attributes::new()
                    .class("flex-h")
                    .class("flex-padding")
                    .class("centering-v-i"),
                Events::new().on_click(|e| {
                    e.target()
                        .and_then(|e| e.dyn_into::<web_sys::Element>().ok())
                        .and_then(|e| e.get_attribute("data-sender-idx"))
                        .and_then(|data| data.parse().ok())
                        .map(|sender_idx| Msg::SetChatSender(sender_idx))
                        .unwrap_or(Msg::NoOp)
                }),
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
        ],
    )
}

fn sender_item(
    block_field: &block::Field,
    resource: &Resource,
    personal_data: &model::PersonalData,
    sender_idx: usize,
    sender: &chat::Sender,
    is_selected: bool,
) -> Html<Msg> {
    use block::chat::item::Icon;

    let attrs = if is_selected {
        Attributes::new().class("icon-selected")
    } else {
        Attributes::new()
    };
    match sender {
        chat::Sender::Player => {
            let icon = personal_data
                .icon()
                .map(|icon_id| Icon::Resource(*icon_id))
                .unwrap_or(Icon::DefaultUser);
            common::chat_icon(
                attrs
                    .class("clickable")
                    .class("icon-small")
                    .string("data-sender-idx", sender_idx.to_string())
                    .title(personal_data.name()),
                &icon,
                personal_data.name(),
                resource,
            )
        }
        chat::Sender::Character(character_id) => {
            if let Some(character) = block_field.get::<block::Character>(character_id) {
                let icon = character
                    .texture_id()
                    .map(|r_id| Icon::Resource(*r_id))
                    .unwrap_or(Icon::DefaultUser);
                common::chat_icon(
                    attrs
                        .class("clickable")
                        .class("icon-small")
                        .string("data-sender-idx", sender_idx.to_string())
                        .title(character.name()),
                    &icon,
                    character.name(),
                    resource,
                )
            } else {
                Html::none()
            }
        }
    }
}
