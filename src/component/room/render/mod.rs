use super::{Msg, State};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

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
                    .map(|files| Msg::LoadFromFileListToTransport(files))
                    .unwrap_or(Msg::NoOp)
            }),
        vec![
            render_header_menu(
                &state.room.id,
                &state.table_state.selecting_tool,
                state.is_2d_mode,
                state.is_low_loading_mode,
            ),
            render_side_menu(),
            render_canvas_container(&state),
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

fn render_header_menu(
    room_id: &String,
    selecting_tool: &TableTool,
    is_2d_mode: bool,
    is_low_loading_mode: bool,
) -> Html<Msg> {
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
                vec![
                    vec![
                        btn::selectable(
                            selecting_tool.is_selector(),
                            Attributes::new(),
                            Events::new()
                                .on_click(|_| Msg::SetSelectingTableTool(TableTool::Selector)),
                            vec![awesome::i("fa-mouse-pointer"), Html::text(" 選択")],
                        ),
                        btn::selectable(
                            selecting_tool.is_pen(),
                            Attributes::new(),
                            Events::new().on_click(|_| Msg::SetSelectingTableTool(TableTool::Pen)),
                            vec![awesome::i("fa-pen"), Html::text(" ペン")],
                        ),
                        btn::selectable(
                            selecting_tool.is_eracer(),
                            Attributes::new(),
                            Events::new()
                                .on_click(|_| Msg::SetSelectingTableTool(TableTool::Eracer)),
                            vec![awesome::i("fa-eraser"), Html::text(" 消しゴム")],
                        ),
                        btn::selectable(
                            selecting_tool.is_measure(),
                            Attributes::new(),
                            Events::new().on_click(|_| {
                                Msg::SetSelectingTableTool(TableTool::Measure(
                                    0.2, false, None, false,
                                ))
                            }),
                            vec![awesome::i("fa-ruler"), Html::text(" 計測")],
                        ),
                    ],
                    table_tool_option(selecting_tool),
                ]
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
                    vec![
                        Html::div(
                            Attributes::new().class("keyvalue"),
                            Events::new(),
                            vec![
                                Html::span(
                                    Attributes::new().class("text-label"),
                                    Events::new(),
                                    vec![Html::text("低負荷モード")],
                                ),
                                btn::toggle(
                                    is_low_loading_mode,
                                    Attributes::new(),
                                    Events::new().on_click(move |_| {
                                        Msg::SetLowLoadingMode(!is_low_loading_mode)
                                    }),
                                ),
                            ],
                        ),
                        Html::div(
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
                                    Events::new().on_click(move |_| Msg::SetIs2dMode(!is_2d_mode)),
                                ),
                            ],
                        ),
                    ],
                )],
            ),
        ],
    )
}

fn table_tool_option(selecting_tool: &TableTool) -> Vec<Html<Msg>> {
    match selecting_tool {
        TableTool::Selector => vec![],
        TableTool::Pen => vec![],
        TableTool::Eracer => vec![],
        TableTool::Measure(line_width, rounded, start_point, with_table_mask) => {
            let rounded = *rounded;
            let line_width = *line_width;
            let with_table_mask = *with_table_mask;
            vec![
                Html::div(
                    Attributes::new().class("keyvalue"),
                    Events::new(),
                    vec![
                        Html::span(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("太さ")],
                        ),
                        Html::input(
                            Attributes::new()
                                .value(line_width.to_string())
                                .type_("number")
                                .string("step", "0.1"),
                            Events::new().on_input({
                                let start_point = start_point.clone();
                                move |w| {
                                    w.parse()
                                        .map(|w| {
                                            Msg::SetSelectingTableTool(TableTool::Measure(
                                                w,
                                                rounded,
                                                start_point,
                                                with_table_mask,
                                            ))
                                        })
                                        .unwrap_or(Msg::NoOp)
                                }
                            }),
                            vec![],
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class("keyvalue"),
                    Events::new(),
                    vec![
                        Html::span(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("円弧")],
                        ),
                        btn::toggle(
                            rounded,
                            Attributes::new(),
                            Events::new().on_click({
                                let start_point = start_point.clone();
                                move |_| {
                                    Msg::SetSelectingTableTool(TableTool::Measure(
                                        line_width,
                                        !rounded,
                                        start_point,
                                        with_table_mask,
                                    ))
                                }
                            }),
                        ),
                    ],
                ),
                Html::div(
                    Attributes::new().class("keyvalue"),
                    Events::new(),
                    vec![
                        Html::span(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("マップマスクを生成")],
                        ),
                        btn::toggle(
                            with_table_mask,
                            Attributes::new(),
                            Events::new().on_click({
                                let start_point = start_point.clone();
                                move |_| {
                                    Msg::SetSelectingTableTool(TableTool::Measure(
                                        line_width,
                                        rounded,
                                        start_point,
                                        !with_table_mask,
                                    ))
                                }
                            }),
                        ),
                    ],
                ),
            ]
        }
    }
}

fn render_side_menu() -> Html<Msg> {
    Html::div(
        Attributes::new().class("panel linear-v"),
        Events::new(),
        vec![
            btn::light(
                Attributes::new().class("pure-button-sidemenu"),
                Events::new().on_click(|_| Msg::OpenChatModeless),
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

fn render_canvas_container(state: &State) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .style("position", "relative")
            .style("z-index", "0"),
        Events::new(),
        vec![
            render_canvas(),
            render_speech_bubble_queue(&state.speech_bubble_queue, &state.resource),
            render_measure_length(&state.table_state.measure_length),
            render_hint(),
            render_table_character_list(
                state.world.characters().map(|(_, x)| x).collect(),
                &state.resource,
            ),
            render_canvas_overlaper(
                &state.table_state,
                &state.focused_object_id,
                state.is_2d_mode,
                &state.world,
                &state.resource,
                &state.chat_data,
                &state.personal_data,
                &state.modelesses,
                &state.modeless_dom,
            ),
            state
                .editing_modeless
                .as_ref()
                .map(|(_, props)| {
                    Html::component(modeless_modal::new(Rc::clone(props)).subscribe(
                        |sub| match sub {
                            modeless_modal::Sub::ReflectToClose(props) => {
                                Msg::CloseModelessModalWithProps(props)
                            }
                        },
                    ))
                })
                .unwrap_or(Html::none()),
        ],
    )
}

fn render_canvas() -> Html<Msg> {
    Html::canvas(
        Attributes::new().id("table").class("cover cover-a"),
        Events::new(),
        vec![],
    )
}

fn render_speech_bubble_queue(
    speech_bubble_queue: &VecDeque<SpeechBubble>,
    resource: &Resource,
) -> Html<Msg> {
    modeless_container(
        Attributes::new().class("cover cover-a"),
        Events::new(),
        speech_bubble_queue
            .iter()
            .map(|speech_bubble| {
                Html::div(
                    Attributes::new()
                        .class("speechbubble")
                        .style("position", "absolute")
                        .style("left", format!("{}px", speech_bubble.position[0]))
                        .style("top", format!("{}px", speech_bubble.position[1])),
                    Events::new(),
                    vec![
                        speech_bubble
                            .texture_id
                            .and_then(|texture_id| resource.get_as_image_url(&texture_id))
                            .map(|image_url| {
                                Html::img(
                                    Attributes::new()
                                        .class("pure-img")
                                        .class("speechbubble-img")
                                        .string("src", image_url.as_str()),
                                    Events::new(),
                                    vec![],
                                )
                            })
                            .unwrap_or(Html::none()),
                        Html::pre(
                            Attributes::new().class("speechbubble-message"),
                            Events::new(),
                            vec![Html::text(&speech_bubble.message)],
                        ),
                    ],
                )
            })
            .collect(),
    )
}

fn render_table_character_list(characters: Vec<&Character>, resource: &Resource) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .class("cover-a")
            .class("flex-v"),
        Events::new(),
        characters
            .into_iter()
            .map(|character| render_table_character_list_item(character, resource))
            .collect(),
    )
}

fn render_table_character_list_item(character: &Character, resource: &Resource) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("chat-item")
            .class("bg-color-light-t")
            .class("container-a"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new()
                    .class("chat-icon linear-v")
                    .style("justify-items", "center"),
                Events::new(),
                vec![{
                    let icon = character
                        .texture_id()
                        .map(|r_id| Icon::Resource(r_id))
                        .unwrap_or(Icon::DefaultUser);
                    common::chat_icon(
                        Attributes::new().class("icon-medium"),
                        &icon,
                        character.name(),
                        resource,
                    )
                }],
            ),
            Html::div(
                Attributes::new().class("chat-args"),
                Events::new(),
                vec![Html::text(character.name())],
            ),
            Html::div(
                Attributes::new()
                    .class("chat-payload")
                    .class("keyvalue")
                    .class("keyvalue-align-start"),
                Events::new(),
                render_table_character_list_item_payload(character.property.selecteds()),
            ),
        ],
    )
}

fn render_table_character_list_item_payload(props: Vec<&Property>) -> Vec<Html<Msg>> {
    props
        .into_iter()
        .map(|prop| match prop.value() {
            PropertyValue::Children(children) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(
                    Attributes::new()
                        .class("keyvalue")
                        .class("keyvalue-align-start"),
                    Events::new(),
                    render_table_character_list_item_payload(children.iter().collect()),
                ),
            ],
            PropertyValue::None => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(Attributes::new(), Events::new(), vec![]),
            ],
            PropertyValue::Num(x) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(x.to_string())],
                ),
            ],
            PropertyValue::Str(x) => vec![
                Html::span(
                    Attributes::new(),
                    Events::new(),
                    vec![Html::text(prop.name())],
                ),
                Html::span(Attributes::new(), Events::new(), vec![Html::text(x)]),
            ],
        })
        .flatten()
        .collect()
}

fn render_canvas_overlaper(
    table_state: &TableState,
    focused_object_id: &Option<u128>,
    is_2d_mode: bool,
    world: &World,
    resource: &Resource,
    chat_tabs: &ChatDataCollection,
    personal_data: &PersonalData,
    modelesses: &ModelessCollection,
    modeless_dom: &Vec<Option<u128>>,
) -> Html<Msg> {
    let focused_object_id = focused_object_id.clone().and_then(|o_id| {
        if world.character(&o_id).is_some() {
            Some(o_id)
        } else if world
            .tablemask(&o_id)
            .map(|t| !t.is_fixed())
            .unwrap_or(false)
        {
            Some(o_id)
        } else {
            None
        }
    });
    modeless_container(
        Attributes::new()
            .class("cover cover-a")
            .style("z-index", "0"),
        Events::new()
            .on_mousemove({
                let selecting_tool = table_state.selecting_tool.clone();
                move |e| {
                    e.stop_propagation();
                    let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                    if e.buttons() & 1 == 0 {
                        Msg::SetCursorWithMouseCoord(mouse_coord)
                    } else if (e.alt_key() || e.ctrl_key()) && !is_2d_mode {
                        Msg::SetCameraRotationWithMouseCoord(mouse_coord)
                    } else {
                        match selecting_tool {
                            TableTool::Selector => match focused_object_id {
                                Some(character_id) => {
                                    Msg::SetObjectPositionWithMouseCoord(character_id, mouse_coord)
                                }
                                None => Msg::SetCameraMovementWithMouseCoord(mouse_coord),
                            },
                            TableTool::Pen => Msg::DrawLineWithMouseCoord(mouse_coord),
                            TableTool::Eracer => Msg::EraceLineWithMouseCoord(mouse_coord),
                            TableTool::Measure(line_width, rounded, Some(start_point), _) => {
                                Msg::SetMeasureStartPointAndEndPointWithMouseCoord(
                                    line_width,
                                    rounded,
                                    start_point,
                                    mouse_coord,
                                )
                            }
                            _ => Msg::SetCursorWithMouseCoord(mouse_coord),
                        }
                    }
                }
            })
            .on("wheel", |e| {
                e.stop_propagation();
                if let Ok(e) = e.dyn_into::<web_sys::WheelEvent>() {
                    Msg::SetCameraMovementWithMouseWheel(e.delta_y())
                } else {
                    Msg::NoOp
                }
            })
            .on_mousedown({
                let selecting_tool = table_state.selecting_tool.clone();
                move |e| {
                    e.stop_propagation();
                    match selecting_tool {
                        TableTool::Measure(line_width, rounded, _, with_table_mask) => {
                            let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                            Msg::SetSelectingTableTool(TableTool::Measure(
                                line_width,
                                rounded,
                                Some(mouse_coord),
                                with_table_mask,
                            ))
                        }
                        _ => Msg::NoOp,
                    }
                }
            })
            .on_mouseup({
                let selecting_tool = table_state.selecting_tool.clone();
                let focused_object_id = focused_object_id.clone();
                move |e| {
                    e.stop_propagation();
                    let mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                    match selecting_tool {
                        TableTool::Selector => match focused_object_id {
                            Some(object_id) => Msg::BindObjectToTableGridToTransport(object_id),
                            None => Msg::NoOp,
                        },
                        TableTool::Measure(line_width, rounded, Some(start_point), true) => {
                            Msg::AddTablemaskWithPointABToTransport(
                                line_width,
                                start_point,
                                mouse_coord,
                                rounded,
                            )
                        }
                        TableTool::Measure(line_width, rounded, _, with_table_mask) => {
                            Msg::SetSelectingTableTool(TableTool::Measure(
                                line_width,
                                rounded,
                                None,
                                with_table_mask,
                            ))
                        }
                        _ => Msg::NoOp,
                    }
                }
            })
            .on_contextmenu(|e| {
                let page_mouse_coord = [e.page_x() as f64, e.page_y() as f64];
                let offset_mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                e.prevent_default();
                e.stop_propagation();
                Msg::OpenContextMenu(page_mouse_coord, offset_mouse_coord)
            }),
        modeless_dom
            .iter()
            .map(|modeless_id| {
                if let Some((state, modeless)) =
                    modeless_id.and_then(|modeless_id| modelesses.get(&modeless_id))
                {
                    match modeless {
                        Modeless::Object { focused, tabs } => modeless::object(
                            modeless_id.unwrap(),
                            state,
                            tabs,
                            *focused,
                            world,
                            resource,
                        ),
                        Modeless::Chat => modeless::chat(
                            modeless_id.unwrap(),
                            state,
                            chat_tabs,
                            personal_data,
                            world,
                            resource,
                        ),
                    }
                } else {
                    Html::div(Attributes::new(), Events::new(), vec![])
                }
            })
            .collect(),
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
