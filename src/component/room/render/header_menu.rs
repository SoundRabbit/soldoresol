use super::super::super::{awesome, btn, dropdown};
use super::super::{
    state::{headermenu, table, Modal, Modeless},
    Msg,
};
use kagura::prelude::*;

pub fn render(
    z_index: u64,
    room_id: &String,
    headermenu_state: Option<&headermenu::State>,
    is_2d_mode: bool,
) -> Html<Msg> {
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
                        headermenu_state.is_some(),
                        btn::primary(
                            Attributes::new(),
                            Events::new().on_click({
                                let is_showing = headermenu_state.is_some();
                                move |_| {
                                    if is_showing {
                                        Msg::SetHeadermenuState(None)
                                    } else {
                                        Msg::SetHeadermenuState(Some(headermenu::State::Config))
                                    }
                                }
                            }),
                            vec![
                                awesome::i("fa-cogs"),
                                Html::text(" 設定 "),
                                if headermenu_state.is_some() {
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
                            Events::new(),
                            vec![
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new(),
                                    vec![
                                        awesome::i("fa-user-cog"),
                                        Html::text(" 個人設定 "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new(),
                                    vec![
                                        awesome::i("fa-layer-group"),
                                        Html::text(" ルーム設定 "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new(),
                                    vec![
                                        awesome::i("fa-object-group"),
                                        Html::text(" シーン設定 "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                                btn::headermenu(
                                    Attributes::new().class("keyvalueoption"),
                                    Events::new(),
                                    vec![
                                        awesome::i("fa-vector-square"),
                                        Html::text(" テーブル設定 "),
                                        awesome::i("fa-external-link-alt"),
                                    ],
                                ),
                            ],
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
                                Events::new().on_click(move |_| Msg::NoOp),
                            ),
                        ],
                    )],
                )],
            ),
        ],
    )
}
