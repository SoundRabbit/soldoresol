uses! {
    super::LoadFrom;
    super::Url;
    super::util::Pack;
    js_sys::Promise;
    wasm_bindgen::JsCast;
    wasm_bindgen_futures::JsFuture;
}

block! {
    [pub ImageData(constructor)]
    (element): Rc<web_sys::HtmlImageElement>;
    (blob): Rc<web_sys::Blob>;
    (url): Url;
    (size): [f64; 2];
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

        let request = unwrap!(web_sys::Request::new_with_str_and_init(&url, &opts).ok());

        let response = unwrap!(JsFuture::from(
            web_sys::window().unwrap().fetch_with_request(&request)
        )
        .await
        .ok());
        let response = unwrap!(response.dyn_into::<web_sys::Response>().ok());
        let blob = unwrap!(response.blob().ok());
        let blob = unwrap!(JsFuture::from(blob).await.ok());
        let blob = unwrap!(blob.dyn_into::<web_sys::Blob>().ok());

        let mut this = unwrap!(Self::load_from(blob).await);
        this.url = url;

        Some(this)
    }
}

#[async_trait(?Send)]
impl Pack for ImageData {
    async fn pack(&self, _: bool) -> JsValue {
        (object! {
            "type": self.blob.type_(),
            "data": self.blob.as_ref()
        })
        .into()
    }
}
