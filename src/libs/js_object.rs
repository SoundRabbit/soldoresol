use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object, js_name=Object)]
    pub type Object;

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &Object, name: &str) -> Option<Object>;

    #[wasm_bindgen(method, indexing_setter)]
    pub fn set(this: &Object, name: &str, value: &JsValue);
}

macro_rules! object {
    { $( $n:tt : $v:expr ),*$(,)? } => {
        {
            #[allow(unused_imports)]
            use wasm_bindgen::{prelude::*, JsCast};
            use crate::libs::js_object::Object;

            let tmp = js_sys::Object::new().dyn_into::<Object>().unwrap();
            $(
                tmp.set($n, &JsValue::from($v));
            )*
            tmp
        }
    };
}

macro_rules! array {
    [ $( $x:expr ),*$(,)? ] => {
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
