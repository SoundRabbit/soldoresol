use super::super::super::super::modal;
use super::Msg;
use crate::{block, Resource};
use kagura::prelude::*;

mod common {
    pub use super::super::super::common::*;
    pub use super::super::common::*;
}

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    tab: &block::chat::Tab,
) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("チャット履歴"),
                modal::body(
                    Attributes::new().class("scroll-v"),
                    Events::new(),
                    tab.iter()
                        .filter_map(|(t, item_id)| {
                            block_field
                                .get::<block::chat::Item>(item_id)
                                .map(|item| (t, item))
                        })
                        .map(|(_, item)| {
                            Html::div(
                                Attributes::new().class("pure-form chat-item"),
                                Events::new(),
                                vec![
                                    common::chat_icon(
                                        Attributes::new().class("icon-medium").class("chat-icon"),
                                        item.icon(),
                                        item.display_name(),
                                        resource,
                                    ),
                                    Html::div(
                                        Attributes::new().class("chat-args"),
                                        Events::new(),
                                        vec![Html::text(
                                            String::from("")
                                                + item.display_name()
                                                + "@"
                                                + item.peer_id(),
                                        )],
                                    ),
                                    Html::div(
                                        Attributes::new().class("chat-payload"),
                                        Events::new(),
                                        vec![
                                            Html::div(
                                                Attributes::new().class("text-wrap"),
                                                Events::new(),
                                                vec![Html::text(item.text())],
                                            ),
                                            if let Some(reply) = item.reply() {
                                                Html::div(
                                                    Attributes::new().class("text-wrap"),
                                                    Events::new(),
                                                    vec![Html::text(reply)],
                                                )
                                            } else {
                                                Html::none()
                                            },
                                        ],
                                    ),
                                ],
                            )
                        })
                        .collect(),
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
