use wasm_bindgen::{prelude::*, JsCast};

pub fn html_image_element() -> web_sys::HtmlImageElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap()
}

pub fn html_canvas_element() -> web_sys::HtmlCanvasElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

pub fn canvas_from_image(img: &web_sys::HtmlImageElement) -> web_sys::HtmlCanvasElement {
    let canvas = html_canvas_element();

    canvas.set_width(img.width());
    canvas.set_height(img.height());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.draw_image_with_html_image_element(img, 0.0, 0.0);

    canvas
}
