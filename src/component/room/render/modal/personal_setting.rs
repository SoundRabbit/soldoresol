use super::super::super::super::{btn, modal};
use super::state::Modal;
use super::Msg;
use crate::{block::chat::item::Icon, model::PersonalData, Resource};
use kagura::prelude::*;

mod common {
    pub use super::super::super::common::*;
    pub use super::super::common::*;
}

pub fn render(resource: &Resource, personal_data: &PersonalData) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                common::header("個人設定"),
                modal::body(
                    Attributes::new().class("scroll-v pure-form"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("chat-item"),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new()
                                    .class("chat-icon linear-v")
                                    .style("justify-items", "center"),
                                Events::new(),
                                vec![
                                    {
                                        let icon = personal_data
                                            .icon()
                                            .map(|r_id| Icon::Resource(*r_id))
                                            .unwrap_or(Icon::DefaultUser);
                                        common::chat_icon(
                                            Attributes::new().class("icon-large"),
                                            &icon,
                                            personal_data.name(),
                                            resource,
                                        )
                                    },
                                    btn::primary(
                                        Attributes::new(),
                                        Events::new()
                                            .on_click(|_| Msg::OpenModal(Modal::SelectPlayerImage)),
                                        vec![Html::text("画像を選択")],
                                    ),
                                ],
                            ),
                            Html::div(
                                Attributes::new().class("chat-args keyvalue"),
                                Events::new(),
                                vec![
                                    Html::label(
                                        Attributes::new().string("for", "player-name"),
                                        Events::new(),
                                        vec![Html::text("プレイヤー名")],
                                    ),
                                    Html::input(
                                        Attributes::new()
                                            .id("player-name")
                                            .value(personal_data.name()),
                                        Events::new()
                                            .on_input(|n| Msg::SetPersonalDataWithPlayerName(n)),
                                        vec![],
                                    ),
                                ],
                            ),
                        ],
                    )],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
