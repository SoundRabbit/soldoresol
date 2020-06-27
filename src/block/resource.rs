use super::{Block, Field};
use crate::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum Resource {
    Image(Rc<web_sys::HtmlImageElement>, Rc<web_sys::Blob>, Rc<String>),
}

impl Block for Resource {
    fn pack(&self) -> Promise<JsValue, ()> {
        unimplemented!();
    }
    fn unpack(field: &Field, val: JsValue) -> Promise<Box<Self>, ()> {
        unimplemented!();
    }
}
