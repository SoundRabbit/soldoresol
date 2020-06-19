use super::ChatItem;
use crate::JsObject;
use js_sys::Array;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::JsCast;

pub struct ChatTab {
    name: String,
    items: Vec<ChatItem>,
}

impl ChatTab {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn as_object(&self) -> JsObject {
        let items = Array::new();

        for item in &self.items {
            items.push(&item.as_object());
        }

        object! {
            name: &self.name,
            items: items
        }
    }
}

impl Deref for ChatTab {
    type Target = Vec<ChatItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ChatTab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl From<JsObject> for ChatTab {
    fn from(object: JsObject) -> Self {
        let name = object
            .get("name")
            .and_then(|x| x.as_string())
            .unwrap_or(String::from(""));
        let js_items = object
            .get("items")
            .and_then(|x| x.dyn_into::<Array>().ok())
            .unwrap_or(Array::new());
        let mut items = vec![];

        for i in 0..js_items.length() {
            if let Ok(item) = js_items.get(i).dyn_into::<JsObject>() {
                items.push(ChatItem::from(item));
            }
        }

        Self { name, items }
    }
}
