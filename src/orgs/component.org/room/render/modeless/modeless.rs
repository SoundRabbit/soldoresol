use super::{
    super::{awesome, btn, modeless},
    common, CharacterSelecterType, ChatDataCollection, ChatSender, Icon, Modal, ModelessState, Msg,
    PersonalData, SelectImageModal,
};
use crate::{
    model::{Character, Property, PropertyValue, Resource, Tablemask, World},
    random_id,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn object(
    modeless_id: u128,
    state: &ModelessState,
    tabs: &Vec<u128>,
    focused: usize,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    let focused_id = tabs[focused];
    frame(
        modeless_id,
        state,
        Attributes::new(),
        Events::new(),
        vec![
            header(
                modeless_id,
                Html::div(Attributes::new(), Events::new(), vec![]),
            ),
            if let Some(character) = world.character(&focused_id) {
                object_character(character, focused_id, resource)
            } else {
                Html::none()
            },
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn object_character(character: &Character, character_id: u128, resource: &Resource) -> Html<Msg> {
    let [width, height] = character.size();
    let width = *width;
    let height = *height;
    modeless::body(
        Attributes::new().class("scroll-v"),
        Events::new(),
        vec![Html::div(
            Attributes::new()
                .class("editormodeless")
                .class("pure-form")
                .class("linear-v"),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class("keyvalueoption"),
                    Events::new(),
                    vec![
                        Html::span(Attributes::new(), Events::new(), vec![Html::text("Name")]),
                        Html::input(
                            Attributes::new().value(character.name()).type_("text"),
                            Events::new().on_input(move |s| {
                                Msg::SetCharacterNameToTransport(character_id, s)
                            }),
                            vec![],
                        ),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::AddCharacterToDb(character_id)),
                            vec![Html::text("保存")],
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new()
                        .class("keyvalue")
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
                                character
                                    .texture_id()
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
                                                SelectImageModal::Character(character_id),
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
                                        .class("keyvalueoption")
                                        .class("container-a"),
                                    Events::new(),
                                    vec![
                                        Html::label(
                                            Attributes::new(),
                                            Events::new(),
                                            vec![Html::text("幅")],
                                        ),
                                        Html::input(
                                            Attributes::new()
                                                .type_("number")
                                                .value(width.to_string()),
                                            Events::new().on_input(move |width| {
                                                width
                                                    .parse()
                                                    .map(|width| {
                                                        Msg::SetCharacterSizeToTransport(
                                                            character_id,
                                                            Some(width),
                                                            Some(height),
                                                        )
                                                    })
                                                    .unwrap_or(Msg::NoOp)
                                            }),
                                            vec![],
                                        ),
                                        btn::secondary(
                                            Attributes::new(),
                                            Events::new().on_click(move |_| {
                                                Msg::SetCharacterSizeToTransport(
                                                    character_id,
                                                    None,
                                                    Some(height),
                                                )
                                            }),
                                            vec![Html::text("画像に合わせる")],
                                        ),
                                        Html::label(
                                            Attributes::new(),
                                            Events::new(),
                                            vec![Html::text("高さ")],
                                        ),
                                        Html::input(
                                            Attributes::new()
                                                .type_("number")
                                                .value(height.to_string()),
                                            Events::new().on_input(move |height| {
                                                height
                                                    .parse()
                                                    .map(|height| {
                                                        Msg::SetCharacterSizeToTransport(
                                                            character_id,
                                                            Some(width),
                                                            Some(height),
                                                        )
                                                    })
                                                    .unwrap_or(Msg::NoOp)
                                            }),
                                            vec![],
                                        ),
                                        btn::secondary(
                                            Attributes::new(),
                                            Events::new().on_click(move |_| {
                                                Msg::SetCharacterSizeToTransport(
                                                    character_id,
                                                    Some(width),
                                                    None,
                                                )
                                            }),
                                            vec![Html::text("画像に合わせる")],
                                        ),
                                    ],
                                ),
                                character_root_property(character_id, &character.property),
                            ],
                        ),
                    ],
                ),
            ],
        )],
    )
}

fn character_root_property(character_id: u128, property: &Property) -> Html<Msg> {
    let property_id = *property.id();
    match property.value() {
        PropertyValue::Children(children) => Html::div(
            Attributes::new()
                .class("container-a")
                .class("keyvalueoption"),
            Events::new(),
            vec![
                children
                    .iter()
                    .map(|property| character_property(character_id, property))
                    .flatten()
                    .collect(),
                btn_add_child_to_character_property(character_id, property_id),
            ]
            .into_iter()
            .flatten()
            .collect(),
        ),
        _ => Html::div(
            Attributes::new()
                .class("container-a")
                .class("keyvalueoption"),
            Events::new(),
            vec![],
        ),
    }
}

fn character_property(character_id: u128, property: &Property) -> Vec<Html<Msg>> {
    let property_id = *property.id();
    match property.value() {
        PropertyValue::None => vec![
            Html::div(
                Attributes::new()
                    .class("keyvalueoption-banner-2")
                    .class("linear-h")
                    .style("grid-auto-columns", "1fr"),
                Events::new(),
                vec![
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(move |_| {
                            Msg::SetCharacterPropertyValueToTransport(
                                character_id,
                                property_id,
                                PropertyValue::Children(vec![Property::new_as_none()]),
                            )
                        }),
                        vec![Html::text(" グループ")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(move |_| {
                            Msg::SetCharacterPropertyValueToTransport(
                                character_id,
                                property_id,
                                PropertyValue::Num(0.0),
                            )
                        }),
                        vec![Html::text("数値")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(move |_| {
                            Msg::SetCharacterPropertyValueToTransport(
                                character_id,
                                property_id,
                                PropertyValue::Str("".into()),
                            )
                        }),
                        vec![Html::text("テキスト")],
                    ),
                ],
            ),
            btn_remove_character_property(character_id, property_id),
        ],
        PropertyValue::Num(n) => vec![
            character_property_key(false, character_id, property_id, property),
            Html::input(
                Attributes::new().value(n.to_string()).type_("number"),
                Events::new().on_input(move |s| {
                    s.parse()
                        .map(|n| {
                            Msg::SetCharacterPropertyValueToTransport(
                                character_id,
                                property_id,
                                PropertyValue::Num(n),
                            )
                        })
                        .unwrap_or(Msg::NoOp)
                }),
                vec![],
            ),
            btn_remove_character_property(character_id, property_id),
        ],
        PropertyValue::Str(s) => vec![
            character_property_key(false, character_id, property_id, property),
            Html::input(
                Attributes::new().value(s),
                Events::new().on_input(move |s| {
                    Msg::SetCharacterPropertyValueToTransport(
                        character_id,
                        property_id,
                        PropertyValue::Str(s),
                    )
                }),
                vec![],
            ),
            btn_remove_character_property(character_id, property_id),
        ],
        PropertyValue::Children(children) => vec![
            character_property_key(true, character_id, property_id, property),
            btn_remove_character_property(character_id, property_id),
            Html::div(
                Attributes::new()
                    .class("container-indent")
                    .class("keyvalueoption")
                    .class("keyvalueoption-banner"),
                Events::new(),
                vec![
                    children
                        .iter()
                        .map(|property| character_property(character_id, property))
                        .flatten()
                        .collect(),
                    btn_add_child_to_character_property(character_id, property_id),
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
        ],
    }
}

fn character_property_key(
    is_banner: bool,
    character_id: u128,
    property_id: u128,
    property: &Property,
) -> Html<Msg> {
    let attributes = Attributes::new().class("centering-v-i");
    let attributes = if is_banner {
        attributes
            .class("keyvalueoption-banner-2")
            .class("keyvalue")
    } else {
        attributes.class("linear-h")
    };
    let is_selected_to_show = property.is_selected_to_show();
    Html::div(
        attributes,
        Events::new(),
        vec![
            btn::check(
                is_selected_to_show,
                Attributes::new(),
                Events::new().on_click(move |_| {
                    Msg::SetCharacterPropertyIsSelectedToShowToTransport(
                        character_id,
                        property_id,
                        !is_selected_to_show,
                    )
                }),
            ),
            Html::input(
                Attributes::new().value(property.name()).type_("text"),
                Events::new().on_input(move |s| {
                    Msg::SetCharacterPropertyNameToTransport(character_id, property_id, s)
                }),
                vec![],
            ),
        ],
    )
}

fn btn_remove_character_property(character_id: u128, property_id: u128) -> Html<Msg> {
    btn::danger(
        Attributes::new(),
        Events::new()
            .on_click(move |_| Msg::RemoveCharacterPropertyToTransport(character_id, property_id)),
        vec![awesome::i("fa-times")],
    )
}

fn btn_add_child_to_character_property(character_id: u128, property_id: u128) -> Vec<Html<Msg>> {
    vec![
        btn::secondary(
            Attributes::new().class("keyvalueoption-banner-2"),
            Events::new().on_click(move |_| {
                Msg::AddChildToCharacterPropertyToTransport(
                    character_id,
                    property_id,
                    Property::new_as_none(),
                )
            }),
            vec![awesome::i("fa-plus")],
        ),
        Html::span(Attributes::new(), Events::new(), vec![]),
    ]
}

pub fn chat(
    modeless_id: u128,
    state: &ModelessState,
    chat_data: &ChatDataCollection,
    personal_data: &PersonalData,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    let selecting_tab_idx = chat_data.selecting_tab_idx;
    let selecting_tab = &chat_data.tabs[selecting_tab_idx];
    let take = chat_data.take;

    frame(
        modeless_id,
        state,
        Attributes::new(),
        Events::new(),
        vec![
            header(
                modeless_id,
                Html::div(Attributes::new(), Events::new(), vec![]),
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
                            if selecting_tab.len() > take {
                                btn::secondary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| Msg::OpenModal(Modal::ChatLog)),
                                    vec![Html::text("全履歴を表示")],
                                )
                            } else {
                                Html::div(Attributes::new(), Events::new(), vec![])
                            },
                            Html::div(
                                Attributes::new()
                                    .style("align-self", "stretch")
                                    .class("scroll-v")
                                    .id("chat-area"),
                                Events::new(),
                                selecting_tab
                                    .iter()
                                    .rev()
                                    .take(take)
                                    .rev()
                                    .map(|item| {
                                        Html::div(
                                            Attributes::new().class("pure-form chat-item"),
                                            Events::new(),
                                            vec![
                                                common::chat_icon(
                                                    Attributes::new()
                                                        .class("icon-medium")
                                                        .class("chat-icon"),
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
                            Html::div(
                                Attributes::new(),
                                Events::new(),
                                vec![
                                    chat_data
                                        .tabs
                                        .iter()
                                        .enumerate()
                                        .map(|(tab_idx, tab)| {
                                            btn::tab(
                                                tab_idx == selecting_tab_idx,
                                                Attributes::new(),
                                                Events::new().on_click(move |_| {
                                                    Msg::SetSelectingChatTabIdx(tab_idx)
                                                }),
                                                tab.name(),
                                            )
                                        })
                                        .collect(),
                                    vec![btn::like_tab(
                                        Attributes::new().class("pure-button-success"),
                                        Events::new()
                                            .on_click(|_| Msg::OpenModal(Modal::ChatTabEditor)),
                                        "設定",
                                    )],
                                ]
                                .into_iter()
                                .flatten()
                                .collect(),
                            ),
                        ],
                    ),
                    Html::div(
                        Attributes::new().class("pure-form linear-v"),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::div(
                                    Attributes::new().class("keyvalue"),
                                    Events::new(),
                                    vec![
                                        btn::info(
                                            Attributes::new().class("aside"),
                                            Events::new().on_click(|_| {
                                                Msg::OpenModal(Modal::CharacterSelecter(
                                                    CharacterSelecterType::ChatSender,
                                                ))
                                            }),
                                            vec![Html::text("送信元")],
                                        ),
                                        Html::div(
                                            Attributes::new()
                                                .class("flex-h")
                                                .class("flex-padding")
                                                .class("centering-v-i"),
                                            Events::new().on_click(|e| {
                                                e.target()
                                                    .and_then(|e| {
                                                        e.dyn_into::<web_sys::Element>().ok()
                                                    })
                                                    .and_then(|e| {
                                                        e.get_attribute("data-sender-idx")
                                                    })
                                                    .and_then(|data| data.parse().ok())
                                                    .map(|sender_idx| {
                                                        Msg::SetChatSender(sender_idx)
                                                    })
                                                    .unwrap_or(Msg::NoOp)
                                            }),
                                            chat_data
                                                .senders
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, sender)| {
                                                    let attrs =
                                                        if idx == chat_data.selecting_sender_idx {
                                                            Attributes::new().class("icon-selected")
                                                        } else {
                                                            Attributes::new()
                                                        };
                                                    match sender {
                                                        ChatSender::Player => {
                                                            let icon = personal_data
                                                                .icon
                                                                .map(|icon_id| {
                                                                    Icon::Resource(icon_id)
                                                                })
                                                                .unwrap_or(Icon::DefaultUser);
                                                            common::chat_icon(
                                                                attrs
                                                                    .class("clickable")
                                                                    .class("icon-small")
                                                                    .string(
                                                                        "data-sender-idx",
                                                                        idx.to_string(),
                                                                    )
                                                                    .title(&personal_data.name),
                                                                &icon,
                                                                &personal_data.name,
                                                                resource,
                                                            )
                                                        }
                                                        ChatSender::Character(character_id) => {
                                                            if let Some(character) =
                                                                world.character(character_id)
                                                            {
                                                                let icon = character
                                                                    .texture_id()
                                                                    .map(|r_id| {
                                                                        Icon::Resource(r_id)
                                                                    })
                                                                    .unwrap_or(Icon::DefaultUser);
                                                                common::chat_icon(
                                                                    attrs
                                                                        .class("clickable")
                                                                        .class("icon-small")
                                                                        .string(
                                                                            "data-sender-idx",
                                                                            idx.to_string(),
                                                                        )
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
                                                })
                                                .collect(),
                                        ),
                                    ],
                                )],
                            ),
                            Html::textarea(
                                Attributes::new()
                                    .style("resize", "none")
                                    .class("text-wrap")
                                    .value(&chat_data.inputing_message),
                                Events::new()
                                    .on_input(|m| Msg::InputChatMessage(m))
                                    .on_keydown(|e| {
                                        if e.key_code() == 13
                                            && !e.shift_key()
                                            && !e.ctrl_key()
                                            && !e.alt_key()
                                        {
                                            e.prevent_default();
                                            Msg::SendChatItemToTransport
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
                                        Events::new().on_click(|_| Msg::SendChatItemToTransport),
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

fn frame(
    modeless_id: u128,
    state: &ModelessState,
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    let attributes = if state
        .grabbed
        .map(|g| g[0] || g[1] || g[2] || g[3])
        .unwrap_or(false)
    {
        attributes.class("grabbed")
    } else {
        attributes
    };
    let attributes = attributes.style("z-index", state.z_index.to_string());
    modeless::frame(
        &state.loc_a,
        &state.loc_b,
        attributes,
        events
            .on_contextmenu(|e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on("wheel", |e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on_mouseup(move |e| {
                e.stop_propagation();
                Msg::GrabModeless(modeless_id, None)
            })
            .on_mousemove({
                let grabbed = state.grabbed.is_some();
                move |e| {
                    e.stop_propagation();
                    if grabbed {
                        Msg::OpenModelessModal(modeless_id)
                    } else {
                        Msg::NoOp
                    }
                }
            })
            .on_mouseleave({
                let grabbed = state.grabbed.is_some();
                move |e| {
                    e.stop_propagation();
                    if grabbed {
                        Msg::OpenModelessModal(modeless_id)
                    } else {
                        Msg::NoOp
                    }
                }
            }),
        vec![
            children,
            vec![Html::div(
                Attributes::new(),
                Events::new().on_mousedown(move |e| {
                    e.stop_propagation();
                    e.target()
                        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        .and_then(|t| t.get_attribute("data-position"))
                        .map(|pos| match pos.as_str() {
                            "top" => {
                                Msg::GrabModeless(modeless_id, Some([true, false, false, false]))
                            }
                            "left" => {
                                Msg::GrabModeless(modeless_id, Some([false, true, false, false]))
                            }
                            "bottom" => {
                                Msg::GrabModeless(modeless_id, Some([false, false, true, false]))
                            }
                            "right" => {
                                Msg::GrabModeless(modeless_id, Some([false, false, false, true]))
                            }
                            "top_left" => {
                                Msg::GrabModeless(modeless_id, Some([true, true, false, false]))
                            }
                            "bottom_left" => {
                                Msg::GrabModeless(modeless_id, Some([false, true, true, false]))
                            }
                            "bottom_right" => {
                                Msg::GrabModeless(modeless_id, Some([false, false, true, true]))
                            }
                            "top_right" => {
                                Msg::GrabModeless(modeless_id, Some([true, false, false, true]))
                            }
                            _ => Msg::NoOp,
                        })
                        .unwrap_or(Msg::NoOp)
                }),
                resizers(),
            )],
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn resizers() -> Vec<Html<Msg>> {
    vec![
        modeless::resizer::top(Attributes::new().string("data-position", "top")),
        modeless::resizer::left(Attributes::new().string("data-position", "left")),
        modeless::resizer::bottom(Attributes::new().string("data-position", "bottom")),
        modeless::resizer::right(Attributes::new().string("data-position", "right")),
        modeless::resizer::top_left(Attributes::new().string("data-position", "top_left")),
        modeless::resizer::bottom_left(Attributes::new().string("data-position", "bottom_left")),
        modeless::resizer::bottom_right(Attributes::new().string("data-position", "bottom_right")),
        modeless::resizer::top_right(Attributes::new().string("data-position", "top_right")),
    ]
}

fn header(modeless_id: u128, header: Html<Msg>) -> Html<Msg> {
    modeless::header(
        Attributes::new()
            .style("display", "grid")
            .style("grid-template-columns", "1fr max-content"),
        Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrabModeless(modeless_id, Some([true, true, true, true]))
        }),
        vec![
            header,
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![btn::close(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::CloseModeless(modeless_id))
                        .on_mousedown(|e| {
                            e.stop_propagation();
                            Msg::NoOp
                        }),
                )],
            ),
        ],
    )
}
