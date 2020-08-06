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
    block_field: &block::Field,
    resource: &Resource,
    modeless_id: model::modeless::ModelessId,
    modeless: &model::Modeless<Modeless>,
    grubbed: Option<model::modeless::ModelessId>,
    tags: impl Iterator<Item = &'a BlockId>,
    memo: &block::Memo,
    selecting_tag_id: &BlockId,
) -> Html {
    let is_grubbed = grubbed.is_some();

    super::frame(
        modeless_id,
        modeless,
        Attributes::new(),
        Events::new(),
        vec![
            super::header(
                modeless_id,
                grubbed,
                Attributes::new().class("frame-header-tab"),
                Events::new(),
                memo_tag_list(block_field, tags, selecting_tag_id),
            ),
            modeless::body(
                Attributes::new()
                    .class("linear-v")
                    .style("grid-template-rows", "1fr"),
                Events::new().on_mousemove(move |e| {
                    if !is_grubbed {
                        e.stop_propagation();
                    }
                    Msg::NoOp
                }),
                vec![Html::div(
                    Attributes::new()
                        .class("linear-v")
                        .style("align-self", "stretch")
                        .style("grid-template-rows", "max-content max-content 1fr"),
                    Events::new(),
                    memo.items()
                        .filter_map(|item_id| {
                            block_field
                                .get::<block::memo::Item>(item_id)
                                .and_then(|item| {
                                    if item.has(selecting_tag_id) {
                                        Some(memo_item(item_id, item))
                                    } else {
                                        None
                                    }
                                })
                        })
                        .collect(),
                )],
            ),
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn memo_item(item_id: &BlockId, item: &block::memo::Item) -> Html {
    Html::div(
        Attributes::new().class("pure-form container-a"),
        Events::new(),
        vec![
            Html::input(Attributes::new().value(item.name()), Events::new(), vec![]),
            Html::textarea(Attributes::new().value(item.text()), Events::new(), vec![]),
        ],
    )
}

fn memo_tag_list<'a>(
    block_field: &block::Field,
    tags: impl Iterator<Item = &'a BlockId>,
    selecting_tag_id: &BlockId,
) -> Html {
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            tags.filter_map(|tag_id| {
                block_field
                    .get::<block::Tag>(tag_id)
                    .map(|tag| (tag_id, tag))
            })
            .map(|(tag_id, tag)| {
                btn::frame_tab(
                    *tag_id == *selecting_tag_id,
                    false,
                    Events::new(),
                    tag.name(),
                )
            })
            .collect(),
            vec![btn::transparent(
                Attributes::new(),
                Events::new().on_click(|_| Msg::AddChatTab),
                vec![awesome::i("fa-plus")],
            )],
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}
