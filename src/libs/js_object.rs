use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object, js_name=Object)]
    pub type JsObject;

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &JsObject, name: &str) -> Option<JsObject>;

    #[wasm_bindgen(method, indexing_setter)]
    pub fn set(this: &JsObject, name: &str, value: &JsValue);
}

macro_rules! object {
    { $( $n:ident : $v:expr ),* } => {
        {
            #[allow(unused_imports)]
            use wasm_bindgen::{prelude::*, JsCast};
            use crate::libs::js_object::JsObject;

            let tmp = js_sys::Object::new().dyn_into::<JsObject>().unwrap();
            $(
                tmp.set(stringify!($n), &JsValue::from($v));
            )*
            tmp
        }
    };
}

macro_rules! array {
    [ $( $x:expr ),* ] => {
        {
            #[allow(unused_imports)]
            use wasm_bindgen::prelude::*;
            use js_sys::Array;

            let tmp = Array::new();
            $(
                tmp.push(&JsValue::from($x));
            )*
            tmp
        }
    };
}
