use super::super::super::super::{btn, modeless};
use super::super::state::Modeless;
use super::Msg;
use crate::{
    block::{self, BlockId},
    model::{self},
    Color, Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

mod boxblock;
mod character;
mod tablemask;

pub fn render(
    block_field: &block::Field,
    resource: &Resource,
    modeless_id: model::modeless::ModelessId,
    modeless: &model::Modeless<Modeless>,
    grubbed: Option<model::modeless::ModelessId>,
    tabs: &Vec<BlockId>,
    focused: usize,
    outlined: Option<&Color>,
) -> Html {
    let attributes = if let Some(color) = outlined {
        Attributes::new().style(
            "box-shadow",
            format!("0 0 0.2rem 0.2rem {}", color.to_string()),
        )
    } else {
        Attributes::new()
    };
    let focused_id = &tabs[focused];
    super::frame(
        modeless_id,
        modeless,
        attributes,
        Events::new(),
        vec![
            super::header(
                modeless_id,
                grubbed,
                Attributes::new().class("frame-header-tab"),
                Events::new().on("drop", move |e| {
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
                        Msg::DropModelessTabToModeless(modeless_id)
                    } else {
                        Msg::NoOp
                    }
                }),
                Html::div(
                    Attributes::new(),
                    Events::new(),
                    tabs.iter()
                        .enumerate()
                        .filter_map(|(tab_idx, block_id)| {
                            if let Some(character) = block_field.get::<block::Character>(block_id) {
                                Some((tab_idx, character.name().to_string()))
                            } else if let Some(tablemask) =
                                block_field.get::<block::table_object::Tablemask>(block_id)
                            {
                                let p = tablemask.position();
                                Some((tab_idx, format!("マップマスク({:.1},{:.1})", p[0], p[1])))
                            } else if let Some(boxblock) =
                                block_field.get::<block::table_object::Boxblock>(block_id)
                            {
                                let p = boxblock.position();
                                Some((
                                    tab_idx,
                                    format!("ブロック({:.1},{:.1},{:.1})", p[0], p[1], p[2]),
                                ))
                            } else {
                                None
                            }
                        })
                        .map(|(tab_idx, name)| {
                            btn::frame_tab(
                                tab_idx == focused,
                                Events::new()
                                    .on_click({
                                        let modeless_id = modeless_id.clone();
                                        move |_| Msg::SetModelessTabIdx(modeless_id, tab_idx)
                                    })
                                    .on_mousedown(move |e| {
                                        e.stop_propagation();
                                        Msg::NoOp
                                    })
                                    .on_mousemove(move |e| {
                                        e.stop_propagation();
                                        Msg::NoOp
                                    })
                                    .on("dragstart", move |e| {
                                        let e = e.dyn_into::<web_sys::DragEvent>().unwrap();
                                        e.stop_propagation();
                                        let dt = e.data_transfer().unwrap();
                                        dt.set_effect_allowed("move");
                                        let _ = dt.set_data(
                                            "application/x-tab-idx",
                                            &tab_idx.to_string(),
                                        );

                                        crate::debug::log_1("dragstart");

                                        Msg::GrubModelessTab(modeless_id, tab_idx)
                                    }),
                                name,
                            )
                        })
                        .collect(),
                ),
            ),
            if let Some(character) = block_field.get::<block::Character>(focused_id) {
                character::render(
                    block_field,
                    resource,
                    grubbed.is_some(),
                    character,
                    focused_id,
                )
            } else if let Some(tablemask) =
                block_field.get::<block::table_object::Tablemask>(focused_id)
            {
                tablemask::render(
                    block_field,
                    resource,
                    grubbed.is_some(),
                    tablemask,
                    focused_id,
                )
            } else if let Some(boxblock) =
                block_field.get::<block::table_object::Boxblock>(focused_id)
            {
                boxblock::render(
                    block_field,
                    resource,
                    grubbed.is_some(),
                    boxblock,
                    focused_id,
                )
            } else {
                Html::none()
            },
            modeless::footer(Attributes::new(), Events::new(), vec![]),
        ],
    )
}
