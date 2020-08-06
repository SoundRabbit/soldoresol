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
    world: &block::World,
    selecting_tag_id: Option<&BlockId>,
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
                memo_tag_list(block_field, world.tags(), selecting_tag_id),
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
                        .class("linear-v-stretch")
                        .style("align-self", "stretch")
                        .style("grid-template-rows", "max-content max-content 1fr"),
                    Events::new(),
                    vec![
                        world
                            .memos()
                            .filter_map(|memo_id| {
                                block_field.get::<block::Memo>(memo_id).and_then(|item| {
                                    if selecting_tag_id
                                        .map(|tag_id| item.has(tag_id))
                                        .unwrap_or(true)
                                    {
                                        Some(memo_item(memo_id, item))
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect(),
                        vec![btn::secondary(
                            Attributes::new(),
                            Events::new(),
                            vec![awesome::i("fa-plus")],
                        )],
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                )],
            ),
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}

fn memo_item(memo_id: &BlockId, memo: &block::Memo) -> Html {
    Html::div(
        Attributes::new().class("pure-form").class("container-a"),
        Events::new(),
        vec![
            Html::input(Attributes::new().value(memo.name()), Events::new(), vec![]),
            Html::textarea(Attributes::new().value(memo.text()), Events::new(), vec![]),
        ],
    )
}

fn memo_tag_list<'a>(
    block_field: &block::Field,
    tags: impl Iterator<Item = &'a BlockId>,
    selecting_tag_id: Option<&BlockId>,
) -> Html {
    Html::div(
        Attributes::new(),
        Events::new(),
        tags.filter_map(|tag_id| {
            block_field
                .get::<block::Tag>(tag_id)
                .map(|tag| (tag_id, tag))
        })
        .map(|(tag_id, tag)| {
            btn::frame_tab(
                selecting_tag_id
                    .map(|t_id| *tag_id == *t_id)
                    .unwrap_or(false),
                false,
                Events::new(),
                tag.name(),
            )
        })
        .collect(),
    )
}
