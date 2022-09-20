use super::util::{Pack, PackDepth};
use super::ArenaMut;
use async_trait::async_trait;
use wasm_bindgen::{prelude::*, JsCast};

mods! {
    pub image_data::ImageData;
    pub block_texture::BlockTexture;
}

#[derive(Clone)]
pub enum Url {
    Local(String),
    Global(String),
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(url) => write!(f, "{}", url),
            Self::Global(url) => write!(f, "{}", url),
        }
    }
}

impl std::ops::Deref for Url {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Local(url) => url,
            Self::Global(url) => url,
        }
    }
}

impl Into<String> for Url {
    fn into(self) -> String {
        match self {
            Self::Local(url) => url,
            Self::Global(url) => url,
        }
    }
}

#[async_trait(?Send)]
impl Pack for Url {
    async fn pack(&self, _: PackDepth) -> JsValue {
        match self {
            Self::Local(url) => (object! {
                "_tag": "Local",
                "_payload": ""
            })
            .into(),
            Self::Global(url) => (object! {
                "_tag": "Global",
                "_payload": url.as_str()
            })
            .into(),
        }
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = unwrap!(data.dyn_ref::<crate::libs::js_object::Object>(); None);
        let tag = unwrap!(data.get("_tag").and_then(|x| x.as_string()); None);
        let payload = unwrap!(data.get("_payload").and_then(|x| x.as_string()); None);

        match tag.as_str() {
            "Global" => Some(Box::new(Self::Global(payload))),
            _ => None,
        }
    }
}

#[async_trait(?Send)]
pub trait LoadFrom<T>: Sized {
    async fn load_from(x: T) -> Option<Self>;
}
