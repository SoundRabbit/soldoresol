use crate::random_id;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct State {
    attributes: Attributes,
    id: Rc<String>,
}

pub enum Msg {}

pub enum Sub {}

pub fn new(
    resource_id: u128,
    img: Rc<web_sys::HtmlImageElement>,
    attributes: Attributes,
) -> Component<Msg, State, Sub> {
    let id = Rc::new(random_id::hex(8));
    let init = {
        let id = Rc::clone(&id);
        move || {
            let state = State {
                attributes,
                id: Rc::clone(&id),
            };
            let task = Cmd::task({
                let id = Rc::clone(&id);
                move |_| {
                    if let Some(el) = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id(&id)
                        .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
                    {
                        if el
                            .get_attribute("data-r_id")
                            .and_then(|r_id| r_id.parse::<u128>().ok())
                            .map(|r_id| r_id != resource_id)
                            .unwrap_or(true)
                        {
                            let _ = el.set_attribute("data-r_id", &resource_id.to_string());

                            let el_width = el.client_width() as f64;
                            let el_height = el.client_height() as f64;

                            let img_width = img.width() as f64;
                            let img_height = img.height() as f64;

                            let pixel_ratio =
                                (el_width / img_width).max(el_height / img_height).min(1.0);

                            let canvas_width = img_width * pixel_ratio;
                            let canvas_height = img_height * pixel_ratio;

                            el.set_width(canvas_width as u32);
                            el.set_height(canvas_height as u32);

                            el.get_context("2d")
                                .ok()
                                .and_then(|context| context)
                                .and_then(|context| {
                                    context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()
                                })
                                .map(|context| {
                                    context.clear_rect(0.0, 0.0, canvas_width, canvas_height);
                                    context.draw_image_with_html_image_element_and_dw_and_dh(
                                        &img,
                                        0.0,
                                        0.0,
                                        canvas_width,
                                        canvas_height,
                                    )
                                });
                        }
                    }
                }
            });
            (state, task)
        }
    };
    Component::new(init, update, render)
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State) -> Html<Msg> {
    Html::canvas(
        state.attributes.clone().id(state.id.as_str()),
        Events::new(),
        vec![],
    )
}
