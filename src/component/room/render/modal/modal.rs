use super::{
    super::{awesome, btn, color_picker, modal},
    common, CharacterSelecterType, ChatDataCollection, ColorPickerType, Modal, Msg, PersonalData,
    SelectImageModal,
};
use crate::model::{Icon, Resource, Table, World};
use kagura::prelude::*;
use std::{collections::HashSet, iter::Iterator};
use wasm_bindgen::JsCast;

pub fn chat_log(chat_data: &ChatDataCollection, resource: &Resource) -> Html<Msg> {
    let selecting_tab_idx = chat_data.selecting_tab_idx;
    let selecting_tab = &chat_data.tabs[selecting_tab_idx];

    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("チャット履歴"),
                modal::body(
                    Attributes::new().class("scroll-v"),
                    Events::new(),
                    selecting_tab
                        .iter()
                        .rev()
                        .rev()
                        .map(|item| {
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
                                            String::from("")
                                                + item.display_name()
                                                + "@"
                                                + item.peer_id(),
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
                        })
                        .collect(),
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

pub fn chat_tab_editor(chat_data: &ChatDataCollection) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            6,
            Attributes::new(),
            Events::new(),
            vec![
                header("チャット履歴"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v")
                        .class("pure-form")
                        .class("keyvalue")
                        .class("keyvalue-rev"),
                    Events::new(),
                    vec![
                        chat_data
                            .tabs
                            .iter()
                            .enumerate()
                            .map(|(idx, tab)| {
                                vec![
                                    Html::input(
                                        Attributes::new().value(tab.name()),
                                        Events::new().on_input(move |name| {
                                            Msg::SetChatTabNameToTransport(idx, name)
                                        }),
                                        vec![],
                                    ),
                                    btn::danger(
                                        Attributes::new(),
                                        Events::new()
                                            .on_click(move |_| Msg::RemoveChatTabToTransport(idx)),
                                        vec![awesome::i("fa-times")],
                                    ),
                                ]
                            })
                            .flatten()
                            .collect(),
                        vec![btn::secondary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::AddChatTabTotransport),
                            vec![awesome::i("fa-plus")],
                        )],
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

fn header(name: impl Into<String>) -> Html<Msg> {
    modal::header(
        Attributes::new()
            .style("display", "grid")
            .style("grid-template-columns", "1fr max-content"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("text-label"),
                Events::new(),
                vec![Html::text(name)],
            ),
            Html::div(
                Attributes::new().class("linear-h"),
                Events::new(),
                vec![btn::close(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::CloseModal),
                )],
            ),
        ],
    )
}
