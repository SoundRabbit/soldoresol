use super::ChatTab;
use crate::JsObject;
use js_sys::Array;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::{prelude::*, JsCast};

pub struct Chat(Vec<ChatTab>);

impl Chat {
    pub fn new(tabs: Vec<ChatTab>) -> Self {
        Self(tabs)
    }

    pub fn as_object(&self) -> JsObject {
        let tabs = Array::new();

        for tab in &self.0 {
            let tab: JsValue = tab.as_object().into();
            tabs.push(&tab);
        }

        tabs.dyn_into::<JsObject>().unwrap()
    }
}

impl Deref for Chat {
    type Target = Vec<ChatTab>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Chat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<JsObject> for Chat {
    fn from(object: JsObject) -> Self {
        let js_tabs = Array::from(&object);
        let mut tabs = vec![];

        for i in 0..js_tabs.length() {
            if let Ok(tab) = js_tabs.get(i).dyn_into::<JsObject>() {
                tabs.push(ChatTab::from(tab));
            }
        }

        Self(tabs)
    }
}
