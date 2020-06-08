use super::{
    super::{awesome, btn, icon, modeless},
    CharacterSelecterType, ChatDataCollection, Icon, Modal, ModelessState, Msg, PersonalData,
    SelectImageModal, Sender,
};
use crate::{
    model::{Character, Resource, Tablemask, World},
    random_id,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! stop_propagation {
    () => {{
        |e| {
            e.stop_propagation();
            Msg::NoOp
        }
    }};
}

pub fn object(
    modeless_idx: usize,
    state: &ModelessState,
    tabs: &Vec<u128>,
    focused: usize,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    let focused_id = tabs[focused];
    frame(
        modeless_idx,
        state,
        Attributes::new(),
        Events::new(),
        vec![
            header(
                modeless_idx,
                Html::div(Attributes::new(), Events::new(), vec![]),
            ),
            if let Some(character) = world.character(&focused_id) {
                object_character(character, focused_id, resource)
            } else if let Some(tablemask) = world.tablemask(&focused_id) {
                object_tablemask(tablemask, focused_id)
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
                .class("flex-h"),
            Events::new().on_mousedown(stop_propagation!()),
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
                                    Msg::OpenModal(Modal::SelectImage(SelectImageModal::Character(
                                        character_id,
                                    )))
                                }
                            }),
                            vec![Html::text("画像を選択")],
                        ),
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
                                    Attributes::new().type_("number").value(width.to_string()),
                                    Events::new().on_input(move |width| {
                                        width
                                            .parse()
                                            .map(|width| {
                                                Msg::SetCharacterSize(
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
                                        Msg::SetCharacterSize(character_id, None, Some(height))
                                    }),
                                    vec![Html::text("画像に合わせる")],
                                ),
                                Html::label(
                                    Attributes::new(),
                                    Events::new(),
                                    vec![Html::text("高さ")],
                                ),
                                Html::input(
                                    Attributes::new().type_("number").value(height.to_string()),
                                    Events::new().on_input(move |height| {
                                        height
                                            .parse()
                                            .map(|height| {
                                                Msg::SetCharacterSize(
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
                                        Msg::SetCharacterSize(character_id, Some(width), None)
                                    }),
                                    vec![Html::text("画像に合わせる")],
                                ),
                            ],
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class("container-a"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("keyvalue"),
                        Events::new(),
                        vec![
                            Html::span(Attributes::new(), Events::new(), vec![Html::text("HP")]),
                            Html::input(
                                Attributes::new()
                                    .value(character.hp().to_string())
                                    .type_("number"),
                                Events::new().on_input(move |s| {
                                    if let Ok(s) = s.parse() {
                                        Msg::SetCharacterHp(character_id, s)
                                    } else {
                                        Msg::NoOp
                                    }
                                }),
                                vec![],
                            ),
                            Html::span(Attributes::new(), Events::new(), vec![Html::text("MP")]),
                            Html::input(
                                Attributes::new()
                                    .value(character.mp().to_string())
                                    .type_("number"),
                                Events::new().on_input(move |s| {
                                    if let Ok(s) = s.parse() {
                                        Msg::SetCharacterMp(character_id, s)
                                    } else {
                                        Msg::NoOp
                                    }
                                }),
                                vec![],
                            ),
                        ],
                    )],
                ),
            ],
        )],
    )
}

fn object_tablemask(tablemask: &Tablemask, tablemask_id: u128) -> Html<Msg> {
    let input_width_id = random_id::hex(4);
    let input_height_id = random_id::hex(4);
    let width = tablemask.size()[0];
    let height = tablemask.size()[1];
    let is_rounded = tablemask.is_rounded();

    modeless::body(
        Attributes::new().class("container-a grid pure-form"),
        Events::new(),
        vec![
            Html::fieldset(
                Attributes::new().class("grid-w-11 keyvalue"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new()
                            .class("text-label")
                            .string("for", &input_width_id),
                        Events::new(),
                        vec![Html::text("width")],
                    ),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .value(width.to_string())
                            .class("pure-input-1")
                            .id(input_width_id),
                        Events::new().on_input({
                            let size_is_binded = tablemask.size_is_binded();
                            move |w| {
                                if let Ok(w) = w.parse() {
                                    Msg::SetTablemaskSizeWithStyle(
                                        tablemask_id,
                                        [w, if size_is_binded { w } else { height }],
                                        is_rounded,
                                    )
                                } else {
                                    Msg::NoOp
                                }
                            }
                        }),
                        vec![],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("grid-w-2 centering-a"),
                Events::new(),
                vec![if tablemask.size_is_binded() {
                    btn::transparent(
                        Attributes::new().class("text-color-light"),
                        Events::new()
                            .on_click(move |_| Msg::SetTablemaskSizeIsBinded(tablemask_id, false)),
                        vec![awesome::i("fa-link")],
                    )
                } else {
                    btn::transparent(
                        Attributes::new().class("text-color-gray"),
                        Events::new()
                            .on_click(move |_| Msg::SetTablemaskSizeIsBinded(tablemask_id, true)),
                        vec![awesome::i("fa-link")],
                    )
                }],
            ),
            Html::fieldset(
                Attributes::new().class("grid-w-11 keyvalue"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new()
                            .class("text-label")
                            .string("for", &input_height_id),
                        Events::new(),
                        vec![Html::text("height")],
                    ),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .value(height.to_string())
                            .class("pure-input-1")
                            .id(input_height_id),
                        Events::new().on_input({
                            let size_is_binded = tablemask.size_is_binded();
                            move |h| {
                                if let Ok(h) = h.parse() {
                                    Msg::SetTablemaskSizeWithStyle(
                                        tablemask_id,
                                        [if size_is_binded { h } else { width }, h],
                                        is_rounded,
                                    )
                                } else {
                                    Msg::NoOp
                                }
                            }
                        }),
                        vec![],
                    ),
                ],
            ),
        ],
    )
}

pub fn chat(
    modeless_idx: usize,
    state: &ModelessState,
    chat_data: &ChatDataCollection,
    personal_data: &PersonalData,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    let selecting_tab_idx = chat_data.selecting_tab_idx;
    let selecting_tab = &chat_data.tabs[selecting_tab_idx];
    frame(
        modeless_idx,
        state,
        Attributes::new(),
        Events::new(),
        vec![
            header(
                modeless_idx,
                Html::div(Attributes::new(), Events::new(), vec![]),
            ),
            modeless::body(
                Attributes::new()
                    .class("linear-v")
                    .style("grid-template-rows", "1fr")
                    .style("row-gap", "0"),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new()
                            .class("container-a linear-v")
                            .style("align-self", "stretch")
                            .style("grid-template-rows", "1fr"),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new()
                                    .style("align-self", "stretch")
                                    .class("scroll-v"),
                                Events::new().on_mousedown(stop_propagation!()),
                                selecting_tab
                                    .items
                                    .iter()
                                    .map(|item| {
                                        Html::div(
                                            Attributes::new().class("pure-form chat-item"),
                                            Events::new(),
                                            vec![
                                                chat_icon(
                                                    Attributes::new()
                                                        .class("icon-medium")
                                                        .class("chat-icon"),
                                                    &item.icon,
                                                    &item.display_name,
                                                    resource,
                                                ),
                                                Html::div(
                                                    Attributes::new().class("chat-args"),
                                                    Events::new(),
                                                    vec![Html::text(
                                                        String::from("")
                                                            + &item.display_name
                                                            + "@"
                                                            + &item.peer_id,
                                                    )],
                                                ),
                                                Html::div(
                                                    Attributes::new().class("chat-payload"),
                                                    Events::new(),
                                                    vec![
                                                        Html::pre(
                                                            Attributes::new().class("text-wrap"),
                                                            Events::new(),
                                                            vec![Html::text(&item.payload)],
                                                        ),
                                                        Html::textarea(
                                                            Attributes::new()
                                                                .class("text-wrap")
                                                                .flag("readonly")
                                                                .value(&item.payload),
                                                            Events::new(),
                                                            vec![],
                                                        ),
                                                    ],
                                                ),
                                            ],
                                        )
                                    })
                                    .collect(),
                            ),
                            Html::div(
                                Attributes::new(),
                                Events::new(),
                                chat_data
                                    .tabs
                                    .iter()
                                    .enumerate()
                                    .map(|(tab_idx, tab)| {
                                        btn::tab(
                                            tab_idx == selecting_tab_idx,
                                            Attributes::new(),
                                            Events::new()
                                                .on_mousedown(stop_propagation!())
                                                .on_click(move |_| {
                                                    Msg::SetSelectingChatTabIdx(tab_idx)
                                                }),
                                            &tab.name,
                                        )
                                    })
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
                                    Attributes::new().class("linear-h"),
                                    Events::new(),
                                    vec![
                                        Html::div(
                                            Attributes::new()
                                                .class("centering")
                                                .class("centering-v"),
                                            Events::new(),
                                            vec![Html::text("From:")],
                                        ),
                                        Html::div(
                                            Attributes::new()
                                                .class("linear-h")
                                                .class("centering-v-i"),
                                            Events::new()
                                                .on_mousedown(stop_propagation!())
                                                .on_click(|e| {
                                                    e.target()
                                                        .and_then(|e| {
                                                            e.dyn_into::<web_sys::Element>().ok()
                                                        })
                                                        .and_then(|e| {
                                                            e.get_attribute("data-character-id")
                                                        })
                                                        .and_then(|data| data.parse().ok())
                                                        .map(|sender_idx| {
                                                            Msg::SetChatSender(sender_idx)
                                                        })
                                                        .unwrap_or(Msg::NoOp)
                                                }),
                                            vec![
                                                chat_data
                                                    .senders
                                                    .iter()
                                                    .enumerate()
                                                    .map(|(idx, sender)| {
                                                        let attrs = if idx
                                                            == chat_data.selecting_sender_idx
                                                        {
                                                            Attributes::new().class("icon-selected")
                                                        } else {
                                                            Attributes::new()
                                                        };
                                                        match sender {
                                                            Sender::Player => {
                                                                let icon = personal_data
                                                                    .icon
                                                                    .map(|icon_id| {
                                                                        Icon::Resource(icon_id)
                                                                    })
                                                                    .unwrap_or(Icon::DefaultUser);
                                                                chat_icon(
                                                                    attrs
                                                                        .class("clickable")
                                                                        .class("icon-small")
                                                                        .string(
                                                                            "data-character-id",
                                                                            idx.to_string(),
                                                                        )
                                                                        .title(&personal_data.name),
                                                                    &icon,
                                                                    &personal_data.name,
                                                                    resource,
                                                                )
                                                            }
                                                            Sender::Character(character_id) => {
                                                                let character =
                                                                    world.character(character_id);
                                                                let icon = character
                                                                    .and_then(|c| c.texture_id())
                                                                    .map(|r_id| {
                                                                        Icon::Resource(r_id)
                                                                    })
                                                                    .unwrap_or(Icon::DefaultUser);
                                                                let name = "";
                                                                chat_icon(
                                                                    attrs
                                                                        .class("clickable")
                                                                        .class("icon-small")
                                                                        .string(
                                                                            "data-character-id",
                                                                            idx.to_string(),
                                                                        )
                                                                        .title(name),
                                                                    &icon,
                                                                    name,
                                                                    resource,
                                                                )
                                                            }
                                                        }
                                                    })
                                                    .collect::<Vec<Html<Msg>>>(),
                                                vec![btn::success(
                                                    Attributes::new()
                                                        .class("icon")
                                                        .class("icon-small")
                                                        .class("icon-rounded")
                                                        .class("bg-color-green")
                                                        .class("text-color-light")
                                                        .class("aside")
                                                        .title("追加"),
                                                    Events::new().on_click(|e| {
                                                        e.stop_propagation();
                                                        Msg::OpenModal(Modal::CharacterSelecter(
                                                            CharacterSelecterType::ChatSender,
                                                        ))
                                                    }),
                                                    vec![awesome::i("fa-plus")],
                                                )],
                                            ]
                                            .into_iter()
                                            .flatten()
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
                                    .on_mousedown(stop_propagation!())
                                    .on_input(|m| Msg::InputChatMessage(m)),
                                vec![],
                            ),
                            Html::div(
                                Attributes::new().class("justify-r"),
                                Events::new(),
                                vec![btn::info(
                                    Attributes::new(),
                                    Events::new()
                                        .on_mousedown(stop_propagation!())
                                        .on_click(|_| Msg::SendChatItem),
                                    vec![awesome::i("fa-paper-plane"), Html::text(" 送信")],
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
    modeless_idx: usize,
    state: &ModelessState,
    attributes: Attributes,
    events: Events<Msg>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    let attributes = if state
        .grubbed
        .map(|g| g[0] || g[1] || g[2] || g[3])
        .unwrap_or(false)
    {
        attributes.class("grubbed")
    } else {
        attributes
    };
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
            .on_mousedown(move |e| {
                e.stop_propagation();
                Msg::GrubModeless(modeless_idx, Some([true, true, true, true]))
            })
            .on_mouseup(move |e| {
                e.stop_propagation();
                Msg::GrubModeless(modeless_idx, None)
            })
            .on_mousemove({
                let grubbed = state.grubbed.is_some();
                move |e| {
                    e.stop_propagation();
                    if grubbed {
                        Msg::OpenModelessModal(modeless_idx)
                    } else {
                        Msg::NoOp
                    }
                }
            })
            .on_mouseleave({
                let grubbed = state.grubbed.is_some();
                move |e| {
                    e.stop_propagation();
                    if grubbed {
                        Msg::OpenModelessModal(modeless_idx)
                    } else {
                        Msg::NoOp
                    }
                }
            }),
        vec![children, resizers(modeless_idx)]
            .into_iter()
            .flatten()
            .collect(),
    )
}

fn resizers(modeless_idx: usize) -> Vec<Html<Msg>> {
    vec![
        modeless::resizer::top(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([true, false, false, false]))
        })),
        modeless::resizer::left(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([false, true, false, false]))
        })),
        modeless::resizer::bottom(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([false, false, true, false]))
        })),
        modeless::resizer::right(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([false, false, false, true]))
        })),
        modeless::resizer::top_left(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([true, true, false, false]))
        })),
        modeless::resizer::bottom_left(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([false, true, true, false]))
        })),
        modeless::resizer::bottom_right(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([false, false, true, true]))
        })),
        modeless::resizer::top_right(Events::new().on_mousedown(move |e| {
            e.stop_propagation();
            Msg::GrubModeless(modeless_idx, Some([true, false, false, true]))
        })),
    ]
}

fn header(modeless_idx: usize, header: Html<Msg>) -> Html<Msg> {
    modeless::header(
        Attributes::new()
            .style("display", "grid")
            .style("grid-template-columns", "1fr max-content"),
        Events::new(),
        vec![
            header,
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![btn::close(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::CloseModeless(modeless_idx)),
                )],
            ),
        ],
    )
}

fn chat_icon(attrs: Attributes, icon: &Icon, alt: &str, resource: &Resource) -> Html<Msg> {
    match icon {
        Icon::None => icon::none(attrs),
        Icon::Resource(r_id) => {
            if let Some(img_url) = resource.get_as_image_url(&r_id) {
                icon::from_img(attrs, img_url.as_str())
            } else {
                icon::from_str(attrs, alt)
            }
        }
        _ => icon::from_str(attrs, &alt),
    }
}
