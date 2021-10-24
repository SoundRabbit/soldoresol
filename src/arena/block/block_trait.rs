pub trait DisplayNamed {
    fn display_name(&self) -> &String;
    fn set_display_name(&mut self, name: String);
}

use super::ArenaRef;
use async_trait::async_trait;
use wasm_bindgen::JsValue;

#[async_trait(?Send)]
pub trait Pack {
    async fn pack(&self) -> JsValue;
}

#[async_trait(?Send)]
pub trait Unpack: Sized {
    async fn unpack(packed: JsValue) -> Option<Self>;
}
