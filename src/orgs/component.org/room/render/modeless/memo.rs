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

pub fn render<'a>(
    modeless_id: model::modeless::ModelessId,
    block_field: &block::Field,
    resource: &Resource,
    modeless: &model::Modeless<Modeless>,
    grabbed: Option<model::modeless::ModelessId>,
    world: &block::World,
    selecting_tab_idx: usize,
) -> Html {
    let is_grabbed = grabbed.is_some();
    let tags = world.tags().collect::<Vec<_>>();
    let selecting_tag_id = if selecting_tab_idx == 0 {
        None
    } else {
        tags.get(selecting_tab_idx - 1).map(|x| x as &BlockId)
    };

    super::frame(
        modeless_id,
        modeless,
        Attributes::new(),
        Events::new(),
        vec![
            super::header(
                modeless_id,
                grabbed,
                Attributes::new().class("frame-header-tab"),
                Events::new(),
                memo_tag_list(modeless_id, block_field, &tags, selecting_tag_id),
            ),
            modeless::body(
                Attributes::new()
                    .class("linear-v")
                    .class("linear-v-stretch")
                    .class("scroll-v"),
                Events::new().on_mousemove(move |e| {
                    if !is_grabbed {
                        e.stop_propagation();
                    }
                    Msg::NoOp
                }),
                vec![
                    world
                        .memos()
                        .filter_map(|memo_id| {
                            block_field.get::<block::Memo>(memo_id).and_then(|item| {
                                if selecting_tag_id
                                    .map(|tag_id| item.has(tag_id))
                                    .unwrap_or(true)
                                {
                                    Some(memo_item(block_field, memo_id, item, &tags))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                    vec![btn::secondary(
                        Attributes::new(),
                        Events::new().on_click({
                            let tag_id = selecting_tag_id.map(|x| x.clone());
                            move |_| Msg::AddMemo(tag_id)
                        }),
                        vec![awesome::i("fa-plus")],
                    )],
                ]
                .into_iter()
                .flatten()
                .collect(),
            ),
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn memo_item(
    block_field: &block::Field,
    memo_id: &BlockId,
    memo: &block::Memo,
    tags: &Vec<&BlockId>,
) -> Html {
    Html::div(
        Attributes::new()
            .class("pure-form")
            .class("container-a")
            .class("keyvalue")
            .class("keyvalue-rev"),
        Events::new(),
        vec![
            Html::input(
                Attributes::new().value(memo.name()),
                Events::new().on_input({
                    let memo_id = memo_id.clone();
                    move |name| Msg::SetMemoName(memo_id, name)
                }),
                vec![],
            ),
            btn::danger(
                Attributes::new(),
                Events::new().on_click({
                    let memo_id = memo_id.clone();
                    move |_| Msg::RemoveMemo(memo_id)
                }),
                vec![awesome::i("fa-times")],
            ),
            Html::div(
                Attributes::new()
                    .class("keyvalue-banner")
                    .class("keyvalueoption"),
                Events::new(),
                vec![
                    text::div("タグ"),
                    Html::div(
                        Attributes::new().class("flex-h"),
                        Events::new(),
                        memo.tags(tags.iter().map(|x| x as &BlockId))
                            .filter_map(|tag_id| tag_name(block_field, &tag_id))
                            .map(|tag_name| {
                                Html::div(
                                    Attributes::new().class("tag"),
                                    Events::new(),
                                    vec![Html::text(tag_name)],
                                )
                            })
                            .collect(),
                    ),
                    btn::secondary(Attributes::new(), Events::new(), vec![Html::text("編集")]),
                ],
            ),
            Html::textarea(
                Attributes::new()
                    .value(memo.text())
                    .class("keyvalue-banner")
                    .style("resize", "none")
                    .nut("rows", 10),
                Events::new().on_input({
                    let memo_id = memo_id.clone();
                    move |text| Msg::SetMemoText(memo_id, text)
                }),
                vec![],
            ),
        ],
    )
}

fn memo_tag_list<'a>(
    modeless_id: model::modeless::ModelessId,
    block_field: &block::Field,
    tags: &Vec<&BlockId>,
    selecting_tag_id: Option<&BlockId>,
) -> Html {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            vec![btn::frame_tab(
                selecting_tag_id.is_none(),
                false,
                Events::new().on_click(move |_| Msg::SetModelessTabIdx(modeless_id, 0)),
                "[全ての共有メモ]",
            )],
            tags.iter()
                .enumerate()
                .filter_map(|(idx, tag_id)| {
                    tag_name(block_field, tag_id)
                        .map(|tag_name| (idx, tag_id as &BlockId, tag_name))
                })
                .map(|(idx, tag_id, tag_name)| {
                    btn::frame_tab(
                        selecting_tag_id
                            .map(|t_id| *tag_id == *t_id)
                            .unwrap_or(false),
                        false,
                        Events::new()
                            .on_click(move |_| Msg::SetModelessTabIdx(modeless_id, idx + 1)),
                        tag_name,
                    )
                })
                .collect(),
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn tag_name<'a>(block_field: &'a block::Field, tag_id: &BlockId) -> Option<&'a String> {
    block_field.get::<block::Tag>(tag_id).map(|tag| tag.name())
}
