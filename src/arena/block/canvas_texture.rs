uses! {}

use super::util::Pack;
use super::BlockMut;
use js_sys::Promise;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

block! {
    [pub CanvasTexture()]
    (element): Rc<web_sys::HtmlCanvasElement>;
    (context): Rc<web_sys::CanvasRenderingContext2d>;
    (size): [f64; 2];
    (buffer_size): [f64; 2];
    (pixel_ratio): [f64; 2];
    (is_mask): bool;
}

impl CanvasTexture {
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
            is_mask: false,
        };

        this.size_set(size);

        this
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

    pub fn size_set(&mut self, size: [f64; 2]) {
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

    pub fn is_mask(&self) -> bool {
        self.is_mask
    }

    pub fn is_mask_set(&mut self, is_mask: bool) {
        self.is_mask = is_mask;
    }
}

#[async_trait(?Send)]
impl Pack for CanvasTexture {
    async fn pack(&self, _is_deep: bool) -> JsValue {
        let element = JsFuture::from(Promise::new(&mut move |resolve, _| {
            let a = Closure::once(Box::new(move |blob| {
                let _ = resolve.call1(&js_sys::global(), &blob);
            }) as Box<dyn FnOnce(JsValue)>);
            let _ = self.element.to_blob(a.as_ref().unchecked_ref());
            a.forget();
        }))
        .await
        .ok()
        .and_then(|x| x.dyn_into::<web_sys::Blob>().ok());
        let element = unwrap_or!(element; JsValue::NULL);

        (object! {
            "element": element,
            "buffer_size": array![self.buffer_size[0], self.buffer_size[1]],
            "size": array![self.size[0], self.size[1]],
            "is_mask": self.is_mask
        })
        .into()
    }
}
