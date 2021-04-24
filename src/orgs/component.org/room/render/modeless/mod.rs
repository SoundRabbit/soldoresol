use super::super::super::{btn, modeless};
use super::{
    common,
    state::{self, Modeless},
};
use super::{Msg, State};
use crate::{
    block::{self, chat::item::Icon, BlockId},
    model::{self, PersonalData},
    Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

mod chat;
mod memo;
mod object;

pub fn render(
    state: &State,
    modeless_id: model::modeless::ModelessId,
    modeless: &model::Modeless<Modeless>,
    grabbed: Option<model::modeless::ModelessId>,
) -> Html {
    match modeless.as_ref() {
        Modeless::Object {
            tabs,
            focused,
            outlined,
            ..
        } => object::render(
            state.block_field(),
            state.resource(),
            modeless_id,
            modeless,
            grabbed,
            tabs,
            *focused,
            outlined.as_ref(),
        ),
        Modeless::Chat => {
            if let Some((chat_data, tab_id, tab)) = state
                .chat_block()
                .and_then(|chat| state.selecting_chat_tab_id().map(|tab_id| (chat, tab_id)))
                .and_then(|(chat, tab_id)| {
                    state
                        .selecting_chat_tab_block()
                        .map(|tab_block| (chat, tab_id, tab_block))
                })
            {
                chat::render(
                    state.block_field(),
                    state.resource(),
                    modeless_id,
                    modeless,
                    grabbed,
                    state.chat(),
                    state.dicebot(),
                    chat_data,
                    state.personal_data(),
                    tab_id,
                    tab,
                )
            } else {
                Html::none()
            }
        }
        Modeless::Memo { focused } => {
            if let Some(world) = state.block_field().get::<block::World>(state.world()) {
                memo::render(
                    modeless_id,
                    state.block_field(),
                    state.resource(),
                    modeless,
                    grabbed,
                    world,
                    *focused,
                )
            } else {
                Html::none()
            }
        }
    }
}

fn frame(
    modeless_id: model::modeless::ModelessId,
    modeless: &model::Modeless<Modeless>,
    attributes: Attributes,
    events: Events,
    children: Vec<Html>,
) -> Html {
    let attributes = attributes.style("z-index", modeless.z_index().to_string());
    modeless::frame(
        modeless,
        attributes,
        events
            .on_mousedown(move |e| {
                e.stop_propagation();
                Msg::FocusModeless(modeless_id)
            })
            .on_contextmenu(|e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on("wheel", |e| {
                e.stop_propagation();
                Msg::NoOp
            })
            .on_click(|e| {
                e.stop_propagation();
                Msg::NoOp
            }),
        vec![
            children,
            vec![Html::div(
                Attributes::new(),
                Events::new().on_mousedown(move |e| {
                    e.stop_propagation();
                    let mouse_pos = [e.page_x() as f64, e.page_y() as f64];
                    e.target()
                        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        .and_then(|t| t.get_attribute("data-position"))
                        .map(|pos| match pos.as_str() {
                            "top" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [true, false, false, false],
                            ),
                            "left" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [false, true, false, false],
                            ),
                            "bottom" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [false, false, true, false],
                            ),
                            "right" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [false, false, false, true],
                            ),
                            "top_left" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [true, true, false, false],
                            ),
                            "bottom_left" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [false, true, true, false],
                            ),
                            "bottom_right" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [false, false, true, true],
                            ),
                            "top_right" => Msg::GrabModeless(
                                modeless_id,
                                mouse_pos,
                                [true, false, false, true],
                            ),
                            _ => Msg::NoOp,
                        })
                        .unwrap_or(Msg::NoOp)
                }),
                resizers(),
            )],
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}

fn resizers() -> Vec<Html> {
    vec![
        modeless::resizer::top(Attributes::new().string("data-position", "top")),
        modeless::resizer::left(Attributes::new().string("data-position", "left")),
        modeless::resizer::bottom(Attributes::new().string("data-position", "bottom")),
        modeless::resizer::right(Attributes::new().string("data-position", "right")),
        modeless::resizer::top_left(Attributes::new().string("data-position", "top_left")),
        modeless::resizer::bottom_left(Attributes::new().string("data-position", "bottom_left")),
        modeless::resizer::bottom_right(Attributes::new().string("data-position", "bottom_right")),
        modeless::resizer::top_right(Attributes::new().string("data-position", "top_right")),
    ]
}

fn header(
    modeless_id: model::modeless::ModelessId,
    grabbed: Option<model::modeless::ModelessId>,
    attributes: Attributes,
    events: Events,
    header: Html,
) -> Html {
    modeless::header(
        attributes
            .style("display", "grid")
            .style("grid-template-columns", "1fr max-content"),
        events.on_mousedown(move |e| {
            let mouse_pos = [e.page_x() as f64, e.page_y() as f64];
            Msg::GrabModeless(modeless_id, mouse_pos, [true, true, true, true])
        }),
        vec![
            header,
            Html::div(
                Attributes::new(),
                Events::new(),
                vec![btn::close(
                    Attributes::new(),
                    Events::new()
                        .on_click(move |e| {
                            e.stop_propagation();
                            Msg::CloseModeless(modeless_id)
                        })
                        .on_mousedown(|e| {
                            e.stop_propagation();
                            Msg::NoOp
                        }),
                )],
            ),
        ],
    )
}
