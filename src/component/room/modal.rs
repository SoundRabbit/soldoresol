use super::{
    super::{awesome, btn, color_picker, modal},
    common, CharacterSelecterType, ChatDataCollection, ColorPickerType, Modal, Msg, PersonalData,
    SelectImageModal,
};
use crate::model::{Icon, Resource, Table, World};
use kagura::prelude::*;
use std::{collections::HashSet, iter::Iterator};
use wasm_bindgen::JsCast;

pub fn resource(resource: &Resource) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("画像"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v grid container")
                        .style("min-height", "50vh"),
                    Events::new(),
                    resource
                        .get_image_urls()
                        .into_iter()
                        .map(|(_, img_url)| {
                            Html::img(
                                Attributes::new()
                                    .class("grid-w-2")
                                    .class("pure-img")
                                    .string("src", img_url.as_str()),
                                Events::new(),
                                vec![],
                            )
                        })
                        .collect(),
                ),
                modal::footer(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
                ),
            ],
        )],
    )
}

pub fn select_image(resource: &Resource, modal_type: &SelectImageModal) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("画像を選択"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v grid container")
                        .style("min-height", "50vh"),
                    Events::new(),
                    resource
                        .get_image_urls()
                        .into_iter()
                        .map(|(data_id, img_url)| {
                            Html::div(
                                Attributes::new().class("grid-w-2 clickable"),
                                Events::new().on_click({
                                    let modal_type = modal_type.clone();
                                    move |_| match modal_type {
                                        SelectImageModal::Character(c_id) => {
                                            Msg::SetCharacterImageToTransport(c_id, data_id)
                                        }
                                        SelectImageModal::Player => {
                                            Msg::SetPersonalDataWithIconImage(data_id)
                                        }
                                        SelectImageModal::Table => {
                                            Msg::SetTableImageToTransport(data_id)
                                        }
                                    }
                                }),
                                vec![Html::img(
                                    Attributes::new()
                                        .class("pure-img")
                                        .string("src", img_url.as_str()),
                                    Events::new(),
                                    vec![],
                                )],
                            )
                        })
                        .collect(),
                ),
                modal::footer(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
                ),
            ],
        )],
    )
}

pub fn personal_setting(personal_data: &PersonalData, resource: &Resource) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("個人設定"),
                modal::body(
                    Attributes::new().class("scroll-v pure-form"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("chat-item"),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new()
                                    .class("chat-icon linear-v")
                                    .style("justify-items", "center"),
                                Events::new(),
                                vec![
                                    {
                                        let icon = personal_data
                                            .icon
                                            .map(|r_id| Icon::Resource(r_id))
                                            .unwrap_or(Icon::DefaultUser);
                                        common::chat_icon(
                                            Attributes::new().class("icon-large"),
                                            &icon,
                                            &personal_data.name,
                                            resource,
                                        )
                                    },
                                    btn::primary(
                                        Attributes::new(),
                                        Events::new().on_click(|_| {
                                            Msg::OpenModal(Modal::SelectImage(
                                                SelectImageModal::Player,
                                            ))
                                        }),
                                        vec![Html::text("画像を選択")],
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new().class("chat-args keyvalue"),
                                Events::new(),
                                vec![
                                    Html::label(
                                        Attributes::new().string("for", "player-name"),
                                        Events::new(),
                                        vec![Html::text("プレイヤー名")],
                                    ),
                                    Html::input(
                                        Attributes::new()
                                            .id("player-name")
                                            .value(&personal_data.name),
                                        Events::new()
                                            .on_input(|n| Msg::SetPersonalDataWithPlayerName(n)),
                                        vec![],
                                    ),
                                ],
                            ),
                        ],
                    )],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

pub fn color_picker(color_picker_type: ColorPickerType) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("色の選択"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v")
                        .class("centering")
                        .class("centering-a"),
                    Events::new(),
                    vec![color_picker::picker(Msg::NoOp, move |color| {
                        Msg::PickColor(color, color_picker_type)
                    })],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

pub fn character_selecter(
    character_selecter_type: CharacterSelecterType,
    selected_character_id: HashSet<u128>,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            6,
            Attributes::new(),
            Events::new(),
            vec![
                header("キャラクターの選択"),
                modal::body(
                    Attributes::new().class("scroll-v"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new()
                            .class("container-a")
                            .class("keyvalueoption"),
                        Events::new().on_click({
                            let selected_character_id = selected_character_id.clone();
                            move |e| {
                                e.target()
                                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                    .and_then(|e| e.get_attribute("data-character-id"))
                                    .and_then(|attr| attr.parse().ok())
                                    .map(move |character_id| {
                                        Msg::SelectCharacter(
                                            character_id,
                                            !selected_character_id.contains(&character_id),
                                            character_selecter_type,
                                        )
                                    })
                                    .unwrap_or(Msg::NoOp)
                            }
                        }),
                        world
                            .characters()
                            .map(|(character_id, character)| {
                                vec![
                                    {
                                        let icon = character
                                            .texture_id()
                                            .map(|r_id| Icon::Resource(r_id))
                                            .unwrap_or(Icon::DefaultUser);
                                        common::chat_icon(
                                            Attributes::new()
                                                .class("icon-medium")
                                                .class("clickable")
                                                .string(
                                                    "data-character-id",
                                                    character_id.to_string(),
                                                ),
                                            &icon,
                                            character.name(),
                                            resource,
                                        )
                                    },
                                    Html::div(
                                        Attributes::new()
                                            .class("clickable")
                                            .string("data-character-id", character_id.to_string()),
                                        Events::new(),
                                        vec![Html::text(character.name())],
                                    ),
                                    btn::check(
                                        selected_character_id.contains(&character_id),
                                        Attributes::new()
                                            .string("data-character-id", character_id.to_string()),
                                        Events::new(),
                                    ),
                                ]
                            })
                            .flatten()
                            .collect(),
                    )],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

pub fn table_setting<'a>(
    table: &Table,
    tables: impl Iterator<Item = (&'a u128, &'a Table)>,
    resource: &Resource,
) -> Html<Msg> {
    let [width, height] = table.size();
    let (width, height) = (*width, *height);
    let is_bind_to_grid = table.is_bind_to_grid();

    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                header("テーブル設定"),
                modal::body(
                    Attributes::new()
                        .class("keyvalue")
                        .class("keyvalue-align-stretch"),
                    Events::new(),
                    vec![
                        Html::div(Attributes::new().class("scroll-v"), Events::new(), vec![]),
                        Html::div(
                            Attributes::new()
                                .class("scroll-v")
                                .class("keyvalue")
                                .class("editormodeless")
                                .class("pure-form")
                                .class("keyvalue-align-start"),
                            Events::new(),
                            vec![
                                Html::div(
                                    Attributes::new()
                                        .class("container-a")
                                        .class("centering")
                                        .class("centering-a"),
                                    Events::new(),
                                    vec![
                                        table
                                            .image_texture_id()
                                            .and_then(|data_id| resource.get_as_image_url(&data_id))
                                            .map(|img_url| {
                                                Html::img(
                                                    Attributes::new()
                                                        .class("pure-img")
                                                        .string("src", img_url.as_str()),
                                                    Events::new(),
                                                    vec![],
                                                )
                                            })
                                            .unwrap_or(Html::none()),
                                        btn::primary(
                                            Attributes::new(),
                                            Events::new().on_click({
                                                move |_| {
                                                    Msg::OpenModal(Modal::SelectImage(
                                                        SelectImageModal::Table,
                                                    ))
                                                }
                                            }),
                                            vec![Html::text("画像を選択")],
                                        ),
                                    ],
                                ),
                                Html::div(
                                    Attributes::new().class("container-a"),
                                    Events::new(),
                                    vec![
                                        Html::div(
                                            Attributes::new()
                                                .class("container-a")
                                                .class("keyvalue"),
                                            Events::new(),
                                            vec![
                                                Html::span(
                                                    Attributes::new(),
                                                    Events::new(),
                                                    vec![Html::text("幅")],
                                                ),
                                                Html::input(
                                                    Attributes::new()
                                                        .type_("number")
                                                        .value(width.to_string()),
                                                    Events::new().on_input(move |width| {
                                                        if let Ok(width) = width.parse::<f64>() {
                                                            Msg::SetTableSizeToTransport([
                                                                width.floor(),
                                                                height,
                                                            ])
                                                        } else {
                                                            Msg::NoOp
                                                        }
                                                    }),
                                                    vec![],
                                                ),
                                                Html::span(
                                                    Attributes::new(),
                                                    Events::new(),
                                                    vec![Html::text("高さ")],
                                                ),
                                                Html::input(
                                                    Attributes::new()
                                                        .type_("number")
                                                        .value(height.to_string()),
                                                    Events::new().on_input(move |height| {
                                                        if let Ok(height) = height.parse::<f64>() {
                                                            Msg::SetTableSizeToTransport([
                                                                width,
                                                                height.floor(),
                                                            ])
                                                        } else {
                                                            Msg::NoOp
                                                        }
                                                    }),
                                                    vec![],
                                                ),
                                            ],
                                        ),
                                        Html::div(
                                            Attributes::new().class("keyvalue").title(""),
                                            Events::new(),
                                            vec![
                                                Html::span(
                                                    Attributes::new().class("text-label"),
                                                    Events::new(),
                                                    vec![Html::text("グリッドにスナップ")],
                                                ),
                                                btn::toggle(
                                                    is_bind_to_grid,
                                                    Attributes::new(),
                                                    Events::new().on_click(move |_| {
                                                        Msg::SetIsBindToGridToTransport(
                                                            !is_bind_to_grid,
                                                        )
                                                    }),
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

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
