use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::color::Color;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Texture {
    element: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
    size: [f64; 2],
    buffer_size: [f64; 2],
    pixel_ratio: [f64; 2],
}

impl Texture {
    fn create_context(element: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        element
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
        let element = Rc::new(element);
        element.set_width(buffer_size[0]);
        element.set_height(buffer_size[1]);
        let context = Self::create_context(&element);
        let context = Rc::new(context);

        let mut this = Self {
            element,
            context,
            pixel_ratio: [1.0, 1.0],
            size: [1.0, 1.0],
            buffer_size: [buffer_size[0] as f64, buffer_size[1] as f64],
        };

        this.set_size(size);

        this
    }

    pub fn clone(this: &Self) -> Self {
        Self {
            element: Rc::clone(&this.element),
            context: Rc::clone(&this.context),
            size: this.size.clone(),
            buffer_size: this.buffer_size.clone(),
            pixel_ratio: this.pixel_ratio.clone(),
        }
    }

    pub fn element(&self) -> &web_sys::HtmlCanvasElement {
        &self.element
    }

    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }

    pub fn buffer_size(&self) -> &[f64; 2] {
        &self.buffer_size
    }

    pub fn set_size(&mut self, size: [f64; 2]) {
        let size = [size[0].max(1.0), size[1].max(1.0)];

        let new_pixel_ratio = [self.buffer_size[0] / size[0], self.buffer_size[1] / size[1]];

        let _ = self.context.scale(
            new_pixel_ratio[0] / self.pixel_ratio[0],
            new_pixel_ratio[1] / self.pixel_ratio[1],
        );

        self.pixel_ratio = new_pixel_ratio;
        self.size = size;
    }

    pub fn texture_position(&self, p: &[f64; 2]) -> [f64; 2] {
        [(p[0] + self.size[0] / 2.0), -(p[1] - self.size[1] / 2.0)]
    }

    pub async fn pack(&self) -> JsValue {
        (object! {}).into()
    }
}
