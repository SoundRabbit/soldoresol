use super::{
    super::{btn, image, modal},
    Msg, PersonalData,
};
use crate::model::Resource;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn resource(resource: &Resource) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new()
            .on("dragover", |e| {
                e.prevent_default();
                Msg::NoOp
            })
            .on("drop", |e| {
                e.prevent_default();
                let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                e.data_transfer()
                    .unwrap()
                    .files()
                    .map(|files| Msg::LoadFromFileList(files))
                    .unwrap_or(Msg::NoOp)
            }),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                modal::header(
                    Attributes::new()
                        .style("display", "grid")
                        .style("grid-template-columns", "1fr max-content"),
                    Events::new(),
                    vec![
                        Html::div(Attributes::new(), Events::new(), vec![]),
                        Html::div(
                            Attributes::new().class("linear-h"),
                            Events::new(),
                            vec![btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::CloseModal),
                            )],
                        ),
                    ],
                ),
                modal::body(
                    Attributes::new()
                        .class("scroll-v grid container")
                        .style("min-height", "50vh"),
                    Events::new(),
                    resource
                        .get_images()
                        .into_iter()
                        .map(|(_, img)| {
                            Html::component(image::new(
                                img,
                                Attributes::new().class("grid-w-2 pure-img"),
                            ))
                        })
                        .collect(),
                ),
                modal::footer(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
                ),
            ],
        )],
    )
}

pub fn select_character_image(character_id: u128, resource: &Resource) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new()
            .on("dragover", |e| {
                e.prevent_default();
                Msg::NoOp
            })
            .on("drop", |e| {
                e.prevent_default();
                let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                e.data_transfer()
                    .unwrap()
                    .files()
                    .map(|files| Msg::LoadFromFileList(files))
                    .unwrap_or(Msg::NoOp)
            }),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                modal::header(
                    Attributes::new()
                        .style("display", "grid")
                        .style("grid-template-columns", "1fr max-content"),
                    Events::new(),
                    vec![
                        Html::div(Attributes::new(), Events::new(), vec![]),
                        Html::div(
                            Attributes::new().class("linear-h"),
                            Events::new(),
                            vec![btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::CloseModal),
                            )],
                        ),
                    ],
                ),
                modal::body(
                    Attributes::new()
                        .class("scroll-v grid container")
                        .style("min-height", "50vh"),
                    Events::new(),
                    resource
                        .get_images()
                        .into_iter()
                        .map(|(data_id, img)| {
                            Html::div(
                                Attributes::new().class("grid-w-2 clickable"),
                                Events::new().on_click(move |_| {
                                    Msg::SetCharacterImage(character_id, data_id, true)
                                }),
                                vec![Html::component(image::new(
                                    img,
                                    Attributes::new().class("pure-img"),
                                ))],
                            )
                        })
                        .collect(),
                ),
                modal::footer(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text("ファイルはドラッグ & ドロップで追加できます。")],
                ),
            ],
        )],
    )
}

pub fn personal_setting(personal_data: &PersonalData) -> Html<Msg> {
    modal::container(
        Attributes::new(),
        Events::new(),
        vec![modal::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                modal::header(
                    Attributes::new()
                        .style("display", "grid")
                        .style("grid-template-columns", "1fr max-content"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("個人設定")],
                        ),
                        Html::div(
                            Attributes::new().class("linear-h"),
                            Events::new(),
                            vec![btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::CloseModal),
                            )],
                        ),
                    ],
                ),
                modal::body(
                    Attributes::new()
                        .class("scroll-v pure-form")
                        .style("min-height", "50vh"),
                    Events::new(),
                    vec![Html::div(
                        Attributes::new().class("keyvalueoption"),
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
                                    .value(&personal_data.name),
                                Events::new(),
                                vec![],
                            ),
                            btn::primary(
                                Attributes::new(),
                                Events::new(),
                                vec![Html::text("リセット")],
                            ),
                        ],
                    )],
                ),
                modal::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
