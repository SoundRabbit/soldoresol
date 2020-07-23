use super::{Msg, State};
use crate::{
    block,
    renderer::{Camera, Renderer},
    resource::Data,
    Resource,
};
use kagura::prelude::*;

mod character_list;
mod overlaper;

pub fn render(z_index: u64, state: &State, world: &block::World) -> Html {
    let some_modeless_is_grubbed = state.modeless().grubbed().is_some();

    Html::div(
        Attributes::new()
            .class("cover")
            .style("position", "relative")
            .style("z-index", z_index.to_string()),
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
            if let Some(selecting_tab) = state.selecting_chat_tab_block() {
                speech_bubble(
                    state.block_field(),
                    state.resource(),
                    selecting_tab,
                    state.camera(),
                    state.canvas_size(),
                    state.pixel_ratio(),
                )
            } else {
                Html::div(Attributes::new(), Events::new(), vec![])
            },
            info(state.table().info()),
            hint(),
            character_list::render(
                state.block_field(),
                world.characters(),
                state.resource(),
                &state.client_id(),
            ),
            overlaper::render(state, state.modeless()),
        ],
    )
}

fn canvas() -> Html {
    Html::canvas(
        Attributes::new().id("table").class("cover cover-a"),
        Events::new(),
        vec![],
    )
}

fn speech_bubble(
    block_field: &block::Field,
    resource: &Resource,
    selecting_tab: &block::chat::Tab,
    camera: &Camera,
    canvas_size: &[f32; 2],
    pixel_ratio: f32,
) -> Html {
    let now = js_sys::Date::now();
    Html::div(
        Attributes::new().class("cover").class("cover-a"),
        Events::new(),
        selecting_tab
            .iter()
            .filter_map(|(time, item_id)| {
                if time > now - 10000.0 {
                    block_field.get::<block::chat::Item>(item_id)
                } else {
                    None
                }
            })
            .filter_map(|item| {
                item.sender()
                    .as_character()
                    .and_then(|c_id| block_field.get::<block::Character>(c_id))
                    .map(|character| (item, character))
            })
            .map(|(item, character)| {
                let pos = character.position();
                let sz = character.size();
                let pos =
                    Renderer::table_position(&[0.0, 0.0, sz[2]], pos, camera, canvas_size, true);
                Html::div(
                    Attributes::new()
                        .class("speechbubble")
                        .style("position", "absolute")
                        .style(
                            "left",
                            format!("{}px", (1.0 + pos[0]) * 0.5 * canvas_size[0] / pixel_ratio),
                        )
                        .style(
                            "top",
                            format!("{}px", (1.0 - pos[1]) * 0.5 * canvas_size[1] / pixel_ratio),
                        ),
                    Events::new(),
                    vec![
                        character
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
                            vec![Html::text(item.text())],
                        ),
                    ],
                )
            })
            .collect(),
    )
}

fn hint() -> Html {
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

fn info(info: &Vec<(String, String)>) -> Html {
    Html::div(
        Attributes::new()
            .class("keyvalue")
            .style("grid-template-columns", "max-content max-content")
            .style("position", "absolute")
            .style("top", "5em")
            .style("right", "5em"),
        Events::new(),
        info.iter()
            .map(|(key, val)| {
                vec![
                    Html::span(Attributes::new(), Events::new(), vec![Html::text(key)]),
                    Html::span(Attributes::new(), Events::new(), vec![Html::text(val)]),
                ]
            })
            .flatten()
            .collect(),
    )
}
