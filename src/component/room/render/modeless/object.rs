use super::super::super::super::{awesome, btn, modeless};
use super::super::{
    common,
    state::{self, chat, table, Modal, Modeless},
};
use super::{Msg, State};
use crate::{
    block::{self, chat::item::Icon, BlockId},
    model::{self, PersonalData},
    resource::Data,
    Color, Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    modeless_id: model::modeless::ModelessId,
    modless: &model::Modeless<Modeless>,
    grubbed: Option<model::modeless::ModelessId>,
    tabs: &Vec<BlockId>,
    focused: usize,
    outlined: Option<&Color>,
) -> Html<Msg> {
    let attributes = if let Some(color) = outlined {
        Attributes::new().style(
            "box-shadow",
            format!("0 0 0.2rem 0.2rem {}", color.to_string()),
        )
    } else {
        Attributes::new()
    };
    let focused_id = &tabs[focused];
    super::frame(
        modeless_id,
        modless,
        attributes,
        Events::new(),
        vec![
            super::header(
                modeless_id,
                grubbed,
                Html::div(
                    Attributes::new(),
                    Events::new(),
                    block_field
                        .listed::<block::Character>(tabs.iter().collect())
                        .map(|(_, character)| Html::text(character.name()))
                        .collect(),
                ),
            ),
            if let Some(character) = block_field.get::<block::Character>(focused_id) {
                character_frame(block_field, resource, character, focused_id)
            } else {
                Html::none()
            },
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn character_frame(
    block_field: &block::Field,
    resource: &Resource,
    character: &block::Character,
    character_id: &BlockId,
) -> Html<Msg> {
    let [width, _, height] = character.size();
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
                                                .value(height.to_string()),
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
                                    ],
                                ),
                                character_root_property(
                                    block_field,
                                    character_id,
                                    character.property_id(),
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        )],
    )
}

fn character_root_property(
    block_field: &block::Field,
    character_id: &BlockId,
    property_id: &BlockId,
) -> Html<Msg> {
    if let Some(property) = block_field.get::<block::Property>(property_id) {
        match property.value() {
            block::property::Value::Children(children) => Html::div(
                Attributes::new()
                    .class("container-a")
                    .class("keyvalueoption"),
                Events::new(),
                vec![
                    block_field
                        .listed::<block::Property>(children.iter().collect())
                        .map(|(property_id, property)| {
                            character_property(block_field, &property_id, property)
                        })
                        .flatten()
                        .collect(),
                    btn_add_child_to_property(property_id.clone()),
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

fn character_property(
    block_field: &block::Field,
    property_id: &BlockId,
    property: &block::Property,
) -> Vec<Html<Msg>> {
    match property.value() {
        block::property::Value::None => vec![
            Html::div(
                Attributes::new()
                    .class("keyvalueoption-banner-2")
                    .class("linear-h")
                    .style("grid-auto-columns", "1fr"),
                Events::new(),
                vec![
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(move |_| Msg::NoOp),
                        vec![Html::text(" グループ")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(move |_| Msg::NoOp),
                        vec![Html::text("数値")],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(move |_| Msg::NoOp),
                        vec![Html::text("テキスト")],
                    ),
                ],
            ),
            btn_remove_property(property_id.clone()),
        ],
        block::property::Value::Num(n) => vec![
            character_property_key(false, property_id, property),
            Html::input(
                Attributes::new().value(n.to_string()).type_("number"),
                Events::new()
                    .on_input(move |s| s.parse().map(|_: f64| Msg::NoOp).unwrap_or(Msg::NoOp)),
                vec![],
            ),
            btn_remove_property(property_id.clone()),
        ],
        block::property::Value::Str(s) => vec![
            character_property_key(false, property_id, property),
            Html::input(
                Attributes::new().value(s),
                Events::new().on_input(move |s| Msg::NoOp),
                vec![],
            ),
            btn_remove_property(property_id.clone()),
        ],
        block::property::Value::Children(children) => vec![
            character_property_key(true, property_id, property),
            btn_remove_property(property_id.clone()),
            Html::div(
                Attributes::new()
                    .class("container-indent")
                    .class("keyvalueoption")
                    .class("keyvalueoption-banner"),
                Events::new(),
                vec![
                    block_field
                        .listed(children.iter().collect())
                        .map(|(property_id, property)| {
                            character_property(block_field, &property_id, property)
                        })
                        .flatten()
                        .collect(),
                    btn_add_child_to_property(property_id.clone()),
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
    property_id: &BlockId,
    property: &block::Property,
) -> Html<Msg> {
    let attributes = Attributes::new().class("centering-v-i");
    let attributes = if is_banner {
        attributes
            .class("keyvalueoption-banner-2")
            .class("keyvalue")
    } else {
        attributes.class("linear-h")
    };
    let is_selected = property.is_selected();
    Html::div(
        attributes,
        Events::new(),
        vec![
            btn::check(
                is_selected,
                Attributes::new(),
                Events::new().on_click(move |_| Msg::NoOp),
            ),
            Html::input(
                Attributes::new().value(property.name()).type_("text"),
                Events::new().on_input(move |s| Msg::NoOp),
                vec![],
            ),
        ],
    )
}

fn btn_remove_property(property_id: BlockId) -> Html<Msg> {
    btn::danger(
        Attributes::new(),
        Events::new().on_click(move |_| Msg::NoOp),
        vec![awesome::i("fa-times")],
    )
}

fn btn_add_child_to_property(property_id: BlockId) -> Vec<Html<Msg>> {
    vec![
        btn::secondary(
            Attributes::new().class("keyvalueoption-banner-2"),
            Events::new().on_click(move |_| Msg::NoOp),
            vec![awesome::i("fa-plus")],
        ),
        Html::span(Attributes::new(), Events::new(), vec![]),
    ]
}
