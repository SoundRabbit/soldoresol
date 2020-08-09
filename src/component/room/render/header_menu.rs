use super::super::super::{awesome, btn, dropdown};
use super::super::{
    state::{headermenu, Modal, Modeless},
    Msg,
};
use kagura::prelude::*;

pub fn render(
    z_index: u64,
    room_id: &String,
    headermenu_state: &headermenu::State,
    is_2d_mode: bool,
) -> Html {
    Html::div(
        Attributes::new()
            .style("z-index", z_index.to_string())
            .style("grid-column", "span 2")
            .class("panel grid"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("grid-w-12 keyvalue pure-form"),
                Events::new(),
                vec![
                    Html::label(
                        Attributes::new().string("for", "roomid"),
                        Events::new(),
                        vec![Html::text("ルームID")],
                    ),
                    Html::input(
                        Attributes::new()
                            .value(room_id)
                            .id("roomid")
                            .flag("readonly"),
                        Events::new(),
                        vec![],
                    ),
                ],
            ),
            Html::div(
                Attributes::new()
                    .class("grid-w-12")
                    .class("justify-r")
                    .class("centering-h"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![btn::danger(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::DisconnectFromRoom),
                        vec![Html::text("ルームから出る")],
                    )],
                )],
            ),
            Html::div(
                Attributes::new().class("grid-w-12").class("linear-h"),
                Events::new(),
                vec![
                    dropdown::bottom_right(
                        headermenu_state.is_config(),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click({
                                let is_showing = headermenu_state.is_config();
                                move |_| {
                                    if is_showing {
                                        Msg::SetHeadermenuState(headermenu::State::None)
                                    } else {
                                        Msg::SetHeadermenuState(headermenu::State::Config)
                                    }
                                }
                            }),
                            vec![
                                awesome::i("fa-cogs"),
                                Html::text(" 設定 "),
                                if headermenu_state.is_config() {
                                    awesome::i("fa-angle-up")
                                } else {
                                    awesome::i("fa-angle-down")
                                },
                            ],
                        ),
                        Html::div(
                            Attributes::new()
                                .class("panel")
                                .class("panel-dark")
                                .class("pure-form")
                                .class("linear-v"),
                            Events::new()
                                .on_click(|_| Msg::SetHeadermenuState(headermenu::State::None)),
                            vec![
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new()
                                        .on_click(|_| Msg::OpenModal(Modal::PersonalSetting)),
                                    vec![
                                        awesome::i("fa-user-cog"),
                                        Html::text(" プレイヤー情報 "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new().on_click(|_| Msg::OpenModal(Modal::TagSetting)),
                                    vec![
                                        awesome::i("fa-tags"),
                                        Html::text(" タグ "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                                // btn::headermenu(
                                //     Attributes::new().class("keyvalueoption"),
                                //     Events::new(),
                                //     vec![
                                //         awesome::i("fa-object-group"),
                                //         Html::text(" シーン設定 "),
                                //         awesome::i("fa-external-link-alt"),
                                //     ],
                                // ),
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new().on_click(|_| Msg::OpenModal(Modal::TableSetting)),
                                    vec![
                                        awesome::i("fa-vector-square"),
                                        Html::text(" テーブル "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                            ],
                        ),
                    ),
                    dropdown::bottom_right(
                        headermenu_state.is_resource(),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click({
                                let is_showing = headermenu_state.is_resource();
                                move |_| {
                                    if is_showing {
                                        Msg::SetHeadermenuState(headermenu::State::None)
                                    } else {
                                        Msg::SetHeadermenuState(headermenu::State::Resource)
                                    }
                                }
                            }),
                            vec![
                                awesome::i("fa-folder"),
                                Html::text(" リソース "),
                                if headermenu_state.is_resource() {
                                    awesome::i("fa-angle-up")
                                } else {
                                    awesome::i("fa-angle-down")
                                },
                            ],
                        ),
                        Html::div(
                            Attributes::new()
                                .class("panel")
                                .class("panel-dark")
                                .class("pure-form")
                                .class("linear-v"),
                            Events::new()
                                .on_click(|_| Msg::SetHeadermenuState(headermenu::State::None)),
                            vec![btn::headermenu(
                                Attributes::new().class("keyvalueoption"),
                                Events::new().on_click(|_| Msg::OpenModal(Modal::Resource)),
                                vec![
                                    awesome::i("fa-images"),
                                    Html::text(" 画像データ "),
                                    awesome::i("fa-external-link-alt"),
                                ],
                            )],
                        ),
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::OpenModeless(Modeless::Chat)),
                        vec![
                            awesome::i("fa-comments"),
                            Html::text(" チャット "),
                            awesome::i("fa-external-link-alt"),
                        ],
                    ),
                    btn::primary(
                        Attributes::new(),
                        Events::new()
                            .on_click(|_| Msg::OpenModeless(Modeless::Memo { focused: 0 })),
                        vec![
                            awesome::i("fa-file-alt"),
                            Html::text(" 共有メモ "),
                            awesome::i("fa-external-link-alt"),
                        ],
                    ),
                ],
            ),
            Html::div(
                Attributes::new()
                    .class("grid-w-12")
                    .class("justify-r")
                    .class("centering-h"),
                Events::new(),
                vec![Html::div(
                    Attributes::new().class("linear-h"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("keyvalue"),
                        Events::new(),
                        vec![
                            Html::span(
                                Attributes::new().class("text-label"),
                                Events::new(),
                                vec![Html::text("2Dモード")],
                            ),
                            btn::toggle(
                                is_2d_mode,
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::SetTableIs2dMode(!is_2d_mode)),
                            ),
                        ],
                    )],
                )],
            ),
        ],
    )
}
