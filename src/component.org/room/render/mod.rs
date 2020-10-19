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

pub fn render(state: &State, _: Vec<Html>) -> Html {
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
                3,
                state.room().id.as_ref(),
                state.headermenu(),
                state.table().is_2d_mode(),
            ),
            side_menu::render(2, state.table().selecting_tool()),
            if let Some(world) = state.block_field().get::<block::World>(state.world()) {
                canvas_container::render(1, &state, world)
            } else {
                Html::none()
            },
            if let Some(contextmenu_state) = state.contextmenu() {
                contextmenu::render(5, state.block_field(), contextmenu_state)
            } else {
                Html::none()
            },
            modal::render(
                4,
                state.block_field(),
                state.resource(),
                state.modal(),
                state,
            ),
        ],
    )
}
