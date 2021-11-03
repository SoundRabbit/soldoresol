use crate::libs::js_object::Object;
use crate::libs::try_ref::TryRef;
use async_trait::async_trait;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

#[async_trait]
trait LoadFrom<T> {
    async fn load_from(x: T) -> Data;
}

pub struct ImageData {
    element: Rc<web_sys::HtmlImageElement>,
    blob: Rc<web_sys::Blob>,
    url: Rc<String>,
    size: [f32; 2],
}

impl ImageData {
    pub fn element(&self) -> Rc<web_sys::HtmlImageElement> {
        Rc::clone(&self.element)
    }

    pub fn url(&self) -> Rc<String> {
        Rc::clone(&self.url)
    }

    pub fn size(&self) -> &[f32; 2] {
        &self.size
    }
}

pub enum Data {
    Image(Rc<ImageData>),
}

impl Data {
    pub async fn pack(&self) -> JsValue {
        match self {
            Self::Image(data) => (object! {
                "type": data.blob.type_(),
                "payload": data.blob.as_ref()
            })
            .into(),
        }
    }

    pub async fn unpack(val: JsValue) -> Option<Self> {
        let obj = val.dyn_into::<js_sys::Object>().unwrap();
        let obj = obj.dyn_into::<Object>().unwrap();
        let blob_type = obj.get("type").unwrap().as_string().unwrap();
        let payload = obj.get("payload").unwrap();
        if let Some(array_buffer) = payload.dyn_ref::<js_sys::ArrayBuffer>() {
            let blob = web_sys::Blob::new_with_buffer_source_sequence_and_options(
                array![array_buffer].as_ref(),
                web_sys::BlobPropertyBag::new().type_(blob_type.as_str()),
            )
            .unwrap();
            Self::from_blob(blob).await
        } else if let Ok(blob) = payload.dyn_into::<web_sys::Blob>() {
            Self::from_blob(blob).await
        } else {
            None
        }
    }

    fn prefix_of<'a>(file_type: &'a str) -> &'a str {
        let file_type: Vec<&str> = file_type.split('/').collect();
        file_type.first().map(|x| x as &str).unwrap_or("")
    }

    pub fn is_able_to_load(file_type: &str) -> bool {
        let prefix = Self::prefix_of(file_type);
        prefix == "image"
    }

    pub async fn from_blob(blob: web_sys::Blob) -> Option<Self> {
        if Self::prefix_of(&blob.type_()) == "image" {
            let image = Rc::new(crate::libs::element::html_image_element());
            let object_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap_or("".into());
            let object_url = Rc::new(object_url);
            let _ = JsFuture::from(Promise::new({
                let image = Rc::clone(&image);
                let object_url = Rc::clone(&object_url);
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

            let width = image.width() as f32;
            let height = image.height() as f32;
            Some(Self::Image(Rc::new(ImageData {
                element: image,
                blob: Rc::new(blob),
                url: object_url,
                size: [width, height],
            })))
        } else {
            None
        }
    }
}

impl TryRef<Rc<ImageData>> for Data {
    fn try_ref(&self) -> Option<&Rc<ImageData>> {
        match self {
            Self::Image(data) => Some(data),
        }
    }
}
