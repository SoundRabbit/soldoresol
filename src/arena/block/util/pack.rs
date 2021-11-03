use crate::libs::random_id::U128Id;
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

#[async_trait(?Send)]
pub trait Pack {
    async fn pack(&self) -> JsValue;
}

#[async_trait(?Send)]
impl Pack for U128Id {
    async fn pack(&self) -> JsValue {
        self.to_jsvalue()
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Vec<T> {
    async fn pack(&self) -> JsValue {
        let list = js_sys::Array::new();

        for item in self {
            list.push(&item.pack().await);
        }

        list.into()
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Rc<RefCell<T>> {
    async fn pack(&self) -> JsValue {
        self.borrow().pack().await
    }
}
