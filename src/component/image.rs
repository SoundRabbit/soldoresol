use crate::JsObject;
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct State {
    attributes: Attributes,
    resource_id: u128,
}

pub enum Msg {}

pub enum Sub {}

pub fn new(
    resource_id: u128,
    img: Rc<web_sys::HtmlImageElement>,
    attributes: Attributes,
) -> Component<Msg, State, Sub> {
    let init = {
        move || {
            let state = State {
                attributes,
                resource_id: resource_id,
            };
            let task = Cmd::task(move |_| {
                let els = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_class_name(format!("_{}", resource_id.to_string()).as_str());
                for i in 0..els.length() {
                    if let Some(el) = els.get_with_index(i) {
                        if el
                            .get_attribute("data-r_id")
                            .and_then(|r_id| r_id.parse::<u128>().ok())
                            .map(|r_id| r_id != resource_id)
                            .unwrap_or(true)
                        {
                            if let Some(el) = el.dyn_into::<web_sys::HtmlImageElement>().ok() {
                                let _ = el.set_attribute("data-r_id", &resource_id.to_string());

                                if let (Some(el), Some(img)) =
                                    (el.dyn_ref::<JsObject>(), img.dyn_ref::<JsObject>())
                                {
                                    el.set(
                                        "src",
                                        img.get("src")
                                            .as_ref()
                                            .map(|a| a as &JsValue)
                                            .unwrap_or(&JsValue::undefined()),
                                    );
                                }
                            }
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
    Html::img(
        state
            .attributes
            .clone()
            .class(format!("_{}", state.resource_id.to_string())),
        Events::new(),
        vec![],
    )
}
