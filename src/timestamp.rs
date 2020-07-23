use ordered_float::OrderedFloat;

pub struct Timestamp(OrderedFloat<f64>);

impl Timestamp {
    pub fn to_string(&self) -> String {
        js_sys::Date::new(&wasm_bindgen::JsValue::from(self.to_f64()))
            .to_locale_string("ja-JP", object! {}.as_ref())
            .as_string()
            .unwrap_or(String::from(""))
    }

    pub fn to_f64(&self) -> f64 {
        self.0.clone().into()
    }
}

impl From<f64> for Timestamp {
    fn from(t: f64) -> Self {
        Self(OrderedFloat(t))
    }
}
