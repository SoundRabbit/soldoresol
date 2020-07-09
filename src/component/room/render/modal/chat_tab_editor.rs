use super::super::super::super::{awesome, btn, modal};
use super::Msg;
use crate::block;
use kagura::prelude::*;

mod common {
    pub use super::super::super::common::*;
    pub use super::super::common::*;
}

pub fn render(block_field: &block::Field, chat: &block::Chat) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            6,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("チャットタブを編集"),
                modal::body(
                    Attributes::new()
                        .class("scroll-v")
                        .class("pure-form")
                        .class("keyvalue")
                        .class("keyvalue-rev"),
                    Events::new(),
                    vec![
                        block_field
                            .listed::<block::chat::Tab>(chat.iter().collect())
                            .map(|(tab_id, tab)| {
                                vec![
                                    Html::input(
                                        Attributes::new().value(tab.name()),
                                        Events::new().on_input({
                                            let tab_id = tab_id.clone();
                                            move |name| Msg::SetChatTabName(tab_id, name)
                                        }),
                                        vec![],
                                    ),
                                    btn::danger(
                                        Attributes::new(),
                                        Events::new().on_click({
                                            let tab_id = tab_id.clone();
                                            move |_| Msg::RemoveChatTab(tab_id)
                                        }),
                                        vec![awesome::i("fa-times")],
                                    ),
                                ]
                            })
                            .flatten()
                            .collect(),
                        vec![btn::secondary(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::AddChatTab),
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
