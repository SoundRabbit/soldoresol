use super::{Block, BlockId, Field};
use crate::{random_id::U128Id, resource::ResourceId, JsObject, Promise};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::collections::HashSet;
use wasm_bindgen::{prelude::*, JsCast};

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
        let items = array![];
        for (time, item) in &self.items {
            let time: f64 = time.clone().into();
            items.push(array![time, item.to_jsvalue()].as_ref());
        }

        let data = object! {
            name: &self.name,
            items: items
        };
        let data: js_sys::Object = data.into();
        let data: JsValue = data.into();
        Promise::new(|resolve| resolve(Some(data)))
    }
    fn unpack(field: &mut Field, val: JsValue) -> Promise<Box<Self>> {
        let self_ = if let Ok(val) = val.dyn_into::<JsObject>() {
            let name = val.get("name").and_then(|name| name.as_string());
            let items = val.get("items").map(|items| {
                let items: js_sys::Object = items.into();
                js_sys::Array::from(&items)
            });
            if let (Some(name), Some(raw_items)) = (name, items) {
                let mut items = BTreeMap::new();
                for item in raw_items.to_vec() {
                    let item = js_sys::Array::from(&item);
                    if let (Some(time), Some(id)) = (
                        item.get(0).as_f64().map(|t| OrderedFloat(t)),
                        U128Id::from_jsvalue(&item.get(1)).map(|id| field.block_id(id)),
                    ) {
                        items.insert(time, id);
                    }
                }

                Some(Box::new(Self { name, items }))
            } else {
                None
            }
        } else {
            None
        };
        Promise::new(move |resolve| resolve(self_))
    }
    fn dependents(&self, field: &Field) -> HashSet<BlockId> {
        let mut deps = set! {};

        for (_, block_id) in &self.items {
            if let Some(block) = field.get::<super::Item>(block_id) {
                let block_deps = block.dependents(field);
                for block_dep in block_deps {
                    deps.insert(block_dep);
                }
                deps.insert(block_id.clone());
            }
        }

        deps
    }

    fn resources(&self, field: &Field) -> HashSet<ResourceId> {
        let mut reses = set! {};

        for (_, block_id) in &self.items {
            if let Some(block) = field.get::<super::Item>(block_id) {
                let block_reses = block.resources(field);
                for block_res in block_reses {
                    reses.insert(block_res);
                }
            }
        }

        reses
    }
}
