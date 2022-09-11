use super::util::{Pack, PackDepth};
use async_trait::async_trait;
use wasm_bindgen::prelude::*;

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
            Self::Local(url) => JsValue::null(),
            Self::Global(url) => (object! { "Global": url }).into(),
        }
    }
}

#[async_trait(?Send)]
pub trait LoadFrom<T>: Sized {
    async fn load_from(x: T) -> Option<Self>;
}
