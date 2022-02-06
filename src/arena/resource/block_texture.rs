uses! {}

use super::util::Pack;
use super::BlockMut;
use super::BlockRef;
use super::ImageData;
use super::LoadFrom;
use js_sys::Promise;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

block! {
    [pub BlockTexture(constructor)]
    (data): ImageData;
}

impl BlockTexture {
    pub fn data(&self) -> &ImageData {
        &self.data
    }
}

#[async_trait(?Send)]
impl LoadFrom<BlockRef<ImageData>> for BlockTexture {
    async fn load_from(img: BlockRef<ImageData>) -> Option<Self> {
        if let Some(this) = img.map(|data| Self { data: data.clone() }) {
            Some(this)
        } else {
            Self::load_from((
                [1024, 1024],
                [
                    BlockRef::none(),
                    BlockRef::none(),
                    BlockRef::none(),
                    BlockRef::none(),
                    BlockRef::none(),
                    BlockRef::none(),
                ],
            ))
            .await
        }
    }
}

#[async_trait(?Send)]
impl LoadFrom<([u32; 2], [BlockRef<ImageData>; 6])> for BlockTexture {
    async fn load_from(
        ([width, height], [img_px, img_py, img_pz, img_nx, img_ny, img_nz]): (
            [u32; 2],
            [BlockRef<ImageData>; 6],
        ),
    ) -> Option<Self> {
        let width_f = width as f64;
        let height_f = height as f64;

        let window = unwrap!(web_sys::window(); None);
        let document = unwrap!(window.document(); None);
        let element = unwrap!(document.create_element("canvas").ok(); None);
        let element = unwrap!(element.dyn_into::<web_sys::HtmlCanvasElement>().ok(); None);
        element.set_width(width);
        element.set_height(height);

        let context = unwrap!(element.get_context("2d").ok().unwrap_or(None); None);
        let context = unwrap!(context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok(); None);

        draw_texture(&context, width_f, height_f, &img_pz, 0.25 * 0.0, 0.0);
        draw_texture(&context, width_f, height_f, &img_ny, 0.25 * 0.0, 0.35);
        draw_texture(&context, width_f, height_f, &img_px, 0.25 * 1.0, 0.35);
        draw_texture(&context, width_f, height_f, &img_py, 0.25 * 2.0, 0.35);
        draw_texture(&context, width_f, height_f, &img_nx, 0.25 * 3.0, 0.35);
        draw_texture(&context, width_f, height_f, &img_nz, 0.25 * 0.0, 0.7);

        let blob = JsFuture::from(Promise::new(&mut move |resolve, _| {
            let a = Closure::once(Box::new(move |blob| {
                let _ = resolve.call1(&js_sys::global(), &blob);
            }) as Box<dyn FnOnce(JsValue)>);
            let _ = element.to_blob(a.as_ref().unchecked_ref());
            a.forget();
        }))
        .await
        .ok()
        .and_then(|x| x.dyn_into::<web_sys::Blob>().ok());

        let blob = unwrap!(blob; None);

        let data = ImageData::load_from(blob).await;
        let data = unwrap!(data; None);

        Some(Self { data })
    }
}

fn draw_texture(
    context: &web_sys::CanvasRenderingContext2d,
    canvas_width: f64,
    canvas_height: f64,
    img: &BlockRef<ImageData>,
    x_offset: f64,
    y_offset: f64,
) {
    img.map(|img| {
        let _ = context.draw_image_with_html_image_element_and_dw_and_dh(
            img.element(),
            canvas_width * x_offset,
            canvas_height * y_offset,
            canvas_width * 0.25,
            canvas_height * 0.3,
        );
    });
}

#[async_trait(?Send)]
impl Pack for BlockTexture {
    async fn pack(&self, is_deep: bool) -> JsValue {
        (object! {
            "data": self.data.pack(is_deep).await
        })
        .into()
    }
}
