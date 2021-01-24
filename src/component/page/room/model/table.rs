use super::super::super::molecule::color_pallet::Pallet;
use std::rc::Rc;

pub enum TableTool {
    Hr(Rc<String>),
    Selector,
    Pen(PenTool),
    Shape,
    Eraser,
}

impl TableTool {
    pub fn clone(this: &Self) -> Self {
        match this {
            Self::Hr(x) => Self::Hr(Rc::clone(x)),
            Self::Selector => Self::Selector,
            Self::Pen(x) => Self::Pen(x.clone()),
            Self::Shape => Self::Shape,
            Self::Eraser => Self::Eraser,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Eraser => "消しゴム",
            Self::Hr(..) => "",
            Self::Pen(..) => "ペン",
            Self::Selector => "選択",
            Self::Shape => "図形",
        }
    }
}

#[derive(Clone)]
pub struct PenTool {
    pub line_width: f64,
    pub pallet: Pallet,
    pub alpha: u8,
}
