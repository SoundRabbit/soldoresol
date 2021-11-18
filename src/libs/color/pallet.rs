use wasm_bindgen::prelude::*;

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
            (self.kind.name()): array![self.idx as f64, self.alpha as f64]
        };
        let object: js_sys::Object = object.into();
        object.into()
    }
}

impl std::fmt::Display for Pallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_color())
    }
}
