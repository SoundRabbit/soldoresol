use super::{
    state::{self, Modal},
    Msg, State,
};
use crate::block::{self};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

mod canvas_container;
mod common;
mod contextmenu;
mod header_menu;
mod modal;
mod modeless;
mod side_menu;

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
            side_menu::render(state.table().selecting_tool()),
            if let Some(world) = state.block_field().get::<block::World>(state.world()) {
                canvas_container::render(&state, world)
            } else {
                Html::none()
            },
            if let Some(contextmenu_state) = state.contextmenu() {
                contextmenu::render(state.block_field(), contextmenu_state)
            } else {
                Html::none()
            },
            modal::render(state.block_field(), state.resource(), state.modal(), state),
        ],
    )
}
