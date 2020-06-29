use super::super::super::super::{btn, modal};
use super::state::Modal;
use super::Msg;
use crate::{
    block::{self, BlockId},
    resource::Data,
    Resource,
};
use kagura::prelude::*;

mod common {
    pub use super::super::common::*;
}

pub fn render<'a>(
    block_field: &block::Field,
    resource: &Resource,
    world: &block::World,
) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("テーブル設定"),
                modal::body(
                    Attributes::new()
                        .class("keyvalue")
                        .class("keyvalue-align-stretch")
                        .class("editormodeless"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class("linear-v").class("scroll-v"),
                            Events::new(),
                            vec![
                                block_field
                                    .listed::<block::Table>(world.tables().collect())
                                    .map(|(table_id, table)| {
                                        let table_id = table_id.clone();
                                        btn::selectable(
                                            table_id == *world.selecting_table(),
                                            Attributes::new(),
                                            Events::new().on_click(move |_| Msg::NoOp),
                                            vec![Html::text(table.name())],
                                        )
                                    })
                                    .collect(),
                                vec![btn::primary(
                                    Attributes::new(),
                                    Events::new().on_click(|_| Msg::NoOp),
                                    vec![Html::text("追加")],
                                )],
                            ]
                            .into_iter()
                            .flatten()
                            .collect(),
                        ),
                        if let Some(table) =
                            block_field.get::<block::Table>(world.selecting_table())
                        {
                            selected_table(resource, world.selecting_table(), table)
                        } else {
                            Html::none()
                        },
                    ],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}

fn selected_table(resource: &Resource, block_id: &BlockId, table: &block::Table) -> Html<Msg> {
    let [width, height] = table.size().clone();
    Html::div(
        Attributes::new()
            .class("scroll-v")
            .class("pure-form")
            .class("linear-v")
            .class("keyvalue-align-start"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("keyvalue"),
                Events::new(),
                vec![
                    Html::span(Attributes::new(), Events::new(), vec![Html::text("Name")]),
                    Html::input(
                        Attributes::new().value(table.name()).type_("text"),
                        Events::new().on_input(move |s| Msg::NoOp),
                        vec![],
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
                            table
                                .image_texture_id()
                                .and_then(|data_id| {
                                    if let Some(Data::Image { url, .. }) = resource.get(&data_id) {
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
                                    let block_id = block_id.clone();
                                    move |_| Msg::OpenModal(Modal::SelectTableImage(block_id))
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
                                Attributes::new().class("container-a").class("keyvalue"),
                                Events::new(),
                                vec![
                                    Html::span(
                                        Attributes::new(),
                                        Events::new(),
                                        vec![Html::text("幅")],
                                    ),
                                    Html::input(
                                        Attributes::new().type_("number").value(width.to_string()),
                                        Events::new().on_input({
                                            let block_id = block_id.clone();
                                            move |width| {
                                                if let Ok(width) = width.parse::<f32>() {
                                                    Msg::SetTableSize(
                                                        block_id,
                                                        [width.floor(), height],
                                                    )
                                                } else {
                                                    Msg::NoOp
                                                }
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
                                        Attributes::new().type_("number").value(height.to_string()),
                                        Events::new().on_input({
                                            let block_id = block_id.clone();
                                            move |height| {
                                                if let Ok(height) = height.parse::<f32>() {
                                                    Msg::SetTableSize(
                                                        block_id,
                                                        [width, height.floor()],
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
                                Attributes::new().class("keyvalue").title(""),
                                Events::new(),
                                vec![
                                    Html::span(
                                        Attributes::new().class("text-label"),
                                        Events::new(),
                                        vec![Html::text("グリッドにスナップ")],
                                    ),
                                    btn::toggle(
                                        table.is_bind_to_grid(),
                                        Attributes::new(),
                                        Events::new().on_click(move |_| Msg::NoOp),
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    )
}