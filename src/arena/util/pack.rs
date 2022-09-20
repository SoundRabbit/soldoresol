use crate::arena::ArenaMut;
use crate::libs::js_object::Object;
use crate::libs::random_id::U128Id;
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum PackDepth {
    OnlyId,
    Recursive,
    FirstBlock,
}

#[async_trait(?Send)]
pub trait Pack {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue;
    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>>;
}

#[async_trait(?Send)]
impl Pack for U128Id {
    async fn pack(&self, _: PackDepth) -> JsValue {
        self.to_jsvalue()
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        Self::from_jsvalue(data).map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for bool {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(*self)
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        data.as_bool().map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for String {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(self)
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        data.as_string().map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for f64 {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(*self)
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        data.as_f64().map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for usize {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(*self as f64)
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        data.as_f64().map(|x| x as usize).map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for u32 {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(*self as f64)
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        data.as_f64().map(|x| x as u32).map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for [f64; 2] {
    async fn pack(&self, _: PackDepth) -> JsValue {
        array![self[0], self[1]].into()
    }
    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(data).to_vec();
        let data_0 = unwrap!(data.get(0).and_then(|x| x.as_f64()); None);
        let data_1 = unwrap!(data.get(1).and_then(|x| x.as_f64()); None);

        Some(Box::new([data_0, data_1]))
    }
}

#[async_trait(?Send)]
impl Pack for [f64; 3] {
    async fn pack(&self, _: PackDepth) -> JsValue {
        array![self[0], self[1], self[2]].into()
    }

    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(data).to_vec();
        let data_0 = unwrap!(data.get(0).and_then(|x| x.as_f64()); None);
        let data_1 = unwrap!(data.get(1).and_then(|x| x.as_f64()); None);
        let data_2 = unwrap!(data.get(2).and_then(|x| x.as_f64()); None);

        Some(Box::new([data_0, data_1, data_2]))
    }
}

#[async_trait(?Send)]
impl Pack for [i32; 3] {
    async fn pack(&self, _: PackDepth) -> JsValue {
        array![self[0], self[1], self[2]].into()
    }

    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(data).to_vec();
        let data_0 = unwrap!(data.get(0).and_then(|x| x.as_f64()).map(|x| x as i32); None);
        let data_1 = unwrap!(data.get(1).and_then(|x| x.as_f64()).map(|x| x as i32); None);
        let data_2 = unwrap!(data.get(2).and_then(|x| x.as_f64()).map(|x| x as i32); None);

        Some(Box::new([data_0, data_1, data_2]))
    }
}

#[async_trait(?Send)]
impl Pack for chrono::DateTime<chrono::Utc> {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(self.to_rfc3339())
    }

    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        let data = unwrap!(data.as_string(); None);

        chrono::DateTime::parse_from_rfc3339(data.as_str())
            .ok()
            .map(|x| x.with_timezone(&chrono::Utc))
            .map(Box::new)
    }
}

#[async_trait(?Send)]
impl Pack for crate::libs::color::Pallet {
    async fn pack(&self, _: PackDepth) -> JsValue {
        self.to_jsvalue()
    }

    async fn unpack(data: &JsValue, _arena: ArenaMut) -> Option<Box<Self>> {
        Self::from_jsvalue(data).map(Box::new)
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Vec<T> {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let list = js_sys::Array::new();

        for item in self {
            list.push(&item.pack(pack_depth).await);
        }

        list.into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(data).to_vec();
        let mut this = vec![];

        for item in data {
            if let Some(item) = T::unpack(&item, ArenaMut::clone(&arena)).await {
                this.push(*item);
            }
        }

        Some(Box::new(this))
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Rc<T> {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        self.pack(pack_depth).await
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = T::unpack(data, arena).await;

        if let Some(data) = data {
            Some(Box::new(Rc::new(*data)))
        } else {
            None
        }
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Rc<RefCell<T>> {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        self.borrow().pack(pack_depth).await
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = T::unpack(data, arena).await;

        if let Some(data) = data {
            Some(Box::new(Rc::new(RefCell::new(*data))))
        } else {
            None
        }
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for Option<T> {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        match self {
            Some(x) => x.pack(pack_depth).await,
            None => JsValue::null(),
        }
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        if data.is_null() {
            return Some(Box::new(None));
        }

        let data = T::unpack(data, arena).await;

        if let Some(data) = data {
            Some(Box::new(Some(*data)))
        } else {
            None
        }
    }
}

#[async_trait(?Send)]
impl<T: Pack, U: Pack> Pack for (T, U) {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        array![self.0.pack(pack_depth).await, self.1.pack(pack_depth).await].into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(data).to_vec();
        if let Some(data) = join_some!(data.get(0), data.get(1)) {
            let data_0 = T::unpack(&data.0, ArenaMut::clone(&arena)).await;
            let data_1 = U::unpack(&data.1, ArenaMut::clone(&arena)).await;

            if let Some((data_0, data_1)) = join_some!(data_0, data_1) {
                return Some(Box::new((*data_0, *data_1)));
            }
        }
        None
    }
}

#[async_trait(?Send)]
impl Pack for regex::Regex {
    async fn pack(&self, _: PackDepth) -> JsValue {
        JsValue::from(self.as_str())
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        data.as_string()
            .and_then(|data| regex::Regex::new(data.as_str()).ok())
            .map(|x| Box::new(x))
    }
}

#[async_trait(?Send)]
impl<T: Pack> Pack for crate::libs::select_list::SelectList<T> {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let data = js_sys::Array::new();

        for item in self.iter() {
            data.push(&item.pack(pack_depth).await);
        }

        (object! {
            "selected": self.selected_idx(),
            "data": data
        })
        .into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        if let Some(data) = data.dyn_ref::<Object>() {
            let selected = data
                .get("selected")
                .and_then(|x| x.as_f64())
                .map(|x| x as usize);
            if let Some((selected, data)) = join_some!(selected, data.get("data")) {
                let data = js_sys::Array::from(&data).to_vec();
                let mut payload = vec![];

                for item in data {
                    let item = T::unpack(&item, ArenaMut::clone(&arena)).await;
                    if let Some(item) = item {
                        payload.push(*item);
                    }
                }

                return Some(Box::new(Self::new(payload, selected)));
            }
        }
        None
    }
}

#[async_trait(?Send)]
impl<K: Pack + Eq + std::hash::Hash, T: Pack> Pack for std::collections::HashMap<K, T> {
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        let data = js_sys::Array::new();

        for (key, value) in self {
            let key = key.pack(pack_depth).await;
            let value = value.pack(pack_depth).await;
            data.push(array![&key, &value].as_ref());
        }

        data.into()
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = js_sys::Array::from(data).to_vec();
        let mut this = std::collections::HashMap::new();

        for item in data {
            let item = js_sys::Array::from(&item).to_vec();
            if let Some((key, value)) = join_some!(item.get(0), item.get(1)) {
                let key = K::unpack(&key, ArenaMut::clone(&arena)).await;
                let value = T::unpack(&value, ArenaMut::clone(&arena)).await;
                if let Some((key, value)) = join_some!(key, value) {
                    this.insert(*key, *value);
                }
            }
        }

        Some(Box::new(this))
    }
}
