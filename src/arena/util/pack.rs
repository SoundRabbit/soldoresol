use crate::libs::random_id::U128Id;
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

#[async_trait(?Send)]
pub trait Pack {
    async fn pack(&self, is_deep: bool) -> JsValue;
}

#[async_trait(?Send)]
impl Pack for U128Id {
    async fn pack(&self, _: bool) -> JsValue {
        self.to_jsvalue()
    }
}

#[async_trait(?Send)]
impl Pack for bool {
    async fn pack(&self, _: bool) -> JsValue {
        JsValue::from(*self)
    }
}

#[async_trait(?Send)]
impl Pack for String {
    async fn pack(&self, _: bool) -> JsValue {
        JsValue::from(self)
    }
}

#[async_trait(?Send)]
impl Pack for f64 {
    async fn pack(&self, _: bool) -> JsValue {
        JsValue::from(*self)
    }
}

#[async_trait(?Send)]
impl Pack for usize {
    async fn pack(&self, _: bool) -> JsValue {
        JsValue::from(*self as f64)
    }
}

#[async_trait(?Send)]
impl Pack for [f64; 2] {
    async fn pack(&self, _: bool) -> JsValue {
        array![self[0], self[1]].into()
    }
}

#[async_trait(?Send)]
impl Pack for [f64; 3] {
    async fn pack(&self, _: bool) -> JsValue {
        array![self[0], self[1], self[2]].into()
    }
}

#[async_trait(?Send)]
impl Pack for [i32; 3] {
    async fn pack(&self, _: bool) -> JsValue {
        array![self[0], self[1], self[2]].into()
    }
}

#[async_trait(?Send)]
impl Pack for chrono::DateTime<chrono::Utc> {
    async fn pack(&self, _: bool) -> JsValue {
        JsValue::from(self.to_rfc3339())
    }
}

#[async_trait(?Send)]
impl Pack for crate::libs::color::Pallet {
    async fn pack(&self, _: bool) -> JsValue {
        self.to_jsvalue()
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Vec<T> {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let list = js_sys::Array::new();

        for item in self {
            list.push(&item.pack(is_deep).await);
        }

        list.into()
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Rc<RefCell<T>> {
    async fn pack(&self, is_deep: bool) -> JsValue {
        self.borrow().pack(is_deep).await
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Option<T> {
    async fn pack(&self, is_deep: bool) -> JsValue {
        match self {
            Some(x) => x.pack(is_deep).await,
            None => JsValue::null(),
        }
    }
}

#[async_trait(?Send)]
impl<T: Pack, U: Pack> Pack for (T, U) {
    async fn pack(&self, is_deep: bool) -> JsValue {
        array![self.0.pack(is_deep).await, self.1.pack(is_deep).await].into()
    }
}

#[async_trait(?Send)]
impl Pack for regex::Regex {
    async fn pack(&self, _: bool) -> JsValue {
        JsValue::from(self.as_str())
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for crate::libs::select_list::SelectList<T> {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let data = js_sys::Array::new();

        for item in self.iter() {
            data.push(&item.pack(is_deep).await);
        }

        (object! {
            "selected": self.selected_idx(),
            "data": data
        })
        .into()
    }
}

#[async_trait(?Send)]
impl<K: Pack, T: Pack> Pack for std::collections::HashMap<K, T> {
    async fn pack(&self, is_deep: bool) -> JsValue {
        let data = js_sys::Array::new();

        for (key, value) in self {
            let key = key.pack(is_deep).await;
            let value = value.pack(is_deep).await;
            data.push(array![&key, &value].as_ref());
        }

        data.into()
    }
}
