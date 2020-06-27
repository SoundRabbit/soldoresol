use super::super::{awesome, btn};
use super::{
    state::{self, Modal, Modeless},
    Msg, State,
};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

mod canvas_container;
mod common;
mod header_menu;
mod modeless;

pub fn render(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .id("app")
            .class("fullscreen")
            .class("unselectable")
            .class("app")
            .style("grid-template-columns", "max-content 1fr"),
        Events::new()
            .on("dragover", |e| {
                e.prevent_default();
                Msg::NoOp
            })
            .on("drop", |e| {
                e.prevent_default();
                e.stop_propagation();
                let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                e.data_transfer()
                    .unwrap()
                    .files()
                    .map(|files| Msg::LoadFromFileList(files))
                    .unwrap_or(Msg::NoOp)
            }),
        vec![
            header_menu::render(
                state.room().id.as_ref(),
                state.table().selecting_tool(),
                state.table().is_2d_mode(),
            ),
            render_side_menu(),
            canvas_container::render(&state),
            render_loading_state(state.loading_resource_num, state.loaded_resource_num),
            render_context_menu(&state.contextmenu, &state.focused_object_id, &state.world),
            render_modals(
                &state.modals,
                &state.world,
                &state.personal_data,
                &state.chat_data,
                &state.resource,
            ),
        ],
    )
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().class("panel linear-v"),
        Events::new(),
        vec![
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenModeless(Modeless::Chat)),
                vec![awesome::i("fa-comments"), Html::text("チャット")],
            ),
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenModal(Modal::TableSetting)),
                vec![awesome::i("fa-layer-group"), Html::text("テーブル設定")],
            ),
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenModal(Modal::Resource)),
                vec![awesome::i("fa-folder"), Html::text("画像")],
            ),
        ],
    )
}

fn render_context_menu(
    contextmenu: &Contextmenu,
    focused_object_id: &Option<u128>,
    world: &World,
) -> Html<Msg> {
    if let Some(focused_object_id) = focused_object_id {
        if let Some(tablemask) = world.tablemask(focused_object_id) {
            render_context_menu_tablemask(contextmenu, *focused_object_id, tablemask)
        } else {
            render_context_menu_character(contextmenu, *focused_object_id)
        }
    } else {
        render_context_menu_default(contextmenu)
    }
}

fn render_context_menu_default(contextmenu: &Contextmenu) -> Html<Msg> {
    contextmenu::render(
        false,
        &contextmenu.state,
        || Box::new(|| Box::new(|msg| Msg::TransportContextMenuMsg(msg))),
        Attributes::new(),
        Events::new(),
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let position = contextmenu.canvas_position.clone();
                        move |_| Msg::AddChracaterWithMouseCoord(position)
                    }),
                    "キャラクターを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let position = contextmenu.canvas_position.clone();
                        move |_| Msg::AddTablemaskWithMouseCoord(position)
                    }),
                    "マップマスクを作成",
                ),
            ],
        )],
    )
}

fn render_context_menu_character(contextmenu: &Contextmenu, object_id: u128) -> Html<Msg> {
    contextmenu::render(
        false,
        &contextmenu.state,
        || Box::new(|| Box::new(|msg| Msg::TransportContextMenuMsg(msg))),
        Attributes::new(),
        Events::new(),
        vec![Html::ul(
            Attributes::new().class("pure-menu-list"),
            Events::new(),
            vec![
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click(move |_| Msg::OpenObjectModeless(object_id)),
                    "編集",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::CloneObjectWithObjectIdToTransport(object_id)),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::RemoveObjectWithObjectIdToTransport(object_id)),
                    "削除",
                ),
            ],
        )],
    )
}

fn render_context_menu_tablemask(
    contextmenu: &Contextmenu,
    object_id: u128,
    tablemask: &Tablemask,
) -> Html<Msg> {
    let is_fixed = tablemask.is_fixed();
    contextmenu::render(
        false,
        &contextmenu.state,
        || Box::new(|| Box::new(|msg| Msg::TransportContextMenuMsg(msg))),
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
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [2., 2.],
                                                true,
                                                is_fixed,
                                                "半径1",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [4., 4.],
                                                true,
                                                is_fixed,
                                                "半径2",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [6., 6.],
                                                true,
                                                is_fixed,
                                                "半径3",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [8., 8.],
                                                true,
                                                is_fixed,
                                                "半径4",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [10., 10.],
                                                true,
                                                is_fixed,
                                                "半径5",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [12., 12.],
                                                true,
                                                is_fixed,
                                                "半径6",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [14., 14.],
                                                true,
                                                is_fixed,
                                                "半径7",
                                            ),
                                        ],
                                    ),
                                    Html::ul(
                                        Attributes::new().class("pure-menu-list"),
                                        Events::new(),
                                        vec![
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [1., 1.],
                                                false,
                                                is_fixed,
                                                "矩形1×1",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [2., 2.],
                                                false,
                                                is_fixed,
                                                "矩形2×2",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [3., 3.],
                                                false,
                                                is_fixed,
                                                "矩形3×3",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [4., 4.],
                                                false,
                                                is_fixed,
                                                "矩形4×4",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [5., 5.],
                                                false,
                                                is_fixed,
                                                "矩形5×5",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [6., 6.],
                                                false,
                                                is_fixed,
                                                "矩形6×6",
                                            ),
                                            render_context_menu_tablemask_resizer(
                                                object_id,
                                                [7., 7.],
                                                false,
                                                is_fixed,
                                                "矩形7×7",
                                            ),
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
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 1.0)
                                    }),
                                    "100%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 0.8)
                                    }),
                                    "80%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 0.6)
                                    }),
                                    "60%",
                                ),
                                btn::contextmenu_text(
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetTablemaskTransparentToTransport(object_id, 0.4)
                                    }),
                                    "40%",
                                ),
                            ],
                        ),
                    ],
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let alpha = tablemask.background_color().alpha;
                        move |_| {
                            Msg::OpenModal(Modal::ColorPicker(ColorPickerType::TablemaskColor(
                                object_id, alpha,
                            )))
                        }
                    }),
                    "色を変更",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new().on_click({
                        let size = tablemask.size().clone();
                        let is_rounded = tablemask.is_rounded();
                        move |_| {
                            Msg::SetTablemaskSizeWithStyleToTransport(
                                object_id, size, is_rounded, !is_fixed,
                            )
                        }
                    }),
                    String::from("固定") + if is_fixed { "解除" } else { "する" },
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::CloneObjectWithObjectIdToTransport(object_id)),
                    "コピーを作成",
                ),
                btn::contextmenu_text(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |_| Msg::RemoveObjectWithObjectIdToTransport(object_id)),
                    "削除",
                ),
            ],
        )],
    )
}

fn render_context_menu_tablemask_resizer(
    object_id: u128,
    size: [f64; 2],
    is_rounded: bool,
    is_fixed: bool,
    text: impl Into<String>,
) -> Html<Msg> {
    btn::contextmenu_text(
        Attributes::new(),
        Events::new().on_click(move |_| {
            Msg::SetTablemaskSizeWithStyleToTransport(object_id, size, is_rounded, is_fixed)
        }),
        text,
    )
}

fn render_loading_state(loading_resource_num: u64, loaded_resource_num: u64) -> Html<Msg> {
    if loading_resource_num == 0 {
        Html::none()
    } else {
        Html::div(
            Attributes::new()
                .class("text-color-light")
                .style("position", "fixed")
                .style("top", "0em")
                .style("right", "0em"),
            Events::new(),
            vec![Html::text(format!(
                "Loading：{} / {}",
                loaded_resource_num,
                loading_resource_num + loaded_resource_num
            ))],
        )
    }
}

fn render_hint() -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("text-color-secondary-d")
            .style("position", "absolute")
            .style("bottom", "5em")
            .style("right", "5em"),
        Events::new(),
        vec![Html::text("Ctrl + ドラッグ or Alt + ドラッグで視界を回転")],
    )
}

fn render_measure_length(measure_length: &Option<f64>) -> Html<Msg> {
    if let Some(measure_length) = measure_length {
        Html::div(
            Attributes::new()
                .style("position", "absolute")
                .style("top", "5em")
                .style("right", "5em"),
            Events::new(),
            vec![Html::text(format!("計測結果：{:.1}", measure_length))],
        )
    } else {
        Html::none()
    }
}

fn render_modals(
    modals: &Vec<Modal>,
    world: &World,
    personal_data: &PersonalData,
    chat_data: &ChatDataCollection,
    resource: &Resource,
) -> Html<Msg> {
    let mut children = vec![];
    for modal in modals {
        let child = match modal {
            Modal::Resource => modal::resource(resource),
            Modal::SelectImage(modal_type) => modal::select_image(resource, modal_type),
            Modal::PersonalSetting => modal::personal_setting(personal_data, resource),
            Modal::TableSetting => modal::table_setting(
                world.selecting_table_id(),
                &world.selecting_table(),
                world.tables(),
                &resource,
            ),
            Modal::ColorPicker(color_picker_type) => modal::color_picker(color_picker_type.clone()),
            Modal::CharacterSelecter(character_selecter_type) => match character_selecter_type {
                CharacterSelecterType::ChatSender => modal::character_selecter(
                    character_selecter_type.clone(),
                    chat_data
                        .senders
                        .iter()
                        .filter_map(|s| s.as_character())
                        .collect(),
                    world,
                    resource,
                ),
            },
            Modal::ChatLog => modal::chat_log(chat_data, resource),
            Modal::ChatTabEditor => modal::chat_tab_editor(chat_data),
        };
        children.push(child);
    }
    Html::div(
        Attributes::new()
            .style("position", "fixied")
            .style("z-index", "1"),
        Events::new(),
        children,
    )
}
