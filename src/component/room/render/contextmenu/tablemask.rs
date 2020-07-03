use super::super::super::super::{btn, contextmenu};
use super::state::{self, Modal};
use super::Msg;
use crate::block::{self, BlockId};
use kagura::prelude::*;

pub fn render(
    z_index: u64,
    contextmenu: &state::contextmenu::State,
    block_id: &BlockId,
    tablemask: &block::table_object::Tablemask,
) -> Html<Msg> {
    contextmenu::div(
        z_index,
        || Msg::CloseContextmenu,
        contextmenu.grobal_position(),
        Attributes::new(),
        Events::new(),
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![
                Html::li(
                    Attributes::new()
                        .class("pure-menu-item")
                        .class("pure-menu-has-children"),
                    Events::new(),
                    vec![
                        btn::contextmenu_text(Attributes::new(), Events::new(), "サイズ"),
                        Html::ul(
                            Attributes::new().class("pure-menu-children"),
                            Events::new(),
                            vec![Html::li(
                                Attributes::new()
                                    .class("pure-menu-item")
                                    .class("linear-h")
                                    .style("display", "grid"),
                                Events::new(),
                                vec![
                                    Html::ul(
                                        Attributes::new().class("pure-menu-list"),
                                        Events::new(),
                                        vec![
                                            resizer(block_id.clone(), [2., 2.], "半径1"),
                                            resizer(block_id.clone(), [4., 4.], "半径2"),
                                            resizer(block_id.clone(), [6., 6.], "半径3"),
                                            resizer(block_id.clone(), [8., 8.], "半径4"),
                                            resizer(block_id.clone(), [10., 10.], "半径5"),
                                            resizer(block_id.clone(), [12., 12.], "半径6"),
                                            resizer(block_id.clone(), [14., 14.], "半径7"),
                                        ],
                                    ),
                                    Html::ul(
                                        Attributes::new().class("pure-menu-list"),
                                        Events::new(),
                                        vec![
                                            resizer(block_id.clone(), [1., 1.], "矩形1×1"),
                                            resizer(block_id.clone(), [2., 2.], "矩形2×2"),
                                            resizer(block_id.clone(), [3., 3.], "矩形3×3"),
                                            resizer(block_id.clone(), [4., 4.], "矩形4×4"),
                                            resizer(block_id.clone(), [5., 5.], "矩形5×5"),
                                            resizer(block_id.clone(), [6., 6.], "矩形6×6"),
                                            resizer(block_id.clone(), [7., 7.], "矩形7×7"),
                                        ],
                                    ),
                                ],
                            )],
                        ),
                    ],
                ),
                Html::li(
                    Attributes::new()
                        .class("pure-menu-item")
                        .class("pure-menu-has-children"),
                    Events::new(),
                    vec![
                        btn::contextmenu_text(Attributes::new(), Events::new(), "不透明度"),
                        Html::ul(
                            Attributes::new().class("pure-menu-children"),
                            Events::new(),
                            vec![
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| Msg::NoOp),
                                    "100%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| Msg::NoOp),
                                    "80%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| Msg::NoOp),
                                    "60%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| Msg::NoOp),
                                    "40%",
                                ),
                            ],
                        ),
                    ],
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| Msg::OpenModal(Modal::TablemaskColorPicker(block_id))
                    }),
                    "色を変更",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({ move |_| Msg::NoOp }),
                    String::from("固定")
                        + if tablemask.is_fixed() {
                            "解除"
                        } else {
                            "する"
                        },
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| Msg::CloneTablemaskToCloseContextmenu(block_id)
                    }),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let block_id = block_id.clone();
                        move |_| Msg::RemoveTablemaskToCloseContextmenu(block_id)
                    }),
                    "削除",
                ),
            ],
        )],
    )
}

fn resizer(block_id: BlockId, size: [f64; 2], text: impl Into<String>) -> Html<Msg> {
    btn::contextmenu_text(
        Attributes::new(),
        Events::new().on_click(move |_| Msg::NoOp),
        text,
    )
}
