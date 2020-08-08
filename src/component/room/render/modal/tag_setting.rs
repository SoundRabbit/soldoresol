use super::super::super::super::{awesome, btn, modal};
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

pub fn render<'a>(block_field: &block::Field, resource: &Resource, world: &block::World) -> Html {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            8,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("タグ設定"),
                modal::body(
                    Attributes::new()
                        .class("linear-v")
                        .class("linear-v-stretch")
                        .class("scroll-v"),
                    Events::new(),
                    vec![
                        block_field
                            .listed::<block::Tag>(world.tags().collect())
                            .map(|(tag_id, tag)| {
                                Html::div(
                                    Attributes::new()
                                        .class("container-a")
                                        .class("pure-form")
                                        .class("keyvalue")
                                        .class("keyvalue-rev"),
                                    Events::new(),
                                    vec![
                                        Html::input(
                                            Attributes::new().value(tag.name()),
                                            Events::new().on_input({
                                                let tag_id = tag_id.clone();
                                                move |name| Msg::SetTagName(tag_id, name)
                                            }),
                                            vec![],
                                        ),
                                        btn::danger(
                                            Attributes::new(),
                                            Events::new().on_click({
                                                let tag_id = tag_id.clone();
                                                move |_| Msg::RemoveTag(tag_id)
                                            }),
                                            vec![awesome::i("fa-times")],
                                        ),
                                    ],
                                )
                            })
                            .collect(),
                        vec![btn::secondary(
                            Attributes::new(),
                            Events::new().on_click(move |_| Msg::AddTag),
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
