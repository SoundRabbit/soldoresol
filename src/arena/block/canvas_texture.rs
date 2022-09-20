use super::super::resource::{ImageData, LoadFrom};
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
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
    async fn pack(&self, _: PackDepth) -> JsValue {
        let blob = JsFuture::from(Promise::new(&mut move |resolve, _| {
            let a = Closure::once(Box::new(move |blob| {
                let _ = resolve.call1(&js_sys::global(), &blob);
            }) as Box<dyn FnOnce(JsValue)>);
            let _ = self.element.to_blob(a.as_ref().unchecked_ref());
            a.forget();
        }))
        .await
        .ok()
        .and_then(|x| x.dyn_into::<web_sys::Blob>().ok());
        let blob_type = blob.as_ref().map(|x| x.type_()).unwrap_or_default();
        let blob = unwrap!(blob; JsValue::NULL);

        (object! {
            "data": blob,
            "type": blob_type.as_str(),
            "buffer_size": array![self.buffer_size[0], self.buffer_size[1]],
            "size": array![self.size[0], self.size[1]],
            "is_mask": self.is_mask
        })
        .into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let blob = data.get("data")?;
        let blob_type = data.get("type")?.as_string()?;
        let buffer_size =
            js_sys::Array::from(unwrap!(data.get("buffer_size"); None).as_ref()).to_vec();
        let buffer_size = [
            buffer_size[0].as_f64().unwrap_or_default(),
            buffer_size[1].as_f64().unwrap_or_default(),
        ];
        let size = js_sys::Array::from(unwrap!(data.get("size"); None).as_ref()).to_vec();
        let size = [
            size[0].as_f64().unwrap_or_default(),
            size[1].as_f64().unwrap_or_default(),
        ];
        let is_mask = data.get("is_mask")?.as_bool()?;

        let data = ImageData::load_from((blob_type, blob.into())).await?;

        let this = Self::new(&[buffer_size[0] as u32, buffer_size[1] as u32], size);

        this.context()
            .draw_image_with_html_image_element_and_dw_and_dh(
                data.element(),
                0.0,
                0.0,
                buffer_size[0],
                buffer_size[1],
            );

        Some(Box::new(this))
    }
}
