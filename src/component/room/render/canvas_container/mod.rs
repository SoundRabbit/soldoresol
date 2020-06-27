use super::{Msg, State};
use crate::block;
use kagura::prelude::*;

mod character_list;
mod overlaper;

pub fn render(state: &State, world: &block::World) -> Html<Msg> {
    Html::div(
        Attributes::new()
            .class("cover")
            .style("position", "relative")
            .style("z-index", "0"),
        Events::new()
            .on_mousedown(|e| {
                Msg::SetLastMouseDownPosition([e.offset_x() as f32, e.offset_y() as f32])
            })
            .on_mousemove(|e| Msg::SetLastMousePosition([e.offset_x() as f32, e.offset_y() as f32]))
            .on_mouseup(|e| {
                Msg::SetLastMouseUpPosition([e.offset_x() as f32, e.offset_y() as f32])
            }),
        vec![
            render_canvas(),
            render_speech_bubble_queue(&state.speech_bubble_queue, &state.resource),
            render_measure_length(&state.table_state.measure_length),
            render_hint(),
            character_list::render(world.characters(), state.resource()),
            overlaper::render(
                state.block_field(),
                state.table(),
                world,
                state.resource(),
                state.chat(),
                state.personal_data(),
                state.modeless(),
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
