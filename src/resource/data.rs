use crate::Promise;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

trait LoadFrom<T> {
    fn load_from(x: T) -> Promise<Data>;
}

pub enum Data {
    Image {
        element: Rc<web_sys::HtmlImageElement>,
        blob: Rc<web_sys::Blob>,
        url: Rc<String>,
    },
}

impl Data {
    pub fn as_image(&self) -> Option<Rc<web_sys::HtmlImageElement>> {
        match self {
            Self::Image { element, .. } => Some(Rc::clone(element)),
        }
    }

    pub fn pack(&self) -> Promise<JsValue> {
        match self {
            Self::Image { blob, .. } => {
                let obj: js_sys::Object = object! {
                    type: blob.type_(),
                    payload: blob.as_ref()
                }
                .into();
                Promise::new(|resolve| {
                    resolve(Some(obj.into()));
                })
            }
        }
    }

    pub fn unpack(val: JsValue) -> Promise<Self> {
        use crate::JsObject;
        let obj = val.dyn_into::<js_sys::Object>().unwrap();
        let obj = obj.dyn_into::<JsObject>().unwrap();
        let blob_type = obj.get("type").unwrap().as_string().unwrap();
        let array_buffer = obj
            .get("payload")
            .unwrap()
            .dyn_into::<js_sys::ArrayBuffer>()
            .ok()
            .unwrap();
        let blob = web_sys::Blob::new_with_buffer_source_sequence_and_options(
            array![&array_buffer].as_ref(),
            web_sys::BlobPropertyBag::new().type_(blob_type.as_str()),
        )
        .unwrap();
        Self::from_blob(blob)
    }

    pub fn is_able_to_load(file_type: &str) -> bool {
        let file_type: Vec<&str> = file_type.split('/').collect();
        let prefix = file_type.first().map(|x| x as &str).unwrap_or("");
        prefix == "image"
    }

    pub fn from_blob(blob: web_sys::Blob) -> Promise<Data> {
        Promise::new(move |resolve| {
            let blob = Rc::new(blob);
            let blob_type = blob.type_();
            let blob_type: Vec<&str> = blob_type.split('/').collect();
            let blob_type = blob_type.first().map(|x| x as &str).unwrap_or("");
            if blob_type == "image" {
                let image = Rc::new(crate::util::html_image_element());
                let object_url =
                    web_sys::Url::create_object_url_with_blob(&blob).unwrap_or("".into());
                let object_url = Rc::new(object_url);
                let a = {
                    let image = Rc::clone(&image);
                    let blob = Rc::clone(&blob);
                    let object_url = Rc::clone(&object_url);
                    Closure::once(Box::new(move || {
                        resolve(Some(Data::Image {
                            element: image,
                            blob: blob,
                            url: object_url,
                        }));
                    }))
                };
                image.set_onload(Some(&a.as_ref().unchecked_ref()));
                image.set_src(&object_url);
                a.forget();
            }
        })
    }
}
