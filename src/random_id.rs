use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub fn u8vec(len: usize) -> Vec<u8> {
    let mut id: Vec<u8> = Vec::new();
    id.resize(len, 0);
    web_sys::window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut id)
        .expect("");
    id
}

pub fn base64url() -> String {
    base64::encode(&u8vec(3 * 6))
        .replace("+", r"@")
        .replace("/", r"#")
}

pub fn u128val() -> u128 {
    let mut buf = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let id = u8vec(16);
    for i in 0..16 {
        buf[i] = id[i];
    }
    u128::from_be_bytes(buf)
}

pub fn u32val() -> u32 {
    let mut buf = [0, 0, 0, 0];
    let id = u8vec(4);
    for i in 0..4 {
        buf[i] = id[i];
    }
    u32::from_be_bytes(buf)
}

#[allow(dead_code)]
pub fn u32color() -> u32 {
    let mut buf = [255, 255, 255, 255];
    let id = u8vec(3);
    for i in 0..3 {
        buf[i + 1] = id[i];
    }
    u32::from_be_bytes(buf)
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct U128Id(Rc<u128>);

impl U128Id {
    pub fn new() -> Self {
        Self(Rc::new(u128val()))
    }

    pub fn to_jsvalue(&self) -> JsValue {
        JsValue::from(self.0.to_string())
    }

    pub fn from_jsvalue(val: &JsValue) -> Option<Self> {
        if let Some(val) = val.as_string().and_then(|x| x.parse().ok()) {
            Some(Self(Rc::new(val)))
        } else {
            None
        }
    }

    pub fn to_u128(&self) -> u128 {
        self.0.as_ref().clone()
    }

    pub fn from_u128(val: u128) -> Self {
        Self(Rc::new(val))
    }
}

impl Clone for U128Id {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
