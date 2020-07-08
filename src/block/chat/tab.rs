use super::{Block, BlockId, Field};
use crate::Promise;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Tab {
    name: String,
    items: BTreeMap<OrderedFloat<f64>, BlockId>,
}

impl Tab {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: BTreeMap::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn insert(&mut self, timestamp: f64, item: BlockId) {
        self.items.insert(OrderedFloat(timestamp), item);
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (f64, &BlockId)> + DoubleEndedIterator {
        self.items.iter().map(|(t, b)| (t.clone().into(), b))
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Block for Tab {
    fn pack(&self) -> Promise<JsValue> {
        let data = object! {};
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        unimplemented!();
    }
}
