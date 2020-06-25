use super::Block;
use super::BlockId;
use wasm_bindgen::prelude::*;

pub mod item;
pub mod tab;

pub use item::Item;
pub use tab::Tab;

pub struct Chat {
    tabs: Vec<BlockId>,
}

impl Chat {
    pub fn new(tabs: Vec<BlockId>) -> Self {
        Self { tabs: tabs }
    }
}

impl Block for Chat {
    fn pack(&self, resolve: impl FnOnce(JsValue) + 'static) {
        let val = js_sys::Array::new();
        for tab_id in &self.tabs {
            val.push(&JsValue::from(tab_id.to_string()));
        }
        resolve(val.into());
    }
    fn unpack(val: JsValue, resolve: impl FnOnce(Option<Box<Self>>) + 'static) {
        let val = js_sys::Array::from(&val).to_vec();
        let mut tabs = vec![];
        for tab in val {
            if let Some(tab_id) = tab.as_string().and_then(|x| x.parse().ok()) {
                tabs.push(tab_id);
            }
        }
        let chat = Self { tabs };
        resolve(Some(Box::new(chat)))
    }
}
