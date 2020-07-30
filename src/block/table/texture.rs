use super::{Block, Field};
use crate::Promise;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone)]
pub struct Texture {
    element: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    size: [f64; 2],
    buffer_size: [f64; 2],
    pixel_ratio: [f64; 2],
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

    pub fn new(buffer_size: &[u32; 2], size: [f64; 2]) -> Self {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        element.set_width(buffer_size[0]);
        element.set_height(buffer_size[1]);
        let context = Self::get_context2d_from_canvas(&element);

        let mut me = Self {
            element,
            context,
            pixel_ratio: [1.0, 1.0],
            size: [1.0, 1.0],
            buffer_size: [buffer_size[0] as f64, buffer_size[1] as f64],
        };
        me.set_size(size);
        me
    }

    pub fn texture_position(&self, p: &[f64; 2]) -> [f64; 2] {
        [(p[0] + self.size[0] / 2.0), -(p[1] - self.size[1] / 2.0)]
    }

    pub fn element(&self) -> &web_sys::HtmlCanvasElement {
        &self.element
    }

    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        let new_pixel_ratio = [self.buffer_size[0] / size[0], self.buffer_size[1] / size[1]];

        let _ = self.context.scale(
            new_pixel_ratio[0] / self.pixel_ratio[0],
            new_pixel_ratio[1] / self.pixel_ratio[1],
        );

        self.pixel_ratio = new_pixel_ratio;
        self.size = size;
    }

    pub fn clear(&self) {
        let [px, py] = self.pixel_ratio.clone();
        self.context
            .clear_rect(0.0, 0.0, self.buffer_size[0] * px, self.buffer_size[1] * py);
    }
}

impl Block for Texture {
    fn pack(&self) -> Promise<JsValue> {
        let w = self.size[0];
        let h = self.size[1];
        let element = self.element.clone();
        let element = element;
        Promise::new(move |resolve| {
            let resolve = RefCell::new(Some(resolve));
            let a = Closure::once(Box::new(move |blob| {
                let obj = object! {
                    buffer: blob,
                    size: array![w, h]
                };
                let obj: js_sys::Object = obj.into();
                let obj: JsValue = obj.into();
                if let Some(resolve) = resolve.borrow_mut().take() {
                    resolve(Some(obj));
                }
            }) as Box<dyn FnOnce(web_sys::Blob)>);
            element.to_blob(a.as_ref().unchecked_ref()).unwrap();
            a.forget();
        })
    }

    fn unpack(_: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        use crate::JsObject;

        let val = val.dyn_into::<JsObject>().unwrap();
        let buffer = val.get("buffer").unwrap();
        let buffer = if let Some(buffer) = buffer.dyn_ref::<js_sys::ArrayBuffer>() {
            web_sys::Blob::new_with_buffer_source_sequence_and_options(
                array![buffer].as_ref(),
                web_sys::BlobPropertyBag::new().type_("image/png"),
            )
            .ok()
        } else if let Ok(buffer) = buffer.dyn_into::<web_sys::Blob>() {
            Some(buffer)
        } else {
            None
        };
        let size = js_sys::Array::from(&val.get("size").unwrap()).to_vec();
        let size = [size[0].as_f64().unwrap(), size[1].as_f64().unwrap()];

        if let Some(blob) = buffer {
            Promise::new(move |resolve| {
                let image = Rc::new(crate::util::html_image_element());
                let a = {
                    let image = Rc::clone(&image);
                    Closure::once(Box::new(move || {
                        let w = image.width();
                        let h = image.height();
                        let me = Self::new(&[w, h], size);
                        let _ = me
                            .context()
                            .draw_image_with_html_image_element_and_dw_and_dh(
                                &image,
                                0.0,
                                0.0,
                                w as f64 / me.pixel_ratio[0],
                                h as f64 / me.pixel_ratio[1],
                            );
                        resolve(Some(Box::new(me)));
                    }))
                };
                image.set_onload(Some(&a.as_ref().unchecked_ref()));
                if let Ok(object_url) = web_sys::Url::create_object_url_with_blob(&blob) {
                    image.set_src(&object_url);
                }
                a.forget();
            })
        } else {
            Promise::new(|resolve| resolve(None))
        }
    }
}
