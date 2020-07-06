use super::super::super::{awesome, btn};
use super::{state::table, Modal, Msg};
use kagura::prelude::*;

pub fn render(
    z_index: u64,
    room_id: &String,
    selecting_tool: &table::Tool,
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
                    vec![
                        btn::primary(
                            Attributes::new().title("プレイヤー名やアイコンなどの管理"),
                            Events::new().on_click(|_| Msg::OpenModal(Modal::PersonalSetting)),
                            vec![awesome::i("fa-user-cog"), Html::text(" 個人設定")],
                        ),
                        btn::danger(
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::DisconnectFromRoom),
                            vec![Html::text("ルームから出る")],
                        ),
                    ],
                )],
            ),
            Html::div(
                Attributes::new().class("grid-w-12"),
                Events::new(),
                vec![btn::primary(
                    Attributes::new(),
                    Events::new(),
                    vec![awesome::i("fa-bars"), Html::text(" ルーム設定")],
                )],
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
