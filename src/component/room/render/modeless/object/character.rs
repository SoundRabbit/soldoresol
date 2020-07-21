use super::super::super::super::super::{awesome, btn, modeless};
use super::super::state::Modal;
use super::Msg;
use crate::{
    block::{self, BlockId},
    resource::Data,
    Resource,
};
use kagura::prelude::*;

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    is_grubbed: bool,
    character: &block::Character,
    character_id: &BlockId,
) -> Html {
    let [width, _, height] = character.size();
    let width = *width;
    let height = *height;
    modeless::body(
        Attributes::new().class("scroll-v"),
        Events::new().on_mousemove(move |e| {
            if !is_grubbed {
                e.stop_propagation();
            }
            Msg::NoOp
        }),
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
                            Events::new().on_input({
                                let character_id = character_id.clone();
                                move |s| Msg::SetCharacterName(character_id, s)
                            }),
                            vec![],
                        ),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::NoOp),
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
                                    .and_then(|data_id| {
                                        if let Some(Data::Image { url, .. }) =
                                            resource.get(&data_id)
                                        {
                                            Some(url)
                                        } else {
                                            None
                                        }
                                    })
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
                                        let character_id = character_id.clone();
                                        move |_| {
                                            Msg::OpenModal(Modal::SelectCharacterImage(
                                                character_id,
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
                                            Events::new().on_input({
                                                let character_id = character_id.clone();
                                                move |width| {
                                                    width
                                                        .parse()
                                                        .map(|width| {
                                                            Msg::SetCharacterSize(
                                                                character_id,
                                                                [Some(width), Some(height)],
                                                            )
                                                        })
                                                        .unwrap_or(Msg::NoOp)
                                                }
                                            }),
                                            vec![],
                                        ),
                                        btn::secondary(
                                            Attributes::new(),
                                            Events::new().on_click({
                                                let character_id = character_id.clone();
                                                move |_| {
                                                    Msg::SetCharacterSize(
                                                        character_id,
                                                        [None, Some(height)],
                                                    )
                                                }
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
                                                .value(height.to_string())
                                                .string("step", "0.5"),
                                            Events::new().on_input({
                                                let character_id = character_id.clone();
                                                move |height| {
                                                    height
                                                        .parse()
                                                        .map(|height| {
                                                            Msg::SetCharacterSize(
                                                                character_id,
                                                                [Some(width), Some(height)],
                                                            )
                                                        })
                                                        .unwrap_or(Msg::NoOp)
                                                }
                                            }),
                                            vec![],
                                        ),
                                        btn::secondary(
                                            Attributes::new(),
                                            Events::new().on_click({
                                                let character_id = character_id.clone();
                                                move |_| {
                                                    Msg::SetCharacterSize(
                                                        character_id,
                                                        [Some(width), None],
                                                    )
                                                }
                                            }),
                                            vec![Html::text("画像に合わせる")],
                                        ),
                                        Html::label(
                                            Attributes::new(),
                                            Events::new(),
                                            vec![Html::text("Z座標")],
                                        ),
                                        Html::input(
                                            Attributes::new()
                                                .type_("number")
                                                .value(character.position()[2].to_string())
                                                .string("step", "0.1"),
                                            Events::new().on_input({
                                                let character_id = character_id.clone();
                                                let mut pos = character.position().clone();
                                                move |height| {
                                                    height
                                                        .parse()
                                                        .map(|z| {
                                                            pos[2] = z;
                                                            Msg::SetCharacterPosition(
                                                                character_id,
                                                                pos,
                                                            )
                                                        })
                                                        .unwrap_or(Msg::NoOp)
                                                }
                                            }),
                                            vec![],
                                        ),
                                        Html::div(Attributes::new(), Events::new(), vec![]),
                                        Html::label(
                                            Attributes::new(),
                                            Events::new(),
                                            vec![Html::text("非公開")],
                                        ),
                                        btn::toggle(
                                            character.is_hidden(),
                                            Attributes::new(),
                                            Events::new().on_click({
                                                let is_hidden = character.is_hidden();
                                                let character_id = character_id.clone();
                                                move |_| {
                                                    Msg::SetCharacterIsHidden(
                                                        character_id,
                                                        !is_hidden,
                                                    )
                                                }
                                            }),
                                        ),
                                    ],
                                ),
                                root_property(block_field, character.property_id()),
                            ],
                        ),
                    ],
                ),
            ],
        )],
    )
}

fn root_property(block_field: &block::Field, prop_id: &BlockId) -> Html {
    if let Some(prop) = block_field.get::<block::Property>(prop_id) {
        match prop.value() {
            block::property::Value::Children(children) => Html::div(
                Attributes::new()
                    .class("container-a")
                    .class("keyvalueoption"),
                Events::new(),
                {
                    let mut prop_children = vec![];
                    for (child_id, prop) in
                        block_field.listed::<block::Property>(children.iter().collect())
                    {
                        prop_children.append(&mut property(block_field, &prop_id, &child_id, prop));
                    }

                    prop_children.append(&mut btn_add_child_to_property(prop_id.clone()));

                    prop_children
                },
            ),
            _ => Html::div(
                Attributes::new()
                    .class("container-a")
                    .class("keyvalueoption"),
                Events::new(),
                vec![],
            ),
        }
    } else {
        Html::div(
            Attributes::new()
                .class("container-a")
                .class("keyvalueoption"),
            Events::new(),
            vec![],
        )
    }
}

fn property(
    block_field: &block::Field,
    parent_id: &BlockId,
    prop_id: &BlockId,
    prop: &block::Property,
) -> Vec<Html> {
    match prop.value() {
        block::property::Value::None => vec![
            Html::div(
                Attributes::new()
                    .class("keyvalueoption-banner-2")
                    .class("linear-h")
                    .class("linear-h-eq"),
                Events::new(),
                vec![
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click({
                            let prop_id = prop_id.clone();
                            move |_| {
                                Msg::SetPropertyValue(
                                    prop_id,
                                    block::property::Value::Children(vec![]),
                                )
                            }
                        }),
                        vec![Html::text(" グループ")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click({
                            let prop_id = prop_id.clone();
                            move |_| {
                                Msg::SetPropertyValue(prop_id, block::property::Value::Num(0.0))
                            }
                        }),
                        vec![Html::text("数値")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click({
                            let prop_id = prop_id.clone();
                            move |_| {
                                Msg::SetPropertyValue(
                                    prop_id,
                                    block::property::Value::Str(String::new()),
                                )
                            }
                        }),
                        vec![Html::text("テキスト")],
                    ),
                ],
            ),
            btn_remove_property(parent_id.clone(), prop_id.clone()),
        ],
        block::property::Value::Num(n) => vec![
            property_key(false, prop_id, prop),
            Html::input(
                Attributes::new().value(n.to_string()).type_("number"),
                Events::new().on_input({
                    let prop_id = prop_id.clone();
                    move |s| {
                        s.parse()
                            .map(|n| Msg::SetPropertyValue(prop_id, block::property::Value::Num(n)))
                            .unwrap_or(Msg::NoOp)
                    }
                }),
                vec![],
            ),
            btn_remove_property(parent_id.clone(), prop_id.clone()),
        ],
        block::property::Value::Str(s) => vec![
            property_key(false, prop_id, prop),
            Html::input(
                Attributes::new().value(s),
                Events::new().on_input({
                    let prop_id = prop_id.clone();
                    move |s| Msg::SetPropertyValue(prop_id, block::property::Value::Str(s))
                }),
                vec![],
            ),
            btn_remove_property(parent_id.clone(), prop_id.clone()),
        ],
        block::property::Value::Children(children) => vec![
            property_key(true, prop_id, prop),
            btn_remove_property(parent_id.clone(), prop_id.clone()),
            Html::div(
                Attributes::new()
                    .class("container-indent")
                    .class("keyvalueoption")
                    .class("keyvalueoption-banner"),
                Events::new(),
                {
                    let mut prop_children = vec![];

                    if prop.is_collapsed() {
                        for (child_id, prop) in
                            block_field.listed::<block::Property>(children.iter().collect())
                        {
                            prop_children.append(&mut property(
                                block_field,
                                &prop_id,
                                &child_id,
                                prop,
                            ));
                        }
                    }

                    prop_children.append(&mut btn_add_child_to_property(prop_id.clone()));

                    prop_children
                },
            ),
        ],
    }
}

fn property_key(is_banner: bool, property_id: &BlockId, property: &block::Property) -> Html {
    let attributes = Attributes::new().class("centering-v-i");
    let attributes = if is_banner {
        attributes
            .class("keyvalueoption-banner-2")
            .class("keyvalueoption-banner-2-collapse")
            .class("keyvalue")
    } else {
        attributes.class("keyvalue")
    };
    let is_selected = property.is_selected();
    Html::div(attributes, Events::new(), {
        let mut children = vec![];
        children.push(btn::check(
            is_selected,
            Attributes::new(),
            Events::new().on_click({
                let property_id = property_id.clone();
                move |_| Msg::SetPropertyIsSelected(property_id, !is_selected)
            }),
        ));
        if is_banner {
            let is_collapsed = property.is_collapsed();
            let icon = if is_collapsed {
                "fa-angle-down"
            } else {
                "fa-angle-right"
            };
            children.push(btn::transparent(
                Attributes::new(),
                Events::new().on_click({
                    let property_id = property_id.clone();
                    move |_| Msg::SetPropertyIsCollapsed(property_id, !is_collapsed)
                }),
                vec![awesome::i(icon)],
            ));
        }
        children.push(Html::input(
            Attributes::new()
                .value(property.name())
                .type_("text")
                .class("key"),
            Events::new().on_input({
                let property_id = property_id.clone();
                move |s| Msg::SetPropertyName(property_id, s)
            }),
            vec![],
        ));
        children
    })
}

fn btn_remove_property(parent_id: BlockId, child_id: BlockId) -> Html {
    btn::danger(
        Attributes::new(),
        Events::new().on_click(move |_| Msg::RemoveProperty(parent_id, child_id)),
        vec![awesome::i("fa-times")],
    )
}

fn btn_add_child_to_property(property_id: BlockId) -> Vec<Html> {
    vec![
        btn::secondary(
            Attributes::new().class("keyvalueoption-banner-2"),
            Events::new().on_click(move |_| Msg::AddChildToProprty(property_id)),
            vec![awesome::i("fa-plus")],
        ),
        Html::span(Attributes::new(), Events::new(), vec![]),
    ]
}
