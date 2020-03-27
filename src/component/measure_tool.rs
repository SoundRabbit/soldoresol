use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::checkbox::checkbox;
use super::form::form;

pub fn measure_tool<Msg>() -> Html<Msg> {
    form(
        Attributes::new().id("measure_tool"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("form-header"),
                Events::new(),
                vec![Html::text("計測オプション")],
            ),
            Html::div(
                Attributes::new().class("form-body"),
                Events::new(),
                vec![
                    checkbox(Attributes::new(), Events::new(), "テーブルに円を表示"),
                    checkbox(Attributes::new(), Events::new(), "折れ線経路を測定"),
                    checkbox(Attributes::new(), Events::new(), "測定内容を共有"),
                ],
            ),
            Html::div(
                Attributes::new().class("form-footer"),
                Events::new(),
                vec![],
            ),
        ],
    )
}
