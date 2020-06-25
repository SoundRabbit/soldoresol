use super::Block;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub struct Texture {
    element: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Texture {
    fn get_context2d_from_canvas(
        canvas: &web_sys::HtmlCanvasElement,
    ) -> web_sys::CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }

    pub fn new(size: &[u32; 2]) -> Self {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        element.set_width(size[0]);
        element.set_height(size[1]);
        let context = Self::get_context2d_from_canvas(&element);
        Self { element, context }
    }

    pub fn reset_context(&mut self) {
        self.context = Self::get_context2d_from_canvas(&self.element);
    }

    pub fn set_size(&mut self, size: &[u32; 2]) {
        self.element.set_width(size[0]);
        self.element.set_height(size[1]);
        self.reset_context();
    }

    pub fn element(&self) -> &web_sys::HtmlCanvasElement {
        &self.element
    }

    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }
}

impl Block for Texture {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {
        let resolve = RefCell::new(Some(Box::new(resolve)));
        let a = Closure::wrap(Box::new(move |blob: JsValue| {
            if let Some(resolve) = resolve.borrow_mut().take() {
                resolve(blob.into());
            }
        }) as Box<dyn FnMut(JsValue)>);
        let _ = self.element.to_blob(a.as_ref().unchecked_ref());
        a.forget();
    }

    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {
        let buffer = val
            .dyn_into::<js_sys::ArrayBuffer>()
            .ok()
            .and_then(|buffer| {
                web_sys::Blob::new_with_buffer_source_sequence_and_options(
                    array![&buffer].as_ref(),
                    web_sys::BlobPropertyBag::new().type_("image/png"),
                )
                .ok()
            });
        if let Some(blob) = buffer {
            let image = Rc::new(crate::util::html_image_element());
            let a = {
                let image = Rc::clone(&image);
                Closure::once(Box::new(move || {
                    let me = Self::new(&[image.width(), image.height()]);
                    let _ = me
                        .context()
                        .draw_image_with_html_image_element(&image, 0.0, 0.0);
                    resolve(Some(Box::new(me)));
                }))
            };
            image.set_onload(Some(&a.as_ref().unchecked_ref()));
            if let Ok(object_url) = web_sys::Url::create_object_url_with_blob(&blob) {
                image.set_src(&object_url);
            }
            a.forget();
        } else {
            resolve(None);
        }
    }
}
