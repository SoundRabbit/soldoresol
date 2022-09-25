use super::super::ArenaMut;
#[allow(unused_imports)]
use super::util::prelude::*;
use super::util::{Pack, PackDepth};
use super::LoadFrom;
use super::Url;
use js_sys::Promise;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

block! {
    [pub ImageData(constructor)]
    (element): Rc<web_sys::HtmlImageElement>;
    (blob): Rc<web_sys::Blob>;
    (url): Url;
    (size): [f64; 2];
    (name): String;
}

impl ImageData {
    pub fn element(&self) -> &Rc<web_sys::HtmlImageElement> {
        &self.element
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn size(&self) -> &[f64; 2] {
        &self.size
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Clone for ImageData {
    fn clone(&self) -> Self {
        Self {
            element: Rc::clone(&self.element),
            blob: Rc::clone(&self.blob),
            url: self.url.clone(),
            size: self.size.clone(),
            name: self.name.clone(),
        }
    }
}

#[async_trait(?Send)]
impl LoadFrom<web_sys::File> for ImageData {
    async fn load_from(file: web_sys::File) -> Option<Self> {
        let name = file.name();
        let blob: web_sys::Blob = file.into();
        let mut this = unwrap!(Self::load_from(blob).await; None);
        this.name = name;
        Some(this)
    }
}

#[async_trait(?Send)]
impl LoadFrom<Rc<web_sys::File>> for ImageData {
    async fn load_from(file: Rc<web_sys::File>) -> Option<Self> {
        let file = file.as_ref().clone();
        Self::load_from(file).await
    }
}

#[async_trait(?Send)]
impl LoadFrom<(String, JsValue)> for ImageData {
    async fn load_from((type_, data): (String, JsValue)) -> Option<Self> {
        let blob = unwrap!(web_sys::Blob::new_with_u8_array_sequence_and_options(
            array![&data].as_ref(),
            web_sys::BlobPropertyBag::new().type_(type_.as_str())
        ).ok(); None);

        Self::load_from(blob).await
    }
}

#[async_trait(?Send)]
impl LoadFrom<web_sys::Blob> for ImageData {
    async fn load_from(blob: web_sys::Blob) -> Option<Self> {
        Self::load_from(Rc::new(blob)).await
    }
}

#[async_trait(?Send)]
impl LoadFrom<Rc<web_sys::Blob>> for ImageData {
    async fn load_from(blob: Rc<web_sys::Blob>) -> Option<Self> {
        if regex::Regex::new(r"^image/.*")
            .unwrap()
            .is_match(&blob.type_())
        {
            let image = Rc::new(crate::libs::element::html_image_element());
            let object_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap_or("".into());
            let _ = JsFuture::from(Promise::new({
                let image = Rc::clone(&image);
                let object_url = object_url.clone();
                &mut move |resolve, _| {
                    let a = Closure::wrap(Box::new(move || {
                        let _ = resolve.call1(&js_sys::global(), &JsValue::null());
                    }) as Box<dyn FnMut()>);
                    image.set_onload(Some(&a.as_ref().unchecked_ref()));
                    image.set_src(&object_url);
                    a.forget();
                }
            }))
            .await;

            let width = image.width() as f64;
            let height = image.height() as f64;

            Some(Self::new(
                image,
                blob,
                Url::Local(object_url),
                [width, height],
                String::from(""),
            ))
        } else {
            None
        }
    }
}

#[async_trait(?Send)]
impl LoadFrom<Url> for ImageData {
    async fn load_from(url: Url) -> Option<Self> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("GET");
        opts.mode(web_sys::RequestMode::Cors);

        let request = unwrap!(web_sys::Request::new_with_str_and_init(&url, &opts).ok(); None);

        let response = unwrap!(JsFuture::from(
            web_sys::window().unwrap().fetch_with_request(&request)
        )
        .await
        .ok(); None);
        let response = unwrap!(response.dyn_into::<web_sys::Response>().ok(); None);
        let blob = unwrap!(response.blob().ok(); None);
        let blob = unwrap!(JsFuture::from(blob).await.ok(); None);
        let blob = unwrap!(blob.dyn_into::<web_sys::Blob>().ok(); None);

        let mut this = unwrap!(Self::load_from(blob).await; None);
        this.url = url;

        Some(this)
    }
}

#[async_trait(?Send)]
impl Pack for ImageData {
    async fn pack(&self, _: PackDepth) -> JsValue {
        (object! {
            "type": self.blob.type_().as_str(),
            "data": self.blob.as_ref(),
            "name": self.name.as_str()
        })
        .into()
    }

    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        let data = unwrap!(data.dyn_ref::<crate::libs::js_object::Object>(); None);
        let array_buffer = unwrap!(data.get("data"); None);
        let blob_type = unwrap!(data.get("type").and_then(|x| x.as_string()); None);
        let name = unwrap!(data.get("name").and_then(|x| x.as_string()); None);

        Self::load_from((blob_type, array_buffer.into()))
            .await
            .map(|mut x| {
                x.name = name;
                Box::new(x)
            })
    }
}
