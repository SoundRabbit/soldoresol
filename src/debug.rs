use wasm_bindgen::prelude::*;

pub fn log_1<T>(x: T)
where
    JsValue: From<T>,
{
    web_sys::console::log_1(&JsValue::from(x));
}

pub fn log_2<T, U>(x1: T, x2: U)
where
    JsValue: From<T>,
    JsValue: From<U>,
{
    web_sys::console::log_2(&JsValue::from(x1), &JsValue::from(x2));
}
