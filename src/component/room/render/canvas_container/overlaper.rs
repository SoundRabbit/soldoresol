use super::super::{
    modeless,
    state::{self, chat, table, Modeless},
};
use super::Msg;
use crate::{
    block,
    model::{self, PersonalData},
    Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn render(
    block_field: &block::Field,
    table: &table::State,
    world: &block::World,
    resource: &Resource,
    chat: &chat::State,
    personal_data: &PersonalData,
    modeless: &model::modeless::Collection<Modeless>,
) -> Html<Msg> {
    let grubbed = modeless.grubbed();

    Html::div(
        Attributes::new()
            .class("cover cover-a")
            .style("z-index", "0"),
        Events::new()
            .on_mousemove({
                let selecting_tool = table.selecting_tool().clone();
                let is_2d_mode = table.is_2d_mode();
                let focused = table.focused().clone();
                let last_mouse_pos = table.last_mouse_position().clone();
                let last_mouse_down_pos = table.last_mouse_down_position().clone();
                let grubbed = grubbed.clone();
                move |e| {
                    let mouse_pos = [e.offset_x() as f32, e.offset_y() as f32];
                    if let Some(modeless_id) = grubbed {
                        let mouse_pos = [e.client_x() as f64, e.client_y() as f64];
                        Msg::DragModeless(modeless_id, mouse_pos)
                    } else if e.buttons() & 1 == 0 {
                        Msg::NoOp
                    } else if (e.alt_key() || e.ctrl_key()) && !is_2d_mode {
                        Msg::SetCameraRotationWithMouseMovement(mouse_pos)
                    } else {
                        match selecting_tool {
                            table::Tool::Selector => match focused {
                                table::Focused::Character(character_id) => {
                                    Msg::SetCharacterPositionWithMousePosition(
                                        character_id,
                                        mouse_pos,
                                    )
                                }
                                table::Focused::Tablemask(tablemask_id) => {
                                    Msg::SetTablemaskPositionWithMousePosition(
                                        tablemask_id,
                                        mouse_pos,
                                    )
                                }
                                table::Focused::None => {
                                    Msg::SetCameraMovementWithMouseMovement(mouse_pos)
                                }
                            },
                            table::Tool::Pen => {
                                Msg::DrawLineWithMousePosition(last_mouse_pos, mouse_pos)
                            }
                            table::Tool::Eracer => {
                                Msg::EraceLineWithMousePosition(last_mouse_pos, mouse_pos)
                            }
                            table::Tool::Measure(..) => {
                                Msg::MeasureLineWithMousePosition(last_mouse_down_pos, mouse_pos)
                            }
                            table::Tool::Area {
                                line_width,
                                is_rounded,
                            } => Msg::NoOp,
                            table::Tool::Route(block_id) => Msg::NoOp,
                        }
                    }
                }
            })
            .on_mouseup({
                let grubbed = grubbed.clone();
                let selecting_tool = table.selecting_tool().clone();
                move |e| {
                    if let Some(modeless_id) = grubbed {
                        Msg::DropModeless(modeless_id)
                    } else if selecting_tool.is_measure() {
                        Msg::SetSelectingTableTool(table::Tool::Measure(None))
                    } else {
                        Msg::NoOp
                    }
                }
            })
            .on("wheel", |e| {
                e.stop_propagation();
                if let Ok(e) = e.dyn_into::<web_sys::WheelEvent>() {
                    Msg::SetCameraMovementWithMouseWheel(e.delta_y() as f32)
                } else {
                    Msg::NoOp
                }
            })
            .on_contextmenu({
                let focused = table.focused().clone();
                move |e| {
                    let page_mouse_coord = [e.page_x() as f64, e.page_y() as f64];
                    let offset_mouse_coord = [e.offset_x() as f64, e.offset_y() as f64];
                    e.prevent_default();
                    e.stop_propagation();

                    match focused {
                        table::Focused::Character(character_id) => {
                            crate::debug::log_1("focused::Character");
                            Msg::OpenContextmenu(
                                page_mouse_coord,
                                offset_mouse_coord,
                                state::Contextmenu::Character(character_id.clone()),
                            )
                        }
                        table::Focused::Tablemask(tablemask_id) => Msg::OpenContextmenu(
                            page_mouse_coord,
                            offset_mouse_coord,
                            state::Contextmenu::Tablemask(tablemask_id.clone()),
                        ),
                        table::Focused::None => Msg::OpenContextmenu(
                            page_mouse_coord,
                            offset_mouse_coord,
                            state::Contextmenu::Default,
                        ),
                    }
                }
            })
            .on("drop", move |e| {
                let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                let dt = e.data_transfer().unwrap();
                if dt
                    .types()
                    .to_vec()
                    .iter()
                    .any(|x| x.as_string().unwrap() == "application/x-tab-idx")
                {
                    e.prevent_default();
                    e.stop_propagation();
                    let current_tagret = e
                        .current_target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlElement>()
                        .unwrap();
                    let cr = current_tagret.get_bounding_client_rect();
                    let x = cr.left();
                    let y = cr.top();
                    let mouse_pos = [e.client_x() as f64 - x, e.client_y() as f64 - y];
                    Msg::DropModelessTab(mouse_pos)
                } else {
                    Msg::NoOp
                }
            }),
        modeless
            .iter()
            .map(|(modeless_id, modeless)| {
                if let Some(modeless) = modeless {
                    modeless::render(
                        block_field,
                        resource,
                        modeless_id,
                        modeless,
                        grubbed.clone(),
                    )
                } else {
                    Html::div(Attributes::new(), Events::new(), vec![])
                }
            })
            .collect(),
    )
}
