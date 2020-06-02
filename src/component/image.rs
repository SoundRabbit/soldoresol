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
                        let width = img.width();
                        let height = img.height();

                        el.set_height(height);
                        el.set_width(width);

                        let width = width as f64;
                        let height = height as f64;

                        el.get_context("2d")
                            .ok()
                            .and_then(|context| context)
                            .and_then(|context| {
                                context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()
                            })
                            .map(|context| {
                                context.clear_rect(0.0, 0.0, width, height);
                                context.draw_image_with_html_image_element_and_dw_and_dh(
                                    &img, 0.0, 0.0, width, height,
                                )
                            });
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
