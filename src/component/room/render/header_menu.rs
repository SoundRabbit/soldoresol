use super::super::super::{awesome, btn};
use super::{state::table, Modal, Msg};
use kagura::prelude::*;

pub fn render(room_id: &String, selecting_tool: &table::Tool, is_2d_mode: bool) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .style("grid-column", "span 2")
            .class("panel grid"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("grid-w-6 keyvalue pure-form"),
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
                    .class("grid-w-18")
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
                Attributes::new()
                    .class("grid-w-12")
                    .class("linear-h")
                    .class("centering-v-i")
                    .class("pure-form"),
                Events::new(),
                vec![vec![
                    btn::selectable(
                        selecting_tool.is_selector(),
                        Attributes::new(),
                        Events::new()
                            .on_click(|_| Msg::SetSelectingTableTool(table::Tool::Selector)),
                        vec![awesome::i("fa-mouse-pointer"), Html::text(" 選択")],
                    ),
                    btn::selectable(
                        selecting_tool.is_pen(),
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Pen)),
                        vec![awesome::i("fa-pen"), Html::text(" ペン")],
                    ),
                    btn::selectable(
                        selecting_tool.is_eracer(),
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetSelectingTableTool(table::Tool::Eracer)),
                        vec![awesome::i("fa-eraser"), Html::text(" 消しゴム")],
                    ),
                    btn::selectable(
                        selecting_tool.is_measure(),
                        Attributes::new(),
                        Events::new()
                            .on_click(|_| Msg::SetSelectingTableTool(table::Tool::Measure)),
                        vec![awesome::i("fa-ruler"), Html::text(" 計測")],
                    ),
                    btn::selectable(
                        selecting_tool.is_area(),
                        Attributes::new(),
                        Events::new()
                            .on_click(|_| Msg::SetSelectingTableTool(table::Tool::Measure)),
                        vec![awesome::i("fa-ruler-combined"), Html::text(" 範囲")],
                    ),
                    btn::selectable(
                        selecting_tool.is_route(),
                        Attributes::new(),
                        Events::new()
                            .on_click(|_| Msg::SetSelectingTableTool(table::Tool::Measure)),
                        vec![awesome::i("fa-route"), Html::text(" 経路")],
                    ),
                ]]
                .into_iter()
                .flatten()
                .collect(),
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
