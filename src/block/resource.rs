use super::{Block, Field};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum Resource {
    Image(Rc<web_sys::HtmlImageElement>, Rc<web_sys::Blob>, Rc<String>),
}

impl Block for Resource {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {}
    fn unpack(field: &Field, val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {}
}
