uses! {
    super::BlockMut;
    super::util::Pack;
    crate::libs::color::Pallet;
}

#[derive(Clone, Copy)]
pub enum Shape {
    Cube,
    Cyliner,
    Sphere,
}

#[async_trait(?Send)]
impl Pack for Shape {
    async fn pack(&self, _: bool) -> JsValue {
        match self {
            Self::Cube => JsValue::from("Cube"),
            Self::Cyliner => JsValue::from("Cyliner"),
            Self::Sphere => JsValue::from("Sphere"),
        }
    }
}

packable! {
    [pub Boxblock]
    size: [f64; 3] = [1.0, 1.0, 1.0];
    position: [f64; 3] = [0.0, 0.0, 0.0];
    shape: Shape = Shape::Cube;
    color: Pallet = Pallet::blue(5);
}

impl Boxblock {
    pub fn size(&self) -> &[f64; 3] {
        &self.size
    }

    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn color(&self) -> &Pallet {
        &self.color
    }
}
