use super::state;
use super::{Msg, State};
use crate::{block, resource::Data, Resource};
use kagura::prelude::*;
use std::collections::VecDeque;

mod character_list;
mod overlaper;

pub fn render(state: &State, world: &block::World) -> Html<Msg> {
    let some_modeless_is_grubbed = state.modeless().some_is_grubbed();

    Html::div(
        Attributes::new()
            .class("cover")
            .style("position", "relative")
            .style("z-index", "0"),
        Events::new()
            .on_mousedown(move |e| {
                if !some_modeless_is_grubbed {
                    Msg::SetLastMouseDownPosition([e.offset_x() as f32, e.offset_y() as f32])
                } else {
                    Msg::NoOp
                }
            })
            .on_mousemove(move |e| {
                if !some_modeless_is_grubbed {
                    if e.buttons() & 1 != 0 {
                        Msg::SetLastMousePosition(false, [e.offset_x() as f32, e.offset_y() as f32])
                    } else {
                        Msg::SetLastMousePosition(true, [e.offset_x() as f32, e.offset_y() as f32])
                    }
                } else {
                    Msg::NoOp
                }
            })
            .on_mouseup(move |e| {
                if !some_modeless_is_grubbed {
                    Msg::SetLastMouseUpPosition([e.offset_x() as f32, e.offset_y() as f32])
                } else {
                    Msg::NoOp
                }
            }),
        vec![
            canvas(),
            speech_bubble(state.speech_bubble(), state.resource()),
            info(state.table().info()),
            hint(),
            character_list::render(state.block_field(), world.characters(), state.resource()),
            overlaper::render(
                state.block_field(),
                state.table(),
                world,
                state.resource(),
                state.chat(),
                state.personal_data(),
                state.modeless(),
            ),
        ],
    )
}

fn canvas() -> Html<Msg> {
    Html::canvas(
        Attributes::new().id("table").class("cover cover-a"),
        Events::new(),
        vec![],
    )
}

fn speech_bubble(
    speech_bubble: &VecDeque<state::speech_bubble::Item>,
    resource: &Resource,
) -> Html<Msg> {
    Html::div(
        Attributes::new().class("cover").class("cover-a"),
        Events::new(),
        speech_bubble
            .iter()
            .map(|speech_bubble| {
                Html::div(
                    Attributes::new()
                        .class("speechbubble")
                        .style("position", "absolute")
                        .style("left", format!("{}px", speech_bubble.position()[0]))
                        .style("top", format!("{}px", speech_bubble.position()[1])),
                    Events::new(),
                    vec![
                        speech_bubble
                            .texture_id()
                            .and_then(|texture_id| {
                                if let Some(Data::Image { url, .. }) = resource.get(texture_id) {
                                    Some(url)
                                } else {
                                    None
                                }
                            })
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
                            vec![Html::text(speech_bubble.message())],
                        ),
                    ],
                )
            })
            .collect(),
    )
}

fn hint() -> Html<Msg> {
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

fn info(info: &Vec<(String, String)>) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("keyvalue")
            .style("grid-template-columns", "max-content max-content")
            .style("position", "absolute")
            .style("top", "5em")
            .style("right", "5em"),
        Events::new(),
        info.iter()
            .map(|(key, val)| vec![Html::text(key), Html::text(val)])
            .flatten()
            .collect(),
    )
}
