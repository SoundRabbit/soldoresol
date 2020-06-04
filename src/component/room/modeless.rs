use super::{
    super::{btn, image, modeless},
    Modal, ModelessState, Msg,
};
use crate::{
    model::{Character, Resource, Tablemask, World},
    random_id,
};
use kagura::prelude::*;

pub fn object(
    modeless_idx: usize,
    state: &ModelessState,
    tabs: &Vec<u128>,
    focused: usize,
    world: &World,
    resource: &Resource,
) -> Html<Msg> {
    let focused_id = tabs[focused];
    modeless::frame(
        &state.loc_a,
        &state.loc_b,
        Attributes::new(),
        Events::new()
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
            }),
        vec![
            modeless::header(
                Attributes::new()
                    .style("display", "grid")
                    .style("grid-template-columns", "1fr max-content"),
                Events::new(),
                vec![
                    Html::div(Attributes::new(), Events::new(), vec![]),
                    Html::div(
                        Attributes::new(),
                        Events::new(),
                        vec![btn::close(
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::CloseModeless(modeless_idx)),
                        )],
                    ),
                ],
            ),
            if let Some(character) = world.character(&focused_id) {
                object_character(character, focused_id, resource)
            } else if let Some(tablemask) = world.tablemask(&focused_id) {
                object_tablemask(tablemask, focused_id)
            } else {
                Html::none()
            },
            modeless::footer(Attributes::new(), Events::new(), vec![]),
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
        ],
    )
}

fn object_character(character: &Character, character_id: u128, resource: &Resource) -> Html<Msg> {
    modeless::body(
        Attributes::new().class("scroll-v flex-h"),
        Events::new(),
        vec![Html::div(
            Attributes::new().class("container-a"),
            Events::new(),
            vec![
                Html::div(
                    Attributes::new().class("centering-a"),
                    Events::new(),
                    vec![
                        character
                            .texture_id()
                            .and_then(|data_id| resource.get_as_image(&data_id))
                            .map(|img| {
                                Html::component(image::new(
                                    img,
                                    Attributes::new().class("pure-img"),
                                ))
                            })
                            .unwrap_or(Html::none()),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click({
                                move |_| Msg::OpenModal(Modal::SelectCharacterImage(character_id))
                            }),
                            vec![Html::text("画像を選択")],
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class("keyvalue pure-form"),
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
                    ],
                ),
                Html::div(
                    Attributes::new().class("keyvalue pure-form"),
                    Events::new(),
                    vec![
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
                                    Msg::SetTablemaskSize(
                                        tablemask_id,
                                        [w, if size_is_binded { w } else { height }],
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
                        Attributes::new().class("fas fa-link text-color-light"),
                        Events::new()
                            .on_click(move |_| Msg::SetTablemaskSizeIsBinded(tablemask_id, false)),
                        vec![],
                    )
                } else {
                    btn::transparent(
                        Attributes::new().class("fas fa-link text-color-gray"),
                        Events::new()
                            .on_click(move |_| Msg::SetTablemaskSizeIsBinded(tablemask_id, true)),
                        vec![],
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
                                    Msg::SetTablemaskSize(
                                        tablemask_id,
                                        [if size_is_binded { h } else { width }, h],
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
