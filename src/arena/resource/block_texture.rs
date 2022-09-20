use super::super::ArenaMut;
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
use super::BlockRef;
use super::ImageData;
use super::LoadFrom;
use js_sys::Promise;
use wasm_bindgen::JsCast;
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
            Self::load_from([
                BlockRef::none(),
                BlockRef::none(),
                BlockRef::none(),
                BlockRef::none(),
                BlockRef::none(),
                BlockRef::none(),
            ])
            .await
        }
    }
}

#[async_trait(?Send)]
impl LoadFrom<[BlockRef<ImageData>; 6]> for BlockTexture {
    async fn load_from(
        [img_px, img_py, img_pz, img_nx, img_ny, img_nz]: [BlockRef<ImageData>; 6],
    ) -> Option<Self> {
        let px_size = img_size(&img_px);
        let py_size = img_size(&img_py);
        let pz_size = img_size(&img_pz);
        let nx_size = img_size(&img_nx);
        let ny_size = img_size(&img_ny);
        let nz_size = img_size(&img_nz);

        let cw = px_size[0].max(py_size[0]).max(nx_size[0]).max(ny_size[0]) * 4.0;
        let cw = cw.max(pz_size[0]).max(nz_size[0]).max(16.0);

        let ch = px_size[1].max(py_size[1]).max(nx_size[1]).max(ny_size[1]) * 2.0;
        let ch = ch.max(pz_size[1] * 4.0).max(nz_size[1] * 4.0).max(16.0);

        let width = cw as u32;
        let height = ch as u32;

        let window = unwrap!(web_sys::window(); None);
        let document = unwrap!(window.document(); None);
        let element = unwrap!(document.create_element("canvas").ok(); None);
        let element = unwrap!(element.dyn_into::<web_sys::HtmlCanvasElement>().ok(); None);
        element.set_width(width);
        element.set_height(height);

        let context = unwrap!(element.get_context("2d").ok().unwrap_or(None); None);
        let context = unwrap!(context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok(); None);
        let ctx = context;

        //テクスチャ座標用のメモ
        //[0.00,0.00][0.25,0.00][0.50,0.00][0.75,0.00][1.00,0.00]
        //[0.00,0.25][0.25,0.25][0.50,0.25][0.75,0.25][1.00,0.25]
        //[0.00,0.75][0.25,0.75][0.50,0.75][0.75,0.75][1.00,0.75]
        //[0.00,1.00][0.25,1.00][0.50,1.00][0.75,1.00][1.00,1.00]

        //PZ PZ PZ PZ
        //NY PX PY NX
        //NZ NZ NZ NZ

        draw_texture(&ctx, cw, ch, &img_pz, &[0.00, 0.00], &[1.00, 0.25]);
        draw_texture(&ctx, cw, ch, &img_ny, &[0.00, 0.25], &[0.25, 0.50]);
        draw_texture(&ctx, cw, ch, &img_px, &[0.25, 0.25], &[0.25, 0.50]);
        draw_texture(&ctx, cw, ch, &img_py, &[0.50, 0.25], &[0.25, 0.50]);
        draw_texture(&ctx, cw, ch, &img_nx, &[0.75, 0.25], &[0.25, 0.50]);
        draw_texture(&ctx, cw, ch, &img_nz, &[0.00, 0.75], &[1.00, 0.25]);

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
    p: &[f64; 2],
    s: &[f64; 2],
) {
    img.map(|img| {
        let _ = context.draw_image_with_html_image_element_and_dw_and_dh(
            img.element(),
            canvas_width * p[0],
            canvas_height * p[1],
            canvas_width * s[0],
            canvas_height * s[1],
        );
    });
}

fn img_size(img: &BlockRef<ImageData>) -> [f64; 2] {
    img.map(|img| img.size().clone()).unwrap_or([0.0, 0.0])
}

#[async_trait(?Send)]
impl Pack for BlockTexture {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        (object! {
            "data": self.data.pack(pack_depth).await
        })
        .into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = unwrap!(data.dyn_ref::<crate::libs::js_object::Object>(); None);
        let data = unwrap!(data.get("data"); None);
        let data =
            unwrap!(BlockRef::<ImageData>::unpack(&data, ArenaMut::clone(&arena)).await; None);
        let this = unwrap!(Self::load_from(*data).await; None);

        Some(Box::new(this))
    }
}
