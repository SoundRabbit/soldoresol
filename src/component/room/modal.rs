use super::{
    super::{awesome, btn, image, modal},
    Modal, Msg, PersonalData, SelectImageModal,
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
                        .map(|(data_id, img)| {
                            Html::component(image::new(
                                data_id,
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

pub fn select_image(resource: &Resource, modal_type: &SelectImageModal) -> Html<Msg> {
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
                                Events::new().on_click({
                                    let modal_type = modal_type.clone();
                                    move |_| match modal_type {
                                        SelectImageModal::Character(c_id) => {
                                            Msg::SetCharacterImage(c_id, data_id, true)
                                        }
                                        SelectImageModal::Player => {
                                            Msg::SetPersonalDataWithIconImage(data_id)
                                        }
                                    }
                                }),
                                vec![Html::component(image::new(
                                    data_id,
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

pub fn personal_setting(personal_data: &PersonalData, resource: &Resource) -> Html<Msg> {
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
                                    personal_data
                                        .icon
                                        .and_then(|r_id| {
                                            resource.get_as_image(&r_id).map(|img| (img, r_id))
                                        })
                                        .map(|(img, r_id)| {
                                            Html::component(image::new(
                                                r_id,
                                                img,
                                                Attributes::new().class("pure-img"),
                                            ))
                                        })
                                        .unwrap_or(Html::div(
                                            Attributes::new().class(concat!(
                                                "icon ",
                                                "icon-large ",
                                                "icon-rounded ",
                                                "bg-color-light ",
                                                "text-color-dark ",
                                            )),
                                            Events::new(),
                                            vec![awesome::i("fa-kiwi-bird")],
                                        )),
                                    btn::primary(
                                        Attributes::new(),
                                        Events::new().on_click(|_| {
                                            Msg::OpenModal(Modal::SelectImage(
                                                SelectImageModal::Player,
                                            ))
                                        }),
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
                                            .value(&personal_data.name),
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
