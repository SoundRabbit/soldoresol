use crate::libs::js_object::Object;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pallet {
    pub alpha: u8,
    pub idx: usize,
    pub kind: Kind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Gray,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

impl Kind {
    fn name(&self) -> &'static str {
        match self {
            Self::Gray => "gray",
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::Pink => "pink",
        }
    }
}

macro_rules! pallet_constructor {
    ($name:ident -> $kind:ident) => {
        pub fn $name(idx: usize) -> Self {
            Self {
                alpha: 100,
                idx: idx,
                kind: Kind::$kind,
            }
        }
    };
}

macro_rules! color_of {
    ($color:ident; $this:expr) => {
        super::color_system::$color($this.alpha, $this.idx)
    };
}

impl Pallet {
    pallet_constructor!(gray -> Gray);
    pallet_constructor!(red -> Red);
    pallet_constructor!(orange -> Orange);
    pallet_constructor!(yellow -> Yellow);
    pallet_constructor!(green -> Green);
    pallet_constructor!(blue -> Blue);
    pallet_constructor!(purple -> Purple);
    pallet_constructor!(pink -> Pink);

    pub fn a(mut self, alpha: u8) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn with_a(&self, alpha: u8) -> Self {
        self.clone().a(alpha)
    }

    pub fn to_color(&self) -> super::Color {
        match &self.kind {
            Kind::Gray => color_of!(gray; self),
            Kind::Red => color_of!(red; self),
            Kind::Orange => color_of!(orange; self),
            Kind::Yellow => color_of!(yellow; self),
            Kind::Green => color_of!(green; self),
            Kind::Blue => color_of!(blue; self),
            Kind::Purple => color_of!(purple; self),
            Kind::Pink => color_of!(pink; self),
        }
    }

    #[allow(unused_parens)]
    pub fn to_jsvalue(&self) -> JsValue {
        let object = object! {
            "_tag": self.kind.name(),
            "_val": array![self.idx as f64, self.alpha as f64]
        };
        let object: js_sys::Object = object.into();
        object.into()
    }

    pub fn from_jsvalue(data: &JsValue) -> Option<Self> {
        let data = unwrap!(data.dyn_ref::<Object>(); None);
        let kind = unwrap!(data.get("_tag").and_then(|x| x.as_string()); None);
        let (idx, alpha) = {
            let val = unwrap!(data.get("_val").map(|x| js_sys::Array::from(&x).to_vec()); None);
            let val_0 = unwrap!(val.get(0).and_then(|x| x.as_f64()); None);
            let val_1 = unwrap!(val.get(1).and_then(|x| x.as_f64()); None);
            (val_0 as usize, val_1 as u8)
        };

        let kind = match kind.as_str() {
            "blue" => Kind::Blue,
            "gray" => Kind::Gray,
            "green" => Kind::Green,
            "orange" => Kind::Orange,
            "pink" => Kind::Pink,
            "purple" => Kind::Purple,
            "red" => Kind::Red,
            "yellow" => Kind::Yellow,
            _ => {
                return None;
            }
        };

        Some(Self { kind, idx, alpha })
    }
}

impl std::fmt::Display for Pallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_color())
    }
}
