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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct U128Id(u32, u32, u32, u32);

impl U128Id {
    pub fn new() -> Self {
        Self(u32val(), u32val(), u32val(), u32val())
    }

    pub fn to_jsvalue(&self) -> JsValue {
        array![self.0, self.1, self.2, self.3].into()
    }

    pub fn from_jsvalue(val: &JsValue) -> Option<Self> {
        let buf = js_sys::Array::from(val);
        let a = buf.get(0).as_f64();
        let b = buf.get(1).as_f64();
        let c = buf.get(0).as_f64();
        let d = buf.get(1).as_f64();
        if let (Some(a), Some(b), Some(c), Some(d)) = (a, b, c, d) {
            Some(Self(a as u32, b as u32, c as u32, d as u32))
        } else {
            None
        }
    }
}
