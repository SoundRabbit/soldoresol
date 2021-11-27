use wasm_bindgen::prelude::*;
pub mod color_system;
pub mod pallet;
pub use pallet::Pallet;

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn to_u8array(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    pub fn to_f32array(&self) -> [f32; 4] {
        [
            (self.red as f32 / 255.0).min(1.0).max(0.0),
            (self.green as f32 / 255.0).min(1.0).max(0.0),
            (self.blue as f32 / 255.0).min(1.0).max(0.0),
            (self.alpha as f32 / 100.0).min(1.0).max(0.0),
        ]
    }

    pub fn to_u32(&self) -> u32 {
        u32::from_be_bytes([self.alpha, self.red, self.green, self.blue])
    }

    pub fn to_string(&self) -> String {
        let str = String::from("rgba(");
        let str = str + &self.red.to_string() + ",";
        let str = str + &self.green.to_string() + ",";
        let str = str + &self.blue.to_string() + ",";
        let str = str + &(self.alpha as f64 / 100.0).to_string() + ")";
        str
    }

    pub fn to_jsvalue(&self) -> JsValue {
        JsValue::from(&self.to_string())
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self {
            red: rgba[0],
            green: rgba[1],
            blue: rgba[2],
            alpha: rgba[3],
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Self {
            red: (rgba[0] * 255.0) as u8,
            green: (rgba[1] * 255.0) as u8,
            blue: (rgba[2] * 255.0) as u8,
            alpha: (rgba[3] * 100.0) as u8,
        }
    }
}

impl From<u32> for Color {
    fn from(argb: u32) -> Self {
        let argb = argb.to_be_bytes();
        Self {
            red: argb[1],
            green: argb[2],
            blue: argb[3],
            alpha: argb[0],
        }
    }
}
